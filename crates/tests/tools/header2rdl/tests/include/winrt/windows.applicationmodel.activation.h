
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
#ifndef __windows2Eapplicationmodel2Eactivation_h__
#define __windows2Eapplicationmodel2Eactivation_h__
#ifndef __windows2Eapplicationmodel2Eactivation_p_h__
#define __windows2Eapplicationmodel2Eactivation_p_h__


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

#if !defined(WINDOWS_APPLICATIONMODEL_BACKGROUND_BACKGROUNDALARMAPPLICATIONCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_BACKGROUND_BACKGROUNDALARMAPPLICATIONCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_BACKGROUND_BACKGROUNDALARMAPPLICATIONCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION 0x50000
#endif // defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION)

#if !defined(WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION)
#define WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)
#define WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)
#define WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION 0x130000
#endif // defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)

#if !defined(WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION)
#define WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION)

#if !defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)
#define WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)

#if !defined(WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION)
#define WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.ApplicationModel.Appointments.AppointmentsProvider.h"
#include "Windows.ApplicationModel.Background.h"
#include "Windows.ApplicationModel.Calls.h"
#include "Windows.ApplicationModel.Contacts.h"
#include "Windows.ApplicationModel.Contacts.Provider.h"
#include "Windows.ApplicationModel.DataTransfer.ShareTarget.h"
#include "Windows.ApplicationModel.Search.h"
#include "Windows.ApplicationModel.UserDataAccounts.Provider.h"
#include "Windows.ApplicationModel.Wallet.h"
#include "Windows.Devices.Enumeration.h"
#include "Windows.Devices.Printers.Extensions.h"
#include "Windows.Media.SpeechRecognition.h"
#include "Windows.Security.Authentication.Web.h"
#include "Windows.Security.Authentication.Web.Provider.h"
#include "Windows.Storage.h"
#include "Windows.Storage.Pickers.Provider.h"
#include "Windows.Storage.Provider.h"
#include "Windows.Storage.Search.h"
#include "Windows.System.h"
#include "Windows.UI.Notifications.h"
#include "Windows.UI.ViewManagement.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
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

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IActivatedEventArgsWithUser;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser ABI::Windows::ApplicationModel::Activation::IActivatedEventArgsWithUser

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IApplicationViewActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IApplicationViewActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IAppointmentsProviderActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IAppointmentsProviderAddAppointmentActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IAppointmentsProviderRemoveAppointmentActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IAppointmentsProviderReplaceAppointmentActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IAppointmentsProviderShowTimeFrameActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IBackgroundActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IBackgroundActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IBarcodeScannerPreviewActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ICachedFileUpdaterActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs ABI::Windows::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ICameraSettingsActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs ABI::Windows::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ICommandLineActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs ABI::Windows::ApplicationModel::Activation::ICommandLineActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ICommandLineActivationOperation;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation ABI::Windows::ApplicationModel::Activation::ICommandLineActivationOperation

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContactActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContactActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContactCallActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContactCallActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContactMapActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContactMapActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContactMessageActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContactMessageActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContactPanelActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContactPanelActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContactPickerActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContactPickerActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContactPostActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContactPostActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContactVideoCallActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContactsProviderActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContactsProviderActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContinuationActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContinuationActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IDeviceActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IDeviceActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IDevicePairingActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IDevicePairingActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IDialReceiverActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IDialReceiverActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFileActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IFileActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFileActivatedEventArgsWithCallerPackageFamilyName;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName ABI::Windows::ApplicationModel::Activation::IFileActivatedEventArgsWithCallerPackageFamilyName

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFileActivatedEventArgsWithNeighboringFiles;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles ABI::Windows::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFileOpenPickerActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFileOpenPickerActivatedEventArgs2;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2 ABI::Windows::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFileOpenPickerContinuationEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs ABI::Windows::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFileSavePickerActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFileSavePickerActivatedEventArgs2;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2 ABI::Windows::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFileSavePickerContinuationEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs ABI::Windows::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFolderPickerContinuationEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs ABI::Windows::ApplicationModel::Activation::IFolderPickerContinuationEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ILaunchActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs ABI::Windows::ApplicationModel::Activation::ILaunchActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ILaunchActivatedEventArgs2;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2 ABI::Windows::ApplicationModel::Activation::ILaunchActivatedEventArgs2

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ILockScreenActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs ABI::Windows::ApplicationModel::Activation::ILockScreenActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ILockScreenCallActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs ABI::Windows::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IPhoneCallActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IPhoneCallActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IPickerReturnedActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IPickerReturnedActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IPrelaunchActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IPrelaunchActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IPrint3DWorkflowActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IPrintTaskSettingsActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IProtocolActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IProtocolActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData ABI::Windows::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IProtocolForResultsActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IRestrictedLaunchActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ISearchActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs ABI::Windows::ApplicationModel::Activation::ISearchActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ISearchActivatedEventArgsWithLinguisticDetails;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails ABI::Windows::ApplicationModel::Activation::ISearchActivatedEventArgsWithLinguisticDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IShareTargetActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IShareTargetActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ISplashScreen;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen ABI::Windows::ApplicationModel::Activation::ISplashScreen

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IStartupTaskActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IStartupTaskActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ITileActivatedInfo;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo ABI::Windows::ApplicationModel::Activation::ITileActivatedInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IToastNotificationActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IToastNotificationActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IUserDataAccountProviderActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IViewSwitcherProvider;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider ABI::Windows::ApplicationModel::Activation::IViewSwitcherProvider

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IVoiceCommandActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IWalletActionActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IWalletActionActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IWebAccountProviderActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IWebAuthenticationBrokerContinuationEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs ABI::Windows::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
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

#ifndef DEF___FIIterator_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIIterator_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("43e29f53-0298-55aa-a6c8-4edd323d9598"))
IIterator<ABI::Windows::Storage::StorageFile*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFile*, ABI::Windows::Storage::IStorageFile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Storage.StorageFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Storage::StorageFile*> __FIIterator_1_Windows__CStorage__CStorageFile_t;
#define __FIIterator_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CStorage__CStorageFile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIIterable_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9ac00304-83ea-5688-87b6-ae38aab65d0b"))
IIterable<ABI::Windows::Storage::StorageFile*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFile*, ABI::Windows::Storage::IStorageFile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Storage.StorageFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Storage::StorageFile*> __FIIterable_1_Windows__CStorage__CStorageFile_t;
#define __FIIterable_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CStorage__CStorageFile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class ShownTileNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IShownTileNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification ABI::Windows::UI::Notifications::IShownTileNotification

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_USE
#define DEF___FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("04d8d70b-7c15-5f60-9894-b21366b427c2"))
IIterator<ABI::Windows::UI::Notifications::ShownTileNotification*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ShownTileNotification*, ABI::Windows::UI::Notifications::IShownTileNotification*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Notifications.ShownTileNotification>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Notifications::ShownTileNotification*> __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_t;
#define __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification_USE
#define DEF___FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1863baee-44f1-5e51-bcdf-a3cdab826a15"))
IIterable<ABI::Windows::UI::Notifications::ShownTileNotification*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ShownTileNotification*, ABI::Windows::UI::Notifications::IShownTileNotification*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Notifications.ShownTileNotification>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Notifications::ShownTileNotification*> __FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification_t;
#define __FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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

#ifndef DEF___FIVectorView_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIVectorView_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("80646519-5e2a-595d-a8cd-2a24b4067f1b"))
IVectorView<ABI::Windows::Storage::StorageFile*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFile*, ABI::Windows::Storage::IStorageFile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Storage.StorageFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Storage::StorageFile*> __FIVectorView_1_Windows__CStorage__CStorageFile_t;
#define __FIVectorView_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CStorage__CStorageFile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_USE
#define DEF___FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2b23baa9-1d54-5440-bd32-86ed70f15c9e"))
IVectorView<ABI::Windows::UI::Notifications::ShownTileNotification*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ShownTileNotification*, ABI::Windows::UI::Notifications::IShownTileNotification*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Notifications.ShownTileNotification>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Notifications::ShownTileNotification*> __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_t;
#define __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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
            namespace Activation {
                class SplashScreen;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7725b2a5-287d-5ed2-a789-2a6a2673c7fe"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Activation::SplashScreen*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Activation::SplashScreen*, ABI::Windows::ApplicationModel::Activation::ISplashScreen*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Activation.SplashScreen, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Activation::SplashScreen*, IInspectable*> __FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace AppointmentsProvider {
                    class AddAppointmentOperation;
                } /* AppointmentsProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIAddAppointmentOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIAddAppointmentOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace AppointmentsProvider {
                    interface IAddAppointmentOperation;
                } /* AppointmentsProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIAddAppointmentOperation ABI::Windows::ApplicationModel::Appointments::AppointmentsProvider::IAddAppointmentOperation

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIAddAppointmentOperation_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace AppointmentsProvider {
                    class RemoveAppointmentOperation;
                } /* AppointmentsProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIRemoveAppointmentOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIRemoveAppointmentOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace AppointmentsProvider {
                    interface IRemoveAppointmentOperation;
                } /* AppointmentsProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIRemoveAppointmentOperation ABI::Windows::ApplicationModel::Appointments::AppointmentsProvider::IRemoveAppointmentOperation

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIRemoveAppointmentOperation_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace AppointmentsProvider {
                    class ReplaceAppointmentOperation;
                } /* AppointmentsProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIReplaceAppointmentOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIReplaceAppointmentOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace AppointmentsProvider {
                    interface IReplaceAppointmentOperation;
                } /* AppointmentsProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIReplaceAppointmentOperation ABI::Windows::ApplicationModel::Appointments::AppointmentsProvider::IReplaceAppointmentOperation

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIReplaceAppointmentOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Background {
                interface IBackgroundTaskInstance;
            } /* Background */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance ABI::Windows::ApplicationModel::Background::IBackgroundTaskInstance

#endif // ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class LockScreenCallUI;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface ILockScreenCallUI;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI ABI::Windows::ApplicationModel::Calls::ILockScreenCallUI

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                class Contact;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                interface IContact;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CIContact ABI::Windows::ApplicationModel::Contacts::IContact

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                class ContactAddress;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactAddress_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactAddress_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                interface IContactAddress;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CIContactAddress ABI::Windows::ApplicationModel::Contacts::IContactAddress

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactAddress_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                class ContactPanel;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactPanel_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactPanel_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                interface IContactPanel;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CIContactPanel ABI::Windows::ApplicationModel::Contacts::IContactPanel

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactPanel_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace Provider {
                    class ContactPickerUI;
                } /* Provider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace Provider {
                    interface IContactPickerUI;
                } /* Provider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI ABI::Windows::ApplicationModel::Contacts::Provider::IContactPickerUI

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace ShareTarget {
                    class ShareOperation;
                } /* ShareTarget */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CShareTarget_CIShareOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CShareTarget_CIShareOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                namespace ShareTarget {
                    interface IShareOperation;
                } /* ShareTarget */
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CShareTarget_CIShareOperation ABI::Windows::ApplicationModel::DataTransfer::ShareTarget::IShareOperation

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CShareTarget_CIShareOperation_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                class SearchPaneQueryLinguisticDetails;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Search {
                interface ISearchPaneQueryLinguisticDetails;
            } /* Search */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails ABI::Windows::ApplicationModel::Search::ISearchPaneQueryLinguisticDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace Provider {
                    interface IUserDataAccountProviderOperation;
                } /* Provider */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation ABI::Windows::ApplicationModel::UserDataAccounts::Provider::IUserDataAccountProviderOperation

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                typedef enum WalletActionKind : int WalletActionKind;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

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
            namespace Printers {
                namespace Extensions {
                    class Print3DWorkflow;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    interface IPrint3DWorkflow;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow ABI::Windows::Devices::Printers::Extensions::IPrint3DWorkflow

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    class PrintTaskConfiguration;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                namespace Extensions {
                    interface IPrintTaskConfiguration;
                } /* Extensions */
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration ABI::Windows::Devices::Printers::Extensions::IPrintTaskConfiguration

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                class ValueSet;
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
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                class SpeechRecognitionResult;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace SpeechRecognition {
                interface ISpeechRecognitionResult;
            } /* SpeechRecognition */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionResult

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountProviderOperation;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation ABI::Windows::Security::Authentication::Web::Provider::IWebAccountProviderOperation

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    class WebAuthenticationResult;
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    interface IWebAuthenticationResult;
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult ABI::Windows::Security::Authentication::Web::IWebAuthenticationResult

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    class FileOpenPickerUI;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    interface IFileOpenPickerUI;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI ABI::Windows::Storage::Pickers::Provider::IFileOpenPickerUI

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    class FileSavePickerUI;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Pickers {
                namespace Provider {
                    interface IFileSavePickerUI;
                } /* Provider */
            } /* Pickers */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI ABI::Windows::Storage::Pickers::Provider::IFileSavePickerUI

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Provider {
                class CachedFileUpdaterUI;
            } /* Provider */
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CProvider_CICachedFileUpdaterUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CProvider_CICachedFileUpdaterUI_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Provider {
                interface ICachedFileUpdaterUI;
            } /* Provider */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CProvider_CICachedFileUpdaterUI ABI::Windows::Storage::Provider::ICachedFileUpdaterUI

#endif // ____x_ABI_CWindows_CStorage_CProvider_CICachedFileUpdaterUI_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Search {
                class StorageFileQueryResult;
            } /* Search */
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CSearch_CIStorageFileQueryResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CSearch_CIStorageFileQueryResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Search {
                interface IStorageFileQueryResult;
            } /* Search */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CSearch_CIStorageFileQueryResult ABI::Windows::Storage::Search::IStorageFileQueryResult

#endif // ____x_ABI_CWindows_CStorage_CSearch_CIStorageFileQueryResult_FWD_DEFINED__

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
        namespace System {
            class ProtocolForResultsOperation;
        } /* System */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CIProtocolForResultsOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIProtocolForResultsOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            interface IProtocolForResultsOperation;
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CIProtocolForResultsOperation ABI::Windows::System::IProtocolForResultsOperation

#endif // ____x_ABI_CWindows_CSystem_CIProtocolForResultsOperation_FWD_DEFINED__

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
        namespace UI {
            namespace ViewManagement {
                class ActivationViewSwitcher;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IActivationViewSwitcher;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher ABI::Windows::UI::ViewManagement::IActivationViewSwitcher

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                typedef enum ActivationKind : int ActivationKind;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                typedef enum ApplicationExecutionState : int ApplicationExecutionState;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                class CommandLineActivationOperation;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                class TileActivatedInfo;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.Activation.ActivationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                enum ActivationKind : int
                {
                    ActivationKind_Launch = 0,
                    ActivationKind_Search = 1,
                    ActivationKind_ShareTarget = 2,
                    ActivationKind_File = 3,
                    ActivationKind_Protocol = 4,
                    ActivationKind_FileOpenPicker = 5,
                    ActivationKind_FileSavePicker = 6,
                    ActivationKind_CachedFileUpdater = 7,
                    ActivationKind_ContactPicker = 8,
                    ActivationKind_Device = 9,
                    ActivationKind_PrintTaskSettings = 10,
                    ActivationKind_CameraSettings = 11,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ActivationKind_RestrictedLaunch = 12,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ActivationKind_AppointmentsProvider = 13,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ActivationKind_Contact = 14,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ActivationKind_LockScreenCall = 15,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ActivationKind_VoiceCommand = 16,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ActivationKind_LockScreen = 17,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ActivationKind_PickerReturned = 1000,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ActivationKind_WalletAction = 1001,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ActivationKind_PickFileContinuation = 1002,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ActivationKind_PickSaveFileContinuation = 1003,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ActivationKind_PickFolderContinuation = 1004,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ActivationKind_WebAuthenticationBrokerContinuation = 1005,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ActivationKind_WebAccountProvider = 1006,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ActivationKind_ComponentUI = 1007,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ActivationKind_ProtocolForResults = 1009,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ActivationKind_ToastNotification = 1010,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
                    ActivationKind_Print3DWorkflow = 1011,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ActivationKind_DialReceiver = 1012,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
                    ActivationKind_DevicePairing = 1013,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    ActivationKind_UserDataAccountsProvider = 1014,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    ActivationKind_FilePickerExperience = 1015,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    ActivationKind_LockScreenComponent = 1016,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    ActivationKind_ContactPanel = 1017,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                    ActivationKind_PrintWorkflowForegroundTask = 1018,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                    ActivationKind_GameUIProvider = 1019,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                    ActivationKind_StartupTask = 1020,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                    ActivationKind_CommandLineLaunch = 1021,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    ActivationKind_BarcodeScannerProvider = 1022,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                    ActivationKind_PrintSupportJobUI = 1023,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                    ActivationKind_PrintSupportSettingsUI = 1024,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                    ActivationKind_PhoneCallActivation = 1025,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                    ActivationKind_VpnForeground = 1026,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
                    ActivationKind_PrintSupportEnterpriseManagementUI = 1027,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
                };
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Activation.ApplicationExecutionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                enum ApplicationExecutionState : int
                {
                    ApplicationExecutionState_NotRunning = 0,
                    ApplicationExecutionState_Running = 1,
                    ApplicationExecutionState_Suspended = 2,
                    ApplicationExecutionState_Terminated = 3,
                    ApplicationExecutionState_ClosedByUser = 4,
                };
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("cf651713-cd08-4fd8-b697-a281b6544e2e")
                IActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::ApplicationModel::Activation::ActivationKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PreviousExecutionState(
                        ABI::Windows::ApplicationModel::Activation::ApplicationExecutionState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SplashScreen(
                        ABI::Windows::ApplicationModel::Activation::ISplashScreen** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActivatedEventArgs = __uuidof(IActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IActivatedEventArgsWithUser[] = L"Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("1cf09b9e-9962-4936-80ff-afc8e8ae5c8c")
                IActivatedEventArgsWithUser : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActivatedEventArgsWithUser = __uuidof(IActivatedEventArgsWithUser);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IApplicationViewActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("930cef4b-b829-40fc-88f4-8513e8a64738")
                IApplicationViewActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentlyShownApplicationViewId(
                        INT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationViewActivatedEventArgs = __uuidof(IApplicationViewActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IAppointmentsProviderActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("3364c405-933c-4e7d-a034-500fb8dcd9f3")
                IAppointmentsProviderActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Verb(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentsProviderActivatedEventArgs = __uuidof(IAppointmentsProviderActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IAppointmentsProviderAddAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IAppointmentsProviderAddAppointmentActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IAppointmentsProviderAddAppointmentActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("a2861367-cee5-4e4d-9ed7-41c34ec18b02")
                IAppointmentsProviderAddAppointmentActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AddAppointmentOperation(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentsProvider::IAddAppointmentOperation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentsProviderAddAppointmentActivatedEventArgs = __uuidof(IAppointmentsProviderAddAppointmentActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IAppointmentsProviderRemoveAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IAppointmentsProviderRemoveAppointmentActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IAppointmentsProviderRemoveAppointmentActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("751f3ab8-0b8e-451c-9f15-966e699bac25")
                IAppointmentsProviderRemoveAppointmentActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RemoveAppointmentOperation(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentsProvider::IRemoveAppointmentOperation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentsProviderRemoveAppointmentActivatedEventArgs = __uuidof(IAppointmentsProviderRemoveAppointmentActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IAppointmentsProviderReplaceAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IAppointmentsProviderReplaceAppointmentActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IAppointmentsProviderReplaceAppointmentActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("1551b7d4-a981-4067-8a62-0524e4ade121")
                IAppointmentsProviderReplaceAppointmentActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ReplaceAppointmentOperation(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentsProvider::IReplaceAppointmentOperation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentsProviderReplaceAppointmentActivatedEventArgs = __uuidof(IAppointmentsProviderReplaceAppointmentActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("3958f065-9841-4ca5-999b-885198b9ef2a")
                IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_InstanceStartDate(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocalId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RoamingId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs = __uuidof(IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IAppointmentsProviderShowTimeFrameActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IAppointmentsProviderShowTimeFrameActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IAppointmentsProviderShowTimeFrameActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("9baeaba6-0e0b-49aa-babc-12b1dc774986")
                IAppointmentsProviderShowTimeFrameActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TimeToShow(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Duration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentsProviderShowTimeFrameActivatedEventArgs = __uuidof(IAppointmentsProviderShowTimeFrameActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IBackgroundActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IBackgroundActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IBackgroundActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("ab14bee0-e760-440e-a91c-44796de3a92d")
                IBackgroundActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TaskInstance(
                        ABI::Windows::ApplicationModel::Background::IBackgroundTaskInstance** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundActivatedEventArgs = __uuidof(IBackgroundActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IBarcodeScannerPreviewActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IBarcodeScannerPreviewActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IBarcodeScannerPreviewActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("6772797c-99bf-4349-af22-e4123560371c")
                IBarcodeScannerPreviewActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBarcodeScannerPreviewActivatedEventArgs = __uuidof(IBarcodeScannerPreviewActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ICachedFileUpdaterActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ICachedFileUpdaterActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ICachedFileUpdaterActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("d06eb1c7-3805-4ecb-b757-6cf15e26fef3")
                ICachedFileUpdaterActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CachedFileUpdaterUI(
                        ABI::Windows::Storage::Provider::ICachedFileUpdaterUI** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICachedFileUpdaterActivatedEventArgs = __uuidof(ICachedFileUpdaterActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ICameraSettingsActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivationCameraSettingsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ICameraSettingsActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ICameraSettingsActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("fb67a508-2dad-490a-9170-dca036eb114b")
                ICameraSettingsActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_VideoDeviceController(
                        IInspectable** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VideoDeviceExtension(
                        IInspectable** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICameraSettingsActivatedEventArgs = __uuidof(ICameraSettingsActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ICommandLineActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ICommandLineActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ICommandLineActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("4506472c-006a-48eb-8afb-d07ab25e3366")
                ICommandLineActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Operation(
                        ABI::Windows::ApplicationModel::Activation::ICommandLineActivationOperation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICommandLineActivatedEventArgs = __uuidof(ICommandLineActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ICommandLineActivationOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Activation.CommandLineActivationOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ICommandLineActivationOperation[] = L"Windows.ApplicationModel.Activation.ICommandLineActivationOperation";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("994b2841-c59e-4f69-bcfd-b61ed4e622eb")
                ICommandLineActivationOperation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Arguments(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentDirectoryPath(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ExitCode(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExitCode(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICommandLineActivationOperation = __uuidof(ICommandLineActivationOperation);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("d627a1c4-c025-4c41-9def-f1eafad075e7")
                IContactActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Verb(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IContactActivatedEventArgs = __uuidof(IContactActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactCallActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactCallActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("c2df14c7-30eb-41c6-b3bc-5b1694f9dab3")
                IContactCallActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceUserId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Contact(
                        ABI::Windows::ApplicationModel::Contacts::IContact** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IContactCallActivatedEventArgs = __uuidof(IContactCallActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactMapActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactMapActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactMapActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("b32bf870-eee7-4ad2-aaf1-a87effcf00a4")
                IContactMapActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Address(
                        ABI::Windows::ApplicationModel::Contacts::IContactAddress** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Contact(
                        ABI::Windows::ApplicationModel::Contacts::IContact** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IContactMapActivatedEventArgs = __uuidof(IContactMapActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactMessageActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactMessageActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactMessageActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("de598db2-0e03-43b0-bf56-bcc40b3162df")
                IContactMessageActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceUserId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Contact(
                        ABI::Windows::ApplicationModel::Contacts::IContact** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IContactMessageActivatedEventArgs = __uuidof(IContactMessageActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactPanelActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactPanelActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactPanelActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("52bb63e4-d3d4-4b63-8051-4af2082cab80")
                IContactPanelActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContactPanel(
                        ABI::Windows::ApplicationModel::Contacts::IContactPanel** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Contact(
                        ABI::Windows::ApplicationModel::Contacts::IContact** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IContactPanelActivatedEventArgs = __uuidof(IContactPanelActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactPickerActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactPickerActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactPickerActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("ce57aae7-6449-45a7-971f-d113be7a8936")
                IContactPickerActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContactPickerUI(
                        ABI::Windows::ApplicationModel::Contacts::Provider::IContactPickerUI** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IContactPickerActivatedEventArgs = __uuidof(IContactPickerActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactPostActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactPostActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactPostActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("b35a3c67-f1e7-4655-ad6e-4857588f552f")
                IContactPostActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceUserId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Contact(
                        ABI::Windows::ApplicationModel::Contacts::IContact** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IContactPostActivatedEventArgs = __uuidof(IContactPostActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactVideoCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactVideoCallActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactVideoCallActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("61079db8-e3e7-4b4f-858d-5c63a96ef684")
                IContactVideoCallActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceUserId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Contact(
                        ABI::Windows::ApplicationModel::Contacts::IContact** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IContactVideoCallActivatedEventArgs = __uuidof(IContactVideoCallActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactsProviderActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactsProviderActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactsProviderActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("4580dca8-5750-4916-aa52-c0829521eb94")
                IContactsProviderActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Verb(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IContactsProviderActivatedEventArgs = __uuidof(IContactsProviderActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContinuationActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("e58106b5-155f-4a94-a742-c7e08f4e188c")
                IContinuationActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContinuationData(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IContinuationActivatedEventArgs = __uuidof(IContinuationActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IDeviceActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IDeviceActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IDeviceActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("cd50b9a9-ce10-44d2-8234-c355a073ef33")
                IDeviceActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceInformationId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Verb(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDeviceActivatedEventArgs = __uuidof(IDeviceActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IDevicePairingActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IDevicePairingActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IDevicePairingActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e")
                IDevicePairingActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceInformation(
                        ABI::Windows::Devices::Enumeration::IDeviceInformation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDevicePairingActivatedEventArgs = __uuidof(IDevicePairingActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IDialReceiverActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IDialReceiverActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IDialReceiverActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("fb777ed7-85ee-456e-a44d-85d730e70aed")
                IDialReceiverActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppName(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDialReceiverActivatedEventArgs = __uuidof(IDialReceiverActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IFileActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("bb2afc33-93b1-42ed-8b26-236dd9c78496")
                IFileActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Files(
                        __FIVectorView_1_Windows__CStorage__CIStorageItem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Verb(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileActivatedEventArgs = __uuidof(IFileActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithCallerPackageFamilyName
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileActivatedEventArgsWithCallerPackageFamilyName[] = L"Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithCallerPackageFamilyName";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("2d60f06b-d25f-4d25-8653-e1c5e1108309")
                IFileActivatedEventArgsWithCallerPackageFamilyName : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CallerPackageFamilyName(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileActivatedEventArgsWithCallerPackageFamilyName = __uuidof(IFileActivatedEventArgsWithCallerPackageFamilyName);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithNeighboringFiles
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IFileActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileActivatedEventArgsWithNeighboringFiles[] = L"Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithNeighboringFiles";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("433ba1a4-e1e2-48fd-b7fc-b5d6eee65033")
                IFileActivatedEventArgsWithNeighboringFiles : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NeighboringFilesQuery(
                        ABI::Windows::Storage::Search::IStorageFileQueryResult** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileActivatedEventArgsWithNeighboringFiles = __uuidof(IFileActivatedEventArgsWithNeighboringFiles);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileOpenPickerActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("72827082-5525-4bf2-bc09-1f5095d4964d")
                IFileOpenPickerActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FileOpenPickerUI(
                        ABI::Windows::Storage::Pickers::Provider::IFileOpenPickerUI** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileOpenPickerActivatedEventArgs = __uuidof(IFileOpenPickerActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileOpenPickerActivatedEventArgs2[] = L"Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("5e731f66-8d1f-45fb-af1d-73205c8fc7a1")
                IFileOpenPickerActivatedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CallerPackageFamilyName(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileOpenPickerActivatedEventArgs2 = __uuidof(IFileOpenPickerActivatedEventArgs2);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileOpenPickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileOpenPickerContinuationEventArgs[] = L"Windows.ApplicationModel.Activation.IFileOpenPickerContinuationEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                DEPRECATED("IFileOpenPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                IFileOpenPickerContinuationEventArgs : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("IFileOpenPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_Files(
                        __FIVectorView_1_Windows__CStorage__CStorageFile** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileOpenPickerContinuationEventArgs = __uuidof(IFileOpenPickerContinuationEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileSavePickerActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("81c19cf1-74e6-4387-82eb-bb8fd64b4346")
                IFileSavePickerActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FileSavePickerUI(
                        ABI::Windows::Storage::Pickers::Provider::IFileSavePickerUI** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileSavePickerActivatedEventArgs = __uuidof(IFileSavePickerActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileSavePickerActivatedEventArgs2[] = L"Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("6b73fe13-2cf2-4d48-8cbc-af67d23f1ce7")
                IFileSavePickerActivatedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CallerPackageFamilyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EnterpriseId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileSavePickerActivatedEventArgs2 = __uuidof(IFileSavePickerActivatedEventArgs2);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileSavePickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileSavePickerContinuationEventArgs[] = L"Windows.ApplicationModel.Activation.IFileSavePickerContinuationEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("2c846fe1-3bad-4f33-8c8b-e46fae824b4b")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                DEPRECATED("IFileSavePickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                IFileSavePickerContinuationEventArgs : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("IFileSavePickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_File(
                        ABI::Windows::Storage::IStorageFile** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileSavePickerContinuationEventArgs = __uuidof(IFileSavePickerContinuationEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFolderPickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFolderPickerContinuationEventArgs[] = L"Windows.ApplicationModel.Activation.IFolderPickerContinuationEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("51882366-9f4b-498f-beb0-42684f6e1c29")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                DEPRECATED("IFolderPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                IFolderPickerContinuationEventArgs : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("IFolderPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_Folder(
                        ABI::Windows::Storage::IStorageFolder** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFolderPickerContinuationEventArgs = __uuidof(IFolderPickerContinuationEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ILaunchActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("fbc93e26-a14a-4b4f-82b0-33bed920af52")
                ILaunchActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Arguments(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TileId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILaunchActivatedEventArgs = __uuidof(ILaunchActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ILaunchActivatedEventArgs2[] = L"Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("0fd37ebc-9dc9-46b5-9ace-bd95d4565345")
                ILaunchActivatedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TileActivatedInfo(
                        ABI::Windows::ApplicationModel::Activation::ITileActivatedInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILaunchActivatedEventArgs2 = __uuidof(ILaunchActivatedEventArgs2);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ILockScreenActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ILockScreenActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ILockScreenActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("3ca77966-6108-4a41-8220-ee7d133c8532")
                ILockScreenActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Info(
                        IInspectable** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILockScreenActivatedEventArgs = __uuidof(ILockScreenActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ILockScreenCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ILockScreenCallActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ILockScreenCallActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("06f37fbe-b5f2-448b-b13e-e328ac1c516a")
                ILockScreenCallActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CallUI(
                        ABI::Windows::ApplicationModel::Calls::ILockScreenCallUI** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILockScreenCallActivatedEventArgs = __uuidof(ILockScreenCallActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IPhoneCallActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IPhoneCallActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IPhoneCallActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("54615221-a3c1-4ced-b62f-8c60523619ad")
                IPhoneCallActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LineId(
                        GUID* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallActivatedEventArgs = __uuidof(IPhoneCallActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IPickerReturnedActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IPickerReturnedActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IPickerReturnedActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("360defb9-a9d3-4984-a4ed-9ec734604921")
                IPickerReturnedActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PickerOperationId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPickerReturnedActivatedEventArgs = __uuidof(IPickerReturnedActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IPrelaunchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IPrelaunchActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IPrelaunchActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("0c44717b-19f7-48d6-b046-cf22826eaa74")
                IPrelaunchActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PrelaunchActivated(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrelaunchActivatedEventArgs = __uuidof(IPrelaunchActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IPrint3DWorkflowActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IPrint3DWorkflowActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IPrint3DWorkflowActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("3f57e78b-f2ac-4619-8302-ef855e1c9b90")
                IPrint3DWorkflowActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Workflow(
                        ABI::Windows::Devices::Printers::Extensions::IPrint3DWorkflow** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrint3DWorkflowActivatedEventArgs = __uuidof(IPrint3DWorkflowActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IPrintTaskSettingsActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IPrintTaskSettingsActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IPrintTaskSettingsActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("ee30a0c9-ce56-4865-ba8e-8954ac271107")
                IPrintTaskSettingsActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Configuration(
                        ABI::Windows::Devices::Printers::Extensions::IPrintTaskConfiguration** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPrintTaskSettingsActivatedEventArgs = __uuidof(IPrintTaskSettingsActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IProtocolActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("6095f4dd-b7c0-46ab-81fe-d90f36d00d24")
                IProtocolActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProtocolActivatedEventArgs = __uuidof(IProtocolActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData[] = L"Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("d84a0c12-5c8f-438c-83cb-c28fcc0b2fdb")
                IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CallerPackageFamilyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Data(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData = __uuidof(IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IProtocolForResultsActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IProtocolForResultsActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IProtocolForResultsActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c")
                IProtocolForResultsActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProtocolForResultsOperation(
                        ABI::Windows::System::IProtocolForResultsOperation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProtocolForResultsActivatedEventArgs = __uuidof(IProtocolForResultsActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IRestrictedLaunchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IRestrictedLaunchActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IRestrictedLaunchActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("e0b7ac81-bfc3-4344-a5da-19fd5a27baae")
                IRestrictedLaunchActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SharedContext(
                        IInspectable** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRestrictedLaunchActivatedEventArgs = __uuidof(IRestrictedLaunchActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ISearchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ISearchActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ISearchActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("8cb36951-58c8-43e3-94bc-41d33f8b630e")
                ISearchActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_QueryText(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Language(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchActivatedEventArgs = __uuidof(ISearchActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ISearchActivatedEventArgsWithLinguisticDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ISearchActivatedEventArgsWithLinguisticDetails[] = L"Windows.ApplicationModel.Activation.ISearchActivatedEventArgsWithLinguisticDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("c09f33da-08ab-4931-9b7c-451025f21f81")
                ISearchActivatedEventArgsWithLinguisticDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LinguisticDetails(
                        ABI::Windows::ApplicationModel::Search::ISearchPaneQueryLinguisticDetails** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISearchActivatedEventArgsWithLinguisticDetails = __uuidof(ISearchActivatedEventArgsWithLinguisticDetails);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IShareTargetActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IShareTargetActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IShareTargetActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("4bdaf9c8-cdb2-4acb-bfc3-6648563378ec")
                IShareTargetActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ShareOperation(
                        ABI::Windows::ApplicationModel::DataTransfer::ShareTarget::IShareOperation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IShareTargetActivatedEventArgs = __uuidof(IShareTargetActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ISplashScreen
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Activation.SplashScreen
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ISplashScreen[] = L"Windows.ApplicationModel.Activation.ISplashScreen";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("ca4d975c-d4d6-43f0-97c0-0833c6391c24")
                ISplashScreen : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ImageLocation(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Dismissed(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Dismissed(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISplashScreen = __uuidof(ISplashScreen);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IStartupTaskActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IStartupTaskActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IStartupTaskActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("03b11a58-5276-4d91-8621-54611864d5fa")
                IStartupTaskActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TaskId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStartupTaskActivatedEventArgs = __uuidof(IStartupTaskActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ITileActivatedInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Activation.TileActivatedInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ITileActivatedInfo[] = L"Windows.ApplicationModel.Activation.ITileActivatedInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("80e4a3b1-3980-4f17-b738-89194e0b8f65")
                ITileActivatedInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RecentlyShownNotifications(
                        __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITileActivatedInfo = __uuidof(ITileActivatedInfo);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IToastNotificationActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IToastNotificationActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IToastNotificationActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("92a86f82-5290-431d-be85-c4aaeeb8685f")
                IToastNotificationActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Argument(
                        HSTRING* argument
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserInput(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotificationActivatedEventArgs = __uuidof(IToastNotificationActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IUserDataAccountProviderActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IUserDataAccountProviderActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IUserDataAccountProviderActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("1bc9f723-8ef1-4a51-a63a-fe711eeab607")
                IUserDataAccountProviderActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Operation(
                        ABI::Windows::ApplicationModel::UserDataAccounts::Provider::IUserDataAccountProviderOperation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataAccountProviderActivatedEventArgs = __uuidof(IUserDataAccountProviderActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IViewSwitcherProvider[] = L"Windows.ApplicationModel.Activation.IViewSwitcherProvider";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("33f288a6-5c2c-4d27-bac7-7536088f1219")
                IViewSwitcherProvider : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ViewSwitcher(
                        ABI::Windows::UI::ViewManagement::IActivationViewSwitcher** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IViewSwitcherProvider = __uuidof(IViewSwitcherProvider);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IVoiceCommandActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IVoiceCommandActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IVoiceCommandActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("ab92dcfd-8d43-4de6-9775-20704b581b00")
                IVoiceCommandActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Result(
                        ABI::Windows::Media::SpeechRecognition::ISpeechRecognitionResult** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVoiceCommandActivatedEventArgs = __uuidof(IVoiceCommandActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IWalletActionActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Wallet.WalletContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IWalletActionActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IWalletActionActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("fcfc027b-1a1a-4d22-923f-ae6f45fa52d9")
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
                DEPRECATED("IWalletActionActivatedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
                IWalletActionActivatedEventArgs : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
                    DEPRECATED("IWalletActionActivatedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_ItemId(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
                    DEPRECATED("IWalletActionActivatedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_ActionKind(
                        ABI::Windows::ApplicationModel::Wallet::WalletActionKind* value
                        ) = 0;
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
                    DEPRECATED("IWalletActionActivatedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_ActionId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWalletActionActivatedEventArgs = __uuidof(IWalletActionActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IWebAccountProviderActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IWebAccountProviderActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IWebAccountProviderActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("72b71774-98ea-4ccf-9752-46d9051004f1")
                IWebAccountProviderActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Operation(
                        ABI::Windows::Security::Authentication::Web::Provider::IWebAccountProviderOperation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebAccountProviderActivatedEventArgs = __uuidof(IWebAccountProviderActivatedEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IWebAuthenticationBrokerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IWebAuthenticationBrokerContinuationEventArgs[] = L"Windows.ApplicationModel.Activation.IWebAuthenticationBrokerContinuationEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                MIDL_INTERFACE("75dda3d4-7714-453d-b7ff-b95e3a1709da")
                IWebAuthenticationBrokerContinuationEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WebAuthenticationResult(
                        ABI::Windows::Security::Authentication::Web::IWebAuthenticationResult** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebAuthenticationBrokerContinuationEventArgs = __uuidof(IWebAuthenticationBrokerContinuationEventArgs);
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderAddAppointmentActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderAddAppointmentActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderAddAppointmentActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_AppointmentsProviderAddAppointmentActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderRemoveAppointmentActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderRemoveAppointmentActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderRemoveAppointmentActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_AppointmentsProviderRemoveAppointmentActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderReplaceAppointmentActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderReplaceAppointmentActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderReplaceAppointmentActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_AppointmentsProviderReplaceAppointmentActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderShowAppointmentDetailsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderShowAppointmentDetailsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_AppointmentsProviderShowAppointmentDetailsActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderShowTimeFrameActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderShowTimeFrameActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderShowTimeFrameActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_AppointmentsProviderShowTimeFrameActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IBackgroundActivatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_BackgroundActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_BackgroundActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_BackgroundActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IBarcodeScannerPreviewActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_BarcodeScannerPreviewActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_BarcodeScannerPreviewActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_BarcodeScannerPreviewActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ICachedFileUpdaterActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_CachedFileUpdaterActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_CachedFileUpdaterActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_CachedFileUpdaterActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivationCameraSettingsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ICameraSettingsActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_CameraSettingsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_CameraSettingsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_CameraSettingsActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ICommandLineActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_CommandLineActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_CommandLineActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_CommandLineActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Activation.CommandLineActivationOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ICommandLineActivationOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_CommandLineActivationOperation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_CommandLineActivationOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_CommandLineActivationOperation[] = L"Windows.ApplicationModel.Activation.CommandLineActivationOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactCallActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactCallActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactCallActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ContactCallActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactMapActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactMapActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactMapActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ContactMapActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactMessageActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactMessageActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactMessageActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ContactMessageActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactPanelActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactPanelActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactPanelActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ContactPanelActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactPickerActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactPickerActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactPickerActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ContactPickerActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactPostActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactPostActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactPostActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ContactPostActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactVideoCallActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactVideoCallActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactVideoCallActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ContactVideoCallActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.DeviceActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IDeviceActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_DeviceActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_DeviceActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_DeviceActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.DeviceActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IDevicePairingActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_DevicePairingActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_DevicePairingActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_DevicePairingActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IDialReceiverActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_DialReceiverActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_DialReceiverActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_DialReceiverActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.FileActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithNeighboringFiles
 *    Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithCallerPackageFamilyName
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_FileActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_FileActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_FileActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.FileActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs2
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_FileOpenPickerActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_FileOpenPickerActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_FileOpenPickerActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileOpenPickerContinuationEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_FileOpenPickerContinuationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_FileOpenPickerContinuationEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("FileOpenPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_FileOpenPickerContinuationEventArgs[] = L"Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs2
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_FileSavePickerActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_FileSavePickerActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_FileSavePickerActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileSavePickerContinuationEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_FileSavePickerContinuationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_FileSavePickerContinuationEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("FileSavePickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_FileSavePickerContinuationEventArgs[] = L"Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFolderPickerContinuationEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_FolderPickerContinuationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_FolderPickerContinuationEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("FolderPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_FolderPickerContinuationEventArgs[] = L"Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.LaunchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IPrelaunchActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *    Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs2
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_LaunchActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_LaunchActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_LaunchActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.LaunchActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ILockScreenActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_LockScreenActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_LockScreenActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_LockScreenActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ILockScreenCallActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_LockScreenCallActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_LockScreenCallActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_LockScreenCallActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_LockScreenComponentActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_LockScreenComponentActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_LockScreenComponentActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IPhoneCallActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_PhoneCallActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_PhoneCallActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_PhoneCallActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IPickerReturnedActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_PickerReturnedActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_PickerReturnedActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_PickerReturnedActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IPrint3DWorkflowActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_Print3DWorkflowActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_Print3DWorkflowActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_Print3DWorkflowActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IPrintTaskSettingsActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_PrintTaskSettingsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_PrintTaskSettingsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_PrintTaskSettingsActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ProtocolActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ProtocolActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ProtocolActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IProtocolForResultsActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ProtocolForResultsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ProtocolForResultsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ProtocolForResultsActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IRestrictedLaunchActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_RestrictedLaunchActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_RestrictedLaunchActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_RestrictedLaunchActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.SearchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ISearchActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.ApplicationModel.Activation.ISearchActivatedEventArgsWithLinguisticDetails
 *    Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_SearchActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_SearchActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_SearchActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.SearchActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IShareTargetActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ShareTargetActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ShareTargetActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ShareTargetActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.SplashScreen
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ISplashScreen ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_SplashScreen_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_SplashScreen_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_SplashScreen[] = L"Windows.ApplicationModel.Activation.SplashScreen";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IStartupTaskActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_StartupTaskActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_StartupTaskActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_StartupTaskActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Activation.TileActivatedInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ITileActivatedInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_TileActivatedInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_TileActivatedInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_TileActivatedInfo[] = L"Windows.ApplicationModel.Activation.TileActivatedInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IToastNotificationActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ToastNotificationActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ToastNotificationActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ToastNotificationActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IUserDataAccountProviderActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_UserDataAccountProviderActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_UserDataAccountProviderActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_UserDataAccountProviderActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IVoiceCommandActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_VoiceCommandActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_VoiceCommandActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_VoiceCommandActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Wallet.WalletContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IWalletActionActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_WalletActionActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_WalletActionActivatedEventArgs_DEFINED
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
DEPRECATED("WalletActionActivatedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_WalletActionActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IWebAccountProviderActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_WebAccountProviderActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_WebAccountProviderActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_WebAccountProviderActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IWebAuthenticationBrokerContinuationEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_WebAuthenticationBrokerContinuationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_WebAuthenticationBrokerContinuationEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_WebAuthenticationBrokerContinuationEventArgs[] = L"Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2 __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2 __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2 __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo __x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider __x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

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

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CStorage__CStorageFile __FIIterator_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CStorage__CStorageFile;

typedef struct __FIIterator_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageFile** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CStorage__CStorageFileVtbl;

interface __FIIterator_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIIterator_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CStorage__CStorageFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CStorage__CStorageFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CStorage__CStorageFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CStorage__CStorageFile_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageFile_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageFile_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageFile_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CStorage__CStorageFile __FIIterable_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CStorage__CStorageFile;

typedef struct __FIIterable_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CStorage__CStorageFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CStorage__CStorageFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CStorage__CStorageFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CStorage__CStorageFile* This,
        __FIIterator_1_Windows__CStorage__CStorageFile** result);

    END_INTERFACE
} __FIIterable_1_Windows__CStorage__CStorageFileVtbl;

interface __FIIterable_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIIterable_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CStorage__CStorageFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CStorage__CStorageFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CStorage__CStorageFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CStorage__CStorageFile_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification;

typedef struct __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotificationVtbl;

interface __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification __FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification;

typedef struct __FIIterable_1_Windows__CUI__CNotifications__CShownTileNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        __FIIterator_1_Windows__CUI__CNotifications__CShownTileNotification** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CNotifications__CShownTileNotificationVtbl;

interface __FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CNotifications__CShownTileNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CNotifications__CShownTileNotification_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CStorage__CStorageFile __FIVectorView_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CStorage__CStorageFile;

typedef struct __FIVectorView_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CIStorageFile** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageFile** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CStorage__CStorageFileVtbl;

interface __FIVectorView_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIVectorView_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification;

typedef struct __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotificationVtbl;

interface __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable __FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable* This,
        __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIAddAppointmentOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIAddAppointmentOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIAddAppointmentOperation __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIAddAppointmentOperation;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIAddAppointmentOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIRemoveAppointmentOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIRemoveAppointmentOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIRemoveAppointmentOperation __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIRemoveAppointmentOperation;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIRemoveAppointmentOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIReplaceAppointmentOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIReplaceAppointmentOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIReplaceAppointmentOperation __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIReplaceAppointmentOperation;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIReplaceAppointmentOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance;

#endif // ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CIContact __x_ABI_CWindows_CApplicationModel_CContacts_CIContact;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactAddress_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactAddress_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CIContactAddress __x_ABI_CWindows_CApplicationModel_CContacts_CIContactAddress;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactAddress_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactPanel_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactPanel_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CIContactPanel __x_ABI_CWindows_CApplicationModel_CContacts_CIContactPanel;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactPanel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CShareTarget_CIShareOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CShareTarget_CIShareOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CShareTarget_CIShareOperation __x_ABI_CWindows_CApplicationModel_CDataTransfer_CShareTarget_CIShareOperation;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CShareTarget_CIShareOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CWallet_CWalletActionKind __x_ABI_CWindows_CApplicationModel_CWallet_CWalletActionKind;

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow;

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration;

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult;

#endif // ____x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI;

#endif // ____x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CProvider_CICachedFileUpdaterUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CProvider_CICachedFileUpdaterUI_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CProvider_CICachedFileUpdaterUI __x_ABI_CWindows_CStorage_CProvider_CICachedFileUpdaterUI;

#endif // ____x_ABI_CWindows_CStorage_CProvider_CICachedFileUpdaterUI_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CSearch_CIStorageFileQueryResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CSearch_CIStorageFileQueryResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CSearch_CIStorageFileQueryResult __x_ABI_CWindows_CStorage_CSearch_CIStorageFileQueryResult;

#endif // ____x_ABI_CWindows_CStorage_CSearch_CIStorageFileQueryResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFolder __x_ABI_CWindows_CStorage_CIStorageFolder;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIProtocolForResultsOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIProtocolForResultsOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIProtocolForResultsOperation __x_ABI_CWindows_CSystem_CIProtocolForResultsOperation;

#endif // ____x_ABI_CWindows_CSystem_CIProtocolForResultsOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CActivation_CActivationKind __x_ABI_CWindows_CApplicationModel_CActivation_CActivationKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CActivation_CApplicationExecutionState __x_ABI_CWindows_CApplicationModel_CActivation_CApplicationExecutionState;

/*
 *
 * Struct Windows.ApplicationModel.Activation.ActivationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CActivation_CActivationKind
{
    ActivationKind_Launch = 0,
    ActivationKind_Search = 1,
    ActivationKind_ShareTarget = 2,
    ActivationKind_File = 3,
    ActivationKind_Protocol = 4,
    ActivationKind_FileOpenPicker = 5,
    ActivationKind_FileSavePicker = 6,
    ActivationKind_CachedFileUpdater = 7,
    ActivationKind_ContactPicker = 8,
    ActivationKind_Device = 9,
    ActivationKind_PrintTaskSettings = 10,
    ActivationKind_CameraSettings = 11,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ActivationKind_RestrictedLaunch = 12,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ActivationKind_AppointmentsProvider = 13,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ActivationKind_Contact = 14,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ActivationKind_LockScreenCall = 15,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ActivationKind_VoiceCommand = 16,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ActivationKind_LockScreen = 17,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ActivationKind_PickerReturned = 1000,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ActivationKind_WalletAction = 1001,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ActivationKind_PickFileContinuation = 1002,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ActivationKind_PickSaveFileContinuation = 1003,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ActivationKind_PickFolderContinuation = 1004,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ActivationKind_WebAuthenticationBrokerContinuation = 1005,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ActivationKind_WebAccountProvider = 1006,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ActivationKind_ComponentUI = 1007,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ActivationKind_ProtocolForResults = 1009,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ActivationKind_ToastNotification = 1010,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
    ActivationKind_Print3DWorkflow = 1011,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ActivationKind_DialReceiver = 1012,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
    ActivationKind_DevicePairing = 1013,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    ActivationKind_UserDataAccountsProvider = 1014,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    ActivationKind_FilePickerExperience = 1015,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    ActivationKind_LockScreenComponent = 1016,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    ActivationKind_ContactPanel = 1017,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    ActivationKind_PrintWorkflowForegroundTask = 1018,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    ActivationKind_GameUIProvider = 1019,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    ActivationKind_StartupTask = 1020,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    ActivationKind_CommandLineLaunch = 1021,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    ActivationKind_BarcodeScannerProvider = 1022,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
    ActivationKind_PrintSupportJobUI = 1023,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
    ActivationKind_PrintSupportSettingsUI = 1024,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
    ActivationKind_PhoneCallActivation = 1025,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
    ActivationKind_VpnForeground = 1026,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
    ActivationKind_PrintSupportEnterpriseManagementUI = 1027,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Activation.ApplicationExecutionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CActivation_CApplicationExecutionState
{
    ApplicationExecutionState_NotRunning = 0,
    ApplicationExecutionState_Running = 1,
    ApplicationExecutionState_Suspended = 2,
    ApplicationExecutionState_Terminated = 3,
    ApplicationExecutionState_ClosedByUser = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs* This,
        enum __x_ABI_CWindows_CApplicationModel_CActivation_CActivationKind* value);
    HRESULT (STDMETHODCALLTYPE* get_PreviousExecutionState)(__x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs* This,
        enum __x_ABI_CWindows_CApplicationModel_CActivation_CApplicationExecutionState* value);
    HRESULT (STDMETHODCALLTYPE* get_SplashScreen)(__x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_get_PreviousExecutionState(This, value) \
    ((This)->lpVtbl->get_PreviousExecutionState(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_get_SplashScreen(This, value) \
    ((This)->lpVtbl->get_SplashScreen(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IActivatedEventArgsWithUser[] = L"Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUserVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUserVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUserVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IApplicationViewActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CurrentlyShownApplicationViewId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_get_CurrentlyShownApplicationViewId(This, value) \
    ((This)->lpVtbl->get_CurrentlyShownApplicationViewId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IAppointmentsProviderActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Verb)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_get_Verb(This, value) \
    ((This)->lpVtbl->get_Verb(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IAppointmentsProviderAddAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IAppointmentsProviderAddAppointmentActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IAppointmentsProviderAddAppointmentActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AddAppointmentOperation)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIAddAppointmentOperation** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_get_AddAppointmentOperation(This, value) \
    ((This)->lpVtbl->get_AddAppointmentOperation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IAppointmentsProviderRemoveAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IAppointmentsProviderRemoveAppointmentActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IAppointmentsProviderRemoveAppointmentActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RemoveAppointmentOperation)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIRemoveAppointmentOperation** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_get_RemoveAppointmentOperation(This, value) \
    ((This)->lpVtbl->get_RemoveAppointmentOperation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IAppointmentsProviderReplaceAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IAppointmentsProviderReplaceAppointmentActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IAppointmentsProviderReplaceAppointmentActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ReplaceAppointmentOperation)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentsProvider_CIReplaceAppointmentOperation** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_get_ReplaceAppointmentOperation(This, value) \
    ((This)->lpVtbl->get_ReplaceAppointmentOperation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InstanceStartDate)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_LocalId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RoamingId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_get_InstanceStartDate(This, value) \
    ((This)->lpVtbl->get_InstanceStartDate(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_get_LocalId(This, value) \
    ((This)->lpVtbl->get_LocalId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_get_RoamingId(This, value) \
    ((This)->lpVtbl->get_RoamingId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IAppointmentsProviderShowTimeFrameActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IAppointmentsProviderShowTimeFrameActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IAppointmentsProviderShowTimeFrameActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TimeToShow)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_get_TimeToShow(This, value) \
    ((This)->lpVtbl->get_TimeToShow(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IBackgroundActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IBackgroundActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IBackgroundActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TaskInstance)(__x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_get_TaskInstance(This, value) \
    ((This)->lpVtbl->get_TaskInstance(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IBarcodeScannerPreviewActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IBarcodeScannerPreviewActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IBarcodeScannerPreviewActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_get_ConnectionId(This, value) \
    ((This)->lpVtbl->get_ConnectionId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ICachedFileUpdaterActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ICachedFileUpdaterActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ICachedFileUpdaterActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CachedFileUpdaterUI)(__x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs* This,
        __x_ABI_CWindows_CStorage_CProvider_CICachedFileUpdaterUI** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_get_CachedFileUpdaterUI(This, value) \
    ((This)->lpVtbl->get_CachedFileUpdaterUI(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ICameraSettingsActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivationCameraSettingsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ICameraSettingsActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ICameraSettingsActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_VideoDeviceController)(__x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_VideoDeviceExtension)(__x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs* This,
        IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_get_VideoDeviceController(This, value) \
    ((This)->lpVtbl->get_VideoDeviceController(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_get_VideoDeviceExtension(This, value) \
    ((This)->lpVtbl->get_VideoDeviceExtension(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ICommandLineActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ICommandLineActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ICommandLineActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Operation)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_get_Operation(This, value) \
    ((This)->lpVtbl->get_Operation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ICommandLineActivationOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Activation.CommandLineActivationOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ICommandLineActivationOperation[] = L"Windows.ApplicationModel.Activation.ICommandLineActivationOperation";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Arguments)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentDirectoryPath)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ExitCode)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ExitCode)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_get_Arguments(This, value) \
    ((This)->lpVtbl->get_Arguments(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_get_CurrentDirectoryPath(This, value) \
    ((This)->lpVtbl->get_CurrentDirectoryPath(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_put_ExitCode(This, value) \
    ((This)->lpVtbl->put_ExitCode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_get_ExitCode(This, value) \
    ((This)->lpVtbl->get_ExitCode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivationOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Verb)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_get_Verb(This, value) \
    ((This)->lpVtbl->get_Verb(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactCallActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactCallActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ServiceId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceUserId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContact** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_get_ServiceId(This, value) \
    ((This)->lpVtbl->get_ServiceId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_get_ServiceUserId(This, value) \
    ((This)->lpVtbl->get_ServiceUserId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactMapActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactMapActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactMapActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Address)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactAddress** value);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContact** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_get_Address(This, value) \
    ((This)->lpVtbl->get_Address(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactMessageActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactMessageActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactMessageActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ServiceId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceUserId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContact** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_get_ServiceId(This, value) \
    ((This)->lpVtbl->get_ServiceId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_get_ServiceUserId(This, value) \
    ((This)->lpVtbl->get_ServiceUserId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactPanelActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactPanelActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactPanelActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContactPanel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactPanel** value);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContact** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_get_ContactPanel(This, value) \
    ((This)->lpVtbl->get_ContactPanel(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactPickerActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactPickerActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactPickerActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContactPickerUI)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_get_ContactPickerUI(This, value) \
    ((This)->lpVtbl->get_ContactPickerUI(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactPostActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactPostActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactPostActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ServiceId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceUserId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContact** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_get_ServiceId(This, value) \
    ((This)->lpVtbl->get_ServiceId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_get_ServiceUserId(This, value) \
    ((This)->lpVtbl->get_ServiceUserId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactVideoCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactVideoCallActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactVideoCallActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ServiceId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceUserId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContact** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_get_ServiceId(This, value) \
    ((This)->lpVtbl->get_ServiceId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_get_ServiceUserId(This, value) \
    ((This)->lpVtbl->get_ServiceUserId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContactsProviderActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContactsProviderActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContactsProviderActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Verb)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_get_Verb(This, value) \
    ((This)->lpVtbl->get_Verb(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContactsProviderActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IContinuationActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContinuationData)(__x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_get_ContinuationData(This, value) \
    ((This)->lpVtbl->get_ContinuationData(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IDeviceActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IDeviceActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IDeviceActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceInformationId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Verb)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_get_DeviceInformationId(This, value) \
    ((This)->lpVtbl->get_DeviceInformationId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_get_Verb(This, value) \
    ((This)->lpVtbl->get_Verb(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IDevicePairingActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IDevicePairingActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IDevicePairingActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceInformation)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_get_DeviceInformation(This, value) \
    ((This)->lpVtbl->get_DeviceInformation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IDialReceiverActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IDialReceiverActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IDialReceiverActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_get_AppName(This, value) \
    ((This)->lpVtbl->get_AppName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IFileActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Files)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs* This,
        __FIVectorView_1_Windows__CStorage__CIStorageItem** value);
    HRESULT (STDMETHODCALLTYPE* get_Verb)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_get_Files(This, value) \
    ((This)->lpVtbl->get_Files(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_get_Verb(This, value) \
    ((This)->lpVtbl->get_Verb(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithCallerPackageFamilyName
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileActivatedEventArgsWithCallerPackageFamilyName[] = L"Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithCallerPackageFamilyName";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyNameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CallerPackageFamilyName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyNameVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyNameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_get_CallerPackageFamilyName(This, value) \
    ((This)->lpVtbl->get_CallerPackageFamilyName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithCallerPackageFamilyName_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithNeighboringFiles
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IFileActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileActivatedEventArgsWithNeighboringFiles[] = L"Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithNeighboringFiles";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFilesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NeighboringFilesQuery)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles* This,
        __x_ABI_CWindows_CStorage_CSearch_CIStorageFileQueryResult** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFilesVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFilesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_get_NeighboringFilesQuery(This, value) \
    ((This)->lpVtbl->get_NeighboringFilesQuery(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileOpenPickerActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FileOpenPickerUI)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs* This,
        __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileOpenPickerUI** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_get_FileOpenPickerUI(This, value) \
    ((This)->lpVtbl->get_FileOpenPickerUI(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileOpenPickerActivatedEventArgs2[] = L"Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs2";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CallerPackageFamilyName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_get_CallerPackageFamilyName(This, value) \
    ((This)->lpVtbl->get_CallerPackageFamilyName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileOpenPickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileOpenPickerContinuationEventArgs[] = L"Windows.ApplicationModel.Activation.IFileOpenPickerContinuationEventArgs";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("IFileOpenPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("IFileOpenPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_Files)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs* This,
        __FIVectorView_1_Windows__CStorage__CStorageFile** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("IFileOpenPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_get_Files(This, value) \
    ((This)->lpVtbl->get_Files(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileSavePickerActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FileSavePickerUI)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs* This,
        __x_ABI_CWindows_CStorage_CPickers_CProvider_CIFileSavePickerUI** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_get_FileSavePickerUI(This, value) \
    ((This)->lpVtbl->get_FileSavePickerUI(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileSavePickerActivatedEventArgs2[] = L"Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs2";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CallerPackageFamilyName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EnterpriseId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_get_CallerPackageFamilyName(This, value) \
    ((This)->lpVtbl->get_CallerPackageFamilyName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_get_EnterpriseId(This, value) \
    ((This)->lpVtbl->get_EnterpriseId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFileSavePickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFileSavePickerContinuationEventArgs[] = L"Windows.ApplicationModel.Activation.IFileSavePickerContinuationEventArgs";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("IFileSavePickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("IFileSavePickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_File)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("IFileSavePickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_get_File(This, value) \
    ((This)->lpVtbl->get_File(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IFolderPickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IFolderPickerContinuationEventArgs[] = L"Windows.ApplicationModel.Activation.IFolderPickerContinuationEventArgs";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("IFolderPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("IFolderPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_Folder)(__x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("IFolderPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_get_Folder(This, value) \
    ((This)->lpVtbl->get_Folder(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ILaunchActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Arguments)(__x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TileId)(__x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_get_Arguments(This, value) \
    ((This)->lpVtbl->get_Arguments(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_get_TileId(This, value) \
    ((This)->lpVtbl->get_TileId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ILaunchActivatedEventArgs2[] = L"Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs2";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TileActivatedInfo)(__x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2* This,
        __x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_get_TileActivatedInfo(This, value) \
    ((This)->lpVtbl->get_TileActivatedInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ILockScreenActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ILockScreenActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ILockScreenActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Info)(__x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs* This,
        IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_get_Info(This, value) \
    ((This)->lpVtbl->get_Info(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ILockScreenCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ILockScreenCallActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ILockScreenCallActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CallUI)(__x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_get_CallUI(This, value) \
    ((This)->lpVtbl->get_CallUI(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IPhoneCallActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IPhoneCallActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IPhoneCallActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LineId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs* This,
        GUID* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_get_LineId(This, value) \
    ((This)->lpVtbl->get_LineId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IPickerReturnedActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IPickerReturnedActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IPickerReturnedActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PickerOperationId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_get_PickerOperationId(This, value) \
    ((This)->lpVtbl->get_PickerOperationId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPickerReturnedActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IPrelaunchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IPrelaunchActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IPrelaunchActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PrelaunchActivated)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_get_PrelaunchActivated(This, value) \
    ((This)->lpVtbl->get_PrelaunchActivated(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IPrint3DWorkflowActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IPrint3DWorkflowActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IPrint3DWorkflowActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Workflow)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs* This,
        __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrint3DWorkflow** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_get_Workflow(This, value) \
    ((This)->lpVtbl->get_Workflow(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IPrintTaskSettingsActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IPrintTaskSettingsActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IPrintTaskSettingsActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Configuration)(__x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs* This,
        __x_ABI_CWindows_CDevices_CPrinters_CExtensions_CIPrintTaskConfiguration** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_get_Configuration(This, value) \
    ((This)->lpVtbl->get_Configuration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IProtocolActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData[] = L"Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CallerPackageFamilyName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_get_CallerPackageFamilyName(This, value) \
    ((This)->lpVtbl->get_CallerPackageFamilyName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IProtocolForResultsActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IProtocolForResultsActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IProtocolForResultsActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProtocolForResultsOperation)(__x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs* This,
        __x_ABI_CWindows_CSystem_CIProtocolForResultsOperation** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_get_ProtocolForResultsOperation(This, value) \
    ((This)->lpVtbl->get_ProtocolForResultsOperation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IRestrictedLaunchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IRestrictedLaunchActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IRestrictedLaunchActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SharedContext)(__x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs* This,
        IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_get_SharedContext(This, value) \
    ((This)->lpVtbl->get_SharedContext(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ISearchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ISearchActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ISearchActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_QueryText)(__x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Language)(__x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_get_QueryText(This, value) \
    ((This)->lpVtbl->get_QueryText(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_get_Language(This, value) \
    ((This)->lpVtbl->get_Language(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ISearchActivatedEventArgsWithLinguisticDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ISearchActivatedEventArgsWithLinguisticDetails[] = L"Windows.ApplicationModel.Activation.ISearchActivatedEventArgsWithLinguisticDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LinguisticDetails)(__x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails* This,
        __x_ABI_CWindows_CApplicationModel_CSearch_CISearchPaneQueryLinguisticDetails** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_get_LinguisticDetails(This, value) \
    ((This)->lpVtbl->get_LinguisticDetails(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IShareTargetActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IShareTargetActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IShareTargetActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ShareOperation)(__x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CShareTarget_CIShareOperation** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_get_ShareOperation(This, value) \
    ((This)->lpVtbl->get_ShareOperation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ISplashScreen
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Activation.SplashScreen
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ISplashScreen[] = L"Windows.ApplicationModel.Activation.ISplashScreen";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreenVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ImageLocation)(__x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* add_Dismissed)(__x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CActivation__CSplashScreen_IInspectable* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_Dismissed)(__x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreenVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreenVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_get_ImageLocation(This, value) \
    ((This)->lpVtbl->get_ImageLocation(This, value))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_add_Dismissed(This, handler, cookie) \
    ((This)->lpVtbl->add_Dismissed(This, handler, cookie))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_remove_Dismissed(This, cookie) \
    ((This)->lpVtbl->remove_Dismissed(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CISplashScreen_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IStartupTaskActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IStartupTaskActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IStartupTaskActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TaskId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_get_TaskId(This, value) \
    ((This)->lpVtbl->get_TaskId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Activation.ITileActivatedInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Activation.TileActivatedInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_ITileActivatedInfo[] = L"Windows.ApplicationModel.Activation.ITileActivatedInfo";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RecentlyShownNotifications)(__x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo* This,
        __FIVectorView_1_Windows__CUI__CNotifications__CShownTileNotification** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_get_RecentlyShownNotifications(This, value) \
    ((This)->lpVtbl->get_RecentlyShownNotifications(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CITileActivatedInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IToastNotificationActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IToastNotificationActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IToastNotificationActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Argument)(__x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs* This,
        HSTRING* argument);
    HRESULT (STDMETHODCALLTYPE* get_UserInput)(__x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_get_Argument(This, argument) \
    ((This)->lpVtbl->get_Argument(This, argument))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_get_UserInput(This, value) \
    ((This)->lpVtbl->get_UserInput(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IUserDataAccountProviderActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IUserDataAccountProviderActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IUserDataAccountProviderActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Operation)(__x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_get_Operation(This, value) \
    ((This)->lpVtbl->get_Operation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IViewSwitcherProvider[] = L"Windows.ApplicationModel.Activation.IViewSwitcherProvider";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ViewSwitcher)(__x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider* This,
        __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProviderVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_get_ViewSwitcher(This, value) \
    ((This)->lpVtbl->get_ViewSwitcher(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIViewSwitcherProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IVoiceCommandActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IVoiceCommandActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IVoiceCommandActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Result)(__x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs* This,
        __x_ABI_CWindows_CMedia_CSpeechRecognition_CISpeechRecognitionResult** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_get_Result(This, value) \
    ((This)->lpVtbl->get_Result(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IWalletActionActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Wallet.WalletContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IWalletActionActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IWalletActionActivatedEventArgs";
typedef struct
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
DEPRECATED("IWalletActionActivatedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
    DEPRECATED("IWalletActionActivatedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_ItemId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
    DEPRECATED("IWalletActionActivatedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_ActionKind)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs* This,
        enum __x_ABI_CWindows_CApplicationModel_CWallet_CWalletActionKind* value);
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
    DEPRECATED("IWalletActionActivatedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_ActionId)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
    DEPRECATED("IWalletActionActivatedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_get_ItemId(This, value) \
    ((This)->lpVtbl->get_ItemId(This, value))

#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
    DEPRECATED("IWalletActionActivatedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_get_ActionKind(This, value) \
    ((This)->lpVtbl->get_ActionKind(This, value))

#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
    DEPRECATED("IWalletActionActivatedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_get_ActionId(This, value) \
    ((This)->lpVtbl->get_ActionId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IWebAccountProviderActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IWebAccountProviderActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.IWebAccountProviderActivatedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Operation)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_get_Operation(This, value) \
    ((This)->lpVtbl->get_Operation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Activation.IWebAuthenticationBrokerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *     Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Activation_IWebAuthenticationBrokerContinuationEventArgs[] = L"Windows.ApplicationModel.Activation.IWebAuthenticationBrokerContinuationEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WebAuthenticationResult)(__x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_get_WebAuthenticationResult(This, result) \
    ((This)->lpVtbl->get_WebAuthenticationResult(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderAddAppointmentActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderAddAppointmentActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderAddAppointmentActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_AppointmentsProviderAddAppointmentActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderRemoveAppointmentActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderRemoveAppointmentActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderRemoveAppointmentActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_AppointmentsProviderRemoveAppointmentActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderReplaceAppointmentActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderReplaceAppointmentActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderReplaceAppointmentActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_AppointmentsProviderReplaceAppointmentActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderShowAppointmentDetailsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderShowAppointmentDetailsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_AppointmentsProviderShowAppointmentDetailsActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderShowTimeFrameActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderShowTimeFrameActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_AppointmentsProviderShowTimeFrameActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_AppointmentsProviderShowTimeFrameActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IBackgroundActivatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_BackgroundActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_BackgroundActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_BackgroundActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IBarcodeScannerPreviewActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_BarcodeScannerPreviewActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_BarcodeScannerPreviewActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_BarcodeScannerPreviewActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ICachedFileUpdaterActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_CachedFileUpdaterActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_CachedFileUpdaterActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_CachedFileUpdaterActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivationCameraSettingsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ICameraSettingsActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_CameraSettingsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_CameraSettingsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_CameraSettingsActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ICommandLineActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_CommandLineActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_CommandLineActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_CommandLineActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Activation.CommandLineActivationOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ICommandLineActivationOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_CommandLineActivationOperation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_CommandLineActivationOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_CommandLineActivationOperation[] = L"Windows.ApplicationModel.Activation.CommandLineActivationOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactCallActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactCallActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactCallActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ContactCallActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactMapActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactMapActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactMapActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ContactMapActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactMessageActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactMessageActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactMessageActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ContactMessageActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactPanelActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactPanelActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactPanelActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ContactPanelActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactPickerActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactPickerActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactPickerActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ContactPickerActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactPostActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactPostActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactPostActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ContactPostActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactVideoCallActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactVideoCallActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ContactVideoCallActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ContactVideoCallActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.DeviceActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IDeviceActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_DeviceActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_DeviceActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_DeviceActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.DeviceActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IDevicePairingActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_DevicePairingActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_DevicePairingActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_DevicePairingActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IDialReceiverActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_DialReceiverActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_DialReceiverActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_DialReceiverActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.FileActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithNeighboringFiles
 *    Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithCallerPackageFamilyName
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_FileActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_FileActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_FileActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.FileActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs2
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_FileOpenPickerActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_FileOpenPickerActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_FileOpenPickerActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileOpenPickerContinuationEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_FileOpenPickerContinuationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_FileOpenPickerContinuationEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("FileOpenPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_FileOpenPickerContinuationEventArgs[] = L"Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs2
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_FileSavePickerActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_FileSavePickerActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_FileSavePickerActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileSavePickerContinuationEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_FileSavePickerContinuationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_FileSavePickerContinuationEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("FileSavePickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_FileSavePickerContinuationEventArgs[] = L"Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFolderPickerContinuationEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_FolderPickerContinuationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_FolderPickerContinuationEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("FolderPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_FolderPickerContinuationEventArgs[] = L"Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.LaunchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IPrelaunchActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *    Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs2
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_LaunchActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_LaunchActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_LaunchActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.LaunchActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ILockScreenActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_LockScreenActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_LockScreenActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_LockScreenActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ILockScreenCallActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_LockScreenCallActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_LockScreenCallActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_LockScreenCallActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_LockScreenComponentActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_LockScreenComponentActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_LockScreenComponentActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IPhoneCallActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_PhoneCallActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_PhoneCallActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_PhoneCallActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IPickerReturnedActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_PickerReturnedActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_PickerReturnedActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_PickerReturnedActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IPrint3DWorkflowActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_Print3DWorkflowActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_Print3DWorkflowActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_Print3DWorkflowActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IPrintTaskSettingsActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_PrintTaskSettingsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_PrintTaskSettingsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_PrintTaskSettingsActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ProtocolActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ProtocolActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ProtocolActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IProtocolForResultsActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ProtocolForResultsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ProtocolForResultsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ProtocolForResultsActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IRestrictedLaunchActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_RestrictedLaunchActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_RestrictedLaunchActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_RestrictedLaunchActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.SearchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ISearchActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.ApplicationModel.Activation.ISearchActivatedEventArgsWithLinguisticDetails
 *    Windows.ApplicationModel.Activation.IViewSwitcherProvider
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_SearchActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_SearchActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_SearchActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.SearchActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IShareTargetActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ShareTargetActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ShareTargetActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ShareTargetActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.SplashScreen
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ISplashScreen ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_SplashScreen_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_SplashScreen_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_SplashScreen[] = L"Windows.ApplicationModel.Activation.SplashScreen";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IStartupTaskActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_StartupTaskActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_StartupTaskActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_StartupTaskActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Activation.TileActivatedInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ITileActivatedInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_TileActivatedInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_TileActivatedInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_TileActivatedInfo[] = L"Windows.ApplicationModel.Activation.TileActivatedInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IToastNotificationActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_ToastNotificationActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_ToastNotificationActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_ToastNotificationActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IUserDataAccountProviderActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_UserDataAccountProviderActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_UserDataAccountProviderActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_UserDataAccountProviderActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IVoiceCommandActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_VoiceCommandActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_VoiceCommandActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_VoiceCommandActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Wallet.WalletContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IWalletActionActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_WalletActionActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_WalletActionActivatedEventArgs_DEFINED
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
DEPRECATED("WalletActionActivatedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_WalletActionActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IWebAccountProviderActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_WebAccountProviderActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_WebAccountProviderActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_WebAccountProviderActivatedEventArgs[] = L"Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IWebAuthenticationBrokerContinuationEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Activation_WebAuthenticationBrokerContinuationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Activation_WebAuthenticationBrokerContinuationEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Activation_WebAuthenticationBrokerContinuationEventArgs[] = L"Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs";
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
#endif // __windows2Eapplicationmodel2Eactivation_p_h__

#endif // __windows2Eapplicationmodel2Eactivation_h__
