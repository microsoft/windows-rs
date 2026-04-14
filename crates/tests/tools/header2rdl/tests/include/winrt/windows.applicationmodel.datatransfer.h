
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
#ifndef __windows2Eapplicationmodel2Edatatransfer_h__
#define __windows2Eapplicationmodel2Edatatransfer_h__
#ifndef __windows2Eapplicationmodel2Edatatransfer_p_h__
#define __windows2Eapplicationmodel2Edatatransfer_p_h__


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

#if !defined(WINDOWS_SECURITY_ENTERPRISEDATA_ENTERPRISEDATACONTRACT_VERSION)
#define WINDOWS_SECURITY_ENTERPRISEDATA_ENTERPRISEDATACONTRACT_VERSION 0x50000
#endif // defined(WINDOWS_SECURITY_ENTERPRISEDATA_ENTERPRISEDATACONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Security.EnterpriseData.h"
#include "Windows.Storage.h"
#include "Windows.Storage.Streams.h"
#include "Windows.UI.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataProviderHandler;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler ABI::Windows::ApplicationModel::DataTransfer::IDataProviderHandler

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IShareProviderHandler;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler ABI::Windows::ApplicationModel::DataTransfer::IShareProviderHandler

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IClipboardContentOptions;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions ABI::Windows::ApplicationModel::DataTransfer::IClipboardContentOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IClipboardHistoryChangedEventArgs;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs ABI::Windows::ApplicationModel::DataTransfer::IClipboardHistoryChangedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IClipboardHistoryItem;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem ABI::Windows::ApplicationModel::DataTransfer::IClipboardHistoryItem

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IClipboardHistoryItemsResult;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult ABI::Windows::ApplicationModel::DataTransfer::IClipboardHistoryItemsResult

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IClipboardStatics;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics ABI::Windows::ApplicationModel::DataTransfer::IClipboardStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IClipboardStatics2;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2 ABI::Windows::ApplicationModel::DataTransfer::IClipboardStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackage;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage ABI::Windows::ApplicationModel::DataTransfer::IDataPackage

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackage2;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2 ABI::Windows::ApplicationModel::DataTransfer::IDataPackage2

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackage3;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3 ABI::Windows::ApplicationModel::DataTransfer::IDataPackage3

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackage4;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4 ABI::Windows::ApplicationModel::DataTransfer::IDataPackage4

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackagePropertySet;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet ABI::Windows::ApplicationModel::DataTransfer::IDataPackagePropertySet

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackagePropertySet2;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2 ABI::Windows::ApplicationModel::DataTransfer::IDataPackagePropertySet2

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackagePropertySet3;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3 ABI::Windows::ApplicationModel::DataTransfer::IDataPackagePropertySet3

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackagePropertySet4;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4 ABI::Windows::ApplicationModel::DataTransfer::IDataPackagePropertySet4

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackagePropertySetView;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView ABI::Windows::ApplicationModel::DataTransfer::IDataPackagePropertySetView

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackagePropertySetView2;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2 ABI::Windows::ApplicationModel::DataTransfer::IDataPackagePropertySetView2

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackagePropertySetView3;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3 ABI::Windows::ApplicationModel::DataTransfer::IDataPackagePropertySetView3

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackagePropertySetView4;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4 ABI::Windows::ApplicationModel::DataTransfer::IDataPackagePropertySetView4

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackagePropertySetView5;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5 ABI::Windows::ApplicationModel::DataTransfer::IDataPackagePropertySetView5

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackageView;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView ABI::Windows::ApplicationModel::DataTransfer::IDataPackageView

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackageView2;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2 ABI::Windows::ApplicationModel::DataTransfer::IDataPackageView2

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackageView3;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3 ABI::Windows::ApplicationModel::DataTransfer::IDataPackageView3

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackageView4;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4 ABI::Windows::ApplicationModel::DataTransfer::IDataPackageView4

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataProviderDeferral;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral ABI::Windows::ApplicationModel::DataTransfer::IDataProviderDeferral

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataProviderRequest;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest ABI::Windows::ApplicationModel::DataTransfer::IDataProviderRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataRequest;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest ABI::Windows::ApplicationModel::DataTransfer::IDataRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataRequestDeferral;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral ABI::Windows::ApplicationModel::DataTransfer::IDataRequestDeferral

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataRequestedEventArgs;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs ABI::Windows::ApplicationModel::DataTransfer::IDataRequestedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataTransferManager;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager ABI::Windows::ApplicationModel::DataTransfer::IDataTransferManager

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataTransferManager2;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2 ABI::Windows::ApplicationModel::DataTransfer::IDataTransferManager2

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataTransferManagerStatics;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics ABI::Windows::ApplicationModel::DataTransfer::IDataTransferManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataTransferManagerStatics2;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2 ABI::Windows::ApplicationModel::DataTransfer::IDataTransferManagerStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataTransferManagerStatics3;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3 ABI::Windows::ApplicationModel::DataTransfer::IDataTransferManagerStatics3

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IHtmlFormatHelperStatics;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics ABI::Windows::ApplicationModel::DataTransfer::IHtmlFormatHelperStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IOperationCompletedEventArgs;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs ABI::Windows::ApplicationModel::DataTransfer::IOperationCompletedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IOperationCompletedEventArgs2;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2 ABI::Windows::ApplicationModel::DataTransfer::IOperationCompletedEventArgs2

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IShareCompletedEventArgs;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs ABI::Windows::ApplicationModel::DataTransfer::IShareCompletedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IShareProvider;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider ABI::Windows::ApplicationModel::DataTransfer::IShareProvider

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IShareProviderFactory;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory ABI::Windows::ApplicationModel::DataTransfer::IShareProviderFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IShareProviderOperation;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation ABI::Windows::ApplicationModel::DataTransfer::IShareProviderOperation

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IShareProvidersRequestedEventArgs;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs ABI::Windows::ApplicationModel::DataTransfer::IShareProvidersRequestedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IShareTargetInfo;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo ABI::Windows::ApplicationModel::DataTransfer::IShareTargetInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IShareUIOptions;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions ABI::Windows::ApplicationModel::DataTransfer::IShareUIOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface ISharedStorageAccessManagerStatics;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics ABI::Windows::ApplicationModel::DataTransfer::ISharedStorageAccessManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IStandardDataFormatsStatics;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics ABI::Windows::ApplicationModel::DataTransfer::IStandardDataFormatsStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IStandardDataFormatsStatics2;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2 ABI::Windows::ApplicationModel::DataTransfer::IStandardDataFormatsStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IStandardDataFormatsStatics3;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3 ABI::Windows::ApplicationModel::DataTransfer::IStandardDataFormatsStatics3

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface ITargetApplicationChosenEventArgs;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs ABI::Windows::ApplicationModel::DataTransfer::ITargetApplicationChosenEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface ITransferTarget;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget ABI::Windows::ApplicationModel::DataTransfer::ITransferTarget

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface ITransferTargetChangedEventArgs;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs ABI::Windows::ApplicationModel::DataTransfer::ITransferTargetChangedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface ITransferTargetDiscoveryOptions;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions ABI::Windows::ApplicationModel::DataTransfer::ITransferTargetDiscoveryOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface ITransferTargetDiscoveryOptionsFactory;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory ABI::Windows::ApplicationModel::DataTransfer::ITransferTargetDiscoveryOptionsFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface ITransferTargetInvokeResult;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult ABI::Windows::ApplicationModel::DataTransfer::ITransferTargetInvokeResult

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface ITransferTargetStatics;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics ABI::Windows::ApplicationModel::DataTransfer::ITransferTargetStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface ITransferTargetWatcher;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher ABI::Windows::ApplicationModel::DataTransfer::ITransferTargetWatcher

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface ITransferTargetWatcherStatics;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics ABI::Windows::ApplicationModel::DataTransfer::ITransferTargetWatcherStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIAsyncOperation_1_IInspectable_USE
#define DEF___FIAsyncOperation_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("abf53c57-ee50-5342-b52a-26e3b8cc024f"))
IAsyncOperation<IInspectable*> : IAsyncOperation_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<IInspectable*> __FIAsyncOperation_1_IInspectable_t;
#define __FIAsyncOperation_1_IInspectable ABI::Windows::Foundation::__FIAsyncOperation_1_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_IInspectable_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_IInspectable_USE
#define DEF___FIAsyncOperationCompletedHandler_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3f08262e-a2e1-5134-9297-e9211f481a2d"))
IAsyncOperationCompletedHandler<IInspectable*> : IAsyncOperationCompletedHandler_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<IInspectable*> __FIAsyncOperationCompletedHandler_1_IInspectable_t;
#define __FIAsyncOperationCompletedHandler_1_IInspectable ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_IInspectable_USE */



#ifndef DEF___FIAsyncOperation_1_HSTRING_USE
#define DEF___FIAsyncOperation_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3e1fe603-f897-5263-b328-0806426b8a79"))
IAsyncOperation<HSTRING> : IAsyncOperation_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<HSTRING> __FIAsyncOperation_1_HSTRING_t;
#define __FIAsyncOperation_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperation_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#define DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b79a741f-7fb5-50ae-9e99-911201ec3d41"))
IAsyncOperationCompletedHandler<HSTRING> : IAsyncOperationCompletedHandler_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<HSTRING> __FIAsyncOperationCompletedHandler_1_HSTRING_t;
#define __FIAsyncOperationCompletedHandler_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class ClipboardHistoryItemsResult;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("677b06b8-0134-5692-b487-4c8e2408ca01"))
IAsyncOperation<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryItemsResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryItemsResult*, ABI::Windows::ApplicationModel::DataTransfer::IClipboardHistoryItemsResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.DataTransfer.ClipboardHistoryItemsResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryItemsResult*> __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("841da82d-a32c-5997-8450-f54af1d5477e"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryItemsResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryItemsResult*, ABI::Windows::ApplicationModel::DataTransfer::IClipboardHistoryItemsResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.DataTransfer.ClipboardHistoryItemsResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryItemsResult*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e5195792-aeab-56e8-bd30-1372c4340bf6"))
IKeyValuePair<HSTRING, ABI::Windows::Storage::Streams::RandomAccessStreamReference*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::Streams::RandomAccessStreamReference*, ABI::Windows::Storage::Streams::IRandomAccessStreamReference*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Storage.Streams.RandomAccessStreamReference>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::Storage::Streams::RandomAccessStreamReference*> __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9419af53-acb8-5328-8853-70ba87eb6ad5"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Storage.Streams.RandomAccessStreamReference>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c9729ba7-5e20-569d-a3d1-97a4e653e5bb"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Storage.Streams.RandomAccessStreamReference>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#define DEF___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0a4ce7a5-dfe0-5796-a438-effdfaa31f1b"))
IMapView<HSTRING, ABI::Windows::Storage::Streams::RandomAccessStreamReference*> : IMapView_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::Streams::RandomAccessStreamReference*, ABI::Windows::Storage::Streams::IRandomAccessStreamReference*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.Storage.Streams.RandomAccessStreamReference>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::Storage::Streams::RandomAccessStreamReference*> __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_t;
#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#define DEF___FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fc012d44-2dcf-5162-be9a-7668675aa590"))
IAsyncOperation<__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference*> : IAsyncOperation_impl<__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IMapView`2<String, Windows.Storage.Streams.RandomAccessStreamReference>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference*> __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_t;
#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference ABI::Windows::Foundation::__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d4cb6b80-821a-5a7b-898d-d58917b31a36"))
IAsyncOperationCompletedHandler<__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference*> : IAsyncOperationCompletedHandler_impl<__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IMapView`2<String, Windows.Storage.Streams.RandomAccessStreamReference>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference*> __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_t;
#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageItem;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageItem ABI::Windows::Storage::IStorageItem

#endif // ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CStorage__CIStorageItem_USE
#define DEF___FIIterator_1_Windows__CStorage__CIStorageItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("05b487c2-3830-5d3c-98da-25fa11542dbd"))
IIterator<ABI::Windows::Storage::IStorageItem*> : IIterator_impl<ABI::Windows::Storage::IStorageItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Storage.IStorageItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Storage::IStorageItem*> __FIIterator_1_Windows__CStorage__CIStorageItem_t;
#define __FIIterator_1_Windows__CStorage__CIStorageItem ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CStorage__CIStorageItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CStorage__CIStorageItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CStorage__CIStorageItem_USE
#define DEF___FIIterable_1_Windows__CStorage__CIStorageItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bb8b8418-65d1-544b-b083-6d172f568c73"))
IIterable<ABI::Windows::Storage::IStorageItem*> : IIterable_impl<ABI::Windows::Storage::IStorageItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Storage.IStorageItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Storage::IStorageItem*> __FIIterable_1_Windows__CStorage__CIStorageItem_t;
#define __FIIterable_1_Windows__CStorage__CIStorageItem ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CStorage__CIStorageItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CStorage__CIStorageItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CStorage__CIStorageItem_USE
#define DEF___FIVectorView_1_Windows__CStorage__CIStorageItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("85575a41-06cb-58d0-b98a-7c8f06e6e9d7"))
IVectorView<ABI::Windows::Storage::IStorageItem*> : IVectorView_impl<ABI::Windows::Storage::IStorageItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Storage.IStorageItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Storage::IStorageItem*> __FIVectorView_1_Windows__CStorage__CIStorageItem_t;
#define __FIVectorView_1_Windows__CStorage__CIStorageItem ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CStorage__CIStorageItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CStorage__CIStorageItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4b1c0fd7-7a01-5e7a-a6fe-be4500283f23"))
IAsyncOperation<__FIVectorView_1_Windows__CStorage__CIStorageItem*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CStorage__CIStorageItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.IStorageItem>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CStorage__CIStorageItem*> __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("51436e75-ace1-5a68-b260-f843b846f0db"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CIStorageItem*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CStorage__CIStorageItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.IStorageItem>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CIStorageItem*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_USE */

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

#ifndef DEF___FIAsyncOperation_1_Windows__CFoundation__CUri_USE
#define DEF___FIAsyncOperation_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("641cb9dd-a28d-59e2-b8db-a227eda6cf2e"))
IAsyncOperation<ABI::Windows::Foundation::Uri*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Foundation::Uri*> __FIAsyncOperation_1_Windows__CFoundation__CUri_t;
#define __FIAsyncOperation_1_Windows__CFoundation__CUri ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CFoundation__CUri_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ad46f1cc-2bb0-585c-9885-03c2780d4d58"))
IAsyncOperationCompletedHandler<ABI::Windows::Foundation::Uri*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Foundation::Uri*> __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace EnterpriseData {
                typedef enum ProtectionPolicyEvaluationResult : int ProtectionPolicyEvaluationResult;
            } /* EnterpriseData */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e8d81715-c56c-5a6b-b738-5df6c2775b7b"))
IAsyncOperation<enum ABI::Windows::Security::EnterpriseData::ProtectionPolicyEvaluationResult> : IAsyncOperation_impl<enum ABI::Windows::Security::EnterpriseData::ProtectionPolicyEvaluationResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.EnterpriseData.ProtectionPolicyEvaluationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Security::EnterpriseData::ProtectionPolicyEvaluationResult> __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2833ba54-a4e1-5c2d-8a7a-136e8510c78b"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Security::EnterpriseData::ProtectionPolicyEvaluationResult> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Security::EnterpriseData::ProtectionPolicyEvaluationResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.EnterpriseData.ProtectionPolicyEvaluationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Security::EnterpriseData::ProtectionPolicyEvaluationResult> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageFile;
        } /* Storage */
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5e52f8ce-aced-5a42-95b4-f674dd84885e"))
IAsyncOperation<ABI::Windows::Storage::StorageFile*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFile*, ABI::Windows::Storage::IStorageFile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.StorageFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::StorageFile*> __FIAsyncOperation_1_Windows__CStorage__CStorageFile_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStorageFile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e521c894-2c26-5946-9e61-2b5e188d01ed"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageFile*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFile*, ABI::Windows::Storage::IStorageFile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.StorageFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageFile*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d90442ca-543c-504b-9eb9-294bcad8a283"))
IAsyncOperation<ABI::Windows::Storage::Streams::RandomAccessStreamReference*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::Streams::RandomAccessStreamReference*, ABI::Windows::Storage::Streams::IRandomAccessStreamReference*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.RandomAccessStreamReference>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::RandomAccessStreamReference*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3d203732-ded7-5d32-87e6-c179781f791f"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::RandomAccessStreamReference*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::Streams::RandomAccessStreamReference*, ABI::Windows::Storage::Streams::IRandomAccessStreamReference*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.RandomAccessStreamReference>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::RandomAccessStreamReference*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class TransferTargetInvokeResult;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b3a0b269-36e0-5c7e-a779-f7c4ae4fba85"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::ApplicationModel::DataTransfer::TransferTargetInvokeResult*, double> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::TransferTargetInvokeResult*, ABI::Windows::ApplicationModel::DataTransfer::ITransferTargetInvokeResult*>, double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.ApplicationModel.DataTransfer.TransferTargetInvokeResult, Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::ApplicationModel::DataTransfer::TransferTargetInvokeResult*, double> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1822bca9-47a2-58ad-8c40-054c4b5aaf2d"))
IAsyncOperationWithProgress<ABI::Windows::ApplicationModel::DataTransfer::TransferTargetInvokeResult*, double> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::TransferTargetInvokeResult*, ABI::Windows::ApplicationModel::DataTransfer::ITransferTargetInvokeResult*>, double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.ApplicationModel.DataTransfer.TransferTargetInvokeResult, Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::ApplicationModel::DataTransfer::TransferTargetInvokeResult*, double> __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_t;
#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("66518d3e-0448-5c56-920e-8a0af567fe91"))
IAsyncOperationProgressHandler<ABI::Windows::ApplicationModel::DataTransfer::TransferTargetInvokeResult*, double> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::TransferTargetInvokeResult*, ABI::Windows::ApplicationModel::DataTransfer::ITransferTargetInvokeResult*>, double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.ApplicationModel.DataTransfer.TransferTargetInvokeResult, Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::ApplicationModel::DataTransfer::TransferTargetInvokeResult*, double> __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000


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
            namespace DataTransfer {
                class ClipboardHistoryItem;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("63fac521-1395-5c95-aaff-9736378a4f2f"))
IIterator<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryItem*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryItem*, ABI::Windows::ApplicationModel::DataTransfer::IClipboardHistoryItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.DataTransfer.ClipboardHistoryItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryItem*> __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_t;
#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5c3705f4-8e46-5ae5-97bf-88220cccffd9"))
IIterable<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryItem*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryItem*, ABI::Windows::ApplicationModel::DataTransfer::IClipboardHistoryItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.DataTransfer.ClipboardHistoryItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryItem*> __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_t;
#define __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class ShareProvider;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("886f5642-e9f9-573b-9213-5840b5062b40"))
IIterator<ABI::Windows::ApplicationModel::DataTransfer::ShareProvider*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::ShareProvider*, ABI::Windows::ApplicationModel::DataTransfer::IShareProvider*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.DataTransfer.ShareProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::DataTransfer::ShareProvider*> __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_t;
#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0903b218-5cad-53e6-9a21-6f4b31c4a409"))
IIterable<ABI::Windows::ApplicationModel::DataTransfer::ShareProvider*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::ShareProvider*, ABI::Windows::ApplicationModel::DataTransfer::IShareProvider*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.DataTransfer.ShareProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::DataTransfer::ShareProvider*> __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_t;
#define __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000


#ifndef DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("09335560-6c6b-5a26-9348-97b781132b20"))
IKeyValuePair<HSTRING, IInspectable*> : IKeyValuePair_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, IInspectable*> __FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5db5fa32-707c-5849-a06b-91c8eb9d10e8"))
IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fe2f3d47-5d47-5499-8374-430c7cda0204"))
IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIMapView_2_HSTRING_IInspectable_USE
#define DEF___FIMapView_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bb78502a-f79d-54fa-92c9-90c5039fdf7e"))
IMapView<HSTRING, IInspectable*> : IMapView_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, IInspectable*> __FIMapView_2_HSTRING_IInspectable_t;
#define __FIMapView_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIMap_2_HSTRING_IInspectable_USE
#define DEF___FIMap_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1b0d3570-0877-5ec2-8a2c-3b9539506aca"))
IMap<HSTRING, IInspectable*> : IMap_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, IInspectable*> __FIMap_2_HSTRING_IInspectable_t;
#define __FIMap_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_IInspectable_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#define DEF___FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e5d2ccfc-825a-5a8e-82aa-095ed5dbd5d1"))
IMap<HSTRING, ABI::Windows::Storage::Streams::RandomAccessStreamReference*> : IMap_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::Streams::RandomAccessStreamReference*, ABI::Windows::Storage::Streams::IRandomAccessStreamReference*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, Windows.Storage.Streams.RandomAccessStreamReference>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, ABI::Windows::Storage::Streams::RandomAccessStreamReference*> __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_t;
#define __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_USE */

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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c91de16d-13ff-55b0-897d-18a131d57ac9"))
IVectorView<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryItem*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryItem*, ABI::Windows::ApplicationModel::DataTransfer::IClipboardHistoryItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.DataTransfer.ClipboardHistoryItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryItem*> __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_t;
#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("946537a2-932c-5b77-ab36-b70650f0bcd5"))
IVectorView<ABI::Windows::ApplicationModel::DataTransfer::ShareProvider*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::ShareProvider*, ABI::Windows::ApplicationModel::DataTransfer::IShareProvider*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.DataTransfer.ShareProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::DataTransfer::ShareProvider*> __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_t;
#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000


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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_USE
#define DEF___FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a1687865-31e2-5536-97ec-292269a78046"))
IVector<ABI::Windows::ApplicationModel::DataTransfer::ShareProvider*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::ShareProvider*, ABI::Windows::ApplicationModel::DataTransfer::IShareProvider*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.ApplicationModel.DataTransfer.ShareProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::ApplicationModel::DataTransfer::ShareProvider*> __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_t;
#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000


#ifndef DEF___FIEventHandler_1_IInspectable_USE
#define DEF___FIEventHandler_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c50898f6-c536-5f47-8583-8b2c2438a13b"))
IEventHandler<IInspectable*> : IEventHandler_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<IInspectable*> __FIEventHandler_1_IInspectable_t;
#define __FIEventHandler_1_IInspectable ABI::Windows::Foundation::__FIEventHandler_1_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_IInspectable_USE */


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class ClipboardHistoryChangedEventArgs;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs_USE
#define DEF___FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("df4aac23-4002-5d4c-a237-2526e344978d"))
IEventHandler<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryChangedEventArgs*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryChangedEventArgs*, ABI::Windows::ApplicationModel::DataTransfer::IClipboardHistoryChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.ApplicationModel.DataTransfer.ClipboardHistoryChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryChangedEventArgs*> __FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs_t;
#define __FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs ABI::Windows::Foundation::__FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CRect_USE
#define DEF___FIReference_1_Windows__CFoundation__CRect_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("80423f11-054f-5eac-afd3-63b6ce15e77b"))
IReference<struct ABI::Windows::Foundation::Rect> : IReference_impl<struct ABI::Windows::Foundation::Rect>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.Rect>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::Rect> __FIReference_1_Windows__CFoundation__CRect_t;
#define __FIReference_1_Windows__CFoundation__CRect ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CRect_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CRect_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class DataPackage;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c156b0c3-1cbc-5ca4-901c-62c5a8ca5cb5"))
ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::DataPackage*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::DataPackage*, ABI::Windows::ApplicationModel::DataTransfer::IDataPackage*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.DataTransfer.DataPackage, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::DataPackage*, IInspectable*> __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class OperationCompletedEventArgs;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("dd48af6c-ef9a-59cb-b326-57d9e2411f21"))
ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::DataPackage*, ABI::Windows::ApplicationModel::DataTransfer::OperationCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::DataPackage*, ABI::Windows::ApplicationModel::DataTransfer::IDataPackage*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::OperationCompletedEventArgs*, ABI::Windows::ApplicationModel::DataTransfer::IOperationCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.DataTransfer.DataPackage, Windows.ApplicationModel.DataTransfer.OperationCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::DataPackage*, ABI::Windows::ApplicationModel::DataTransfer::OperationCompletedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class ShareCompletedEventArgs;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f8f7e24a-56fe-58df-bc15-2365aec03966"))
ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::DataPackage*, ABI::Windows::ApplicationModel::DataTransfer::ShareCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::DataPackage*, ABI::Windows::ApplicationModel::DataTransfer::IDataPackage*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::ShareCompletedEventArgs*, ABI::Windows::ApplicationModel::DataTransfer::IShareCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.DataTransfer.DataPackage, Windows.ApplicationModel.DataTransfer.ShareCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::DataPackage*, ABI::Windows::ApplicationModel::DataTransfer::ShareCompletedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class DataTransferManager;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class DataRequestedEventArgs;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ec6f9cc8-46d0-5e0e-b4d2-7d7773ae37a0"))
ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::DataTransferManager*, ABI::Windows::ApplicationModel::DataTransfer::DataRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::DataTransferManager*, ABI::Windows::ApplicationModel::DataTransfer::IDataTransferManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::DataRequestedEventArgs*, ABI::Windows::ApplicationModel::DataTransfer::IDataRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.DataTransfer.DataTransferManager, Windows.ApplicationModel.DataTransfer.DataRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::DataTransferManager*, ABI::Windows::ApplicationModel::DataTransfer::DataRequestedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class ShareProvidersRequestedEventArgs;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("147e2860-7397-582f-80db-b8685383a937"))
ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::DataTransferManager*, ABI::Windows::ApplicationModel::DataTransfer::ShareProvidersRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::DataTransferManager*, ABI::Windows::ApplicationModel::DataTransfer::IDataTransferManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::ShareProvidersRequestedEventArgs*, ABI::Windows::ApplicationModel::DataTransfer::IShareProvidersRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.DataTransfer.DataTransferManager, Windows.ApplicationModel.DataTransfer.ShareProvidersRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::DataTransferManager*, ABI::Windows::ApplicationModel::DataTransfer::ShareProvidersRequestedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class TargetApplicationChosenEventArgs;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c4ac1ba2-7851-5a44-bc8d-3d7c713f1f41"))
ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::DataTransferManager*, ABI::Windows::ApplicationModel::DataTransfer::TargetApplicationChosenEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::DataTransferManager*, ABI::Windows::ApplicationModel::DataTransfer::IDataTransferManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::TargetApplicationChosenEventArgs*, ABI::Windows::ApplicationModel::DataTransfer::ITargetApplicationChosenEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.DataTransfer.DataTransferManager, Windows.ApplicationModel.DataTransfer.TargetApplicationChosenEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::DataTransferManager*, ABI::Windows::ApplicationModel::DataTransfer::TargetApplicationChosenEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class TransferTargetWatcher;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2d0d4dd2-4dc7-533a-9601-84d15554f6e4"))
ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::TransferTargetWatcher*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::TransferTargetWatcher*, ABI::Windows::ApplicationModel::DataTransfer::ITransferTargetWatcher*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.DataTransfer.TransferTargetWatcher, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::TransferTargetWatcher*, IInspectable*> __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class TransferTargetChangedEventArgs;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9ebd72fb-43ed-57a0-862a-cda366a68827"))
ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::TransferTargetWatcher*, ABI::Windows::ApplicationModel::DataTransfer::TransferTargetChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::TransferTargetWatcher*, ABI::Windows::ApplicationModel::DataTransfer::ITransferTargetWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::TransferTargetChangedEventArgs*, ABI::Windows::ApplicationModel::DataTransfer::ITransferTargetChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.DataTransfer.TransferTargetWatcher, Windows.ApplicationModel.DataTransfer.TransferTargetChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::DataTransfer::TransferTargetWatcher*, ABI::Windows::ApplicationModel::DataTransfer::TransferTargetChangedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
        namespace UI {
            typedef struct Color Color;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct WindowId WindowId;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                typedef enum ClipboardHistoryItemsResultStatus : int ClipboardHistoryItemsResultStatus;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                typedef enum DataPackageOperation : unsigned int DataPackageOperation;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                typedef enum SetHistoryItemAsContentStatus : int SetHistoryItemAsContentStatus;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                typedef enum ShareUITheme : int ShareUITheme;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class ClipboardContentOptions;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class DataPackagePropertySet;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class DataPackagePropertySetView;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class DataPackageView;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class DataProviderDeferral;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class DataProviderRequest;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class DataRequest;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class DataRequestDeferral;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class ShareProviderOperation;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class ShareTargetInfo;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class ShareUIOptions;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class TransferTarget;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class TransferTargetDiscoveryOptions;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.DataTransfer.ClipboardHistoryItemsResultStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                enum ClipboardHistoryItemsResultStatus : int
                {
                    ClipboardHistoryItemsResultStatus_Success = 0,
                    ClipboardHistoryItemsResultStatus_AccessDenied = 1,
                    ClipboardHistoryItemsResultStatus_ClipboardHistoryDisabled = 2,
                };
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.ApplicationModel.DataTransfer.DataPackageOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                enum DataPackageOperation : unsigned int
                {
                    DataPackageOperation_None = 0,
                    DataPackageOperation_Copy = 0x1,
                    DataPackageOperation_Move = 0x2,
                    DataPackageOperation_Link = 0x4,
                };

                DEFINE_ENUM_FLAG_OPERATORS(DataPackageOperation)
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.DataTransfer.SetHistoryItemAsContentStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                enum SetHistoryItemAsContentStatus : int
                {
                    SetHistoryItemAsContentStatus_Success = 0,
                    SetHistoryItemAsContentStatus_AccessDenied = 1,
                    SetHistoryItemAsContentStatus_ItemDeleted = 2,
                };
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.ApplicationModel.DataTransfer.ShareUITheme
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                enum ShareUITheme : int
                {
                    ShareUITheme_Default = 0,
                    ShareUITheme_Light = 1,
                    ShareUITheme_Dark = 2,
                };
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Delegate Windows.ApplicationModel.DataTransfer.DataProviderHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("e7ecd720-f2f4-4a2d-920e-170a2f482a27")
                IDataProviderHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataProviderRequest* request
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataProviderHandler = __uuidof(IDataProviderHandler);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.ApplicationModel.DataTransfer.ShareProviderHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("e7f9d9ba-e1ba-4e4d-bd65-d43845d3212f")
                IShareProviderHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::ApplicationModel::DataTransfer::IShareProviderOperation* operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IShareProviderHandler = __uuidof(IShareProviderHandler);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IClipboardContentOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ClipboardContentOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IClipboardContentOptions[] = L"Windows.ApplicationModel.DataTransfer.IClipboardContentOptions";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("e888a98c-ad4b-5447-a056-ab3556276d2b")
                IClipboardContentOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsRoamable(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsRoamable(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsAllowedInHistory(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsAllowedInHistory(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RoamingFormats(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HistoryFormats(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IClipboardContentOptions = __uuidof(IClipboardContentOptions);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IClipboardHistoryChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ClipboardHistoryChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IClipboardHistoryChangedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.IClipboardHistoryChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("c0be453f-8ea2-53ce-9aba-8d2212573452")
                IClipboardHistoryChangedEventArgs : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IClipboardHistoryChangedEventArgs = __uuidof(IClipboardHistoryChangedEventArgs);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IClipboardHistoryItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ClipboardHistoryItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IClipboardHistoryItem[] = L"Windows.ApplicationModel.DataTransfer.IClipboardHistoryItem";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("0173bd8a-afff-5c50-ab92-3d19f481ec58")
                IClipboardHistoryItem : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Content(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataPackageView** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IClipboardHistoryItem = __uuidof(IClipboardHistoryItem);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IClipboardHistoryItemsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ClipboardHistoryItemsResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IClipboardHistoryItemsResult[] = L"Windows.ApplicationModel.DataTransfer.IClipboardHistoryItemsResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("e6dfdee6-0ee2-52e3-852b-f295db65939a")
                IClipboardHistoryItemsResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::ApplicationModel::DataTransfer::ClipboardHistoryItemsResultStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Items(
                        __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IClipboardHistoryItemsResult = __uuidof(IClipboardHistoryItemsResult);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IClipboardStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.Clipboard
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IClipboardStatics[] = L"Windows.ApplicationModel.DataTransfer.IClipboardStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("c627e291-34e2-4963-8eed-93cbb0ea3d70")
                IClipboardStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetContent(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataPackageView** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetContent(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataPackage* content
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Flush(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Clear(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ContentChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ContentChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IClipboardStatics = __uuidof(IClipboardStatics);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IClipboardStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.Clipboard
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IClipboardStatics2[] = L"Windows.ApplicationModel.DataTransfer.IClipboardStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("d2ac1b6a-d29f-554b-b303-f0452345fe02")
                IClipboardStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetHistoryItemsAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearHistory(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeleteItemFromHistory(
                        ABI::Windows::ApplicationModel::DataTransfer::IClipboardHistoryItem* item,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetHistoryItemAsContent(
                        ABI::Windows::ApplicationModel::DataTransfer::IClipboardHistoryItem* item,
                        ABI::Windows::ApplicationModel::DataTransfer::SetHistoryItemAsContentStatus* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsHistoryEnabled(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsRoamingEnabled(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetContentWithOptions(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataPackage* content,
                        ABI::Windows::ApplicationModel::DataTransfer::IClipboardContentOptions* options,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_HistoryChanged(
                        __FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_HistoryChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RoamingEnabledChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RoamingEnabledChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_HistoryEnabledChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_HistoryEnabledChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IClipboardStatics2 = __uuidof(IClipboardStatics2);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackage[] = L"Windows.ApplicationModel.DataTransfer.IDataPackage";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("61ebf5c7-efea-4346-9554-981d7e198ffe")
                IDataPackage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetView(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataPackageView** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataPackagePropertySet** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RequestedOperation(
                        ABI::Windows::ApplicationModel::DataTransfer::DataPackageOperation* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RequestedOperation(
                        ABI::Windows::ApplicationModel::DataTransfer::DataPackageOperation value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_OperationCompleted(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_OperationCompleted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Destroyed(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Destroyed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetData(
                        HSTRING formatId,
                        IInspectable* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetDataProvider(
                        HSTRING formatId,
                        ABI::Windows::ApplicationModel::DataTransfer::IDataProviderHandler* delayRenderer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetText(
                        HSTRING value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SetUri may be altered or unavailable for releases after Windows Phone 'OSVersion' (TBD).Instead, use SetWebLink or SetApplicationLink.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE SetUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetHtmlFormat(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ResourceMap(
                        __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetRtf(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetBitmap(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetStorageItemsReadOnly(
                        __FIIterable_1_Windows__CStorage__CIStorageItem* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetStorageItems(
                        __FIIterable_1_Windows__CStorage__CIStorageItem* value,
                        boolean readOnly
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPackage = __uuidof(IDataPackage);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackage2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackage2[] = L"Windows.ApplicationModel.DataTransfer.IDataPackage2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("041c1fe9-2409-45e1-a538-4c53eeee04a7")
                IDataPackage2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetApplicationLink(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetWebLink(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPackage2 = __uuidof(IDataPackage2);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackage3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackage3[] = L"Windows.ApplicationModel.DataTransfer.IDataPackage3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("88f31f5d-787b-4d32-965a-a9838105a056")
                IDataPackage3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_ShareCompleted(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ShareCompleted(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPackage3 = __uuidof(IDataPackage3);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackage4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackage4[] = L"Windows.ApplicationModel.DataTransfer.IDataPackage4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("13a24ec8-9382-536f-852a-3045e1b29a3b")
                IDataPackage4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_ShareCanceled(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ShareCanceled(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPackage4 = __uuidof(IDataPackage4);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySet
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IMap`2<String, Object>
 *     Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySet[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("cd1c93eb-4c4c-443a-a8d3-f5c241e91689")
                IDataPackagePropertySet : public IInspectable
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
                    virtual HRESULT STDMETHODCALLTYPE get_Thumbnail(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Thumbnail(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FileTypes(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ApplicationName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ApplicationName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ApplicationListingUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ApplicationListingUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPackagePropertySet = __uuidof(IDataPackagePropertySet);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySet
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySet2[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("eb505d4a-9800-46aa-b181-7b6f0f2b919a")
                IDataPackagePropertySet2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContentSourceWebLink(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ContentSourceWebLink(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContentSourceApplicationLink(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ContentSourceApplicationLink(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PackageFamilyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PackageFamilyName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Square30x30Logo(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Square30x30Logo(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LogoBackgroundColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LogoBackgroundColor(
                        ABI::Windows::UI::Color value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPackagePropertySet2 = __uuidof(IDataPackagePropertySet2);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySet
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySet3[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("9e87fd9b-5205-401b-874a-455653bd39e8")
                IDataPackagePropertySet3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_EnterpriseId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_EnterpriseId(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPackagePropertySet3 = __uuidof(IDataPackagePropertySet3);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySet
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySet4[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("6390ebf5-1739-4c74-b22f-865fab5e8545")
                IDataPackagePropertySet4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContentSourceUserActivityJson(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ContentSourceUserActivityJson(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPackagePropertySet4 = __uuidof(IDataPackagePropertySet4);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySetView[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("b94cec01-0c1a-4c57-be55-75d01289735d")
                IDataPackagePropertySetView : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Thumbnail(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FileTypes(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ApplicationName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ApplicationListingUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPackagePropertySetView = __uuidof(IDataPackagePropertySetView);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySetView2[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("6054509b-8ebe-4feb-9c1e-75e69de54b84")
                IDataPackagePropertySetView2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PackageFamilyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContentSourceWebLink(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContentSourceApplicationLink(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Square30x30Logo(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LogoBackgroundColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPackagePropertySetView2 = __uuidof(IDataPackagePropertySetView2);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySetView3[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("db764ce5-d174-495c-84fc-1a51f6ab45d7")
                IDataPackagePropertySetView3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_EnterpriseId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPackagePropertySetView3 = __uuidof(IDataPackagePropertySetView3);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySetView4[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("4474c80d-d16f-40ae-9580-6f8562b94235")
                IDataPackagePropertySetView4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContentSourceUserActivityJson(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPackagePropertySetView4 = __uuidof(IDataPackagePropertySetView4);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySetView5[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView5";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("6f0a9445-3760-50bb-8523-c4202ded7d78")
                IDataPackagePropertySetView5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsFromRoamingClipboard(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPackagePropertySetView5 = __uuidof(IDataPackagePropertySetView5);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackageView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackageView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackageView[] = L"Windows.ApplicationModel.DataTransfer.IDataPackageView";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("7b840471-5900-4d85-a90b-10cb85fe3552")
                IDataPackageView : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataPackagePropertySetView** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RequestedOperation(
                        ABI::Windows::ApplicationModel::DataTransfer::DataPackageOperation* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReportOperationCompleted(
                        ABI::Windows::ApplicationModel::DataTransfer::DataPackageOperation value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AvailableFormats(
                        __FIVectorView_1_HSTRING** formatIds
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Contains(
                        HSTRING formatId,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDataAsync(
                        HSTRING formatId,
                        __FIAsyncOperation_1_IInspectable** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTextAsync(
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCustomTextAsync(
                        HSTRING formatId,
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("GetUriAsync may be altered or unavailable for releases after Windows 8.1. Instead, use GetWebLinkAsync or GetApplicationLinkAsync.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE GetUriAsync(
                        __FIAsyncOperation_1_Windows__CFoundation__CUri** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetHtmlFormatAsync(
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetResourceMapAsync(
                        __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetRtfAsync(
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetBitmapAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetStorageItemsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPackageView = __uuidof(IDataPackageView);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackageView2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackageView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackageView2[] = L"Windows.ApplicationModel.DataTransfer.IDataPackageView2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("40ecba95-2450-4c1d-b6b4-ed45463dee9c")
                IDataPackageView2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetApplicationLinkAsync(
                        __FIAsyncOperation_1_Windows__CFoundation__CUri** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetWebLinkAsync(
                        __FIAsyncOperation_1_Windows__CFoundation__CUri** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPackageView2 = __uuidof(IDataPackageView2);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackageView3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackageView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackageView3[] = L"Windows.ApplicationModel.DataTransfer.IDataPackageView3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("d37771a8-ddad-4288-8428-d1cae394128b")
                IDataPackageView3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                        __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestAccessWithEnterpriseIdAsync(
                        HSTRING enterpriseId,
                        __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UnlockAndAssumeEnterpriseIdentity(
                        ABI::Windows::Security::EnterpriseData::ProtectionPolicyEvaluationResult* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPackageView3 = __uuidof(IDataPackageView3);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackageView4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackageView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackageView4[] = L"Windows.ApplicationModel.DataTransfer.IDataPackageView4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("dfe96f1f-e042-4433-a09f-26d6ffda8b85")
                IDataPackageView4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetAcceptedFormatId(
                        HSTRING formatId
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataPackageView4 = __uuidof(IDataPackageView4);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataProviderDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataProviderDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataProviderDeferral[] = L"Windows.ApplicationModel.DataTransfer.IDataProviderDeferral";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("c2cf2373-2d26-43d9-b69d-dcb86d03f6da")
                IDataProviderDeferral : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataProviderDeferral = __uuidof(IDataProviderDeferral);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataProviderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataProviderRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataProviderRequest[] = L"Windows.ApplicationModel.DataTransfer.IDataProviderRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("ebbc7157-d3c8-47da-acde-f82388d5f716")
                IDataProviderRequest : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FormatId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Deadline(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataProviderDeferral** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetData(
                        IInspectable* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataProviderRequest = __uuidof(IDataProviderRequest);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataRequest[] = L"Windows.ApplicationModel.DataTransfer.IDataRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("4341ae3b-fc12-4e53-8c02-ac714c415a27")
                IDataRequest : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Data(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataPackage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Data(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataPackage* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Deadline(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FailWithDisplayText(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataRequestDeferral** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataRequest = __uuidof(IDataRequest);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataRequestDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataRequestDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataRequestDeferral[] = L"Windows.ApplicationModel.DataTransfer.IDataRequestDeferral";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("6dc4b89f-0386-4263-87c1-ed7dce30890e")
                IDataRequestDeferral : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataRequestDeferral = __uuidof(IDataRequestDeferral);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataRequestedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.IDataRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("cb8ba807-6ac5-43c9-8ac5-9ba232163182")
                IDataRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Request(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataRequest** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataRequestedEventArgs = __uuidof(IDataRequestedEventArgs);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataTransferManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataTransferManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataTransferManager[] = L"Windows.ApplicationModel.DataTransfer.IDataTransferManager";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("a5caee9b-8708-49d1-8d36-67d25a8da00c")
                IDataTransferManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_DataRequested(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DataRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_TargetApplicationChosen(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_TargetApplicationChosen(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataTransferManager = __uuidof(IDataTransferManager);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataTransferManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataTransferManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataTransferManager2[] = L"Windows.ApplicationModel.DataTransfer.IDataTransferManager2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("30ae7d71-8ba8-4c02-8e3f-ddb23b388715")
                IDataTransferManager2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_ShareProvidersRequested(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ShareProvidersRequested(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataTransferManager2 = __uuidof(IDataTransferManager2);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataTransferManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataTransferManagerStatics[] = L"Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("a9da01aa-e00e-4cfe-aa44-2dd932dca3d8")
                IDataTransferManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ShowShareUI(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataTransferManager** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataTransferManagerStatics = __uuidof(IDataTransferManagerStatics);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataTransferManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataTransferManagerStatics2[] = L"Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("c54ec2ec-9f97-4d63-9868-395e271ad8f5")
                IDataTransferManagerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsSupported(
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataTransferManagerStatics2 = __uuidof(IDataTransferManagerStatics2);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataTransferManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataTransferManagerStatics3[] = L"Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("05845473-6c82-4f5c-ac23-62e458361fac")
                IDataTransferManagerStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ShowShareUIWithOptions(
                        ABI::Windows::ApplicationModel::DataTransfer::IShareUIOptions* options
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDataTransferManagerStatics3 = __uuidof(IDataTransferManagerStatics3);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IHtmlFormatHelperStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.HtmlFormatHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IHtmlFormatHelperStatics[] = L"Windows.ApplicationModel.DataTransfer.IHtmlFormatHelperStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("e22e7749-dd70-446f-aefc-61cee59f655e")
                IHtmlFormatHelperStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetStaticFragment(
                        HSTRING htmlFormat,
                        HSTRING* htmlFragment
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateHtmlFormat(
                        HSTRING htmlFragment,
                        HSTRING* htmlFormat
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHtmlFormatHelperStatics = __uuidof(IHtmlFormatHelperStatics);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.OperationCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IOperationCompletedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("e7af329d-051d-4fab-b1a9-47fd77f70a41")
                IOperationCompletedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Operation(
                        ABI::Windows::ApplicationModel::DataTransfer::DataPackageOperation* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IOperationCompletedEventArgs = __uuidof(IOperationCompletedEventArgs);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.OperationCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IOperationCompletedEventArgs2[] = L"Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("858fa073-1e19-4105-b2f7-c8478808d562")
                IOperationCompletedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AcceptedFormatId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IOperationCompletedEventArgs2 = __uuidof(IOperationCompletedEventArgs2);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IShareCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ShareCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IShareCompletedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.IShareCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("4574c442-f913-4f60-9df7-cc4060ab1916")
                IShareCompletedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ShareTarget(
                        ABI::Windows::ApplicationModel::DataTransfer::IShareTargetInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IShareCompletedEventArgs = __uuidof(IShareCompletedEventArgs);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IShareProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ShareProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IShareProvider[] = L"Windows.ApplicationModel.DataTransfer.IShareProvider";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("2fabe026-443e-4cda-af25-8d81070efd80")
                IShareProvider : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayIcon(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BackgroundColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Tag(
                        IInspectable** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Tag(
                        IInspectable* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IShareProvider = __uuidof(IShareProvider);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IShareProviderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ShareProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IShareProviderFactory[] = L"Windows.ApplicationModel.DataTransfer.IShareProviderFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("172a174c-e79e-4f6d-b07d-128f469e0296")
                IShareProviderFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING title,
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* displayIcon,
                        ABI::Windows::UI::Color backgroundColor,
                        ABI::Windows::ApplicationModel::DataTransfer::IShareProviderHandler* handler,
                        ABI::Windows::ApplicationModel::DataTransfer::IShareProvider** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IShareProviderFactory = __uuidof(IShareProviderFactory);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IShareProviderOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ShareProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IShareProviderOperation[] = L"Windows.ApplicationModel.DataTransfer.IShareProviderOperation";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("19cef937-d435-4179-b6af-14e0492b69f6")
                IShareProviderOperation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Data(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataPackageView** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Provider(
                        ABI::Windows::ApplicationModel::DataTransfer::IShareProvider** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReportCompleted(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IShareProviderOperation = __uuidof(IShareProviderOperation);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IShareProvidersRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ShareProvidersRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IShareProvidersRequestedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.IShareProvidersRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("f888f356-a3f8-4fce-85e4-8826e63be799")
                IShareProvidersRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Providers(
                        __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Data(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataPackageView** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IShareProvidersRequestedEventArgs = __uuidof(IShareProvidersRequestedEventArgs);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IShareTargetInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ShareTargetInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IShareTargetInfo[] = L"Windows.ApplicationModel.DataTransfer.IShareTargetInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("385be607-c6e8-4114-b294-28f3bb6f9904")
                IShareTargetInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppUserModelId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShareProvider(
                        ABI::Windows::ApplicationModel::DataTransfer::IShareProvider** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IShareTargetInfo = __uuidof(IShareTargetInfo);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IShareUIOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ShareUIOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IShareUIOptions[] = L"Windows.ApplicationModel.DataTransfer.IShareUIOptions";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("72fa8a80-342f-4d90-9551-2ae04e37680c")
                IShareUIOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Theme(
                        ABI::Windows::ApplicationModel::DataTransfer::ShareUITheme* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Theme(
                        ABI::Windows::ApplicationModel::DataTransfer::ShareUITheme value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelectionRect(
                        __FIReference_1_Windows__CFoundation__CRect** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SelectionRect(
                        __FIReference_1_Windows__CFoundation__CRect* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IShareUIOptions = __uuidof(IShareUIOptions);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ISharedStorageAccessManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.SharedStorageAccessManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ISharedStorageAccessManagerStatics[] = L"Windows.ApplicationModel.DataTransfer.ISharedStorageAccessManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("c6132ada-34b1-4849-bd5f-d09fee3158c5")
                ISharedStorageAccessManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE AddFile(
                        ABI::Windows::Storage::IStorageFile* file,
                        HSTRING* outToken
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RedeemTokenForFileAsync(
                        HSTRING token,
                        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveFile(
                        HSTRING token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISharedStorageAccessManagerStatics = __uuidof(ISharedStorageAccessManagerStatics);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.StandardDataFormats
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IStandardDataFormatsStatics[] = L"Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("7ed681a1-a880-40c9-b4ed-0bee1e15f549")
                IStandardDataFormatsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Uri may be altered or unavailable for releases after Windows Phone 'OSVersion' (TBD). Instead, use WebLink or ApplicationLink.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Html(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Rtf(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Bitmap(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StorageItems(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStandardDataFormatsStatics = __uuidof(IStandardDataFormatsStatics);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.StandardDataFormats
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IStandardDataFormatsStatics2[] = L"Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("42a254f4-9d76-42e8-861b-47c25dd0cf71")
                IStandardDataFormatsStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WebLink(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ApplicationLink(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStandardDataFormatsStatics2 = __uuidof(IStandardDataFormatsStatics2);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.StandardDataFormats
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IStandardDataFormatsStatics3[] = L"Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("3b57b069-01d4-474c-8b5f-bc8e27f38b21")
                IStandardDataFormatsStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_UserActivityJsonArray(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStandardDataFormatsStatics3 = __uuidof(IStandardDataFormatsStatics3);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITargetApplicationChosenEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TargetApplicationChosenEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITargetApplicationChosenEventArgs[] = L"Windows.ApplicationModel.DataTransfer.ITargetApplicationChosenEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("ca6fb8ac-2987-4ee3-9c54-d8afbcb86c1d")
                ITargetApplicationChosenEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ApplicationName(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITargetApplicationChosenEventArgs = __uuidof(ITargetApplicationChosenEventArgs);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITransferTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TransferTarget
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITransferTarget[] = L"Windows.ApplicationModel.DataTransfer.ITransferTarget";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("897e04e5-60c2-5eae-909f-e6257e32c644")
                ITransferTarget : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Label(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayIcon(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITransferTarget = __uuidof(ITransferTarget);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITransferTargetChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TransferTargetChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITransferTargetChangedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.ITransferTargetChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("d513d198-4174-53cf-a06e-4cd263d0dfef")
                ITransferTargetChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Target(
                        ABI::Windows::ApplicationModel::DataTransfer::ITransferTarget** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITransferTargetChangedEventArgs = __uuidof(ITransferTargetChangedEventArgs);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITransferTargetDiscoveryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TransferTargetDiscoveryOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITransferTargetDiscoveryOptions[] = L"Windows.ApplicationModel.DataTransfer.ITransferTargetDiscoveryOptions";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("712fe3b5-644f-5f6b-97b6-3a3400999ed7")
                ITransferTargetDiscoveryOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DataPackage(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataPackageView** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxAppTargets(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MaxAppTargets(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AllowedTargetAppIds(
                        UINT32* valueLength,
                        HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AllowedTargetAppIds(
                        UINT32 valueLength,
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITransferTargetDiscoveryOptions = __uuidof(ITransferTargetDiscoveryOptions);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITransferTargetDiscoveryOptionsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TransferTargetDiscoveryOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITransferTargetDiscoveryOptionsFactory[] = L"Windows.ApplicationModel.DataTransfer.ITransferTargetDiscoveryOptionsFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("ec4b7ffc-cbc6-5e12-8e9b-d5e892f2c6f8")
                ITransferTargetDiscoveryOptionsFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataPackageView* dataPackage,
                        ABI::Windows::ApplicationModel::DataTransfer::ITransferTargetDiscoveryOptions** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITransferTargetDiscoveryOptionsFactory = __uuidof(ITransferTargetDiscoveryOptionsFactory);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITransferTargetInvokeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TransferTargetInvokeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITransferTargetInvokeResult[] = L"Windows.ApplicationModel.DataTransfer.ITransferTargetInvokeResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("15f220a6-cffe-56f5-b403-ed44e9c3ad38")
                ITransferTargetInvokeResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Succeeded(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITransferTargetInvokeResult = __uuidof(ITransferTargetInvokeResult);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITransferTargetStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TransferTarget
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITransferTargetStatics[] = L"Windows.ApplicationModel.DataTransfer.ITransferTargetStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("815b8804-e7f1-5f37-b52f-be1ceba9a59e")
                ITransferTargetStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWatcher(
                        ABI::Windows::ApplicationModel::DataTransfer::ITransferTargetDiscoveryOptions* options,
                        ABI::Windows::ApplicationModel::DataTransfer::ITransferTargetWatcher** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITransferTargetStatics = __uuidof(ITransferTargetStatics);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITransferTargetWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TransferTargetWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITransferTargetWatcher[] = L"Windows.ApplicationModel.DataTransfer.ITransferTargetWatcher";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("2f85ca29-0100-5d09-907c-fe554d2fcd1a")
                ITransferTargetWatcher : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TransferToAsync(
                        ABI::Windows::ApplicationModel::DataTransfer::ITransferTarget* target,
                        ABI::Windows::UI::WindowId parentWindowHandle,
                        __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Added(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Added(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Removed(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Removed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Updated(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Updated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_EnumerationCompleted(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_EnumerationCompleted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Stopped(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Stopped(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITransferTargetWatcher = __uuidof(ITransferTargetWatcher);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITransferTargetWatcherStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TransferTargetWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITransferTargetWatcherStatics[] = L"Windows.ApplicationModel.DataTransfer.ITransferTargetWatcherStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                MIDL_INTERFACE("a24b3528-db4e-5bdd-9d30-dcb192c701f5")
                ITransferTargetWatcherStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsSupported(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataPackageView* dataPackage,
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITransferTargetWatcherStatics = __uuidof(ITransferTargetWatcherStatics);
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.Clipboard
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IClipboardStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IClipboardStatics2 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_Clipboard_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_Clipboard_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_Clipboard[] = L"Windows.ApplicationModel.DataTransfer.Clipboard";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ClipboardContentOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IClipboardContentOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ClipboardContentOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ClipboardContentOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ClipboardContentOptions[] = L"Windows.ApplicationModel.DataTransfer.ClipboardContentOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ClipboardHistoryChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IClipboardHistoryChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ClipboardHistoryChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ClipboardHistoryChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ClipboardHistoryChangedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.ClipboardHistoryChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ClipboardHistoryItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IClipboardHistoryItem ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ClipboardHistoryItem_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ClipboardHistoryItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ClipboardHistoryItem[] = L"Windows.ApplicationModel.DataTransfer.ClipboardHistoryItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ClipboardHistoryItemsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IClipboardHistoryItemsResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ClipboardHistoryItemsResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ClipboardHistoryItemsResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ClipboardHistoryItemsResult[] = L"Windows.ApplicationModel.DataTransfer.ClipboardHistoryItemsResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataPackage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataPackage ** Default Interface **
 *    Windows.ApplicationModel.DataTransfer.IDataPackage2
 *    Windows.ApplicationModel.DataTransfer.IDataPackage3
 *    Windows.ApplicationModel.DataTransfer.IDataPackage4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataPackage_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataPackage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataPackage[] = L"Windows.ApplicationModel.DataTransfer.DataPackage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataPackagePropertySet
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet ** Default Interface **
 *    Windows.Foundation.Collections.IMap`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet2
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet3
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataPackagePropertySet_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataPackagePropertySet_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataPackagePropertySet[] = L"Windows.ApplicationModel.DataTransfer.DataPackagePropertySet";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView ** Default Interface **
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView2
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView3
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView4
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView5
 *    Windows.Foundation.Collections.IMapView`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataPackagePropertySetView_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataPackagePropertySetView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataPackagePropertySetView[] = L"Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataPackageView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataPackageView ** Default Interface **
 *    Windows.ApplicationModel.DataTransfer.IDataPackageView2
 *    Windows.ApplicationModel.DataTransfer.IDataPackageView3
 *    Windows.ApplicationModel.DataTransfer.IDataPackageView4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataPackageView_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataPackageView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataPackageView[] = L"Windows.ApplicationModel.DataTransfer.DataPackageView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataProviderDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataProviderDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataProviderDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataProviderDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataProviderDeferral[] = L"Windows.ApplicationModel.DataTransfer.DataProviderDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataProviderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataProviderRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataProviderRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataProviderRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataProviderRequest[] = L"Windows.ApplicationModel.DataTransfer.DataProviderRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataRequest[] = L"Windows.ApplicationModel.DataTransfer.DataRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataRequestDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataRequestDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataRequestDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataRequestDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataRequestDeferral[] = L"Windows.ApplicationModel.DataTransfer.DataRequestDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataRequestedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.DataRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataTransferManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics3 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataTransferManager ** Default Interface **
 *    Windows.ApplicationModel.DataTransfer.IDataTransferManager2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataTransferManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataTransferManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataTransferManager[] = L"Windows.ApplicationModel.DataTransfer.DataTransferManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.HtmlFormatHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IHtmlFormatHelperStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_HtmlFormatHelper_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_HtmlFormatHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_HtmlFormatHelper[] = L"Windows.ApplicationModel.DataTransfer.HtmlFormatHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.OperationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_OperationCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_OperationCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_OperationCompletedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.OperationCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ShareCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IShareCompletedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ShareCompletedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.ShareCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ShareProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.DataTransfer.IShareProviderFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IShareProvider ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareProvider_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ShareProvider[] = L"Windows.ApplicationModel.DataTransfer.ShareProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ShareProviderOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IShareProviderOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareProviderOperation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareProviderOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ShareProviderOperation[] = L"Windows.ApplicationModel.DataTransfer.ShareProviderOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ShareProvidersRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IShareProvidersRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareProvidersRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareProvidersRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ShareProvidersRequestedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.ShareProvidersRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ShareTargetInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IShareTargetInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareTargetInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareTargetInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ShareTargetInfo[] = L"Windows.ApplicationModel.DataTransfer.ShareTargetInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ShareUIOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IShareUIOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareUIOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareUIOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ShareUIOptions[] = L"Windows.ApplicationModel.DataTransfer.ShareUIOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.SharedStorageAccessManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.ISharedStorageAccessManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_SharedStorageAccessManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_SharedStorageAccessManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_SharedStorageAccessManager[] = L"Windows.ApplicationModel.DataTransfer.SharedStorageAccessManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.StandardDataFormats
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics3 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_StandardDataFormats_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_StandardDataFormats_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_StandardDataFormats[] = L"Windows.ApplicationModel.DataTransfer.StandardDataFormats";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.TargetApplicationChosenEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.ITargetApplicationChosenEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TargetApplicationChosenEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TargetApplicationChosenEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_TargetApplicationChosenEventArgs[] = L"Windows.ApplicationModel.DataTransfer.TargetApplicationChosenEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.TransferTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.ITransferTargetStatics interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.ITransferTarget ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTarget_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTarget_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_TransferTarget[] = L"Windows.ApplicationModel.DataTransfer.TransferTarget";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.TransferTargetChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.ITransferTargetChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTargetChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTargetChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_TransferTargetChangedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.TransferTargetChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.TransferTargetDiscoveryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.DataTransfer.ITransferTargetDiscoveryOptionsFactory interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.ITransferTargetDiscoveryOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTargetDiscoveryOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTargetDiscoveryOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_TransferTargetDiscoveryOptions[] = L"Windows.ApplicationModel.DataTransfer.TransferTargetDiscoveryOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.TransferTargetInvokeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.ITransferTargetInvokeResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTargetInvokeResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTargetInvokeResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_TransferTargetInvokeResult[] = L"Windows.ApplicationModel.DataTransfer.TransferTargetInvokeResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.TransferTargetWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.ITransferTargetWatcherStatics interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.ITransferTargetWatcher ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTargetWatcher_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTargetWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_TransferTargetWatcher[] = L"Windows.ApplicationModel.DataTransfer.TransferTargetWatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics __x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3 __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_IInspectable __FIAsyncOperationCompletedHandler_1_IInspectable;

#if !defined(____FIAsyncOperation_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_IInspectable __FIAsyncOperation_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_IInspectable;

typedef struct __FIAsyncOperation_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_IInspectable* This,
        __FIAsyncOperationCompletedHandler_1_IInspectable* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_IInspectable* This,
        __FIAsyncOperationCompletedHandler_1_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_IInspectable* This,
        IInspectable** result);

    END_INTERFACE
} __FIAsyncOperation_1_IInspectableVtbl;

interface __FIAsyncOperation_1_IInspectable
{
    CONST_VTBL struct __FIAsyncOperation_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_IInspectable_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_IInspectable_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_IInspectable_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_IInspectable __FIAsyncOperationCompletedHandler_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_IInspectable;

typedef struct __FIAsyncOperationCompletedHandler_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_IInspectable* This,
        __FIAsyncOperation_1_IInspectable* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_IInspectableVtbl;

interface __FIAsyncOperationCompletedHandler_1_IInspectable
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_IInspectable_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

#if !defined(____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_HSTRING __FIAsyncOperation_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_HSTRING;

typedef struct __FIAsyncOperation_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIAsyncOperation_1_HSTRINGVtbl;

interface __FIAsyncOperation_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperation_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_HSTRING_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_HSTRING_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_HSTRING_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_HSTRING;

typedef struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        __FIAsyncOperation_1_HSTRING* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl;

interface __FIAsyncOperationCompletedHandler_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference;

typedef struct __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        HSTRING key,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference** first,
        __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl;

interface __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference;

typedef struct __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl;

interface __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference
{
    CONST_VTBL struct __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference;

typedef struct __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageItem __x_ABI_CWindows_CStorage_CIStorageItem;

#endif // ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CStorage__CIStorageItem __FIIterator_1_Windows__CStorage__CIStorageItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CStorage__CIStorageItem;

typedef struct __FIIterator_1_Windows__CStorage__CIStorageItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CStorage__CIStorageItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CStorage__CIStorageItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CStorage__CIStorageItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CStorage__CIStorageItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CStorage__CIStorageItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CStorage__CIStorageItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CStorage__CIStorageItem* This,
        __x_ABI_CWindows_CStorage_CIStorageItem** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CStorage__CIStorageItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CStorage__CIStorageItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CStorage__CIStorageItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageItem** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CStorage__CIStorageItemVtbl;

interface __FIIterator_1_Windows__CStorage__CIStorageItem
{
    CONST_VTBL struct __FIIterator_1_Windows__CStorage__CIStorageItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CStorage__CIStorageItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CStorage__CIStorageItem __FIIterable_1_Windows__CStorage__CIStorageItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CStorage__CIStorageItem;

typedef struct __FIIterable_1_Windows__CStorage__CIStorageItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CStorage__CIStorageItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CStorage__CIStorageItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CStorage__CIStorageItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CStorage__CIStorageItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CStorage__CIStorageItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CStorage__CIStorageItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CStorage__CIStorageItem* This,
        __FIIterator_1_Windows__CStorage__CIStorageItem** result);

    END_INTERFACE
} __FIIterable_1_Windows__CStorage__CIStorageItemVtbl;

interface __FIIterable_1_Windows__CStorage__CIStorageItem
{
    CONST_VTBL struct __FIIterable_1_Windows__CStorage__CIStorageItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CStorage__CIStorageItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CStorage__CIStorageItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CStorage__CIStorageItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CStorage__CIStorageItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CStorage__CIStorageItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CStorage__CIStorageItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CStorage__CIStorageItem_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CStorage__CIStorageItem __FIVectorView_1_Windows__CStorage__CIStorageItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CStorage__CIStorageItem;

typedef struct __FIVectorView_1_Windows__CStorage__CIStorageItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CIStorageItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        __x_ABI_CWindows_CStorage_CIStorageItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageItem** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CStorage__CIStorageItemVtbl;

interface __FIVectorView_1_Windows__CStorage__CIStorageItem
{
    CONST_VTBL struct __FIVectorView_1_Windows__CStorage__CIStorageItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        __FIVectorView_1_Windows__CStorage__CIStorageItem** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItemVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItemVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CFoundation__CUri __FIAsyncOperation_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CFoundation__CUri;

typedef struct __FIAsyncOperation_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CFoundation__CUriVtbl;

interface __FIAsyncOperation_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri* This,
        __FIAsyncOperation_1_Windows__CFoundation__CUri* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUriVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CSecurity_CEnterpriseData_CProtectionPolicyEvaluationResult __x_ABI_CWindows_CSecurity_CEnterpriseData_CProtectionPolicyEvaluationResult;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult* This,
        enum __x_ABI_CWindows_CSecurity_CEnterpriseData_CProtectionPolicyEvaluationResult* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStorageFile __FIAsyncOperation_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStorageFile;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStorageFileVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFileVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This,
        __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_doubleVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This,
        __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This,
        __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_doubleVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* This,
        __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double* asyncInfo,
        DOUBLE progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_doubleVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem;

typedef struct __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem;

typedef struct __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider;

typedef struct __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProviderVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider;

typedef struct __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        __FIIterator_1_Windows__CApplicationModel__CDataTransfer__CShareProvider** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProviderVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if !defined(____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_IInspectable __FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        IInspectable** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIKeyValuePair_2_HSTRING_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_IInspectable** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

#if !defined(____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_IInspectable;

typedef struct __FIMapView_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_IInspectable* This,
        __FIMapView_2_HSTRING_IInspectable** first,
        __FIMapView_2_HSTRING_IInspectable** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_IInspectableVtbl;

interface __FIMapView_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIMapView_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_IInspectable __FIMap_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_IInspectable;

typedef struct __FIMap_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_IInspectable* This,
        __FIMapView_2_HSTRING_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable* value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_IInspectable* This);

    END_INTERFACE
} __FIMap_2_HSTRING_IInspectableVtbl;

interface __FIMap_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIMap_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_IInspectable_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_IInspectable_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_IInspectable_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_IInspectable_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference;

typedef struct __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        HSTRING key,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        HSTRING key,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference* This);

    END_INTERFACE
} __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl;

interface __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference
{
    CONST_VTBL struct __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference_INTERFACE_DEFINED__
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProviderVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider;

typedef struct __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CShareProvider** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider** items);

    END_INTERFACE
} __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProviderVtbl;

interface __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider
{
    CONST_VTBL struct __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if !defined(____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_IInspectable __FIEventHandler_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_IInspectable;

typedef struct __FIEventHandler_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_IInspectable* This,
        IInspectable* sender,
        IInspectable* args);

    END_INTERFACE
} __FIEventHandler_1_IInspectableVtbl;

interface __FIEventHandler_1_IInspectable
{
    CONST_VTBL struct __FIEventHandler_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs __FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs;

typedef struct __FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgsVtbl;

interface __FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CRect_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CRect_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CRect __FIReference_1_Windows__CFoundation__CRect;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CRect;

typedef struct __FIReference_1_Windows__CFoundation__CRectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CRect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CRect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CRect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CRect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CRect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CRect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CRect* This,
        struct __x_ABI_CWindows_CFoundation_CRect* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CRectVtbl;

interface __FIReference_1_Windows__CFoundation__CRect
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CRectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CRect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CRect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CRect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CRect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CRect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CRect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CRect_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CRect_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* sender,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* sender,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager* sender,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager* sender,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager* sender,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* sender,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

typedef struct __x_ABI_CWindows_CUI_CWindowId __x_ABI_CWindows_CUI_CWindowId;

typedef enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CClipboardHistoryItemsResultStatus __x_ABI_CWindows_CApplicationModel_CDataTransfer_CClipboardHistoryItemsResultStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDataPackageOperation __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDataPackageOperation;

typedef enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CSetHistoryItemAsContentStatus __x_ABI_CWindows_CApplicationModel_CDataTransfer_CSetHistoryItemAsContentStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CShareUITheme __x_ABI_CWindows_CApplicationModel_CDataTransfer_CShareUITheme;

/*
 *
 * Struct Windows.ApplicationModel.DataTransfer.ClipboardHistoryItemsResultStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CClipboardHistoryItemsResultStatus
{
    ClipboardHistoryItemsResultStatus_Success = 0,
    ClipboardHistoryItemsResultStatus_AccessDenied = 1,
    ClipboardHistoryItemsResultStatus_ClipboardHistoryDisabled = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.ApplicationModel.DataTransfer.DataPackageOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDataPackageOperation
{
    DataPackageOperation_None = 0,
    DataPackageOperation_Copy = 0x1,
    DataPackageOperation_Move = 0x2,
    DataPackageOperation_Link = 0x4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.DataTransfer.SetHistoryItemAsContentStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CSetHistoryItemAsContentStatus
{
    SetHistoryItemAsContentStatus_Success = 0,
    SetHistoryItemAsContentStatus_AccessDenied = 1,
    SetHistoryItemAsContentStatus_ItemDeleted = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.ApplicationModel.DataTransfer.ShareUITheme
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CShareUITheme
{
    ShareUITheme_Default = 0,
    ShareUITheme_Light = 1,
    ShareUITheme_Dark = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Delegate Windows.ApplicationModel.DataTransfer.DataProviderHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest* request);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandlerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler_Invoke(This, request) \
    ((This)->lpVtbl->Invoke(This, request))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.ApplicationModel.DataTransfer.ShareProviderHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation* operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandlerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler_Invoke(This, operation) \
    ((This)->lpVtbl->Invoke(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IClipboardContentOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ClipboardContentOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IClipboardContentOptions[] = L"Windows.ApplicationModel.DataTransfer.IClipboardContentOptions";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsRoamable)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsRoamable)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsAllowedInHistory)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsAllowedInHistory)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_RoamingFormats)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_HistoryFormats)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions* This,
        __FIVector_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptionsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_get_IsRoamable(This, value) \
    ((This)->lpVtbl->get_IsRoamable(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_put_IsRoamable(This, value) \
    ((This)->lpVtbl->put_IsRoamable(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_get_IsAllowedInHistory(This, value) \
    ((This)->lpVtbl->get_IsAllowedInHistory(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_put_IsAllowedInHistory(This, value) \
    ((This)->lpVtbl->put_IsAllowedInHistory(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_get_RoamingFormats(This, value) \
    ((This)->lpVtbl->get_RoamingFormats(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_get_HistoryFormats(This, value) \
    ((This)->lpVtbl->get_HistoryFormats(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IClipboardHistoryChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ClipboardHistoryChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IClipboardHistoryChangedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.IClipboardHistoryChangedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IClipboardHistoryItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ClipboardHistoryItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IClipboardHistoryItem[] = L"Windows.ApplicationModel.DataTransfer.IClipboardHistoryItem";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IClipboardHistoryItemsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ClipboardHistoryItemsResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IClipboardHistoryItemsResult[] = L"Windows.ApplicationModel.DataTransfer.IClipboardHistoryItemsResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CClipboardHistoryItemsResultStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Items)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult* This,
        __FIVectorView_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItem** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_get_Items(This, value) \
    ((This)->lpVtbl->get_Items(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItemsResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IClipboardStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.Clipboard
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IClipboardStatics[] = L"Windows.ApplicationModel.DataTransfer.IClipboardStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetContent)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView** result);
    HRESULT (STDMETHODCALLTYPE* SetContent)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* content);
    HRESULT (STDMETHODCALLTYPE* Flush)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics* This);
    HRESULT (STDMETHODCALLTYPE* add_ContentChanged)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ContentChanged)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_GetContent(This, result) \
    ((This)->lpVtbl->GetContent(This, result))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_SetContent(This, content) \
    ((This)->lpVtbl->SetContent(This, content))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_Flush(This) \
    ((This)->lpVtbl->Flush(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_add_ContentChanged(This, handler, token) \
    ((This)->lpVtbl->add_ContentChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_remove_ContentChanged(This, token) \
    ((This)->lpVtbl->remove_ContentChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IClipboardStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.Clipboard
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IClipboardStatics2[] = L"Windows.ApplicationModel.DataTransfer.IClipboardStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetHistoryItemsAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryItemsResult** operation);
    HRESULT (STDMETHODCALLTYPE* ClearHistory)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* DeleteItemFromHistory)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem* item,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetHistoryItemAsContent)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardHistoryItem* item,
        enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CSetHistoryItemAsContentStatus* result);
    HRESULT (STDMETHODCALLTYPE* IsHistoryEnabled)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsRoamingEnabled)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetContentWithOptions)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* content,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardContentOptions* options,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* add_HistoryChanged)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This,
        __FIEventHandler_1_Windows__CApplicationModel__CDataTransfer__CClipboardHistoryChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_HistoryChanged)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_RoamingEnabledChanged)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RoamingEnabledChanged)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_HistoryEnabledChanged)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_HistoryEnabledChanged)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_GetHistoryItemsAsync(This, operation) \
    ((This)->lpVtbl->GetHistoryItemsAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_ClearHistory(This, result) \
    ((This)->lpVtbl->ClearHistory(This, result))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_DeleteItemFromHistory(This, item, result) \
    ((This)->lpVtbl->DeleteItemFromHistory(This, item, result))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_SetHistoryItemAsContent(This, item, result) \
    ((This)->lpVtbl->SetHistoryItemAsContent(This, item, result))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_IsHistoryEnabled(This, result) \
    ((This)->lpVtbl->IsHistoryEnabled(This, result))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_IsRoamingEnabled(This, result) \
    ((This)->lpVtbl->IsRoamingEnabled(This, result))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_SetContentWithOptions(This, content, options, result) \
    ((This)->lpVtbl->SetContentWithOptions(This, content, options, result))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_add_HistoryChanged(This, handler, token) \
    ((This)->lpVtbl->add_HistoryChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_remove_HistoryChanged(This, token) \
    ((This)->lpVtbl->remove_HistoryChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_add_RoamingEnabledChanged(This, handler, token) \
    ((This)->lpVtbl->add_RoamingEnabledChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_remove_RoamingEnabledChanged(This, token) \
    ((This)->lpVtbl->remove_RoamingEnabledChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_add_HistoryEnabledChanged(This, handler, token) \
    ((This)->lpVtbl->add_HistoryEnabledChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_remove_HistoryEnabledChanged(This, token) \
    ((This)->lpVtbl->remove_HistoryEnabledChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIClipboardStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackage[] = L"Windows.ApplicationModel.DataTransfer.IDataPackage";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetView)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView** result);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet** value);
    HRESULT (STDMETHODCALLTYPE* get_RequestedOperation)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDataPackageOperation* value);
    HRESULT (STDMETHODCALLTYPE* put_RequestedOperation)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDataPackageOperation value);
    HRESULT (STDMETHODCALLTYPE* add_OperationCompleted)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__COperationCompletedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_OperationCompleted)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Destroyed)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Destroyed)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* SetData)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        HSTRING formatId,
        IInspectable* value);
    HRESULT (STDMETHODCALLTYPE* SetDataProvider)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        HSTRING formatId,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderHandler* delayRenderer);
    HRESULT (STDMETHODCALLTYPE* SetText)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SetUri may be altered or unavailable for releases after Windows Phone 'OSVersion' (TBD).Instead, use SetWebLink or SetApplicationLink.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* SetUri)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* SetHtmlFormat)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceMap)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        __FIMap_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference** value);
    HRESULT (STDMETHODCALLTYPE* SetRtf)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* SetBitmap)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value);
    HRESULT (STDMETHODCALLTYPE* SetStorageItemsReadOnly)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        __FIIterable_1_Windows__CStorage__CIStorageItem* value);
    HRESULT (STDMETHODCALLTYPE* SetStorageItems)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* This,
        __FIIterable_1_Windows__CStorage__CIStorageItem* value,
        boolean readOnly);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_get_RequestedOperation(This, value) \
    ((This)->lpVtbl->get_RequestedOperation(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_put_RequestedOperation(This, value) \
    ((This)->lpVtbl->put_RequestedOperation(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_add_OperationCompleted(This, handler, token) \
    ((This)->lpVtbl->add_OperationCompleted(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_remove_OperationCompleted(This, token) \
    ((This)->lpVtbl->remove_OperationCompleted(This, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_add_Destroyed(This, handler, token) \
    ((This)->lpVtbl->add_Destroyed(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_remove_Destroyed(This, token) \
    ((This)->lpVtbl->remove_Destroyed(This, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_SetData(This, formatId, value) \
    ((This)->lpVtbl->SetData(This, formatId, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_SetDataProvider(This, formatId, delayRenderer) \
    ((This)->lpVtbl->SetDataProvider(This, formatId, delayRenderer))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_SetText(This, value) \
    ((This)->lpVtbl->SetText(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SetUri may be altered or unavailable for releases after Windows Phone 'OSVersion' (TBD).Instead, use SetWebLink or SetApplicationLink.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_SetUri(This, value) \
    ((This)->lpVtbl->SetUri(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_SetHtmlFormat(This, value) \
    ((This)->lpVtbl->SetHtmlFormat(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_get_ResourceMap(This, value) \
    ((This)->lpVtbl->get_ResourceMap(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_SetRtf(This, value) \
    ((This)->lpVtbl->SetRtf(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_SetBitmap(This, value) \
    ((This)->lpVtbl->SetBitmap(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_SetStorageItemsReadOnly(This, value) \
    ((This)->lpVtbl->SetStorageItemsReadOnly(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_SetStorageItems(This, value, readOnly) \
    ((This)->lpVtbl->SetStorageItems(This, value, readOnly))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackage2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackage2[] = L"Windows.ApplicationModel.DataTransfer.IDataPackage2";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetApplicationLink)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* SetWebLink)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_SetApplicationLink(This, value) \
    ((This)->lpVtbl->SetApplicationLink(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_SetWebLink(This, value) \
    ((This)->lpVtbl->SetWebLink(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackage3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackage3[] = L"Windows.ApplicationModel.DataTransfer.IDataPackage3";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ShareCompleted)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_Windows__CApplicationModel__CDataTransfer__CShareCompletedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ShareCompleted)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_add_ShareCompleted(This, handler, token) \
    ((This)->lpVtbl->add_ShareCompleted(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_remove_ShareCompleted(This, token) \
    ((This)->lpVtbl->remove_ShareCompleted(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackage4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackage4[] = L"Windows.ApplicationModel.DataTransfer.IDataPackage4";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ShareCanceled)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataPackage_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ShareCanceled)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_add_ShareCanceled(This, handler, token) \
    ((This)->lpVtbl->add_ShareCanceled(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_remove_ShareCanceled(This, token) \
    ((This)->lpVtbl->remove_ShareCanceled(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySet
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IMap`2<String, Object>
 *     Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySet[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Thumbnail)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
    HRESULT (STDMETHODCALLTYPE* put_Thumbnail)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value);
    HRESULT (STDMETHODCALLTYPE* get_FileTypes)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_ApplicationName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ApplicationName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ApplicationListingUri)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_ApplicationListingUri)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_get_Thumbnail(This, value) \
    ((This)->lpVtbl->get_Thumbnail(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_put_Thumbnail(This, value) \
    ((This)->lpVtbl->put_Thumbnail(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_get_FileTypes(This, value) \
    ((This)->lpVtbl->get_FileTypes(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_get_ApplicationName(This, value) \
    ((This)->lpVtbl->get_ApplicationName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_put_ApplicationName(This, value) \
    ((This)->lpVtbl->put_ApplicationName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_get_ApplicationListingUri(This, value) \
    ((This)->lpVtbl->get_ApplicationListingUri(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_put_ApplicationListingUri(This, value) \
    ((This)->lpVtbl->put_ApplicationListingUri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySet
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySet2[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet2";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContentSourceWebLink)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_ContentSourceWebLink)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_ContentSourceApplicationLink)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_ContentSourceApplicationLink)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_PackageFamilyName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_PackageFamilyName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Square30x30Logo)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
    HRESULT (STDMETHODCALLTYPE* put_Square30x30Logo)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value);
    HRESULT (STDMETHODCALLTYPE* get_LogoBackgroundColor)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_LogoBackgroundColor)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2* This,
        struct __x_ABI_CWindows_CUI_CColor value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_get_ContentSourceWebLink(This, value) \
    ((This)->lpVtbl->get_ContentSourceWebLink(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_put_ContentSourceWebLink(This, value) \
    ((This)->lpVtbl->put_ContentSourceWebLink(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_get_ContentSourceApplicationLink(This, value) \
    ((This)->lpVtbl->get_ContentSourceApplicationLink(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_put_ContentSourceApplicationLink(This, value) \
    ((This)->lpVtbl->put_ContentSourceApplicationLink(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_get_PackageFamilyName(This, value) \
    ((This)->lpVtbl->get_PackageFamilyName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_put_PackageFamilyName(This, value) \
    ((This)->lpVtbl->put_PackageFamilyName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_get_Square30x30Logo(This, value) \
    ((This)->lpVtbl->get_Square30x30Logo(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_put_Square30x30Logo(This, value) \
    ((This)->lpVtbl->put_Square30x30Logo(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_get_LogoBackgroundColor(This, value) \
    ((This)->lpVtbl->get_LogoBackgroundColor(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_put_LogoBackgroundColor(This, value) \
    ((This)->lpVtbl->put_LogoBackgroundColor(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySet
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySet3[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet3";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EnterpriseId)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_EnterpriseId)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_get_EnterpriseId(This, value) \
    ((This)->lpVtbl->get_EnterpriseId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_put_EnterpriseId(This, value) \
    ((This)->lpVtbl->put_EnterpriseId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySet
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySet4[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet4";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContentSourceUserActivityJson)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ContentSourceUserActivityJson)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_get_ContentSourceUserActivityJson(This, value) \
    ((This)->lpVtbl->get_ContentSourceUserActivityJson(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_put_ContentSourceUserActivityJson(This, value) \
    ((This)->lpVtbl->put_ContentSourceUserActivityJson(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySet4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySetView[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Thumbnail)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
    HRESULT (STDMETHODCALLTYPE* get_FileTypes)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_ApplicationName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ApplicationListingUri)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetViewVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_get_Thumbnail(This, value) \
    ((This)->lpVtbl->get_Thumbnail(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_get_FileTypes(This, value) \
    ((This)->lpVtbl->get_FileTypes(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_get_ApplicationName(This, value) \
    ((This)->lpVtbl->get_ApplicationName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_get_ApplicationListingUri(This, value) \
    ((This)->lpVtbl->get_ApplicationListingUri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySetView2[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView2";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PackageFamilyName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ContentSourceWebLink)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_ContentSourceApplicationLink)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_Square30x30Logo)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
    HRESULT (STDMETHODCALLTYPE* get_LogoBackgroundColor)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2* This,
        struct __x_ABI_CWindows_CUI_CColor* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_get_PackageFamilyName(This, value) \
    ((This)->lpVtbl->get_PackageFamilyName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_get_ContentSourceWebLink(This, value) \
    ((This)->lpVtbl->get_ContentSourceWebLink(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_get_ContentSourceApplicationLink(This, value) \
    ((This)->lpVtbl->get_ContentSourceApplicationLink(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_get_Square30x30Logo(This, value) \
    ((This)->lpVtbl->get_Square30x30Logo(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_get_LogoBackgroundColor(This, value) \
    ((This)->lpVtbl->get_LogoBackgroundColor(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySetView3[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView3";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EnterpriseId)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_get_EnterpriseId(This, value) \
    ((This)->lpVtbl->get_EnterpriseId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySetView4[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView4";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContentSourceUserActivityJson)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_get_ContentSourceUserActivityJson(This, value) \
    ((This)->lpVtbl->get_ContentSourceUserActivityJson(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackagePropertySetView5[] = L"Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView5";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsFromRoamingClipboard)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_get_IsFromRoamingClipboard(This, value) \
    ((This)->lpVtbl->get_IsFromRoamingClipboard(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackageView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackageView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackageView[] = L"Windows.ApplicationModel.DataTransfer.IDataPackageView";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackagePropertySetView** value);
    HRESULT (STDMETHODCALLTYPE* get_RequestedOperation)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDataPackageOperation* value);
    HRESULT (STDMETHODCALLTYPE* ReportOperationCompleted)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDataPackageOperation value);
    HRESULT (STDMETHODCALLTYPE* get_AvailableFormats)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        __FIVectorView_1_HSTRING** formatIds);
    HRESULT (STDMETHODCALLTYPE* Contains)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        HSTRING formatId,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetDataAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        HSTRING formatId,
        __FIAsyncOperation_1_IInspectable** operation);
    HRESULT (STDMETHODCALLTYPE* GetTextAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* GetCustomTextAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        HSTRING formatId,
        __FIAsyncOperation_1_HSTRING** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("GetUriAsync may be altered or unavailable for releases after Windows 8.1. Instead, use GetWebLinkAsync or GetApplicationLinkAsync.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* GetUriAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        __FIAsyncOperation_1_Windows__CFoundation__CUri** operation);
    HRESULT (STDMETHODCALLTYPE* GetHtmlFormatAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* GetResourceMapAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CRandomAccessStreamReference** operation);
    HRESULT (STDMETHODCALLTYPE* GetRtfAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* GetBitmapAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CRandomAccessStreamReference** operation);
    HRESULT (STDMETHODCALLTYPE* GetStorageItemsAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageViewVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_get_RequestedOperation(This, value) \
    ((This)->lpVtbl->get_RequestedOperation(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_ReportOperationCompleted(This, value) \
    ((This)->lpVtbl->ReportOperationCompleted(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_get_AvailableFormats(This, formatIds) \
    ((This)->lpVtbl->get_AvailableFormats(This, formatIds))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_Contains(This, formatId, value) \
    ((This)->lpVtbl->Contains(This, formatId, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_GetDataAsync(This, formatId, operation) \
    ((This)->lpVtbl->GetDataAsync(This, formatId, operation))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_GetTextAsync(This, operation) \
    ((This)->lpVtbl->GetTextAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_GetCustomTextAsync(This, formatId, operation) \
    ((This)->lpVtbl->GetCustomTextAsync(This, formatId, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("GetUriAsync may be altered or unavailable for releases after Windows 8.1. Instead, use GetWebLinkAsync or GetApplicationLinkAsync.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_GetUriAsync(This, operation) \
    ((This)->lpVtbl->GetUriAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_GetHtmlFormatAsync(This, operation) \
    ((This)->lpVtbl->GetHtmlFormatAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_GetResourceMapAsync(This, operation) \
    ((This)->lpVtbl->GetResourceMapAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_GetRtfAsync(This, operation) \
    ((This)->lpVtbl->GetRtfAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_GetBitmapAsync(This, operation) \
    ((This)->lpVtbl->GetBitmapAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_GetStorageItemsAsync(This, operation) \
    ((This)->lpVtbl->GetStorageItemsAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackageView2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackageView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackageView2[] = L"Windows.ApplicationModel.DataTransfer.IDataPackageView2";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetApplicationLinkAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2* This,
        __FIAsyncOperation_1_Windows__CFoundation__CUri** operation);
    HRESULT (STDMETHODCALLTYPE* GetWebLinkAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2* This,
        __FIAsyncOperation_1_Windows__CFoundation__CUri** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_GetApplicationLinkAsync(This, operation) \
    ((This)->lpVtbl->GetApplicationLinkAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_GetWebLinkAsync(This, operation) \
    ((This)->lpVtbl->GetWebLinkAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackageView3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackageView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackageView3[] = L"Windows.ApplicationModel.DataTransfer.IDataPackageView3";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3* This,
        __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult** operation);
    HRESULT (STDMETHODCALLTYPE* RequestAccessWithEnterpriseIdAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3* This,
        HSTRING enterpriseId,
        __FIAsyncOperation_1_Windows__CSecurity__CEnterpriseData__CProtectionPolicyEvaluationResult** operation);
    HRESULT (STDMETHODCALLTYPE* UnlockAndAssumeEnterpriseIdentity)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3* This,
        enum __x_ABI_CWindows_CSecurity_CEnterpriseData_CProtectionPolicyEvaluationResult* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_RequestAccessAsync(This, operation) \
    ((This)->lpVtbl->RequestAccessAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_RequestAccessWithEnterpriseIdAsync(This, enterpriseId, operation) \
    ((This)->lpVtbl->RequestAccessWithEnterpriseIdAsync(This, enterpriseId, operation))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_UnlockAndAssumeEnterpriseIdentity(This, result) \
    ((This)->lpVtbl->UnlockAndAssumeEnterpriseIdentity(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataPackageView4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataPackageView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataPackageView4[] = L"Windows.ApplicationModel.DataTransfer.IDataPackageView4";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetAcceptedFormatId)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4* This,
        HSTRING formatId);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_SetAcceptedFormatId(This, formatId) \
    ((This)->lpVtbl->SetAcceptedFormatId(This, formatId))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataProviderDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataProviderDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataProviderDeferral[] = L"Windows.ApplicationModel.DataTransfer.IDataProviderDeferral";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferralVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataProviderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataProviderRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataProviderRequest[] = L"Windows.ApplicationModel.DataTransfer.IDataProviderRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FormatId)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Deadline)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderDeferral** value);
    HRESULT (STDMETHODCALLTYPE* SetData)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest* This,
        IInspectable* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_get_FormatId(This, value) \
    ((This)->lpVtbl->get_FormatId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_get_Deadline(This, value) \
    ((This)->lpVtbl->get_Deadline(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_SetData(This, value) \
    ((This)->lpVtbl->SetData(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataProviderRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataRequest[] = L"Windows.ApplicationModel.DataTransfer.IDataRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage** value);
    HRESULT (STDMETHODCALLTYPE* put_Data)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* value);
    HRESULT (STDMETHODCALLTYPE* get_Deadline)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* FailWithDisplayText)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_put_Data(This, value) \
    ((This)->lpVtbl->put_Data(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_get_Deadline(This, value) \
    ((This)->lpVtbl->get_Deadline(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_FailWithDisplayText(This, value) \
    ((This)->lpVtbl->FailWithDisplayText(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataRequestDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataRequestDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataRequestDeferral[] = L"Windows.ApplicationModel.DataTransfer.IDataRequestDeferral";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferralVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataRequestedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.IDataRequestedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataTransferManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataTransferManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataTransferManager[] = L"Windows.ApplicationModel.DataTransfer.IDataTransferManager";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_DataRequested)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CDataRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DataRequested)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_TargetApplicationChosen)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CTargetApplicationChosenEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TargetApplicationChosen)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_add_DataRequested(This, handler, token) \
    ((This)->lpVtbl->add_DataRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_remove_DataRequested(This, token) \
    ((This)->lpVtbl->remove_DataRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_add_TargetApplicationChosen(This, handler, token) \
    ((This)->lpVtbl->add_TargetApplicationChosen(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_remove_TargetApplicationChosen(This, token) \
    ((This)->lpVtbl->remove_TargetApplicationChosen(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataTransferManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataTransferManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataTransferManager2[] = L"Windows.ApplicationModel.DataTransfer.IDataTransferManager2";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ShareProvidersRequested)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CDataTransferManager_Windows__CApplicationModel__CDataTransfer__CShareProvidersRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ShareProvidersRequested)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_add_ShareProvidersRequested(This, handler, token) \
    ((This)->lpVtbl->add_ShareProvidersRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_remove_ShareProvidersRequested(This, token) \
    ((This)->lpVtbl->remove_ShareProvidersRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataTransferManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataTransferManagerStatics[] = L"Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ShowShareUI)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManager** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_ShowShareUI(This) \
    ((This)->lpVtbl->ShowShareUI(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_GetForCurrentView(This, result) \
    ((This)->lpVtbl->GetForCurrentView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataTransferManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataTransferManagerStatics2[] = L"Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_IsSupported(This, result) \
    ((This)->lpVtbl->IsSupported(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.DataTransferManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IDataTransferManagerStatics3[] = L"Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics3";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ShowShareUIWithOptions)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions* options);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_ShowShareUIWithOptions(This, options) \
    ((This)->lpVtbl->ShowShareUIWithOptions(This, options))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataTransferManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IHtmlFormatHelperStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.HtmlFormatHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IHtmlFormatHelperStatics[] = L"Windows.ApplicationModel.DataTransfer.IHtmlFormatHelperStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetStaticFragment)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics* This,
        HSTRING htmlFormat,
        HSTRING* htmlFragment);
    HRESULT (STDMETHODCALLTYPE* CreateHtmlFormat)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics* This,
        HSTRING htmlFragment,
        HSTRING* htmlFormat);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_GetStaticFragment(This, htmlFormat, htmlFragment) \
    ((This)->lpVtbl->GetStaticFragment(This, htmlFormat, htmlFragment))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_CreateHtmlFormat(This, htmlFragment, htmlFormat) \
    ((This)->lpVtbl->CreateHtmlFormat(This, htmlFragment, htmlFormat))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIHtmlFormatHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.OperationCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IOperationCompletedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Operation)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs* This,
        enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CDataPackageOperation* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_get_Operation(This, value) \
    ((This)->lpVtbl->get_Operation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.OperationCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IOperationCompletedEventArgs2[] = L"Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs2";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AcceptedFormatId)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_get_AcceptedFormatId(This, value) \
    ((This)->lpVtbl->get_AcceptedFormatId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIOperationCompletedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IShareCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ShareCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IShareCompletedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.IShareCompletedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ShareTarget)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_get_ShareTarget(This, value) \
    ((This)->lpVtbl->get_ShareTarget(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IShareProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ShareProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IShareProvider[] = L"Windows.ApplicationModel.DataTransfer.IShareProvider";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayIcon)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
    HRESULT (STDMETHODCALLTYPE* get_BackgroundColor)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Tag)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* put_Tag)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider* This,
        IInspectable* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_get_DisplayIcon(This, value) \
    ((This)->lpVtbl->get_DisplayIcon(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_get_BackgroundColor(This, value) \
    ((This)->lpVtbl->get_BackgroundColor(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_get_Tag(This, value) \
    ((This)->lpVtbl->get_Tag(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_put_Tag(This, value) \
    ((This)->lpVtbl->put_Tag(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IShareProviderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ShareProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IShareProviderFactory[] = L"Windows.ApplicationModel.DataTransfer.IShareProviderFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory* This,
        HSTRING title,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* displayIcon,
        struct __x_ABI_CWindows_CUI_CColor backgroundColor,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderHandler* handler,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_Create(This, title, displayIcon, backgroundColor, handler, result) \
    ((This)->lpVtbl->Create(This, title, displayIcon, backgroundColor, handler, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IShareProviderOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ShareProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IShareProviderOperation[] = L"Windows.ApplicationModel.DataTransfer.IShareProviderOperation";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView** value);
    HRESULT (STDMETHODCALLTYPE* get_Provider)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider** value);
    HRESULT (STDMETHODCALLTYPE* ReportCompleted)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_get_Provider(This, value) \
    ((This)->lpVtbl->get_Provider(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_ReportCompleted(This) \
    ((This)->lpVtbl->ReportCompleted(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProviderOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IShareProvidersRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ShareProvidersRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IShareProvidersRequestedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.IShareProvidersRequestedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Providers)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs* This,
        __FIVector_1_Windows__CApplicationModel__CDataTransfer__CShareProvider** value);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_get_Providers(This, value) \
    ((This)->lpVtbl->get_Providers(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvidersRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IShareTargetInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ShareTargetInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IShareTargetInfo[] = L"Windows.ApplicationModel.DataTransfer.IShareTargetInfo";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppUserModelId)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ShareProvider)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareProvider** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_get_AppUserModelId(This, value) \
    ((This)->lpVtbl->get_AppUserModelId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_get_ShareProvider(This, value) \
    ((This)->lpVtbl->get_ShareProvider(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareTargetInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IShareUIOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.ShareUIOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IShareUIOptions[] = L"Windows.ApplicationModel.DataTransfer.IShareUIOptions";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Theme)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CShareUITheme* value);
    HRESULT (STDMETHODCALLTYPE* put_Theme)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CDataTransfer_CShareUITheme value);
    HRESULT (STDMETHODCALLTYPE* get_SelectionRect)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions* This,
        __FIReference_1_Windows__CFoundation__CRect** value);
    HRESULT (STDMETHODCALLTYPE* put_SelectionRect)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions* This,
        __FIReference_1_Windows__CFoundation__CRect* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptionsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_get_Theme(This, value) \
    ((This)->lpVtbl->get_Theme(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_put_Theme(This, value) \
    ((This)->lpVtbl->put_Theme(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_get_SelectionRect(This, value) \
    ((This)->lpVtbl->get_SelectionRect(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_put_SelectionRect(This, value) \
    ((This)->lpVtbl->put_SelectionRect(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIShareUIOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ISharedStorageAccessManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.SharedStorageAccessManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ISharedStorageAccessManagerStatics[] = L"Windows.ApplicationModel.DataTransfer.ISharedStorageAccessManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddFile)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        HSTRING* outToken);
    HRESULT (STDMETHODCALLTYPE* RedeemTokenForFileAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics* This,
        HSTRING token,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* RemoveFile)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics* This,
        HSTRING token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_AddFile(This, file, outToken) \
    ((This)->lpVtbl->AddFile(This, file, outToken))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_RedeemTokenForFileAsync(This, token, operation) \
    ((This)->lpVtbl->RedeemTokenForFileAsync(This, token, operation))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_RemoveFile(This, token) \
    ((This)->lpVtbl->RemoveFile(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CISharedStorageAccessManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.StandardDataFormats
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IStandardDataFormatsStatics[] = L"Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Uri may be altered or unavailable for releases after Windows Phone 'OSVersion' (TBD). Instead, use WebLink or ApplicationLink.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Html)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Rtf)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Bitmap)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_StorageItems)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Uri may be altered or unavailable for releases after Windows Phone 'OSVersion' (TBD). Instead, use WebLink or ApplicationLink.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_get_Html(This, value) \
    ((This)->lpVtbl->get_Html(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_get_Rtf(This, value) \
    ((This)->lpVtbl->get_Rtf(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_get_Bitmap(This, value) \
    ((This)->lpVtbl->get_Bitmap(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_get_StorageItems(This, value) \
    ((This)->lpVtbl->get_StorageItems(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.StandardDataFormats
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IStandardDataFormatsStatics2[] = L"Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WebLink)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ApplicationLink)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_get_WebLink(This, value) \
    ((This)->lpVtbl->get_WebLink(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_get_ApplicationLink(This, value) \
    ((This)->lpVtbl->get_ApplicationLink(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.StandardDataFormats
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_IStandardDataFormatsStatics3[] = L"Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics3";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UserActivityJsonArray)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_get_UserActivityJsonArray(This, value) \
    ((This)->lpVtbl->get_UserActivityJsonArray(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIStandardDataFormatsStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITargetApplicationChosenEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TargetApplicationChosenEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITargetApplicationChosenEventArgs[] = L"Windows.ApplicationModel.DataTransfer.ITargetApplicationChosenEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ApplicationName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_get_ApplicationName(This, value) \
    ((This)->lpVtbl->get_ApplicationName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITargetApplicationChosenEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITransferTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TransferTarget
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITransferTarget[] = L"Windows.ApplicationModel.DataTransfer.ITransferTarget";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Label)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayIcon)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_get_Label(This, value) \
    ((This)->lpVtbl->get_Label(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_get_DisplayIcon(This, value) \
    ((This)->lpVtbl->get_DisplayIcon(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITransferTargetChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TransferTargetChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITransferTargetChangedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.ITransferTargetChangedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Target)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_get_Target(This, value) \
    ((This)->lpVtbl->get_Target(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITransferTargetDiscoveryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TransferTargetDiscoveryOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITransferTargetDiscoveryOptions[] = L"Windows.ApplicationModel.DataTransfer.ITransferTargetDiscoveryOptions";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DataPackage)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView** value);
    HRESULT (STDMETHODCALLTYPE* get_MaxAppTargets)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_MaxAppTargets)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_AllowedTargetAppIds)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions* This,
        UINT32* valueLength,
        HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* put_AllowedTargetAppIds)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions* This,
        UINT32 valueLength,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_get_DataPackage(This, value) \
    ((This)->lpVtbl->get_DataPackage(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_get_MaxAppTargets(This, value) \
    ((This)->lpVtbl->get_MaxAppTargets(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_put_MaxAppTargets(This, value) \
    ((This)->lpVtbl->put_MaxAppTargets(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_get_AllowedTargetAppIds(This, valueLength, value) \
    ((This)->lpVtbl->get_AllowedTargetAppIds(This, valueLength, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_put_AllowedTargetAppIds(This, valueLength, value) \
    ((This)->lpVtbl->put_AllowedTargetAppIds(This, valueLength, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITransferTargetDiscoveryOptionsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TransferTargetDiscoveryOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITransferTargetDiscoveryOptionsFactory[] = L"Windows.ApplicationModel.DataTransfer.ITransferTargetDiscoveryOptionsFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* dataPackage,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_CreateInstance(This, dataPackage, value) \
    ((This)->lpVtbl->CreateInstance(This, dataPackage, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITransferTargetInvokeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TransferTargetInvokeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITransferTargetInvokeResult[] = L"Windows.ApplicationModel.DataTransfer.ITransferTargetInvokeResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Succeeded)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_get_Succeeded(This, value) \
    ((This)->lpVtbl->get_Succeeded(This, value))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetInvokeResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITransferTargetStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TransferTarget
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITransferTargetStatics[] = L"Windows.ApplicationModel.DataTransfer.ITransferTargetStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWatcher)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetDiscoveryOptions* options,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_CreateWatcher(This, options, result) \
    ((This)->lpVtbl->CreateWatcher(This, options, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITransferTargetWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TransferTargetWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITransferTargetWatcher[] = L"Windows.ApplicationModel.DataTransfer.ITransferTargetWatcher";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This);
    HRESULT (STDMETHODCALLTYPE* TransferToAsync)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTarget* target,
        struct __x_ABI_CWindows_CUI_CWindowId parentWindowHandle,
        __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetInvokeResult_double** operation);
    HRESULT (STDMETHODCALLTYPE* add_Added)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Added)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Removed)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Removed)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Updated)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_Windows__CApplicationModel__CDataTransfer__CTransferTargetChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Updated)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_EnumerationCompleted)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EnumerationCompleted)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Stopped)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CDataTransfer__CTransferTargetWatcher_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Stopped)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_TransferToAsync(This, target, parentWindowHandle, operation) \
    ((This)->lpVtbl->TransferToAsync(This, target, parentWindowHandle, operation))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_add_Added(This, handler, token) \
    ((This)->lpVtbl->add_Added(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_remove_Added(This, token) \
    ((This)->lpVtbl->remove_Added(This, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_add_Removed(This, handler, token) \
    ((This)->lpVtbl->add_Removed(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_remove_Removed(This, token) \
    ((This)->lpVtbl->remove_Removed(This, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_add_Updated(This, handler, token) \
    ((This)->lpVtbl->add_Updated(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_remove_Updated(This, token) \
    ((This)->lpVtbl->remove_Updated(This, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_add_EnumerationCompleted(This, handler, token) \
    ((This)->lpVtbl->add_EnumerationCompleted(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_remove_EnumerationCompleted(This, token) \
    ((This)->lpVtbl->remove_EnumerationCompleted(This, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_add_Stopped(This, handler, token) \
    ((This)->lpVtbl->add_Stopped(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_remove_Stopped(This, token) \
    ((This)->lpVtbl->remove_Stopped(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.ApplicationModel.DataTransfer.ITransferTargetWatcherStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DataTransfer.TransferTargetWatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_DataTransfer_ITransferTargetWatcherStatics[] = L"Windows.ApplicationModel.DataTransfer.ITransferTargetWatcherStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackageView* dataPackage,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_IsSupported(This, dataPackage, result) \
    ((This)->lpVtbl->IsSupported(This, dataPackage, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CDataTransfer_CITransferTargetWatcherStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.Clipboard
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IClipboardStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IClipboardStatics2 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_Clipboard_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_Clipboard_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_Clipboard[] = L"Windows.ApplicationModel.DataTransfer.Clipboard";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ClipboardContentOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IClipboardContentOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ClipboardContentOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ClipboardContentOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ClipboardContentOptions[] = L"Windows.ApplicationModel.DataTransfer.ClipboardContentOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ClipboardHistoryChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IClipboardHistoryChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ClipboardHistoryChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ClipboardHistoryChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ClipboardHistoryChangedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.ClipboardHistoryChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ClipboardHistoryItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IClipboardHistoryItem ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ClipboardHistoryItem_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ClipboardHistoryItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ClipboardHistoryItem[] = L"Windows.ApplicationModel.DataTransfer.ClipboardHistoryItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ClipboardHistoryItemsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IClipboardHistoryItemsResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ClipboardHistoryItemsResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ClipboardHistoryItemsResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ClipboardHistoryItemsResult[] = L"Windows.ApplicationModel.DataTransfer.ClipboardHistoryItemsResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataPackage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataPackage ** Default Interface **
 *    Windows.ApplicationModel.DataTransfer.IDataPackage2
 *    Windows.ApplicationModel.DataTransfer.IDataPackage3
 *    Windows.ApplicationModel.DataTransfer.IDataPackage4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataPackage_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataPackage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataPackage[] = L"Windows.ApplicationModel.DataTransfer.DataPackage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataPackagePropertySet
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet ** Default Interface **
 *    Windows.Foundation.Collections.IMap`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet2
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet3
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySet4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataPackagePropertySet_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataPackagePropertySet_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataPackagePropertySet[] = L"Windows.ApplicationModel.DataTransfer.DataPackagePropertySet";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView ** Default Interface **
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView2
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView3
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView4
 *    Windows.ApplicationModel.DataTransfer.IDataPackagePropertySetView5
 *    Windows.Foundation.Collections.IMapView`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataPackagePropertySetView_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataPackagePropertySetView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataPackagePropertySetView[] = L"Windows.ApplicationModel.DataTransfer.DataPackagePropertySetView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataPackageView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataPackageView ** Default Interface **
 *    Windows.ApplicationModel.DataTransfer.IDataPackageView2
 *    Windows.ApplicationModel.DataTransfer.IDataPackageView3
 *    Windows.ApplicationModel.DataTransfer.IDataPackageView4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataPackageView_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataPackageView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataPackageView[] = L"Windows.ApplicationModel.DataTransfer.DataPackageView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataProviderDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataProviderDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataProviderDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataProviderDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataProviderDeferral[] = L"Windows.ApplicationModel.DataTransfer.DataProviderDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataProviderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataProviderRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataProviderRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataProviderRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataProviderRequest[] = L"Windows.ApplicationModel.DataTransfer.DataProviderRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataRequest[] = L"Windows.ApplicationModel.DataTransfer.DataRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataRequestDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataRequestDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataRequestDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataRequestDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataRequestDeferral[] = L"Windows.ApplicationModel.DataTransfer.DataRequestDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataRequestedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.DataRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.DataTransferManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics3 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IDataTransferManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IDataTransferManager ** Default Interface **
 *    Windows.ApplicationModel.DataTransfer.IDataTransferManager2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataTransferManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_DataTransferManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_DataTransferManager[] = L"Windows.ApplicationModel.DataTransfer.DataTransferManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.HtmlFormatHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IHtmlFormatHelperStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_HtmlFormatHelper_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_HtmlFormatHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_HtmlFormatHelper[] = L"Windows.ApplicationModel.DataTransfer.HtmlFormatHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.OperationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.DataTransfer.IOperationCompletedEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_OperationCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_OperationCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_OperationCompletedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.OperationCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ShareCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IShareCompletedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ShareCompletedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.ShareCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ShareProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.DataTransfer.IShareProviderFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IShareProvider ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareProvider_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ShareProvider[] = L"Windows.ApplicationModel.DataTransfer.ShareProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ShareProviderOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IShareProviderOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareProviderOperation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareProviderOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ShareProviderOperation[] = L"Windows.ApplicationModel.DataTransfer.ShareProviderOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ShareProvidersRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IShareProvidersRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareProvidersRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareProvidersRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ShareProvidersRequestedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.ShareProvidersRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ShareTargetInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IShareTargetInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareTargetInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareTargetInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ShareTargetInfo[] = L"Windows.ApplicationModel.DataTransfer.ShareTargetInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.ShareUIOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.IShareUIOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareUIOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_ShareUIOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_ShareUIOptions[] = L"Windows.ApplicationModel.DataTransfer.ShareUIOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.SharedStorageAccessManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.ISharedStorageAccessManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_SharedStorageAccessManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_SharedStorageAccessManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_SharedStorageAccessManager[] = L"Windows.ApplicationModel.DataTransfer.SharedStorageAccessManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.StandardDataFormats
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics3 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.IStandardDataFormatsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_StandardDataFormats_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_StandardDataFormats_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_StandardDataFormats[] = L"Windows.ApplicationModel.DataTransfer.StandardDataFormats";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.TargetApplicationChosenEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.ITargetApplicationChosenEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TargetApplicationChosenEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TargetApplicationChosenEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_TargetApplicationChosenEventArgs[] = L"Windows.ApplicationModel.DataTransfer.TargetApplicationChosenEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.TransferTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.ITransferTargetStatics interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.ITransferTarget ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTarget_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTarget_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_TransferTarget[] = L"Windows.ApplicationModel.DataTransfer.TransferTarget";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.TransferTargetChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.ITransferTargetChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTargetChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTargetChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_TransferTargetChangedEventArgs[] = L"Windows.ApplicationModel.DataTransfer.TransferTargetChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.TransferTargetDiscoveryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.DataTransfer.ITransferTargetDiscoveryOptionsFactory interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.ITransferTargetDiscoveryOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTargetDiscoveryOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTargetDiscoveryOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_TransferTargetDiscoveryOptions[] = L"Windows.ApplicationModel.DataTransfer.TransferTargetDiscoveryOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.TransferTargetInvokeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.ITransferTargetInvokeResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTargetInvokeResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTargetInvokeResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_TransferTargetInvokeResult[] = L"Windows.ApplicationModel.DataTransfer.TransferTargetInvokeResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.ApplicationModel.DataTransfer.TransferTargetWatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.DataTransfer.ITransferTargetWatcherStatics interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.DataTransfer.ITransferTargetWatcher ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTargetWatcher_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DataTransfer_TransferTargetWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DataTransfer_TransferTargetWatcher[] = L"Windows.ApplicationModel.DataTransfer.TransferTargetWatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Edatatransfer_p_h__

#endif // __windows2Eapplicationmodel2Edatatransfer_h__
