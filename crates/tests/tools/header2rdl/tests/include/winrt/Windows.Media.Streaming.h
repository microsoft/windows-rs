/* Header file automatically generated from windows.media.streaming.idl */
/*
 * File built with Microsoft(R) MIDLRT Compiler Engine Version 10.00.0231 
 */

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
#ifndef __windows2Emedia2Estreaming_h__
#define __windows2Emedia2Estreaming_h__
#ifndef __windows2Emedia2Estreaming_p_h__
#define __windows2Emedia2Estreaming_p_h__


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

#if !defined(WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION 0x50000
#endif // defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_APPLICATIONMODEL_COMMUNICATIONBLOCKING_COMMUNICATIONBLOCKINGCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_SEARCH_SEARCHCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION)

#if !defined(WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION)
#define WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_DEVICES_POWER_POWERGRIDAPICONTRACT_VERSION)

#if !defined(WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION)
#define WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_DEVICES_PRINTERS_EXTENSIONS_EXTENSIONSCONTRACT_VERSION)

#if !defined(WINDOWS_DEVICES_SMARTCARDS_SMARTCARDBACKGROUNDTRIGGERCONTRACT_VERSION)
#define WINDOWS_DEVICES_SMARTCARDS_SMARTCARDBACKGROUNDTRIGGERCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_DEVICES_SMARTCARDS_SMARTCARDBACKGROUNDTRIGGERCONTRACT_VERSION)

#if !defined(WINDOWS_DEVICES_SMARTCARDS_SMARTCARDEMULATORCONTRACT_VERSION)
#define WINDOWS_DEVICES_SMARTCARDS_SMARTCARDEMULATORCONTRACT_VERSION 0x60000
#endif // defined(WINDOWS_DEVICES_SMARTCARDS_SMARTCARDEMULATORCONTRACT_VERSION)

#if !defined(WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION)
#define WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)
#define WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)
#define WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION 0x130000
#endif // defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)

#if !defined(WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION)
#define WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION)

#if !defined(WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION)
#define WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_CAPTURE_APPBROADCASTCONTRACT_VERSION)
#define WINDOWS_MEDIA_CAPTURE_APPBROADCASTCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_MEDIA_CAPTURE_APPBROADCASTCONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_CAPTURE_APPCAPTURECONTRACT_VERSION)
#define WINDOWS_MEDIA_CAPTURE_APPCAPTURECONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_MEDIA_CAPTURE_APPCAPTURECONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_CAPTURE_APPCAPTUREMETADATACONTRACT_VERSION)
#define WINDOWS_MEDIA_CAPTURE_APPCAPTUREMETADATACONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_CAPTURE_APPCAPTUREMETADATACONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_CAPTURE_CAMERACAPTUREUICONTRACT_VERSION)
#define WINDOWS_MEDIA_CAPTURE_CAMERACAPTUREUICONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_CAPTURE_CAMERACAPTUREUICONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_CAPTURE_GAMEBARCONTRACT_VERSION)
#define WINDOWS_MEDIA_CAPTURE_GAMEBARCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_CAPTURE_GAMEBARCONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION)
#define WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_DEVICES_CALLCONTROLCONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION)
#define WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_PROTECTION_PROTECTIONRENEWALCONTRACT_VERSION)
#define WINDOWS_MEDIA_PROTECTION_PROTECTIONRENEWALCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_PROTECTION_PROTECTIONRENEWALCONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION)
#define WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION)

#if !defined(WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION)
#define WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_NETWORKING_CONNECTIVITY_WWANCONTRACT_VERSION)

#if !defined(WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION)
#define WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION)

#if !defined(WINDOWS_PHONE_PHONECONTRACT_VERSION)
#define WINDOWS_PHONE_PHONECONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_PHONE_PHONECONTRACT_VERSION)

#if !defined(WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION)
#define WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION)

#if !defined(WINDOWS_SECURITY_ENTERPRISEDATA_ENTERPRISEDATACONTRACT_VERSION)
#define WINDOWS_SECURITY_ENTERPRISEDATA_ENTERPRISEDATACONTRACT_VERSION 0x50000
#endif // defined(WINDOWS_SECURITY_ENTERPRISEDATA_ENTERPRISEDATACONTRACT_VERSION)

#if !defined(WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION)
#define WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION)

#if !defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)
#define WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)

#if !defined(WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION)
#define WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION)

#if !defined(WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION)
#define WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION)

#if !defined(WINDOWS_UI_WEBUI_CORE_WEBUICOMMANDBARCONTRACT_VERSION)
#define WINDOWS_UI_WEBUI_CORE_WEBUICOMMANDBARCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_WEBUI_CORE_WEBUICOMMANDBARCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Storage.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                interface IConnectionStatusHandler;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler ABI::Windows::Media::Streaming::IConnectionStatusHandler

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                interface IDeviceControllerFinderHandler;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler ABI::Windows::Media::Streaming::IDeviceControllerFinderHandler

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                interface IRenderingParametersUpdateHandler;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler ABI::Windows::Media::Streaming::IRenderingParametersUpdateHandler

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                interface ITransportParametersUpdateHandler;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler ABI::Windows::Media::Streaming::ITransportParametersUpdateHandler

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                interface IActiveBasicDevice;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice ABI::Windows::Media::Streaming::IActiveBasicDevice

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                interface IActiveBasicDeviceStatics;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics ABI::Windows::Media::Streaming::IActiveBasicDeviceStatics

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                interface IBasicDevice;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice ABI::Windows::Media::Streaming::IBasicDevice

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                interface IDeviceController;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController ABI::Windows::Media::Streaming::IDeviceController

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                interface IDeviceIcon;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon ABI::Windows::Media::Streaming::IDeviceIcon

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                interface IDevicePair;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair ABI::Windows::Media::Streaming::IDevicePair

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                interface IMediaRenderer;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer ABI::Windows::Media::Streaming::IMediaRenderer

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                interface IMediaRendererActionInformation;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation ABI::Windows::Media::Streaming::IMediaRendererActionInformation

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                interface IMediaRendererFactory;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory ABI::Windows::Media::Streaming::IMediaRendererFactory

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                interface IStreamSelectorStatics;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics ABI::Windows::Media::Streaming::IStreamSelectorStatics

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                interface ITransportParameters;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters ABI::Windows::Media::Streaming::ITransportParameters

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_USE
#define DEF___FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("84a8c766-4bc5-5757-9f1b-f61cfd9e5693"))
IIterator<ABI::Windows::Media::Streaming::IBasicDevice*> : IIterator_impl<ABI::Windows::Media::Streaming::IBasicDevice*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Streaming.IBasicDevice>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Streaming::IBasicDevice*> __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_t;
#define __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice ABI::Windows::Foundation::Collections::IIterator<ABI::Windows::Media::Streaming::IBasicDevice*>
//#define __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_t ABI::Windows::Foundation::Collections::IIterator<ABI::Windows::Media::Streaming::IBasicDevice*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_USE */


#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice_USE
#define DEF___FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7d468b5e-763b-59cd-a086-ec6d8be0d858"))
IIterable<ABI::Windows::Media::Streaming::IBasicDevice*> : IIterable_impl<ABI::Windows::Media::Streaming::IBasicDevice*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Streaming.IBasicDevice>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Streaming::IBasicDevice*> __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice_t;
#define __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice ABI::Windows::Foundation::Collections::IIterable<ABI::Windows::Media::Streaming::IBasicDevice*>
//#define __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice_t ABI::Windows::Foundation::Collections::IIterable<ABI::Windows::Media::Streaming::IBasicDevice*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice_USE */


#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_USE
#define DEF___FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("57fd211a-4ef0-58a0-90e2-7c3b816102c9"))
IIterator<ABI::Windows::Media::Streaming::IDeviceIcon*> : IIterator_impl<ABI::Windows::Media::Streaming::IDeviceIcon*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Streaming.IDeviceIcon>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Streaming::IDeviceIcon*> __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_t;
#define __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon ABI::Windows::Foundation::Collections::IIterator<ABI::Windows::Media::Streaming::IDeviceIcon*>
//#define __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_t ABI::Windows::Foundation::Collections::IIterator<ABI::Windows::Media::Streaming::IDeviceIcon*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_USE */


#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon_USE
#define DEF___FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("16077ee6-dcfc-53aa-ab0e-d666ac819d6c"))
IIterable<ABI::Windows::Media::Streaming::IDeviceIcon*> : IIterable_impl<ABI::Windows::Media::Streaming::IDeviceIcon*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Streaming.IDeviceIcon>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Streaming::IDeviceIcon*> __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon_t;
#define __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon ABI::Windows::Foundation::Collections::IIterable<ABI::Windows::Media::Streaming::IDeviceIcon*>
//#define __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon_t ABI::Windows::Foundation::Collections::IIterable<ABI::Windows::Media::Streaming::IDeviceIcon*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon_USE */


#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                struct PlaySpeed;
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */


#ifndef DEF___FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_USE
#define DEF___FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fd051cd8-25c7-5780-9606-b35062137d21"))
IIterator<struct ABI::Windows::Media::Streaming::PlaySpeed> : IIterator_impl<struct ABI::Windows::Media::Streaming::PlaySpeed> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Streaming.PlaySpeed>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<struct ABI::Windows::Media::Streaming::PlaySpeed> __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_t;
#define __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed ABI::Windows::Foundation::Collections::IIterator<ABI::Windows::Media::Streaming::PlaySpeed>
//#define __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_t ABI::Windows::Foundation::Collections::IIterator<ABI::Windows::Media::Streaming::PlaySpeed>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_USE */





#ifndef DEF___FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed_USE
#define DEF___FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c4a17a40-8c62-5884-822b-502526970b0d"))
IIterable<struct ABI::Windows::Media::Streaming::PlaySpeed> : IIterable_impl<struct ABI::Windows::Media::Streaming::PlaySpeed> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Streaming.PlaySpeed>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<struct ABI::Windows::Media::Streaming::PlaySpeed> __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed_t;
#define __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed ABI::Windows::Foundation::Collections::IIterable<ABI::Windows::Media::Streaming::PlaySpeed>
//#define __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed_t ABI::Windows::Foundation::Collections::IIterable<ABI::Windows::Media::Streaming::PlaySpeed>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed_USE */




#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_USE
#define DEF___FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a55cf16b-71a2-5525-ac3b-2f5bc1eeec46"))
IVectorView<ABI::Windows::Media::Streaming::IBasicDevice*> : IVectorView_impl<ABI::Windows::Media::Streaming::IBasicDevice*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Streaming.IBasicDevice>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Streaming::IBasicDevice*> __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_t;
#define __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice ABI::Windows::Foundation::Collections::IVectorView<ABI::Windows::Media::Streaming::IBasicDevice*>
//#define __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_t ABI::Windows::Foundation::Collections::IVectorView<ABI::Windows::Media::Streaming::IBasicDevice*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_USE */


#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_USE
#define DEF___FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ff195e52-48eb-5709-be50-3a3914c189db"))
IVectorView<ABI::Windows::Media::Streaming::IDeviceIcon*> : IVectorView_impl<ABI::Windows::Media::Streaming::IDeviceIcon*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Streaming.IDeviceIcon>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Streaming::IDeviceIcon*> __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_t;
#define __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon ABI::Windows::Foundation::Collections::IVectorView<ABI::Windows::Media::Streaming::IDeviceIcon*>
//#define __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_t ABI::Windows::Foundation::Collections::IVectorView<ABI::Windows::Media::Streaming::IDeviceIcon*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_USE */


#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000



#ifndef DEF___FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_USE
#define DEF___FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1295caf3-c1da-54ea-ac66-da2c044f9eb0"))
IVectorView<struct ABI::Windows::Media::Streaming::PlaySpeed> : IVectorView_impl<struct ABI::Windows::Media::Streaming::PlaySpeed> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Streaming.PlaySpeed>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<struct ABI::Windows::Media::Streaming::PlaySpeed> __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_t;
#define __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed ABI::Windows::Foundation::Collections::IVectorView<ABI::Windows::Media::Streaming::PlaySpeed>
//#define __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_t ABI::Windows::Foundation::Collections::IVectorView<ABI::Windows::Media::Streaming::PlaySpeed>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_USE */




#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_USE
#define DEF___FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4c58be45-d16f-5b3a-840d-a6b4e20b7088"))
IVector<ABI::Windows::Media::Streaming::IBasicDevice*> : IVector_impl<ABI::Windows::Media::Streaming::IBasicDevice*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Media.Streaming.IBasicDevice>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Media::Streaming::IBasicDevice*> __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_t;
#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice ABI::Windows::Foundation::Collections::IVector<ABI::Windows::Media::Streaming::IBasicDevice*>
//#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_t ABI::Windows::Foundation::Collections::IVector<ABI::Windows::Media::Streaming::IBasicDevice*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_USE */


#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_USE
#define DEF___FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a32d7731-05f6-55a2-930f-1cf5a12b19ae"))
IVector<ABI::Windows::Media::Streaming::IDeviceIcon*> : IVector_impl<ABI::Windows::Media::Streaming::IDeviceIcon*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Media.Streaming.IDeviceIcon>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Media::Streaming::IDeviceIcon*> __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_t;
#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon ABI::Windows::Foundation::Collections::IVector<ABI::Windows::Media::Streaming::IDeviceIcon*>
//#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_t ABI::Windows::Foundation::Collections::IVector<ABI::Windows::Media::Streaming::IDeviceIcon*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_USE */


#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000



#ifndef DEF___FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_USE
#define DEF___FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fde57c75-5b86-5921-8ffb-101b0a184230"))
IVector<struct ABI::Windows::Media::Streaming::PlaySpeed> : IVector_impl<struct ABI::Windows::Media::Streaming::PlaySpeed> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Media.Streaming.PlaySpeed>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<struct ABI::Windows::Media::Streaming::PlaySpeed> __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_t;
#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed ABI::Windows::Foundation::Collections::IVector<ABI::Windows::Media::Streaming::PlaySpeed>
//#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_t ABI::Windows::Foundation::Collections::IVector<ABI::Windows::Media::Streaming::PlaySpeed>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_USE */



namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                class ActiveBasicDevice;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4c0d279c-4b4e-5657-b609-6f15ff54212e"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::ActiveBasicDevice*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::ActiveBasicDevice*, ABI::Windows::Media::Streaming::IActiveBasicDevice*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Streaming.ActiveBasicDevice>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::ActiveBasicDevice*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice ABI::Windows::Foundation::IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::IActiveBasicDevice*>
//#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice_t ABI::Windows::Foundation::IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::IActiveBasicDevice*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice_USE */


#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0c4a4010-9046-5bff-8cc5-cbcde4c2d5c1"))
IAsyncOperation<ABI::Windows::Media::Streaming::ActiveBasicDevice*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::ActiveBasicDevice*, ABI::Windows::Media::Streaming::IActiveBasicDevice*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Streaming.ActiveBasicDevice>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Streaming::ActiveBasicDevice*> __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_t;
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice ABI::Windows::Foundation::IAsyncOperation<ABI::Windows::Media::Streaming::IActiveBasicDevice*>
//#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_t ABI::Windows::Foundation::IAsyncOperation<ABI::Windows::Media::Streaming::IActiveBasicDevice*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_USE */


#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                class DevicePair;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cde8d503-5136-5cb9-ab04-11502ec5afeb"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::DevicePair*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::DevicePair*, ABI::Windows::Media::Streaming::IDevicePair*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Streaming.DevicePair>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::DevicePair*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair_t
/* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair ABI::Windows::Foundation::IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::IDevicePair*>
//#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair_t ABI::Windows::Foundation::IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::IDevicePair*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair_USE */


#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d3d31100-6a6e-5775-ae0b-55cbeb23cacf"))
IAsyncOperation<ABI::Windows::Media::Streaming::DevicePair*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::DevicePair*, ABI::Windows::Media::Streaming::IDevicePair*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Streaming.DevicePair>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Streaming::DevicePair*> __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_t;
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_t
/* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair ABI::Windows::Foundation::IAsyncOperation<ABI::Windows::Media::Streaming::IDevicePair*>
//#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_t ABI::Windows::Foundation::IAsyncOperation<ABI::Windows::Media::Streaming::IDevicePair*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_USE */


#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                class MediaRenderer;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f0d971af-e054-5616-9fdf-0903b9ceb182"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::MediaRenderer*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::MediaRenderer*, ABI::Windows::Media::Streaming::IMediaRenderer*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Streaming.MediaRenderer>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::MediaRenderer*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer_t
/* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer ABI::Windows::Foundation::IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::IMediaRenderer*>
//#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer_t ABI::Windows::Foundation::IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::IMediaRenderer*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer_USE */


#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("557dd3fb-4710-5059-921c-0dee68361fb5"))
IAsyncOperation<ABI::Windows::Media::Streaming::MediaRenderer*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::MediaRenderer*, ABI::Windows::Media::Streaming::IMediaRenderer*>> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Streaming.MediaRenderer>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Streaming::MediaRenderer*> __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_t;
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_t
/* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer ABI::Windows::Foundation::IAsyncOperation<ABI::Windows::Media::Streaming::IMediaRenderer*>
//#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_t ABI::Windows::Foundation::IAsyncOperation<ABI::Windows::Media::Streaming::IMediaRenderer*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_USE */


#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                struct PositionInformation;
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */


#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("adc7daf4-9a69-5d0b-aec8-e2ee3292d178"))
IAsyncOperationCompletedHandler<struct ABI::Windows::Media::Streaming::PositionInformation> : IAsyncOperationCompletedHandler_impl<struct ABI::Windows::Media::Streaming::PositionInformation> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Streaming.PositionInformation>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<struct ABI::Windows::Media::Streaming::PositionInformation> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation ABI::Windows::Foundation::IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::PositionInformation>
//#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation_t ABI::Windows::Foundation::IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::PositionInformation>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation_USE */





#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e2b45a37-e1c1-5e80-8962-a134d7f3557c"))
IAsyncOperation<struct ABI::Windows::Media::Streaming::PositionInformation> : IAsyncOperation_impl<struct ABI::Windows::Media::Streaming::PositionInformation> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Streaming.PositionInformation>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<struct ABI::Windows::Media::Streaming::PositionInformation> __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_t;
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation ABI::Windows::Foundation::IAsyncOperation<ABI::Windows::Media::Streaming::PositionInformation>
//#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_t ABI::Windows::Foundation::IAsyncOperation<ABI::Windows::Media::Streaming::PositionInformation>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_USE */



namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                struct TransportInformation;
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */


#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9970f463-bcd0-55b9-94cd-8932d42446ca"))
IAsyncOperationCompletedHandler<struct ABI::Windows::Media::Streaming::TransportInformation> : IAsyncOperationCompletedHandler_impl<struct ABI::Windows::Media::Streaming::TransportInformation> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Streaming.TransportInformation>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<struct ABI::Windows::Media::Streaming::TransportInformation> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation ABI::Windows::Foundation::IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::TransportInformation>
//#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation_t ABI::Windows::Foundation::IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::TransportInformation>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation_USE */





#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f99e7d9c-2274-5f3d-89e7-f5f862ba0334"))
IAsyncOperation<struct ABI::Windows::Media::Streaming::TransportInformation> : IAsyncOperation_impl<struct ABI::Windows::Media::Streaming::TransportInformation> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Streaming.TransportInformation>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<struct ABI::Windows::Media::Streaming::TransportInformation> __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_t;
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_t
/* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation ABI::Windows::Foundation::IAsyncOperation<ABI::Windows::Media::Streaming::TransportInformation>
//#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_t ABI::Windows::Foundation::IAsyncOperation<ABI::Windows::Media::Streaming::TransportInformation>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_USE */




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

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterator_1_HSTRING ABI::Windows::Foundation::Collections::IIterator<HSTRING>
//#define __FIIterator_1_HSTRING_t ABI::Windows::Foundation::Collections::IIterator<HSTRING>
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

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterable_1_HSTRING ABI::Windows::Foundation::Collections::IIterable<HSTRING>
//#define __FIIterable_1_HSTRING_t ABI::Windows::Foundation::Collections::IIterable<HSTRING>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_HSTRING_USE */




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

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIVectorView_1_HSTRING ABI::Windows::Foundation::Collections::IVectorView<HSTRING>
//#define __FIVectorView_1_HSTRING_t ABI::Windows::Foundation::Collections::IVectorView<HSTRING>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_HSTRING_USE */




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

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIVector_1_HSTRING ABI::Windows::Foundation::Collections::IVector<HSTRING>
//#define __FIVector_1_HSTRING_t ABI::Windows::Foundation::Collections::IVector<HSTRING>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_HSTRING_USE */




#ifndef DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE
#define DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9343b6e7-e3d2-5e4a-ab2d-2bce4919a6a4"))
IAsyncOperationCompletedHandler<UINT32> : IAsyncOperationCompletedHandler_impl<UINT32> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<UInt32>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<UINT32> __FIAsyncOperationCompletedHandler_1_UINT32_t;
#define __FIAsyncOperationCompletedHandler_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperationCompletedHandler_1_UINT32 ABI::Windows::Foundation::IAsyncOperationCompletedHandler<UINT32>
//#define __FIAsyncOperationCompletedHandler_1_UINT32_t ABI::Windows::Foundation::IAsyncOperationCompletedHandler<UINT32>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE */




#ifndef DEF___FIAsyncOperation_1_UINT32_USE
#define DEF___FIAsyncOperation_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ef60385f-be78-584b-aaef-7829ada2b0de"))
IAsyncOperation<UINT32> : IAsyncOperation_impl<UINT32> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.IAsyncOperation`1<UInt32>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<UINT32> __FIAsyncOperation_1_UINT32_t;
#define __FIAsyncOperation_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperation_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperation_1_UINT32 ABI::Windows::Foundation::IAsyncOperation<UINT32>
//#define __FIAsyncOperation_1_UINT32_t ABI::Windows::Foundation::IAsyncOperation<UINT32>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_UINT32_USE */




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

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperationCompletedHandler_1_boolean ABI::Windows::Foundation::IAsyncOperationCompletedHandler<boolean>
//#define __FIAsyncOperationCompletedHandler_1_boolean_t ABI::Windows::Foundation::IAsyncOperationCompletedHandler<boolean>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_boolean_USE */




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

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperation_1_boolean ABI::Windows::Foundation::IAsyncOperation<boolean>
//#define __FIAsyncOperation_1_boolean_t ABI::Windows::Foundation::IAsyncOperation<boolean>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_boolean_USE */



#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStreamWithContentType;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3dddecf4-1d39-58e8-83b1-dbed541c7f35"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IRandomAccessStreamWithContentType>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_t
/* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType ABI::Windows::Foundation::IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*>
//#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_t ABI::Windows::Foundation::IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE */


#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c4a57c5e-32b0-55b3-ad13-ce1c23041ed6"))
IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IRandomAccessStreamWithContentType>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_t
/* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType ABI::Windows::Foundation::IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*>
//#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_t ABI::Windows::Foundation::IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_USE */


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


#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___F__CIPropertySet_USE
#define DEF___FIIterator_1___F__CIPropertySet_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d79a75c8-b1d2-544d-9b09-7f7900a34efb"))
IIterator<ABI::Windows::Foundation::Collections::IPropertySet*> : IIterator_impl<ABI::Windows::Foundation::Collections::IPropertySet*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IPropertySet>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Foundation::Collections::IPropertySet*> __FIIterator_1___F__CIPropertySet_t;
#define __FIIterator_1___F__CIPropertySet ABI::Windows::Foundation::Collections::__FIIterator_1___F__CIPropertySet_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterator_1___F__CIPropertySet ABI::Windows::Foundation::Collections::IIterator<ABI::Windows::Foundation::Collections::IPropertySet*>
//#define __FIIterator_1___F__CIPropertySet_t ABI::Windows::Foundation::Collections::IIterator<ABI::Windows::Foundation::Collections::IPropertySet*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___F__CIPropertySet_USE */


#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000


#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___F__CIPropertySet_USE
#define DEF___FIIterable_1___F__CIPropertySet_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("489b756d-be43-5abb-b9a0-a47254103339"))
IIterable<ABI::Windows::Foundation::Collections::IPropertySet*> : IIterable_impl<ABI::Windows::Foundation::Collections::IPropertySet*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IPropertySet>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Foundation::Collections::IPropertySet*> __FIIterable_1___F__CIPropertySet_t;
#define __FIIterable_1___F__CIPropertySet ABI::Windows::Foundation::Collections::__FIIterable_1___F__CIPropertySet_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterable_1___F__CIPropertySet ABI::Windows::Foundation::Collections::IIterable<ABI::Windows::Foundation::Collections::IPropertySet*>
//#define __FIIterable_1___F__CIPropertySet_t ABI::Windows::Foundation::Collections::IIterable<ABI::Windows::Foundation::Collections::IPropertySet*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___F__CIPropertySet_USE */


#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000


#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1___F__CIPropertySet_USE
#define DEF___FIVectorView_1___F__CIPropertySet_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c25d9a17-c31e-5311-8122-3c04d28af9fc"))
IVectorView<ABI::Windows::Foundation::Collections::IPropertySet*> : IVectorView_impl<ABI::Windows::Foundation::Collections::IPropertySet*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Foundation.Collections.IPropertySet>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Foundation::Collections::IPropertySet*> __FIVectorView_1___F__CIPropertySet_t;
#define __FIVectorView_1___F__CIPropertySet ABI::Windows::Foundation::Collections::__FIVectorView_1___F__CIPropertySet_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIVectorView_1___F__CIPropertySet ABI::Windows::Foundation::Collections::IVectorView<ABI::Windows::Foundation::Collections::IPropertySet*>
//#define __FIVectorView_1___F__CIPropertySet_t ABI::Windows::Foundation::Collections::IVectorView<ABI::Windows::Foundation::Collections::IPropertySet*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1___F__CIPropertySet_USE */


#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000


#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("af4e2f8a-92ca-5640-865c-9948fbde4495"))
IAsyncOperationCompletedHandler<__FIVectorView_1___F__CIPropertySet*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1___F__CIPropertySet*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Foundation.Collections.IPropertySet>>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1___F__CIPropertySet*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet_t
/* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet ABI::Windows::Foundation::IAsyncOperationCompletedHandler<ABI::Windows::Foundation::Collections::IVectorView<ABI::Windows::Foundation::Collections::IPropertySet*>*>
//#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet_t ABI::Windows::Foundation::IAsyncOperationCompletedHandler<ABI::Windows::Foundation::Collections::IVectorView<ABI::Windows::Foundation::Collections::IPropertySet*>*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet_USE */


#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000


#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("216f9390-ea3d-5465-a789-6394a47eb4a4"))
IAsyncOperation<__FIVectorView_1___F__CIPropertySet*> : IAsyncOperation_impl<__FIVectorView_1___F__CIPropertySet*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Foundation.Collections.IPropertySet>>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1___F__CIPropertySet*> __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_t;
#define __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_t
/* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet ABI::Windows::Foundation::IAsyncOperation<ABI::Windows::Foundation::Collections::IVectorView<ABI::Windows::Foundation::Collections::IPropertySet*>*>
//#define __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_t ABI::Windows::Foundation::IAsyncOperation<ABI::Windows::Foundation::Collections::IVectorView<ABI::Windows::Foundation::Collections::IPropertySet*>*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_USE */


#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000




namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                
                typedef enum ConnectionStatus : int ConnectionStatus;
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                
                typedef enum DeviceTypes : unsigned int DeviceTypes;
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                
                typedef enum TransportState : int TransportState;
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                
                typedef enum TransportStatus : int TransportStatus;
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                
                typedef struct PlaySpeed PlaySpeed;
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                
                typedef struct PositionInformation PositionInformation;
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                
                typedef struct RenderingParameters RenderingParameters;
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                
                typedef struct TrackInformation TrackInformation;
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                
                typedef struct TransportInformation TransportInformation;
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

















namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                class BasicDevice;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */


namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                class CreateMediaRendererOperation;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */


namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                class DeviceController;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */














/*
 *
 * Struct Windows.Media.Streaming.ConnectionStatus
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */

#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [v1_enum, contract] */
                enum ConnectionStatus : int
                {
                    ConnectionStatus_Online = 0,
                    ConnectionStatus_Offline = 1,
                    ConnectionStatus_Sleeping = 2,
                };
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Struct Windows.Media.Streaming.DeviceTypes
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */

#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [v1_enum, flags, contract] */
                enum DeviceTypes : unsigned int
                {
                    DeviceTypes_Unknown = 0,
                    DeviceTypes_DigitalMediaRenderer = 0x1,
                    DeviceTypes_DigitalMediaServer = 0x2,
                    DeviceTypes_DigitalMediaPlayer = 0x4,
                };
                
                DEFINE_ENUM_FLAG_OPERATORS(DeviceTypes)
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Struct Windows.Media.Streaming.TransportState
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */

#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [v1_enum, contract] */
                enum TransportState : int
                {
                    TransportState_Unknown = 0,
                    TransportState_Stopped = 1,
                    TransportState_Playing = 2,
                    TransportState_Transitioning = 3,
                    TransportState_Paused = 4,
                    TransportState_Recording = 5,
                    TransportState_NoMediaPresent = 6,
                    TransportState_Last = 7,
                };
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Struct Windows.Media.Streaming.TransportStatus
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */

#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [v1_enum, contract] */
                enum TransportStatus : int
                {
                    TransportStatus_Unknown = 0,
                    TransportStatus_Ok = 1,
                    TransportStatus_ErrorOccurred = 2,
                    TransportStatus_Last = 3,
                };
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Struct Windows.Media.Streaming.PlaySpeed
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [contract] */
                struct PlaySpeed
                {
                    INT32 Numerator;
                    UINT32 Denominator;
                };
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Struct Windows.Media.Streaming.TrackInformation
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [contract] */
                struct TrackInformation
                {
                    UINT32 Track;
                    UINT32 TrackId;
                    ABI::Windows::Foundation::TimeSpan TrackDuration;
                };
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Struct Windows.Media.Streaming.PositionInformation
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [contract] */
                struct PositionInformation
                {
                    ABI::Windows::Media::Streaming::TrackInformation trackInformation;
                    ABI::Windows::Foundation::TimeSpan relativeTime;
                };
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Struct Windows.Media.Streaming.RenderingParameters
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [contract] */
                struct RenderingParameters
                {
                    UINT32 volume;
                    ::boolean mute;
                };
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Struct Windows.Media.Streaming.TransportInformation
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [contract] */
                struct TransportInformation
                {
                    ABI::Windows::Media::Streaming::TransportState CurrentTransportState;
                    ABI::Windows::Media::Streaming::TransportStatus CurrentTransportStatus;
                    ABI::Windows::Media::Streaming::PlaySpeed CurrentSpeed;
                };
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Delegate Windows.Media.Streaming.ConnectionStatusHandler
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [object, uuid("B571C28C-A472-48D5-88D2-8ADCAF1B8813"), contract] */
                MIDL_INTERFACE("B571C28C-A472-48D5-88D2-8ADCAF1B8813")
                IConnectionStatusHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::IBasicDevice * sender,
                        /* [in] */ABI::Windows::Media::Streaming::ConnectionStatus arg
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IConnectionStatusHandler=__uuidof(IConnectionStatusHandler);
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Delegate Windows.Media.Streaming.DeviceControllerFinderHandler
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [object, uuid("A88A7D06-988C-4403-9D8A-015BED140B34"), contract] */
                MIDL_INTERFACE("A88A7D06-988C-4403-9D8A-015BED140B34")
                IDeviceControllerFinderHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::IDeviceController * sender,
                        /* [in] */__RPC__in HSTRING uniqueDeviceName,
                        /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::IBasicDevice * device
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IDeviceControllerFinderHandler=__uuidof(IDeviceControllerFinderHandler);
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Delegate Windows.Media.Streaming.RenderingParametersUpdateHandler
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [object, uuid("3A2D9D45-72E9-4311-B46C-27C8BB7E6CB3"), contract] */
                MIDL_INTERFACE("3A2D9D45-72E9-4311-B46C-27C8BB7E6CB3")
                IRenderingParametersUpdateHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::IMediaRenderer * sender,
                        /* [in] */ABI::Windows::Media::Streaming::RenderingParameters arg
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IRenderingParametersUpdateHandler=__uuidof(IRenderingParametersUpdateHandler);
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Delegate Windows.Media.Streaming.TransportParametersUpdateHandler
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [object, uuid("16FD02D5-DA61-49D7-AAB2-76867DD42DB7"), contract] */
                MIDL_INTERFACE("16FD02D5-DA61-49D7-AAB2-76867DD42DB7")
                ITransportParametersUpdateHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::IMediaRenderer * sender,
                        /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::ITransportParameters * arg
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_ITransportParametersUpdateHandler=__uuidof(ITransportParametersUpdateHandler);
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IActiveBasicDevice
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Streaming.IBasicDevice
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IActiveBasicDevice[] = L"Windows.Media.Streaming.IActiveBasicDevice";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [object, uuid("B64D6974-6E79-49AF-9933-908B6E9A160C"), contract] */
                MIDL_INTERFACE("B64D6974-6E79-49AF-9933-908B6E9A160C")
                IActiveBasicDevice : public IInspectable
                {
                public:
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_MaxVolume(
                        /* [retval, out] */__RPC__out UINT32 * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IsMuteSupported(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IsSetNextSourceSupported(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IsAudioSupported(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IsVideoSupported(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IsImageSupported(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IsSearchSupported(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCachedSinkProtocolInfo(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetCachedSinkProtocolInfo(
                        /* [in] */__RPC__in HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCachedExtraSinkProtocolInfo(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetEffectiveBandwidth(
                        /* [in] */::boolean transmitSpeed,
                        /* [retval, out] */__RPC__out UINT64 * currentSpeed
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCachedBitrateMeasurement(
                        /* [in] */GUID physicalNetworkInterface,
                        /* [retval, out] */__RPC__out UINT64 * bitrate
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetCachedBitrateMeasurement(
                        /* [in] */GUID physicalNetworkInterface,
                        /* [in] */UINT64 bitrate
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_LogicalNetworkInterface(
                        /* [retval, out] */__RPC__out GUID * logicalNetworkInterface
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_PhysicalNetworkInterface(
                        /* [retval, out] */__RPC__out GUID * physicalNetworkInterface
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyStreamingStatus(
                        /* [in] */::boolean fIsStreaming
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IActiveBasicDevice=__uuidof(IActiveBasicDevice);
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IActiveBasicDeviceStatics
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IActiveBasicDeviceStatics[] = L"Windows.Media.Streaming.IActiveBasicDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [object, uuid("6D33255D-3642-4618-9DB6-43524F4DEADC"), contract] */
                MIDL_INTERFACE("6D33255D-3642-4618-9DB6-43524F4DEADC")
                IActiveBasicDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateBasicDeviceAsync(
                        /* [in] */__RPC__in HSTRING deviceIdentifier,
                        /* [in] */ABI::Windows::Media::Streaming::DeviceTypes type,
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CloneBasicDeviceAsync(
                        /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::IBasicDevice * basicDevice,
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDevicesOnMatchingNetworkAsync(
                        /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::IActiveBasicDevice * server,
                        /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::IActiveBasicDevice * renderer,
                        /* [in] */::boolean optimizeForProxying,
                        /* [in] */::boolean allowChangeRendererNetwork,
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair * * operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateDevicesOnMatchingNetworkAsync(
                        /* [in] */__RPC__in HSTRING serverUDN,
                        /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::IActiveBasicDevice * renderer,
                        /* [in] */::boolean optimizeForProxying,
                        /* [in] */::boolean allowChangeRendererNetwork,
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair * * operation
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IActiveBasicDeviceStatics=__uuidof(IActiveBasicDeviceStatics);
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IBasicDevice
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IBasicDevice[] = L"Windows.Media.Streaming.IBasicDevice";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [object, uuid("F4F26CBB-7962-48B7-80F7-C3A5D753BCB0"), contract] */
                MIDL_INTERFACE("F4F26CBB-7962-48B7-80F7-C3A5D753BCB0")
                IBasicDevice : public IInspectable
                {
                public:
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_FriendlyName(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
                        ) = 0;
                    /* [propput] */virtual HRESULT STDMETHODCALLTYPE put_FriendlyName(
                        /* [in] */__RPC__in HSTRING value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ManufacturerName(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ManufacturerUrl(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_UniqueDeviceName(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ModelName(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ModelNumber(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ModelUrl(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Description(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_SerialNumber(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_PresentationUrl(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_RemoteStreamingUrls(
                        /* [retval, out] */__RPC__deref_out_opt __FIVector_1_HSTRING * * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_PhysicalAddresses(
                        /* [retval, out] */__RPC__deref_out_opt __FIVector_1_HSTRING * * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IpAddresses(
                        /* [retval, out] */__RPC__deref_out_opt __FIVector_1_HSTRING * * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_CanWakeDevices(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_DiscoveredOnCurrentNetwork(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Type(
                        /* [retval, out] */__RPC__out ABI::Windows::Media::Streaming::DeviceTypes * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Icons(
                        /* [retval, out] */__RPC__deref_out_opt __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ConnectionStatus(
                        /* [retval, out] */__RPC__out ABI::Windows::Media::Streaming::ConnectionStatus * value
                        ) = 0;
                    /* [eventadd] */virtual HRESULT STDMETHODCALLTYPE add_ConnectionStatusChanged(
                        /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::IConnectionStatusHandler  * handler,
                        /* [retval, out] */__RPC__out EventRegistrationToken * token
                        ) = 0;
                    /* [eventremove] */virtual HRESULT STDMETHODCALLTYPE remove_ConnectionStatusChanged(
                        /* [in] */EventRegistrationToken token
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IBasicDevice=__uuidof(IBasicDevice);
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IDeviceController
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IDeviceController[] = L"Windows.Media.Streaming.IDeviceController";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [object, uuid("4FEEB26D-50A7-402B-896A-BE95064D6BFF"), contract] */
                MIDL_INTERFACE("4FEEB26D-50A7-402B-896A-BE95064D6BFF")
                IDeviceController : public IInspectable
                {
                public:
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_CachedDevices(
                        /* [retval, out] */__RPC__deref_out_opt __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddDevice(
                        /* [in] */__RPC__in HSTRING uniqueDeviceName
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveDevice(
                        /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::IBasicDevice * device
                        ) = 0;
                    /* [eventadd] */virtual HRESULT STDMETHODCALLTYPE add_DeviceArrival(
                        /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::IDeviceControllerFinderHandler  * handler,
                        /* [retval, out] */__RPC__out EventRegistrationToken * token
                        ) = 0;
                    /* [eventremove] */virtual HRESULT STDMETHODCALLTYPE remove_DeviceArrival(
                        /* [in] */EventRegistrationToken token
                        ) = 0;
                    /* [eventadd] */virtual HRESULT STDMETHODCALLTYPE add_DeviceDeparture(
                        /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::IDeviceControllerFinderHandler  * handler,
                        /* [retval, out] */__RPC__out EventRegistrationToken * token
                        ) = 0;
                    /* [eventremove] */virtual HRESULT STDMETHODCALLTYPE remove_DeviceDeparture(
                        /* [in] */EventRegistrationToken token
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IDeviceController=__uuidof(IDeviceController);
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIDeviceController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IDeviceIcon
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IDeviceIcon[] = L"Windows.Media.Streaming.IDeviceIcon";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [object, uuid("8FFB1A1E-023D-4DE1-B556-AB5ABF01929C"), contract] */
                MIDL_INTERFACE("8FFB1A1E-023D-4DE1-B556-AB5ABF01929C")
                IDeviceIcon : public IInspectable
                {
                public:
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Width(
                        /* [retval, out] */__RPC__out UINT32 * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Height(
                        /* [retval, out] */__RPC__out UINT32 * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ContentType(
                        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Stream(
                        /* [retval, out] */__RPC__deref_out_opt ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType * * value
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IDeviceIcon=__uuidof(IDeviceIcon);
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IDevicePair
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IDevicePair[] = L"Windows.Media.Streaming.IDevicePair";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [object, uuid("F1A423F1-B7B4-449C-A90D-AEA8E17C5E5F"), contract] */
                MIDL_INTERFACE("F1A423F1-B7B4-449C-A90D-AEA8E17C5E5F")
                IDevicePair : public IInspectable
                {
                public:
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Server(
                        /* [retval, out] */__RPC__deref_out_opt ABI::Windows::Media::Streaming::IActiveBasicDevice * * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Renderer(
                        /* [retval, out] */__RPC__deref_out_opt ABI::Windows::Media::Streaming::IActiveBasicDevice * * value
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IDevicePair=__uuidof(IDevicePair);
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIDevicePair;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IMediaRenderer
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Streaming.IBasicDevice
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IMediaRenderer[] = L"Windows.Media.Streaming.IMediaRenderer";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [object, uuid("2C012EC3-D975-47FB-96AC-A6418B326D2B"), contract] */
                MIDL_INTERFACE("2C012EC3-D975-47FB-96AC-A6418B326D2B")
                IMediaRenderer : public IInspectable
                {
                public:
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IsAudioSupported(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IsVideoSupported(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IsImageSupported(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ActionInformation(
                        /* [retval, out] */__RPC__deref_out_opt ABI::Windows::Media::Streaming::IMediaRendererActionInformation * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetSourceFromUriAsync(
                        /* [in] */__RPC__in HSTRING URI,
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_UINT32 * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetSourceFromStreamAsync(
                        /* [in] */__RPC__in_opt IInspectable * stream,
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_UINT32 * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetSourceFromMediaSourceAsync(
                        /* [in] */__RPC__in_opt IInspectable * mediaSource,
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_UINT32 * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetNextSourceFromUriAsync(
                        /* [in] */__RPC__in HSTRING URI,
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_UINT32 * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetNextSourceFromStreamAsync(
                        /* [in] */__RPC__in_opt IInspectable * stream,
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_UINT32 * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetNextSourceFromMediaSourceAsync(
                        /* [in] */__RPC__in_opt IInspectable * mediaSource,
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_UINT32 * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PlayAsync(
                        /* [retval, out] */__RPC__deref_out_opt ABI::Windows::Foundation::IAsyncAction * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PlayAtSpeedAsync(
                        /* [in] */ABI::Windows::Media::Streaming::PlaySpeed playSpeed,
                        /* [retval, out] */__RPC__deref_out_opt ABI::Windows::Foundation::IAsyncAction * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopAsync(
                        /* [retval, out] */__RPC__deref_out_opt ABI::Windows::Foundation::IAsyncAction * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PauseAsync(
                        /* [retval, out] */__RPC__deref_out_opt ABI::Windows::Foundation::IAsyncAction * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetMuteAsync(
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_boolean * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetMuteAsync(
                        /* [in] */::boolean mute,
                        /* [retval, out] */__RPC__deref_out_opt ABI::Windows::Foundation::IAsyncAction * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetVolumeAsync(
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_UINT32 * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetVolumeAsync(
                        /* [in] */UINT32 volume,
                        /* [retval, out] */__RPC__deref_out_opt ABI::Windows::Foundation::IAsyncAction * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SeekAsync(
                        /* [in] */ABI::Windows::Foundation::TimeSpan target,
                        /* [retval, out] */__RPC__deref_out_opt ABI::Windows::Foundation::IAsyncAction * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTransportInformationAsync(
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPositionInformationAsync(
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation * * value
                        ) = 0;
                    /* [eventadd] */virtual HRESULT STDMETHODCALLTYPE add_TransportParametersUpdate(
                        /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::ITransportParametersUpdateHandler  * handler,
                        /* [retval, out] */__RPC__out EventRegistrationToken * token
                        ) = 0;
                    /* [eventremove] */virtual HRESULT STDMETHODCALLTYPE remove_TransportParametersUpdate(
                        /* [in] */EventRegistrationToken token
                        ) = 0;
                    /* [eventadd] */virtual HRESULT STDMETHODCALLTYPE add_RenderingParametersUpdate(
                        /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::IRenderingParametersUpdateHandler  * handler,
                        /* [retval, out] */__RPC__out EventRegistrationToken * token
                        ) = 0;
                    /* [eventremove] */virtual HRESULT STDMETHODCALLTYPE remove_RenderingParametersUpdate(
                        /* [in] */EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NextAsync(
                        /* [retval, out] */__RPC__deref_out_opt ABI::Windows::Foundation::IAsyncAction * * value
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IMediaRenderer=__uuidof(IMediaRenderer);
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IMediaRendererActionInformation
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IMediaRendererActionInformation[] = L"Windows.Media.Streaming.IMediaRendererActionInformation";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [object, uuid("66FBBFEE-5AB0-4A4F-8D15-E5056B26BEDA"), contract] */
                MIDL_INTERFACE("66FBBFEE-5AB0-4A4F-8D15-E5056B26BEDA")
                IMediaRendererActionInformation : public IInspectable
                {
                public:
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IsMuteAvailable(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IsPauseAvailable(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IsPlayAvailable(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IsSeekAvailable(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IsSetNextSourceAvailable(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IsStopAvailable(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IsVolumeAvailable(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_PlaySpeeds(
                        /* [retval, out] */__RPC__deref_out_opt __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * * value
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IMediaRendererActionInformation=__uuidof(IMediaRendererActionInformation);
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IMediaRendererFactory
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IMediaRendererFactory[] = L"Windows.Media.Streaming.IMediaRendererFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [object, uuid("657AB43D-B909-42B2-94D0-E3A0B134E8C9"), contract] */
                MIDL_INTERFACE("657AB43D-B909-42B2-94D0-E3A0B134E8C9")
                IMediaRendererFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateMediaRendererAsync(
                        /* [in] */__RPC__in HSTRING deviceIdentifier,
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateMediaRendererFromBasicDeviceAsync(
                        /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::IBasicDevice * basicDevice,
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer * * value
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IMediaRendererFactory=__uuidof(IMediaRendererFactory);
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IStreamSelectorStatics
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IStreamSelectorStatics[] = L"Windows.Media.Streaming.IStreamSelectorStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [object, uuid("8A4CD4A1-ED85-4E0F-BD68-8A6862E4636D"), contract] */
                MIDL_INTERFACE("8A4CD4A1-ED85-4E0F-BD68-8A6862E4636D")
                IStreamSelectorStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SelectBestStreamAsync(
                        /* [in] */__RPC__in_opt ABI::Windows::Storage::IStorageFile * storageFile,
                        /* [in] */__RPC__in_opt ABI::Windows::Foundation::Collections::IPropertySet * selectorProperties,
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetStreamPropertiesAsync(
                        /* [in] */__RPC__in_opt ABI::Windows::Storage::IStorageFile * storageFile,
                        /* [in] */__RPC__in_opt ABI::Windows::Foundation::Collections::IPropertySet * selectorProperties,
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SelectBestStreamFromStreamAsync(
                        /* [in] */__RPC__in_opt ABI::Windows::Storage::Streams::IRandomAccessStream * stream,
                        /* [in] */__RPC__in_opt ABI::Windows::Foundation::Collections::IPropertySet * selectorProperties,
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType * * value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetStreamPropertiesFromStreamAsync(
                        /* [in] */__RPC__in_opt ABI::Windows::Storage::Streams::IRandomAccessStream * stream,
                        /* [in] */__RPC__in_opt ABI::Windows::Foundation::Collections::IPropertySet * selectorProperties,
                        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet * * value
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_IStreamSelectorStatics=__uuidof(IStreamSelectorStatics);
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.ITransportParameters
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_ITransportParameters[] = L"Windows.Media.Streaming.ITransportParameters";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                /* [object, uuid("EB0C4E24-2283-438D-8FFF-DBE9DF1CB2CC"), contract] */
                MIDL_INTERFACE("EB0C4E24-2283-438D-8FFF-DBE9DF1CB2CC")
                ITransportParameters : public IInspectable
                {
                public:
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ActionInformation(
                        /* [retval, out] */__RPC__deref_out_opt ABI::Windows::Media::Streaming::IMediaRendererActionInformation * * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_TrackInformation(
                        /* [retval, out] */__RPC__out ABI::Windows::Media::Streaming::TrackInformation * value
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_TransportInformation(
                        /* [retval, out] */__RPC__out ABI::Windows::Media::Streaming::TransportInformation * value
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_ITransportParameters=__uuidof(ITransportParameters);
                
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CITransportParameters;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.Media.Streaming.ActiveBasicDevice
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Media.Streaming.StreamingContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Streaming.IActiveBasicDeviceStatics interface starting with version 1.0 of the Windows.Media.Streaming.StreamingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.IActiveBasicDevice ** Default Interface **
 *    Windows.Media.Streaming.IBasicDevice
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Media_Streaming_ActiveBasicDevice_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_ActiveBasicDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_ActiveBasicDevice[] = L"Windows.Media.Streaming.ActiveBasicDevice";
#endif
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.Media.Streaming.BasicDevice
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Media.Streaming.StreamingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.IBasicDevice ** Default Interface **
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Media_Streaming_BasicDevice_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_BasicDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_BasicDevice[] = L"Windows.Media.Streaming.BasicDevice";
#endif
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.Media.Streaming.CreateMediaRendererOperation
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncOperation_1_Windows.Media.Streaming.MediaRenderer ** Default Interface **
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Media_Streaming_CreateMediaRendererOperation_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_CreateMediaRendererOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_CreateMediaRendererOperation[] = L"Windows.Media.Streaming.CreateMediaRendererOperation";
#endif
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.Media.Streaming.DeviceController
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Media.Streaming.StreamingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.IDeviceController ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Media_Streaming_DeviceController_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_DeviceController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_DeviceController[] = L"Windows.Media.Streaming.DeviceController";
#endif
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.Media.Streaming.DevicePair
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.IDevicePair ** Default Interface **
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Media_Streaming_DevicePair_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_DevicePair_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_DevicePair[] = L"Windows.Media.Streaming.DevicePair";
#endif
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.Media.Streaming.MediaRenderer
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Media.Streaming.StreamingContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Streaming.IMediaRendererFactory interface starting with version 1.0 of the Windows.Media.Streaming.StreamingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.IMediaRenderer ** Default Interface **
 *    Windows.Media.Streaming.IBasicDevice
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Media_Streaming_MediaRenderer_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_MediaRenderer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_MediaRenderer[] = L"Windows.Media.Streaming.MediaRenderer";
#endif
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.Media.Streaming.StreamSelector
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Streaming.IStreamSelectorStatics interface starting with version 1.0 of the Windows.Media.Streaming.StreamingContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_StreamSelector_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_StreamSelector_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_StreamSelector[] = L"Windows.Media.Streaming.StreamSelector";
#endif
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000




#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler __x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler __x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice;

typedef struct __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Current )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice * This, /* [retval][out] */ __RPC__out __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * *current);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HasCurrent )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *MoveNext )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDeviceVtbl;

interface __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDeviceVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_get_Current(This,current)	\
    ( (This)->lpVtbl -> get_Current(This,current) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_get_HasCurrent(This,hasCurrent)	\
    ( (This)->lpVtbl -> get_HasCurrent(This,hasCurrent) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_MoveNext(This,hasCurrent)	\
    ( (This)->lpVtbl -> MoveNext(This,hasCurrent) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_GetMany(This,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,capacity,items,actual) ) 

#endif /* COBJMACROS */


#endif // ____FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice_INTERFACE_DEFINED__

#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice;

typedef  struct __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice * This);

    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
                                           /* [out] */ __RPC__out ULONG *iidCount,
                                           /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *First )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice * This, /* [retval][out] */ __RPC__deref_out_opt __FIIterator_1_Windows__CMedia__CStreaming__CIBasicDevice **first);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDeviceVtbl;

interface __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDeviceVtbl *lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice_First(This,first)	\
    ( (This)->lpVtbl -> First(This,first) ) 

#endif /* COBJMACROS */


#endif // ____FIIterable_1_Windows__CMedia__CStreaming__CIBasicDevice_INTERFACE_DEFINED__

#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon;

typedef struct __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIconVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Current )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon * This, /* [retval][out] */ __RPC__out __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * *current);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HasCurrent )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *MoveNext )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIconVtbl;

interface __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIconVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_get_Current(This,current)	\
    ( (This)->lpVtbl -> get_Current(This,current) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_get_HasCurrent(This,hasCurrent)	\
    ( (This)->lpVtbl -> get_HasCurrent(This,hasCurrent) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_MoveNext(This,hasCurrent)	\
    ( (This)->lpVtbl -> MoveNext(This,hasCurrent) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetMany(This,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,capacity,items,actual) ) 

#endif /* COBJMACROS */


#endif // ____FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon_INTERFACE_DEFINED__

#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon;

typedef  struct __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIconVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon * This);

    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
                                           /* [out] */ __RPC__out ULONG *iidCount,
                                           /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *First )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon * This, /* [retval][out] */ __RPC__deref_out_opt __FIIterator_1_Windows__CMedia__CStreaming__CIDeviceIcon **first);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIconVtbl;

interface __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIconVtbl *lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon_First(This,first)	\
    ( (This)->lpVtbl -> First(This,first) ) 

#endif /* COBJMACROS */


#endif // ____FIIterable_1_Windows__CMedia__CStreaming__CIDeviceIcon_INTERFACE_DEFINED__

#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

struct __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed;

#if !defined(____FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed;

typedef struct __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeedVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Current )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed * This, /* [retval][out] */ __RPC__out struct __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed *current);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HasCurrent )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *MoveNext )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) struct __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeedVtbl;

interface __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeedVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_get_Current(This,current)	\
    ( (This)->lpVtbl -> get_Current(This,current) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_get_HasCurrent(This,hasCurrent)	\
    ( (This)->lpVtbl -> get_HasCurrent(This,hasCurrent) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_MoveNext(This,hasCurrent)	\
    ( (This)->lpVtbl -> MoveNext(This,hasCurrent) ) 

#define __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_GetMany(This,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,capacity,items,actual) ) 

#endif /* COBJMACROS */


#endif // ____FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed_INTERFACE_DEFINED__



#if !defined(____FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed;

typedef  struct __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeedVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed * This);

    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
                                           /* [out] */ __RPC__out ULONG *iidCount,
                                           /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *First )(__RPC__in __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed * This, /* [retval][out] */ __RPC__deref_out_opt __FIIterator_1_Windows__CMedia__CStreaming__CPlaySpeed **first);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeedVtbl;

interface __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeedVtbl *lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed_First(This,first)	\
    ( (This)->lpVtbl -> First(This,first) ) 

#endif /* COBJMACROS */


#endif // ____FIIterable_1_Windows__CMedia__CStreaming__CPlaySpeed_INTERFACE_DEFINED__



#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice;

typedef struct __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )( __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice * This);

    ULONG ( STDMETHODCALLTYPE *Release )( __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )( __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
                                            /* [out] */ __RPC__out ULONG *iidCount,
                                            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
        __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
        __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *GetAt )( 
                                         __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
                                         /* [in] */ unsigned int index,
                                         /* [retval][out] */ __RPC__out __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * *item);

        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
            /* [retval][out] */ __RPC__out unsigned int *size);

        HRESULT ( STDMETHODCALLTYPE *IndexOf )( 
                                               __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
            /* [in] */ __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * item,
            /* [out] */ __RPC__out unsigned int *index,
            /* [retval][out] */ __RPC__out boolean *found);

        HRESULT ( STDMETHODCALLTYPE *GetMany )( 
                                               __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
            /* [in] */ unsigned int startIndex,
            /* [in] */ unsigned int capacity,
            /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * *items,
            /* [retval][out] */ __RPC__out unsigned int *actual);

        END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDeviceVtbl;

interface __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDeviceVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_GetAt(This,index,item)	\
    ( (This)->lpVtbl -> GetAt(This,index,item) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_IndexOf(This,item,index,found)	\
    ( (This)->lpVtbl -> IndexOf(This,item,index,found) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_GetMany(This,startIndex,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,startIndex,capacity,items,actual) ) 

#endif /* COBJMACROS */



#endif // ____FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice_INTERFACE_DEFINED__

#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon;

typedef struct __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIconVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )( __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon * This);

    ULONG ( STDMETHODCALLTYPE *Release )( __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )( __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
                                            /* [out] */ __RPC__out ULONG *iidCount,
                                            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
        __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
        __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *GetAt )( 
                                         __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
                                         /* [in] */ unsigned int index,
                                         /* [retval][out] */ __RPC__out __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * *item);

        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
            /* [retval][out] */ __RPC__out unsigned int *size);

        HRESULT ( STDMETHODCALLTYPE *IndexOf )( 
                                               __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
            /* [in] */ __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * item,
            /* [out] */ __RPC__out unsigned int *index,
            /* [retval][out] */ __RPC__out boolean *found);

        HRESULT ( STDMETHODCALLTYPE *GetMany )( 
                                               __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
            /* [in] */ unsigned int startIndex,
            /* [in] */ unsigned int capacity,
            /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * *items,
            /* [retval][out] */ __RPC__out unsigned int *actual);

        END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIconVtbl;

interface __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIconVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetAt(This,index,item)	\
    ( (This)->lpVtbl -> GetAt(This,index,item) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_IndexOf(This,item,index,found)	\
    ( (This)->lpVtbl -> IndexOf(This,item,index,found) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetMany(This,startIndex,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,startIndex,capacity,items,actual) ) 

#endif /* COBJMACROS */



#endif // ____FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon_INTERFACE_DEFINED__

#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if !defined(____FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed;

typedef struct __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeedVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )( __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed * This);

    ULONG ( STDMETHODCALLTYPE *Release )( __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )( __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
                                            /* [out] */ __RPC__out ULONG *iidCount,
                                            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
        __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
        __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *GetAt )( 
                                         __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
                                         /* [in] */ unsigned int index,
                                         /* [retval][out] */ __RPC__out struct __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed *item);

        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
            /* [retval][out] */ __RPC__out unsigned int *size);

        HRESULT ( STDMETHODCALLTYPE *IndexOf )( 
                                               __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
            /* [in] */ struct __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed item,
            /* [out] */ __RPC__out unsigned int *index,
            /* [retval][out] */ __RPC__out boolean *found);

        HRESULT ( STDMETHODCALLTYPE *GetMany )( 
                                               __RPC__in __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
            /* [in] */ unsigned int startIndex,
            /* [in] */ unsigned int capacity,
            /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) struct __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed *items,
            /* [retval][out] */ __RPC__out unsigned int *actual);

        END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeedVtbl;

interface __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeedVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_GetAt(This,index,item)	\
    ( (This)->lpVtbl -> GetAt(This,index,item) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_IndexOf(This,item,index,found)	\
    ( (This)->lpVtbl -> IndexOf(This,item,index,found) ) 

#define __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_GetMany(This,startIndex,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,startIndex,capacity,items,actual) ) 

#endif /* COBJMACROS */



#endif // ____FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed_INTERFACE_DEFINED__



#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice;

typedef struct __FIVector_1_Windows__CMedia__CStreaming__CIBasicDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This, /* [out] */ __RPC__deref_out_opt __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *GetAt )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
        /* [in] */ unsigned int index,
        /* [retval][out] */ __RPC__deref_out_opt __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * *item);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
        __RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
        /* [retval][out] */ __RPC__out unsigned int *size);

    HRESULT ( STDMETHODCALLTYPE *GetView )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This, /* [retval][out] */ __RPC__deref_out_opt __FIVectorView_1_Windows__CMedia__CStreaming__CIBasicDevice **view);

    HRESULT ( STDMETHODCALLTYPE *IndexOf )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
        /* [in] */ __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * item,
        /* [out] */ __RPC__out unsigned int *index,
        /* [retval][out] */ __RPC__out boolean *found);

    HRESULT ( STDMETHODCALLTYPE *SetAt )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
        /* [in] */ unsigned int index,
        /* [in] */ __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * item);

    HRESULT ( STDMETHODCALLTYPE *InsertAt )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
        /* [in] */ unsigned int index,
        /* [in] */ __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * item);

    HRESULT ( STDMETHODCALLTYPE *RemoveAt )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This, /* [in] */ unsigned int index);
    HRESULT ( STDMETHODCALLTYPE *Append )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This, /* [in] */ __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * item);
    HRESULT ( STDMETHODCALLTYPE *RemoveAtEnd )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This);
    HRESULT ( STDMETHODCALLTYPE *Clear )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
        /* [in] */ unsigned int startIndex,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    HRESULT ( STDMETHODCALLTYPE *ReplaceAll )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * This,
        /* [in] */ unsigned int count,
        /* [size_is][in] */ __RPC__in_ecount_full(count) __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * *value);

    END_INTERFACE
} __FIVector_1_Windows__CMedia__CStreaming__CIBasicDeviceVtbl;

interface __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice
{
    CONST_VTBL struct __FIVector_1_Windows__CMedia__CStreaming__CIBasicDeviceVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_GetAt(This,index,item)	\
    ( (This)->lpVtbl -> GetAt(This,index,item) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_GetView(This,view)	\
    ( (This)->lpVtbl -> GetView(This,view) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_IndexOf(This,item,index,found)	\
    ( (This)->lpVtbl -> IndexOf(This,item,index,found) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_SetAt(This,index,item)	\
    ( (This)->lpVtbl -> SetAt(This,index,item) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_InsertAt(This,index,item)	\
    ( (This)->lpVtbl -> InsertAt(This,index,item) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_Append(This,item)	\
    ( (This)->lpVtbl -> Append(This,item) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_RemoveAtEnd(This)	\
    ( (This)->lpVtbl -> RemoveAtEnd(This) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_GetMany(This,startIndex,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,startIndex,capacity,items,actual) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_ReplaceAll(This,count,value)	\
    ( (This)->lpVtbl -> ReplaceAll(This,count,value) ) 

#endif /* COBJMACROS */



#endif // ____FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice_INTERFACE_DEFINED__

#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon;

typedef struct __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIconVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This, /* [out] */ __RPC__deref_out_opt __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *GetAt )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
        /* [in] */ unsigned int index,
        /* [retval][out] */ __RPC__deref_out_opt __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * *item);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
        __RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
        /* [retval][out] */ __RPC__out unsigned int *size);

    HRESULT ( STDMETHODCALLTYPE *GetView )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This, /* [retval][out] */ __RPC__deref_out_opt __FIVectorView_1_Windows__CMedia__CStreaming__CIDeviceIcon **view);

    HRESULT ( STDMETHODCALLTYPE *IndexOf )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
        /* [in] */ __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * item,
        /* [out] */ __RPC__out unsigned int *index,
        /* [retval][out] */ __RPC__out boolean *found);

    HRESULT ( STDMETHODCALLTYPE *SetAt )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
        /* [in] */ unsigned int index,
        /* [in] */ __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * item);

    HRESULT ( STDMETHODCALLTYPE *InsertAt )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
        /* [in] */ unsigned int index,
        /* [in] */ __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * item);

    HRESULT ( STDMETHODCALLTYPE *RemoveAt )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This, /* [in] */ unsigned int index);
    HRESULT ( STDMETHODCALLTYPE *Append )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This, /* [in] */ __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * item);
    HRESULT ( STDMETHODCALLTYPE *RemoveAtEnd )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This);
    HRESULT ( STDMETHODCALLTYPE *Clear )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
        /* [in] */ unsigned int startIndex,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    HRESULT ( STDMETHODCALLTYPE *ReplaceAll )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * This,
        /* [in] */ unsigned int count,
        /* [size_is][in] */ __RPC__in_ecount_full(count) __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * *value);

    END_INTERFACE
} __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIconVtbl;

interface __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon
{
    CONST_VTBL struct __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIconVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetAt(This,index,item)	\
    ( (This)->lpVtbl -> GetAt(This,index,item) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetView(This,view)	\
    ( (This)->lpVtbl -> GetView(This,view) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_IndexOf(This,item,index,found)	\
    ( (This)->lpVtbl -> IndexOf(This,item,index,found) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_SetAt(This,index,item)	\
    ( (This)->lpVtbl -> SetAt(This,index,item) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_InsertAt(This,index,item)	\
    ( (This)->lpVtbl -> InsertAt(This,index,item) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_Append(This,item)	\
    ( (This)->lpVtbl -> Append(This,item) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_RemoveAtEnd(This)	\
    ( (This)->lpVtbl -> RemoveAtEnd(This) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_GetMany(This,startIndex,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,startIndex,capacity,items,actual) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_ReplaceAll(This,count,value)	\
    ( (This)->lpVtbl -> ReplaceAll(This,count,value) ) 

#endif /* COBJMACROS */



#endif // ____FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon_INTERFACE_DEFINED__

#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if !defined(____FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed;

typedef struct __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeedVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This, /* [out] */ __RPC__deref_out_opt struct __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *GetAt )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
        /* [in] */ unsigned int index,
        /* [retval][out] */ __RPC__deref_out_opt struct __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed *item);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
        __RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
        /* [retval][out] */ __RPC__out unsigned int *size);

    HRESULT ( STDMETHODCALLTYPE *GetView )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This, /* [retval][out] */ __RPC__deref_out_opt __FIVectorView_1_Windows__CMedia__CStreaming__CPlaySpeed **view);

    HRESULT ( STDMETHODCALLTYPE *IndexOf )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
        /* [in] */ __RPC__in struct __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed item,
        /* [out] */ __RPC__out unsigned int *index,
        /* [retval][out] */ __RPC__out boolean *found);

    HRESULT ( STDMETHODCALLTYPE *SetAt )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
        /* [in] */ unsigned int index,
        /* [in] */ __RPC__in struct __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed item);

    HRESULT ( STDMETHODCALLTYPE *InsertAt )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
        /* [in] */ unsigned int index,
        /* [in] */ __RPC__in struct __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed item);

    HRESULT ( STDMETHODCALLTYPE *RemoveAt )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This, /* [in] */ unsigned int index);
    HRESULT ( STDMETHODCALLTYPE *Append )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This, /* [in] */ __RPC__in struct __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed item);
    HRESULT ( STDMETHODCALLTYPE *RemoveAtEnd )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This);
    HRESULT ( STDMETHODCALLTYPE *Clear )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
        /* [in] */ unsigned int startIndex,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) struct __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    HRESULT ( STDMETHODCALLTYPE *ReplaceAll )(__RPC__in __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * This,
        /* [in] */ unsigned int count,
        /* [size_is][in] */ __RPC__in_ecount_full(count) struct __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed *value);

    END_INTERFACE
} __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeedVtbl;

interface __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed
{
    CONST_VTBL struct __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeedVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_GetAt(This,index,item)	\
    ( (This)->lpVtbl -> GetAt(This,index,item) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_GetView(This,view)	\
    ( (This)->lpVtbl -> GetView(This,view) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_IndexOf(This,item,index,found)	\
    ( (This)->lpVtbl -> IndexOf(This,item,index,found) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_SetAt(This,index,item)	\
    ( (This)->lpVtbl -> SetAt(This,index,item) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_InsertAt(This,index,item)	\
    ( (This)->lpVtbl -> InsertAt(This,index,item) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_Append(This,item)	\
    ( (This)->lpVtbl -> Append(This,item) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_RemoveAtEnd(This)	\
    ( (This)->lpVtbl -> RemoveAtEnd(This) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_GetMany(This,startIndex,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,startIndex,capacity,items,actual) ) 

#define __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_ReplaceAll(This,count,value)	\
    ( (This)->lpVtbl -> ReplaceAll(This,count,value) ) 

#endif /* COBJMACROS */



#endif // ____FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed_INTERFACE_DEFINED__



#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice;

// Forward declare the async operation.
typedef interface __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice * This);

    HRESULT ( STDMETHODCALLTYPE *Invoke )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice * This,/* [in] */ __RPC__in_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice *asyncInfo, /* [in] */ AsyncStatus status);
    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDeviceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDeviceVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice_Invoke(This,asyncInfo,status)	\
    ( (This)->lpVtbl -> Invoke(This,asyncInfo,status) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice_INTERFACE_DEFINED__

#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDeviceVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Completed )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice * This, /* [in] */ __RPC__in_opt __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice *handler);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Completed )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice * This, /* [retval][out] */ __RPC__deref_out_opt __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CActiveBasicDevice **handler);
    HRESULT ( STDMETHODCALLTYPE *GetResults )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice * This, /* [retval][out] */ __RPC__out __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * *results);
    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDeviceVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDeviceVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_put_Completed(This,handler)	\
    ( (This)->lpVtbl -> put_Completed(This,handler) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_get_Completed(This,handler)	\
    ( (This)->lpVtbl -> get_Completed(This,handler) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_GetResults(This,results)	\
    ( (This)->lpVtbl -> GetResults(This,results) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice_INTERFACE_DEFINED__

#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair;

// Forward declare the async operation.
typedef interface __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePairVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair * This);

    HRESULT ( STDMETHODCALLTYPE *Invoke )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair * This,/* [in] */ __RPC__in_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair *asyncInfo, /* [in] */ AsyncStatus status);
    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePairVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePairVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair_Invoke(This,asyncInfo,status)	\
    ( (This)->lpVtbl -> Invoke(This,asyncInfo,status) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair_INTERFACE_DEFINED__

#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePairVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Completed )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair * This, /* [in] */ __RPC__in_opt __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair *handler);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Completed )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair * This, /* [retval][out] */ __RPC__deref_out_opt __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CDevicePair **handler);
    HRESULT ( STDMETHODCALLTYPE *GetResults )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair * This, /* [retval][out] */ __RPC__out __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair * *results);
    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePairVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePairVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_put_Completed(This,handler)	\
    ( (This)->lpVtbl -> put_Completed(This,handler) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_get_Completed(This,handler)	\
    ( (This)->lpVtbl -> get_Completed(This,handler) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_GetResults(This,results)	\
    ( (This)->lpVtbl -> GetResults(This,results) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair_INTERFACE_DEFINED__

#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer;

// Forward declare the async operation.
typedef interface __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRendererVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer * This);

    HRESULT ( STDMETHODCALLTYPE *Invoke )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer * This,/* [in] */ __RPC__in_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer *asyncInfo, /* [in] */ AsyncStatus status);
    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRendererVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRendererVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer_Invoke(This,asyncInfo,status)	\
    ( (This)->lpVtbl -> Invoke(This,asyncInfo,status) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer_INTERFACE_DEFINED__

#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRendererVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Completed )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer * This, /* [in] */ __RPC__in_opt __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer *handler);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Completed )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer * This, /* [retval][out] */ __RPC__deref_out_opt __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CMediaRenderer **handler);
    HRESULT ( STDMETHODCALLTYPE *GetResults )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer * This, /* [retval][out] */ __RPC__out __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * *results);
    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRendererVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRendererVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_put_Completed(This,handler)	\
    ( (This)->lpVtbl -> put_Completed(This,handler) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_get_Completed(This,handler)	\
    ( (This)->lpVtbl -> get_Completed(This,handler) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_GetResults(This,results)	\
    ( (This)->lpVtbl -> GetResults(This,results) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer_INTERFACE_DEFINED__

#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

struct __x_ABI_CWindows_CMedia_CStreaming_CPositionInformation;

#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation;

// Forward declare the async operation.
typedef interface __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation * This);

    HRESULT ( STDMETHODCALLTYPE *Invoke )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation * This,/* [in] */ __RPC__in_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation *asyncInfo, /* [in] */ AsyncStatus status);
    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformationVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformationVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation_Invoke(This,asyncInfo,status)	\
    ( (This)->lpVtbl -> Invoke(This,asyncInfo,status) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation_INTERFACE_DEFINED__



#if !defined(____FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformationVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Completed )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation * This, /* [in] */ __RPC__in_opt __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation *handler);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Completed )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation * This, /* [retval][out] */ __RPC__deref_out_opt __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CPositionInformation **handler);
    HRESULT ( STDMETHODCALLTYPE *GetResults )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation * This, /* [retval][out] */ __RPC__out struct __x_ABI_CWindows_CMedia_CStreaming_CPositionInformation *results);
    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformationVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformationVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_put_Completed(This,handler)	\
    ( (This)->lpVtbl -> put_Completed(This,handler) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_get_Completed(This,handler)	\
    ( (This)->lpVtbl -> get_Completed(This,handler) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_GetResults(This,results)	\
    ( (This)->lpVtbl -> GetResults(This,results) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation_INTERFACE_DEFINED__


struct __x_ABI_CWindows_CMedia_CStreaming_CTransportInformation;

#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation;

// Forward declare the async operation.
typedef interface __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation * This);

    HRESULT ( STDMETHODCALLTYPE *Invoke )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation * This,/* [in] */ __RPC__in_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation *asyncInfo, /* [in] */ AsyncStatus status);
    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformationVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformationVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation_Invoke(This,asyncInfo,status)	\
    ( (This)->lpVtbl -> Invoke(This,asyncInfo,status) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation_INTERFACE_DEFINED__



#if !defined(____FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformationVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Completed )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation * This, /* [in] */ __RPC__in_opt __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation *handler);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Completed )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation * This, /* [retval][out] */ __RPC__deref_out_opt __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CTransportInformation **handler);
    HRESULT ( STDMETHODCALLTYPE *GetResults )(__RPC__in __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation * This, /* [retval][out] */ __RPC__out struct __x_ABI_CWindows_CMedia_CStreaming_CTransportInformation *results);
    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformationVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformationVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_put_Completed(This,handler)	\
    ( (This)->lpVtbl -> put_Completed(This,handler) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_get_Completed(This,handler)	\
    ( (This)->lpVtbl -> get_Completed(This,handler) ) 
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_GetResults(This,results)	\
    ( (This)->lpVtbl -> GetResults(This,results) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation_INTERFACE_DEFINED__


#if !defined(____FIIterator_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1_HSTRING __FIIterator_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_HSTRING;

typedef struct __FIIterator_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterator_1_HSTRING * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterator_1_HSTRING * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterator_1_HSTRING * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterator_1_HSTRING * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterator_1_HSTRING * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterator_1_HSTRING * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Current )(__RPC__in __FIIterator_1_HSTRING * This, /* [retval][out] */ __RPC__out HSTRING *current);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HasCurrent )(__RPC__in __FIIterator_1_HSTRING * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *MoveNext )(__RPC__in __FIIterator_1_HSTRING * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIIterator_1_HSTRING * This,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) HSTRING *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    END_INTERFACE
} __FIIterator_1_HSTRINGVtbl;

interface __FIIterator_1_HSTRING
{
    CONST_VTBL struct __FIIterator_1_HSTRINGVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIIterator_1_HSTRING_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterator_1_HSTRING_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterator_1_HSTRING_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterator_1_HSTRING_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterator_1_HSTRING_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterator_1_HSTRING_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterator_1_HSTRING_get_Current(This,current)	\
    ( (This)->lpVtbl -> get_Current(This,current) ) 

#define __FIIterator_1_HSTRING_get_HasCurrent(This,hasCurrent)	\
    ( (This)->lpVtbl -> get_HasCurrent(This,hasCurrent) ) 

#define __FIIterator_1_HSTRING_MoveNext(This,hasCurrent)	\
    ( (This)->lpVtbl -> MoveNext(This,hasCurrent) ) 

#define __FIIterator_1_HSTRING_GetMany(This,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,capacity,items,actual) ) 

#endif /* COBJMACROS */


#endif // ____FIIterator_1_HSTRING_INTERFACE_DEFINED__


#if !defined(____FIIterable_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1_HSTRING __FIIterable_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_HSTRING;

typedef  struct __FIIterable_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterable_1_HSTRING * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterable_1_HSTRING * This);

    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterable_1_HSTRING * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterable_1_HSTRING * This,
                                           /* [out] */ __RPC__out ULONG *iidCount,
                                           /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterable_1_HSTRING * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterable_1_HSTRING * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *First )(__RPC__in __FIIterable_1_HSTRING * This, /* [retval][out] */ __RPC__deref_out_opt __FIIterator_1_HSTRING **first);

    END_INTERFACE
} __FIIterable_1_HSTRINGVtbl;

interface __FIIterable_1_HSTRING
{
    CONST_VTBL struct __FIIterable_1_HSTRINGVtbl *lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_HSTRING_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterable_1_HSTRING_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterable_1_HSTRING_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterable_1_HSTRING_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterable_1_HSTRING_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterable_1_HSTRING_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterable_1_HSTRING_First(This,first)	\
    ( (This)->lpVtbl -> First(This,first) ) 

#endif /* COBJMACROS */


#endif // ____FIIterable_1_HSTRING_INTERFACE_DEFINED__


#if !defined(____FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_HSTRING __FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_HSTRING;

typedef struct __FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIVectorView_1_HSTRING * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )( __RPC__in __FIVectorView_1_HSTRING * This);

    ULONG ( STDMETHODCALLTYPE *Release )( __RPC__in __FIVectorView_1_HSTRING * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )( __RPC__in __FIVectorView_1_HSTRING * This,
                                            /* [out] */ __RPC__out ULONG *iidCount,
                                            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
        __RPC__in __FIVectorView_1_HSTRING * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
        __RPC__in __FIVectorView_1_HSTRING * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *GetAt )( 
                                         __RPC__in __FIVectorView_1_HSTRING * This,
                                         /* [in] */ unsigned int index,
                                         /* [retval][out] */ __RPC__out HSTRING *item);

        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in __FIVectorView_1_HSTRING * This,
            /* [retval][out] */ __RPC__out unsigned int *size);

        HRESULT ( STDMETHODCALLTYPE *IndexOf )( 
                                               __RPC__in __FIVectorView_1_HSTRING * This,
            /* [in] */ HSTRING item,
            /* [out] */ __RPC__out unsigned int *index,
            /* [retval][out] */ __RPC__out boolean *found);

        HRESULT ( STDMETHODCALLTYPE *GetMany )( 
                                               __RPC__in __FIVectorView_1_HSTRING * This,
            /* [in] */ unsigned int startIndex,
            /* [in] */ unsigned int capacity,
            /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) HSTRING *items,
            /* [retval][out] */ __RPC__out unsigned int *actual);

        END_INTERFACE
} __FIVectorView_1_HSTRINGVtbl;

interface __FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIVectorView_1_HSTRINGVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIVectorView_1_HSTRING_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIVectorView_1_HSTRING_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIVectorView_1_HSTRING_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIVectorView_1_HSTRING_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIVectorView_1_HSTRING_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIVectorView_1_HSTRING_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIVectorView_1_HSTRING_GetAt(This,index,item)	\
    ( (This)->lpVtbl -> GetAt(This,index,item) ) 

#define __FIVectorView_1_HSTRING_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 

#define __FIVectorView_1_HSTRING_IndexOf(This,item,index,found)	\
    ( (This)->lpVtbl -> IndexOf(This,item,index,found) ) 

#define __FIVectorView_1_HSTRING_GetMany(This,startIndex,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,startIndex,capacity,items,actual) ) 

#endif /* COBJMACROS */



#endif // ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__


#if !defined(____FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVector_1_HSTRING __FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_HSTRING;

typedef struct __FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIVector_1_HSTRING * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIVector_1_HSTRING * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIVector_1_HSTRING * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIVector_1_HSTRING * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIVector_1_HSTRING * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIVector_1_HSTRING * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *GetAt )(__RPC__in __FIVector_1_HSTRING * This,
        /* [in] */ unsigned int index,
        /* [retval][out] */ __RPC__deref_out_opt HSTRING *item);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
        __RPC__in __FIVector_1_HSTRING * This,
        /* [retval][out] */ __RPC__out unsigned int *size);

    HRESULT ( STDMETHODCALLTYPE *GetView )(__RPC__in __FIVector_1_HSTRING * This, /* [retval][out] */ __RPC__deref_out_opt __FIVectorView_1_HSTRING **view);

    HRESULT ( STDMETHODCALLTYPE *IndexOf )(__RPC__in __FIVector_1_HSTRING * This,
        /* [in] */ __RPC__in HSTRING item,
        /* [out] */ __RPC__out unsigned int *index,
        /* [retval][out] */ __RPC__out boolean *found);

    HRESULT ( STDMETHODCALLTYPE *SetAt )(__RPC__in __FIVector_1_HSTRING * This,
        /* [in] */ unsigned int index,
        /* [in] */ __RPC__in HSTRING item);

    HRESULT ( STDMETHODCALLTYPE *InsertAt )(__RPC__in __FIVector_1_HSTRING * This,
        /* [in] */ unsigned int index,
        /* [in] */ __RPC__in HSTRING item);

    HRESULT ( STDMETHODCALLTYPE *RemoveAt )(__RPC__in __FIVector_1_HSTRING * This, /* [in] */ unsigned int index);
    HRESULT ( STDMETHODCALLTYPE *Append )(__RPC__in __FIVector_1_HSTRING * This, /* [in] */ __RPC__in HSTRING item);
    HRESULT ( STDMETHODCALLTYPE *RemoveAtEnd )(__RPC__in __FIVector_1_HSTRING * This);
    HRESULT ( STDMETHODCALLTYPE *Clear )(__RPC__in __FIVector_1_HSTRING * This);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIVector_1_HSTRING * This,
        /* [in] */ unsigned int startIndex,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) HSTRING *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    HRESULT ( STDMETHODCALLTYPE *ReplaceAll )(__RPC__in __FIVector_1_HSTRING * This,
        /* [in] */ unsigned int count,
        /* [size_is][in] */ __RPC__in_ecount_full(count) HSTRING *value);

    END_INTERFACE
} __FIVector_1_HSTRINGVtbl;

interface __FIVector_1_HSTRING
{
    CONST_VTBL struct __FIVector_1_HSTRINGVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIVector_1_HSTRING_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIVector_1_HSTRING_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIVector_1_HSTRING_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIVector_1_HSTRING_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIVector_1_HSTRING_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIVector_1_HSTRING_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIVector_1_HSTRING_GetAt(This,index,item)	\
    ( (This)->lpVtbl -> GetAt(This,index,item) ) 

#define __FIVector_1_HSTRING_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 

#define __FIVector_1_HSTRING_GetView(This,view)	\
    ( (This)->lpVtbl -> GetView(This,view) ) 

#define __FIVector_1_HSTRING_IndexOf(This,item,index,found)	\
    ( (This)->lpVtbl -> IndexOf(This,item,index,found) ) 

#define __FIVector_1_HSTRING_SetAt(This,index,item)	\
    ( (This)->lpVtbl -> SetAt(This,index,item) ) 

#define __FIVector_1_HSTRING_InsertAt(This,index,item)	\
    ( (This)->lpVtbl -> InsertAt(This,index,item) ) 

#define __FIVector_1_HSTRING_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define __FIVector_1_HSTRING_Append(This,item)	\
    ( (This)->lpVtbl -> Append(This,item) ) 

#define __FIVector_1_HSTRING_RemoveAtEnd(This)	\
    ( (This)->lpVtbl -> RemoveAtEnd(This) ) 

#define __FIVector_1_HSTRING_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define __FIVector_1_HSTRING_GetMany(This,startIndex,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,startIndex,capacity,items,actual) ) 

#define __FIVector_1_HSTRING_ReplaceAll(This,count,value)	\
    ( (This)->lpVtbl -> ReplaceAll(This,count,value) ) 

#endif /* COBJMACROS */



#endif // ____FIVector_1_HSTRING_INTERFACE_DEFINED__


#if !defined(____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_UINT32 __FIAsyncOperationCompletedHandler_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_UINT32;

// Forward declare the async operation.
typedef interface __FIAsyncOperation_1_UINT32 __FIAsyncOperation_1_UINT32;

typedef struct __FIAsyncOperationCompletedHandler_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperationCompletedHandler_1_UINT32 * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperationCompletedHandler_1_UINT32 * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperationCompletedHandler_1_UINT32 * This);

    HRESULT ( STDMETHODCALLTYPE *Invoke )(__RPC__in __FIAsyncOperationCompletedHandler_1_UINT32 * This,/* [in] */ __RPC__in_opt __FIAsyncOperation_1_UINT32 *asyncInfo, /* [in] */ AsyncStatus status);
    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_UINT32Vtbl;

interface __FIAsyncOperationCompletedHandler_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_UINT32Vtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperationCompletedHandler_1_UINT32_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperationCompletedHandler_1_UINT32_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperationCompletedHandler_1_UINT32_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperationCompletedHandler_1_UINT32_Invoke(This,asyncInfo,status)	\
    ( (This)->lpVtbl -> Invoke(This,asyncInfo,status) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__


#if !defined(____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_UINT32 __FIAsyncOperation_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_UINT32;

typedef struct __FIAsyncOperation_1_UINT32Vtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperation_1_UINT32 * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperation_1_UINT32 * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperation_1_UINT32 * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIAsyncOperation_1_UINT32 * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIAsyncOperation_1_UINT32 * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIAsyncOperation_1_UINT32 * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Completed )(__RPC__in __FIAsyncOperation_1_UINT32 * This, /* [in] */ __RPC__in_opt __FIAsyncOperationCompletedHandler_1_UINT32 *handler);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Completed )(__RPC__in __FIAsyncOperation_1_UINT32 * This, /* [retval][out] */ __RPC__deref_out_opt __FIAsyncOperationCompletedHandler_1_UINT32 **handler);
    HRESULT ( STDMETHODCALLTYPE *GetResults )(__RPC__in __FIAsyncOperation_1_UINT32 * This, /* [retval][out] */ __RPC__out unsigned int *results);
    END_INTERFACE
} __FIAsyncOperation_1_UINT32Vtbl;

interface __FIAsyncOperation_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperation_1_UINT32Vtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperation_1_UINT32_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperation_1_UINT32_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperation_1_UINT32_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperation_1_UINT32_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 
#define __FIAsyncOperation_1_UINT32_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 
#define __FIAsyncOperation_1_UINT32_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 

#define __FIAsyncOperation_1_UINT32_put_Completed(This,handler)	\
    ( (This)->lpVtbl -> put_Completed(This,handler) ) 
#define __FIAsyncOperation_1_UINT32_get_Completed(This,handler)	\
    ( (This)->lpVtbl -> get_Completed(This,handler) ) 
#define __FIAsyncOperation_1_UINT32_GetResults(This,results)	\
    ( (This)->lpVtbl -> GetResults(This,results) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__


#if !defined(____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_boolean __FIAsyncOperationCompletedHandler_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_boolean;

// Forward declare the async operation.
typedef interface __FIAsyncOperation_1_boolean __FIAsyncOperation_1_boolean;

typedef struct __FIAsyncOperationCompletedHandler_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperationCompletedHandler_1_boolean * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperationCompletedHandler_1_boolean * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperationCompletedHandler_1_boolean * This);

    HRESULT ( STDMETHODCALLTYPE *Invoke )(__RPC__in __FIAsyncOperationCompletedHandler_1_boolean * This,/* [in] */ __RPC__in_opt __FIAsyncOperation_1_boolean *asyncInfo, /* [in] */ AsyncStatus status);
    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_booleanVtbl;

interface __FIAsyncOperationCompletedHandler_1_boolean
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_booleanVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperationCompletedHandler_1_boolean_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperationCompletedHandler_1_boolean_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperationCompletedHandler_1_boolean_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperationCompletedHandler_1_boolean_Invoke(This,asyncInfo,status)	\
    ( (This)->lpVtbl -> Invoke(This,asyncInfo,status) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__


#if !defined(____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_boolean __FIAsyncOperation_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_boolean;

typedef struct __FIAsyncOperation_1_booleanVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperation_1_boolean * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperation_1_boolean * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperation_1_boolean * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIAsyncOperation_1_boolean * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIAsyncOperation_1_boolean * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIAsyncOperation_1_boolean * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Completed )(__RPC__in __FIAsyncOperation_1_boolean * This, /* [in] */ __RPC__in_opt __FIAsyncOperationCompletedHandler_1_boolean *handler);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Completed )(__RPC__in __FIAsyncOperation_1_boolean * This, /* [retval][out] */ __RPC__deref_out_opt __FIAsyncOperationCompletedHandler_1_boolean **handler);
    HRESULT ( STDMETHODCALLTYPE *GetResults )(__RPC__in __FIAsyncOperation_1_boolean * This, /* [retval][out] */ __RPC__out boolean *results);
    END_INTERFACE
} __FIAsyncOperation_1_booleanVtbl;

interface __FIAsyncOperation_1_boolean
{
    CONST_VTBL struct __FIAsyncOperation_1_booleanVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperation_1_boolean_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperation_1_boolean_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperation_1_boolean_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperation_1_boolean_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 
#define __FIAsyncOperation_1_boolean_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 
#define __FIAsyncOperation_1_boolean_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 

#define __FIAsyncOperation_1_boolean_put_Completed(This,handler)	\
    ( (This)->lpVtbl -> put_Completed(This,handler) ) 
#define __FIAsyncOperation_1_boolean_get_Completed(This,handler)	\
    ( (This)->lpVtbl -> get_Completed(This,handler) ) 
#define __FIAsyncOperation_1_boolean_GetResults(This,results)	\
    ( (This)->lpVtbl -> GetResults(This,results) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__


#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType;

// Forward declare the async operation.
typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType * This);

    HRESULT ( STDMETHODCALLTYPE *Invoke )(__RPC__in __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType * This,/* [in] */ __RPC__in_opt __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType *asyncInfo, /* [in] */ AsyncStatus status);
    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_Invoke(This,asyncInfo,status)	\
    ( (This)->lpVtbl -> Invoke(This,asyncInfo,status) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Completed )(__RPC__in __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType * This, /* [in] */ __RPC__in_opt __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType *handler);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Completed )(__RPC__in __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType * This, /* [retval][out] */ __RPC__deref_out_opt __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType **handler);
    HRESULT ( STDMETHODCALLTYPE *GetResults )(__RPC__in __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType * This, /* [retval][out] */ __RPC__out __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType * *results);
    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentTypeVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_put_Completed(This,handler)	\
    ( (This)->lpVtbl -> put_Completed(This,handler) ) 
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_get_Completed(This,handler)	\
    ( (This)->lpVtbl -> get_Completed(This,handler) ) 
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_GetResults(This,results)	\
    ( (This)->lpVtbl -> GetResults(This,results) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType_INTERFACE_DEFINED__

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__


#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___F__CIPropertySet_INTERFACE_DEFINED__)
#define ____FIIterator_1___F__CIPropertySet_INTERFACE_DEFINED__

typedef interface __FIIterator_1___F__CIPropertySet __FIIterator_1___F__CIPropertySet;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___F__CIPropertySet;

typedef struct __FIIterator_1___F__CIPropertySetVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterator_1___F__CIPropertySet * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterator_1___F__CIPropertySet * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterator_1___F__CIPropertySet * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterator_1___F__CIPropertySet * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterator_1___F__CIPropertySet * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterator_1___F__CIPropertySet * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Current )(__RPC__in __FIIterator_1___F__CIPropertySet * This, /* [retval][out] */ __RPC__out __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet * *current);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HasCurrent )(__RPC__in __FIIterator_1___F__CIPropertySet * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *MoveNext )(__RPC__in __FIIterator_1___F__CIPropertySet * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIIterator_1___F__CIPropertySet * This,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet * *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    END_INTERFACE
} __FIIterator_1___F__CIPropertySetVtbl;

interface __FIIterator_1___F__CIPropertySet
{
    CONST_VTBL struct __FIIterator_1___F__CIPropertySetVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIIterator_1___F__CIPropertySet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterator_1___F__CIPropertySet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterator_1___F__CIPropertySet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterator_1___F__CIPropertySet_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterator_1___F__CIPropertySet_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterator_1___F__CIPropertySet_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterator_1___F__CIPropertySet_get_Current(This,current)	\
    ( (This)->lpVtbl -> get_Current(This,current) ) 

#define __FIIterator_1___F__CIPropertySet_get_HasCurrent(This,hasCurrent)	\
    ( (This)->lpVtbl -> get_HasCurrent(This,hasCurrent) ) 

#define __FIIterator_1___F__CIPropertySet_MoveNext(This,hasCurrent)	\
    ( (This)->lpVtbl -> MoveNext(This,hasCurrent) ) 

#define __FIIterator_1___F__CIPropertySet_GetMany(This,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,capacity,items,actual) ) 

#endif /* COBJMACROS */


#endif // ____FIIterator_1___F__CIPropertySet_INTERFACE_DEFINED__

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000


#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___F__CIPropertySet_INTERFACE_DEFINED__)
#define ____FIIterable_1___F__CIPropertySet_INTERFACE_DEFINED__

typedef interface __FIIterable_1___F__CIPropertySet __FIIterable_1___F__CIPropertySet;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___F__CIPropertySet;

typedef  struct __FIIterable_1___F__CIPropertySetVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterable_1___F__CIPropertySet * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterable_1___F__CIPropertySet * This);

    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterable_1___F__CIPropertySet * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterable_1___F__CIPropertySet * This,
                                           /* [out] */ __RPC__out ULONG *iidCount,
                                           /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterable_1___F__CIPropertySet * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterable_1___F__CIPropertySet * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *First )(__RPC__in __FIIterable_1___F__CIPropertySet * This, /* [retval][out] */ __RPC__deref_out_opt __FIIterator_1___F__CIPropertySet **first);

    END_INTERFACE
} __FIIterable_1___F__CIPropertySetVtbl;

interface __FIIterable_1___F__CIPropertySet
{
    CONST_VTBL struct __FIIterable_1___F__CIPropertySetVtbl *lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___F__CIPropertySet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterable_1___F__CIPropertySet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterable_1___F__CIPropertySet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterable_1___F__CIPropertySet_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterable_1___F__CIPropertySet_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterable_1___F__CIPropertySet_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterable_1___F__CIPropertySet_First(This,first)	\
    ( (This)->lpVtbl -> First(This,first) ) 

#endif /* COBJMACROS */


#endif // ____FIIterable_1___F__CIPropertySet_INTERFACE_DEFINED__

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000


#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1___F__CIPropertySet_INTERFACE_DEFINED__)
#define ____FIVectorView_1___F__CIPropertySet_INTERFACE_DEFINED__

typedef interface __FIVectorView_1___F__CIPropertySet __FIVectorView_1___F__CIPropertySet;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1___F__CIPropertySet;

typedef struct __FIVectorView_1___F__CIPropertySetVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIVectorView_1___F__CIPropertySet * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )( __RPC__in __FIVectorView_1___F__CIPropertySet * This);

    ULONG ( STDMETHODCALLTYPE *Release )( __RPC__in __FIVectorView_1___F__CIPropertySet * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )( __RPC__in __FIVectorView_1___F__CIPropertySet * This,
                                            /* [out] */ __RPC__out ULONG *iidCount,
                                            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
        __RPC__in __FIVectorView_1___F__CIPropertySet * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
        __RPC__in __FIVectorView_1___F__CIPropertySet * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *GetAt )( 
                                         __RPC__in __FIVectorView_1___F__CIPropertySet * This,
                                         /* [in] */ unsigned int index,
                                         /* [retval][out] */ __RPC__out __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet * *item);

        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in __FIVectorView_1___F__CIPropertySet * This,
            /* [retval][out] */ __RPC__out unsigned int *size);

        HRESULT ( STDMETHODCALLTYPE *IndexOf )( 
                                               __RPC__in __FIVectorView_1___F__CIPropertySet * This,
            /* [in] */ __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet * item,
            /* [out] */ __RPC__out unsigned int *index,
            /* [retval][out] */ __RPC__out boolean *found);

        HRESULT ( STDMETHODCALLTYPE *GetMany )( 
                                               __RPC__in __FIVectorView_1___F__CIPropertySet * This,
            /* [in] */ unsigned int startIndex,
            /* [in] */ unsigned int capacity,
            /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet * *items,
            /* [retval][out] */ __RPC__out unsigned int *actual);

        END_INTERFACE
} __FIVectorView_1___F__CIPropertySetVtbl;

interface __FIVectorView_1___F__CIPropertySet
{
    CONST_VTBL struct __FIVectorView_1___F__CIPropertySetVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIVectorView_1___F__CIPropertySet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIVectorView_1___F__CIPropertySet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIVectorView_1___F__CIPropertySet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIVectorView_1___F__CIPropertySet_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIVectorView_1___F__CIPropertySet_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIVectorView_1___F__CIPropertySet_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIVectorView_1___F__CIPropertySet_GetAt(This,index,item)	\
    ( (This)->lpVtbl -> GetAt(This,index,item) ) 

#define __FIVectorView_1___F__CIPropertySet_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 

#define __FIVectorView_1___F__CIPropertySet_IndexOf(This,item,index,found)	\
    ( (This)->lpVtbl -> IndexOf(This,item,index,found) ) 

#define __FIVectorView_1___F__CIPropertySet_GetMany(This,startIndex,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,startIndex,capacity,items,actual) ) 

#endif /* COBJMACROS */



#endif // ____FIVectorView_1___F__CIPropertySet_INTERFACE_DEFINED__

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000


#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet;

// Forward declare the async operation.
typedef interface __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySetVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet * This);

    HRESULT ( STDMETHODCALLTYPE *Invoke )(__RPC__in __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet * This,/* [in] */ __RPC__in_opt __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet *asyncInfo, /* [in] */ AsyncStatus status);
    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySetVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySetVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet_Invoke(This,asyncInfo,status)	\
    ( (This)->lpVtbl -> Invoke(This,asyncInfo,status) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet_INTERFACE_DEFINED__

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000


#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet;

typedef struct __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySetVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Completed )(__RPC__in __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet * This, /* [in] */ __RPC__in_opt __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet *handler);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Completed )(__RPC__in __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet * This, /* [retval][out] */ __RPC__deref_out_opt __FIAsyncOperationCompletedHandler_1___FIVectorView_1___F__CIPropertySet **handler);
    HRESULT ( STDMETHODCALLTYPE *GetResults )(__RPC__in __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet * This, /* [retval][out] */ __RPC__out __FIVectorView_1___F__CIPropertySet * *results);
    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySetVtbl;

interface __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySetVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 
#define __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 
#define __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 

#define __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_put_Completed(This,handler)	\
    ( (This)->lpVtbl -> put_Completed(This,handler) ) 
#define __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_get_Completed(This,handler)	\
    ( (This)->lpVtbl -> get_Completed(This,handler) ) 
#define __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_GetResults(This,results)	\
    ( (This)->lpVtbl -> GetResults(This,results) ) 
#endif /* COBJMACROS */


#endif // ____FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet_INTERFACE_DEFINED__

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000




typedef enum __x_ABI_CWindows_CMedia_CStreaming_CConnectionStatus __x_ABI_CWindows_CMedia_CStreaming_CConnectionStatus;


typedef enum __x_ABI_CWindows_CMedia_CStreaming_CDeviceTypes __x_ABI_CWindows_CMedia_CStreaming_CDeviceTypes;


typedef enum __x_ABI_CWindows_CMedia_CStreaming_CTransportState __x_ABI_CWindows_CMedia_CStreaming_CTransportState;


typedef enum __x_ABI_CWindows_CMedia_CStreaming_CTransportStatus __x_ABI_CWindows_CMedia_CStreaming_CTransportStatus;


typedef struct __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed;


typedef struct __x_ABI_CWindows_CMedia_CStreaming_CPositionInformation __x_ABI_CWindows_CMedia_CStreaming_CPositionInformation;


typedef struct __x_ABI_CWindows_CMedia_CStreaming_CRenderingParameters __x_ABI_CWindows_CMedia_CStreaming_CRenderingParameters;


typedef struct __x_ABI_CWindows_CMedia_CStreaming_CTrackInformation __x_ABI_CWindows_CMedia_CStreaming_CTrackInformation;


typedef struct __x_ABI_CWindows_CMedia_CStreaming_CTransportInformation __x_ABI_CWindows_CMedia_CStreaming_CTransportInformation;
































/*
 *
 * Struct Windows.Media.Streaming.ConnectionStatus
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */

#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
/* [v1_enum, contract] */
enum __x_ABI_CWindows_CMedia_CStreaming_CConnectionStatus
{
    ConnectionStatus_Online = 0,
    ConnectionStatus_Offline = 1,
    ConnectionStatus_Sleeping = 2,
};
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Struct Windows.Media.Streaming.DeviceTypes
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */

#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
/* [v1_enum, flags, contract] */
enum __x_ABI_CWindows_CMedia_CStreaming_CDeviceTypes
{
    DeviceTypes_Unknown = 0,
    DeviceTypes_DigitalMediaRenderer = 0x1,
    DeviceTypes_DigitalMediaServer = 0x2,
    DeviceTypes_DigitalMediaPlayer = 0x4,
};
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Struct Windows.Media.Streaming.TransportState
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */

#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
/* [v1_enum, contract] */
enum __x_ABI_CWindows_CMedia_CStreaming_CTransportState
{
    TransportState_Unknown = 0,
    TransportState_Stopped = 1,
    TransportState_Playing = 2,
    TransportState_Transitioning = 3,
    TransportState_Paused = 4,
    TransportState_Recording = 5,
    TransportState_NoMediaPresent = 6,
    TransportState_Last = 7,
};
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Struct Windows.Media.Streaming.TransportStatus
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */

#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
/* [v1_enum, contract] */
enum __x_ABI_CWindows_CMedia_CStreaming_CTransportStatus
{
    TransportStatus_Unknown = 0,
    TransportStatus_Ok = 1,
    TransportStatus_ErrorOccurred = 2,
    TransportStatus_Last = 3,
};
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Struct Windows.Media.Streaming.PlaySpeed
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

/* [contract] */
struct __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed
{
    INT32 Numerator;
    UINT32 Denominator;
};
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Struct Windows.Media.Streaming.TrackInformation
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

/* [contract] */
struct __x_ABI_CWindows_CMedia_CStreaming_CTrackInformation
{
    UINT32 Track;
    UINT32 TrackId;
    __x_ABI_CWindows_CFoundation_CTimeSpan TrackDuration;
};
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Struct Windows.Media.Streaming.PositionInformation
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

/* [contract] */
struct __x_ABI_CWindows_CMedia_CStreaming_CPositionInformation
{
    __x_ABI_CWindows_CMedia_CStreaming_CTrackInformation trackInformation;
    __x_ABI_CWindows_CFoundation_CTimeSpan relativeTime;
};
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Struct Windows.Media.Streaming.RenderingParameters
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

/* [contract] */
struct __x_ABI_CWindows_CMedia_CStreaming_CRenderingParameters
{
    UINT32 volume;
    boolean mute;
};
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Struct Windows.Media.Streaming.TransportInformation
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

/* [contract] */
struct __x_ABI_CWindows_CMedia_CStreaming_CTransportInformation
{
    __x_ABI_CWindows_CMedia_CStreaming_CTransportState CurrentTransportState;
    __x_ABI_CWindows_CMedia_CStreaming_CTransportStatus CurrentTransportStatus;
    __x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed CurrentSpeed;
};
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Delegate Windows.Media.Streaming.ConnectionStatusHandler
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler_INTERFACE_DEFINED__
/* [object, uuid("B571C28C-A472-48D5-88D2-8ADCAF1B8813"), contract] */
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandlerVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject);

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler * This);

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler * This);
HRESULT ( STDMETHODCALLTYPE *Invoke )(
        __x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * sender,
        /* [in] */__x_ABI_CWindows_CMedia_CStreaming_CConnectionStatus arg
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandlerVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandlerVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler_Invoke(This,sender,arg) \
    ( (This)->lpVtbl->Invoke(This,sender,arg) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Delegate Windows.Media.Streaming.DeviceControllerFinderHandler
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler_INTERFACE_DEFINED__
/* [object, uuid("A88A7D06-988C-4403-9D8A-015BED140B34"), contract] */
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandlerVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject);

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler * This);

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler * This);
HRESULT ( STDMETHODCALLTYPE *Invoke )(
        __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController * sender,
        /* [in] */__RPC__in HSTRING uniqueDeviceName,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * device
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandlerVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandlerVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler_Invoke(This,sender,uniqueDeviceName,device) \
    ( (This)->lpVtbl->Invoke(This,sender,uniqueDeviceName,device) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Delegate Windows.Media.Streaming.RenderingParametersUpdateHandler
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler_INTERFACE_DEFINED__
/* [object, uuid("3A2D9D45-72E9-4311-B46C-27C8BB7E6CB3"), contract] */
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandlerVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject);

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler * This);

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler * This);
HRESULT ( STDMETHODCALLTYPE *Invoke )(
        __x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * sender,
        /* [in] */__x_ABI_CWindows_CMedia_CStreaming_CRenderingParameters arg
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandlerVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandlerVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler_Invoke(This,sender,arg) \
    ( (This)->lpVtbl->Invoke(This,sender,arg) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Delegate Windows.Media.Streaming.TransportParametersUpdateHandler
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler_INTERFACE_DEFINED__
/* [object, uuid("16FD02D5-DA61-49D7-AAB2-76867DD42DB7"), contract] */
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandlerVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject);

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler * This);

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler * This);
HRESULT ( STDMETHODCALLTYPE *Invoke )(
        __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * sender,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters * arg
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandlerVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandlerVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler_Invoke(This,sender,arg) \
    ( (This)->lpVtbl->Invoke(This,sender,arg) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IActiveBasicDevice
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Streaming.IBasicDevice
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IActiveBasicDevice[] = L"Windows.Media.Streaming.IActiveBasicDevice";
/* [object, uuid("B64D6974-6E79-49AF-9933-908B6E9A160C"), contract] */
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_MaxVolume )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
        /* [retval, out] */__RPC__out UINT32 * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IsMuteSupported )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IsSetNextSourceSupported )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IsAudioSupported )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IsVideoSupported )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IsImageSupported )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IsSearchSupported )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    HRESULT ( STDMETHODCALLTYPE *GetCachedSinkProtocolInfo )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
        );
    HRESULT ( STDMETHODCALLTYPE *SetCachedSinkProtocolInfo )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
        /* [in] */__RPC__in HSTRING value
        );
    HRESULT ( STDMETHODCALLTYPE *GetCachedExtraSinkProtocolInfo )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
        );
    HRESULT ( STDMETHODCALLTYPE *GetEffectiveBandwidth )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
        /* [in] */boolean transmitSpeed,
        /* [retval, out] */__RPC__out UINT64 * currentSpeed
        );
    HRESULT ( STDMETHODCALLTYPE *GetCachedBitrateMeasurement )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
        /* [in] */GUID physicalNetworkInterface,
        /* [retval, out] */__RPC__out UINT64 * bitrate
        );
    HRESULT ( STDMETHODCALLTYPE *SetCachedBitrateMeasurement )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
        /* [in] */GUID physicalNetworkInterface,
        /* [in] */UINT64 bitrate
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_LogicalNetworkInterface )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
        /* [retval, out] */__RPC__out GUID * logicalNetworkInterface
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_PhysicalNetworkInterface )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
        /* [retval, out] */__RPC__out GUID * physicalNetworkInterface
        );
    HRESULT ( STDMETHODCALLTYPE *NotifyStreamingStatus )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * This,
        /* [in] */boolean fIsStreaming
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_get_MaxVolume(This,value) \
    ( (This)->lpVtbl->get_MaxVolume(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_get_IsMuteSupported(This,value) \
    ( (This)->lpVtbl->get_IsMuteSupported(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_get_IsSetNextSourceSupported(This,value) \
    ( (This)->lpVtbl->get_IsSetNextSourceSupported(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_get_IsAudioSupported(This,value) \
    ( (This)->lpVtbl->get_IsAudioSupported(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_get_IsVideoSupported(This,value) \
    ( (This)->lpVtbl->get_IsVideoSupported(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_get_IsImageSupported(This,value) \
    ( (This)->lpVtbl->get_IsImageSupported(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_get_IsSearchSupported(This,value) \
    ( (This)->lpVtbl->get_IsSearchSupported(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_GetCachedSinkProtocolInfo(This,value) \
    ( (This)->lpVtbl->GetCachedSinkProtocolInfo(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_SetCachedSinkProtocolInfo(This,value) \
    ( (This)->lpVtbl->SetCachedSinkProtocolInfo(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_GetCachedExtraSinkProtocolInfo(This,value) \
    ( (This)->lpVtbl->GetCachedExtraSinkProtocolInfo(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_GetEffectiveBandwidth(This,transmitSpeed,currentSpeed) \
    ( (This)->lpVtbl->GetEffectiveBandwidth(This,transmitSpeed,currentSpeed) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_GetCachedBitrateMeasurement(This,physicalNetworkInterface,bitrate) \
    ( (This)->lpVtbl->GetCachedBitrateMeasurement(This,physicalNetworkInterface,bitrate) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_SetCachedBitrateMeasurement(This,physicalNetworkInterface,bitrate) \
    ( (This)->lpVtbl->SetCachedBitrateMeasurement(This,physicalNetworkInterface,bitrate) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_get_LogicalNetworkInterface(This,logicalNetworkInterface) \
    ( (This)->lpVtbl->get_LogicalNetworkInterface(This,logicalNetworkInterface) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_get_PhysicalNetworkInterface(This,physicalNetworkInterface) \
    ( (This)->lpVtbl->get_PhysicalNetworkInterface(This,physicalNetworkInterface) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_NotifyStreamingStatus(This,fIsStreaming) \
    ( (This)->lpVtbl->NotifyStreamingStatus(This,fIsStreaming) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IActiveBasicDeviceStatics
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IActiveBasicDeviceStatics[] = L"Windows.Media.Streaming.IActiveBasicDeviceStatics";
/* [object, uuid("6D33255D-3642-4618-9DB6-43524F4DEADC"), contract] */
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStaticsVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
HRESULT ( STDMETHODCALLTYPE *CreateBasicDeviceAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics * This,
        /* [in] */__RPC__in HSTRING deviceIdentifier,
        /* [in] */__x_ABI_CWindows_CMedia_CStreaming_CDeviceTypes type,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice * * value
        );
    HRESULT ( STDMETHODCALLTYPE *CloneBasicDeviceAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * basicDevice,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CActiveBasicDevice * * value
        );
    HRESULT ( STDMETHODCALLTYPE *GetDevicesOnMatchingNetworkAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * server,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * renderer,
        /* [in] */boolean optimizeForProxying,
        /* [in] */boolean allowChangeRendererNetwork,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair * * operation
        );
    HRESULT ( STDMETHODCALLTYPE *CreateDevicesOnMatchingNetworkAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics * This,
        /* [in] */__RPC__in HSTRING serverUDN,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * renderer,
        /* [in] */boolean optimizeForProxying,
        /* [in] */boolean allowChangeRendererNetwork,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CDevicePair * * operation
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStaticsVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_CreateBasicDeviceAsync(This,deviceIdentifier,type,value) \
    ( (This)->lpVtbl->CreateBasicDeviceAsync(This,deviceIdentifier,type,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_CloneBasicDeviceAsync(This,basicDevice,value) \
    ( (This)->lpVtbl->CloneBasicDeviceAsync(This,basicDevice,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_GetDevicesOnMatchingNetworkAsync(This,server,renderer,optimizeForProxying,allowChangeRendererNetwork,operation) \
    ( (This)->lpVtbl->GetDevicesOnMatchingNetworkAsync(This,server,renderer,optimizeForProxying,allowChangeRendererNetwork,operation) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_CreateDevicesOnMatchingNetworkAsync(This,serverUDN,renderer,optimizeForProxying,allowChangeRendererNetwork,operation) \
    ( (This)->lpVtbl->CreateDevicesOnMatchingNetworkAsync(This,serverUDN,renderer,optimizeForProxying,allowChangeRendererNetwork,operation) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IBasicDevice
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IBasicDevice[] = L"Windows.Media.Streaming.IBasicDevice";
/* [object, uuid("F4F26CBB-7962-48B7-80F7-C3A5D753BCB0"), contract] */
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CIBasicDeviceVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
        );
    /* [propput] */HRESULT ( STDMETHODCALLTYPE *put_FriendlyName )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [in] */__RPC__in HSTRING value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ManufacturerName )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ManufacturerUrl )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_UniqueDeviceName )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ModelName )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ModelNumber )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ModelUrl )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Description )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_SerialNumber )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_PresentationUrl )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_RemoteStreamingUrls )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__deref_out_opt __FIVector_1_HSTRING * * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_PhysicalAddresses )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__deref_out_opt __FIVector_1_HSTRING * * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IpAddresses )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__deref_out_opt __FIVector_1_HSTRING * * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_CanWakeDevices )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_DiscoveredOnCurrentNetwork )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Type )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__out __x_ABI_CWindows_CMedia_CStreaming_CDeviceTypes * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Icons )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__deref_out_opt __FIVector_1_Windows__CMedia__CStreaming__CIDeviceIcon * * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ConnectionStatus )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [retval, out] */__RPC__out __x_ABI_CWindows_CMedia_CStreaming_CConnectionStatus * value
        );
    /* [eventadd] */HRESULT ( STDMETHODCALLTYPE *add_ConnectionStatusChanged )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CIConnectionStatusHandler  * handler,
        /* [retval, out] */__RPC__out EventRegistrationToken * token
        );
    /* [eventremove] */HRESULT ( STDMETHODCALLTYPE *remove_ConnectionStatusChanged )(
        __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * This,
        /* [in] */EventRegistrationToken token
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CMedia_CStreaming_CIBasicDeviceVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CIBasicDeviceVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_FriendlyName(This,value) \
    ( (This)->lpVtbl->get_FriendlyName(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_put_FriendlyName(This,value) \
    ( (This)->lpVtbl->put_FriendlyName(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_ManufacturerName(This,value) \
    ( (This)->lpVtbl->get_ManufacturerName(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_ManufacturerUrl(This,value) \
    ( (This)->lpVtbl->get_ManufacturerUrl(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_UniqueDeviceName(This,value) \
    ( (This)->lpVtbl->get_UniqueDeviceName(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_ModelName(This,value) \
    ( (This)->lpVtbl->get_ModelName(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_ModelNumber(This,value) \
    ( (This)->lpVtbl->get_ModelNumber(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_ModelUrl(This,value) \
    ( (This)->lpVtbl->get_ModelUrl(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_Description(This,value) \
    ( (This)->lpVtbl->get_Description(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_SerialNumber(This,value) \
    ( (This)->lpVtbl->get_SerialNumber(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_PresentationUrl(This,value) \
    ( (This)->lpVtbl->get_PresentationUrl(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_RemoteStreamingUrls(This,value) \
    ( (This)->lpVtbl->get_RemoteStreamingUrls(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_PhysicalAddresses(This,value) \
    ( (This)->lpVtbl->get_PhysicalAddresses(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_IpAddresses(This,value) \
    ( (This)->lpVtbl->get_IpAddresses(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_CanWakeDevices(This,value) \
    ( (This)->lpVtbl->get_CanWakeDevices(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_DiscoveredOnCurrentNetwork(This,value) \
    ( (This)->lpVtbl->get_DiscoveredOnCurrentNetwork(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_Type(This,value) \
    ( (This)->lpVtbl->get_Type(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_Icons(This,value) \
    ( (This)->lpVtbl->get_Icons(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_get_ConnectionStatus(This,value) \
    ( (This)->lpVtbl->get_ConnectionStatus(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_add_ConnectionStatusChanged(This,handler,token) \
    ( (This)->lpVtbl->add_ConnectionStatusChanged(This,handler,token) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_remove_ConnectionStatusChanged(This,token) \
    ( (This)->lpVtbl->remove_ConnectionStatusChanged(This,token) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IDeviceController
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IDeviceController[] = L"Windows.Media.Streaming.IDeviceController";
/* [object, uuid("4FEEB26D-50A7-402B-896A-BE95064D6BFF"), contract] */
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_CachedDevices )(
        __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController * This,
        /* [retval, out] */__RPC__deref_out_opt __FIVector_1_Windows__CMedia__CStreaming__CIBasicDevice * * value
        );
    HRESULT ( STDMETHODCALLTYPE *AddDevice )(
        __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController * This,
        /* [in] */__RPC__in HSTRING uniqueDeviceName
        );
    HRESULT ( STDMETHODCALLTYPE *RemoveDevice )(
        __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * device
        );
    /* [eventadd] */HRESULT ( STDMETHODCALLTYPE *add_DeviceArrival )(
        __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler  * handler,
        /* [retval, out] */__RPC__out EventRegistrationToken * token
        );
    /* [eventremove] */HRESULT ( STDMETHODCALLTYPE *remove_DeviceArrival )(
        __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController * This,
        /* [in] */EventRegistrationToken token
        );
    /* [eventadd] */HRESULT ( STDMETHODCALLTYPE *add_DeviceDeparture )(
        __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerFinderHandler  * handler,
        /* [retval, out] */__RPC__out EventRegistrationToken * token
        );
    /* [eventremove] */HRESULT ( STDMETHODCALLTYPE *remove_DeviceDeparture )(
        __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController * This,
        /* [in] */EventRegistrationToken token
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CIDeviceControllerVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_get_CachedDevices(This,value) \
    ( (This)->lpVtbl->get_CachedDevices(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_AddDevice(This,uniqueDeviceName) \
    ( (This)->lpVtbl->AddDevice(This,uniqueDeviceName) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_RemoveDevice(This,device) \
    ( (This)->lpVtbl->RemoveDevice(This,device) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_add_DeviceArrival(This,handler,token) \
    ( (This)->lpVtbl->add_DeviceArrival(This,handler,token) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_remove_DeviceArrival(This,token) \
    ( (This)->lpVtbl->remove_DeviceArrival(This,token) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_add_DeviceDeparture(This,handler,token) \
    ( (This)->lpVtbl->add_DeviceDeparture(This,handler,token) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_remove_DeviceDeparture(This,token) \
    ( (This)->lpVtbl->remove_DeviceDeparture(This,token) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIDeviceController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIDeviceController_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IDeviceIcon
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IDeviceIcon[] = L"Windows.Media.Streaming.IDeviceIcon";
/* [object, uuid("8FFB1A1E-023D-4DE1-B556-AB5ABF01929C"), contract] */
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIconVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Width )(
        __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * This,
        /* [retval, out] */__RPC__out UINT32 * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Height )(
        __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * This,
        /* [retval, out] */__RPC__out UINT32 * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ContentType )(
        __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Stream )(
        __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon * This,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType * * value
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIconVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIconVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_get_Width(This,value) \
    ( (This)->lpVtbl->get_Width(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_get_Height(This,value) \
    ( (This)->lpVtbl->get_Height(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_get_ContentType(This,value) \
    ( (This)->lpVtbl->get_ContentType(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_get_Stream(This,value) \
    ( (This)->lpVtbl->get_Stream(This,value) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIDeviceIcon_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IDevicePair
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IDevicePair[] = L"Windows.Media.Streaming.IDevicePair";
/* [object, uuid("F1A423F1-B7B4-449C-A90D-AEA8E17C5E5F"), contract] */
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CIDevicePairVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Server )(
        __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair * This,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Renderer )(
        __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair * This,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CWindows_CMedia_CStreaming_CIActiveBasicDevice * * value
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CMedia_CStreaming_CIDevicePairVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CIDevicePairVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_get_Server(This,value) \
    ( (This)->lpVtbl->get_Server(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_get_Renderer(This,value) \
    ( (This)->lpVtbl->get_Renderer(This,value) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIDevicePair;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIDevicePair_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IMediaRenderer
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Streaming.IBasicDevice
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IMediaRenderer[] = L"Windows.Media.Streaming.IMediaRenderer";
/* [object, uuid("2C012EC3-D975-47FB-96AC-A6418B326D2B"), contract] */
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IsAudioSupported )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IsVideoSupported )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IsImageSupported )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ActionInformation )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation * * value
        );
    HRESULT ( STDMETHODCALLTYPE *SetSourceFromUriAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [in] */__RPC__in HSTRING URI,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_UINT32 * * value
        );
    HRESULT ( STDMETHODCALLTYPE *SetSourceFromStreamAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [in] */__RPC__in_opt IInspectable * stream,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_UINT32 * * value
        );
    HRESULT ( STDMETHODCALLTYPE *SetSourceFromMediaSourceAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [in] */__RPC__in_opt IInspectable * mediaSource,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_UINT32 * * value
        );
    HRESULT ( STDMETHODCALLTYPE *SetNextSourceFromUriAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [in] */__RPC__in HSTRING URI,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_UINT32 * * value
        );
    HRESULT ( STDMETHODCALLTYPE *SetNextSourceFromStreamAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [in] */__RPC__in_opt IInspectable * stream,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_UINT32 * * value
        );
    HRESULT ( STDMETHODCALLTYPE *SetNextSourceFromMediaSourceAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [in] */__RPC__in_opt IInspectable * mediaSource,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_UINT32 * * value
        );
    HRESULT ( STDMETHODCALLTYPE *PlayAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CWindows_CFoundation_CIAsyncAction * * value
        );
    HRESULT ( STDMETHODCALLTYPE *PlayAtSpeedAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [in] */__x_ABI_CWindows_CMedia_CStreaming_CPlaySpeed playSpeed,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CWindows_CFoundation_CIAsyncAction * * value
        );
    HRESULT ( STDMETHODCALLTYPE *StopAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CWindows_CFoundation_CIAsyncAction * * value
        );
    HRESULT ( STDMETHODCALLTYPE *PauseAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CWindows_CFoundation_CIAsyncAction * * value
        );
    HRESULT ( STDMETHODCALLTYPE *GetMuteAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_boolean * * value
        );
    HRESULT ( STDMETHODCALLTYPE *SetMuteAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [in] */boolean mute,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CWindows_CFoundation_CIAsyncAction * * value
        );
    HRESULT ( STDMETHODCALLTYPE *GetVolumeAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_UINT32 * * value
        );
    HRESULT ( STDMETHODCALLTYPE *SetVolumeAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [in] */UINT32 volume,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CWindows_CFoundation_CIAsyncAction * * value
        );
    HRESULT ( STDMETHODCALLTYPE *SeekAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [in] */__x_ABI_CWindows_CFoundation_CTimeSpan target,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CWindows_CFoundation_CIAsyncAction * * value
        );
    HRESULT ( STDMETHODCALLTYPE *GetTransportInformationAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CTransportInformation * * value
        );
    HRESULT ( STDMETHODCALLTYPE *GetPositionInformationAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CPositionInformation * * value
        );
    /* [eventadd] */HRESULT ( STDMETHODCALLTYPE *add_TransportParametersUpdate )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersUpdateHandler  * handler,
        /* [retval, out] */__RPC__out EventRegistrationToken * token
        );
    /* [eventremove] */HRESULT ( STDMETHODCALLTYPE *remove_TransportParametersUpdate )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [in] */EventRegistrationToken token
        );
    /* [eventadd] */HRESULT ( STDMETHODCALLTYPE *add_RenderingParametersUpdate )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CIRenderingParametersUpdateHandler  * handler,
        /* [retval, out] */__RPC__out EventRegistrationToken * token
        );
    /* [eventremove] */HRESULT ( STDMETHODCALLTYPE *remove_RenderingParametersUpdate )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [in] */EventRegistrationToken token
        );
    HRESULT ( STDMETHODCALLTYPE *NextAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer * This,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CWindows_CFoundation_CIAsyncAction * * value
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_get_IsAudioSupported(This,value) \
    ( (This)->lpVtbl->get_IsAudioSupported(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_get_IsVideoSupported(This,value) \
    ( (This)->lpVtbl->get_IsVideoSupported(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_get_IsImageSupported(This,value) \
    ( (This)->lpVtbl->get_IsImageSupported(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_get_ActionInformation(This,value) \
    ( (This)->lpVtbl->get_ActionInformation(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_SetSourceFromUriAsync(This,URI,value) \
    ( (This)->lpVtbl->SetSourceFromUriAsync(This,URI,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_SetSourceFromStreamAsync(This,stream,value) \
    ( (This)->lpVtbl->SetSourceFromStreamAsync(This,stream,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_SetSourceFromMediaSourceAsync(This,mediaSource,value) \
    ( (This)->lpVtbl->SetSourceFromMediaSourceAsync(This,mediaSource,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_SetNextSourceFromUriAsync(This,URI,value) \
    ( (This)->lpVtbl->SetNextSourceFromUriAsync(This,URI,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_SetNextSourceFromStreamAsync(This,stream,value) \
    ( (This)->lpVtbl->SetNextSourceFromStreamAsync(This,stream,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_SetNextSourceFromMediaSourceAsync(This,mediaSource,value) \
    ( (This)->lpVtbl->SetNextSourceFromMediaSourceAsync(This,mediaSource,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_PlayAsync(This,value) \
    ( (This)->lpVtbl->PlayAsync(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_PlayAtSpeedAsync(This,playSpeed,value) \
    ( (This)->lpVtbl->PlayAtSpeedAsync(This,playSpeed,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_StopAsync(This,value) \
    ( (This)->lpVtbl->StopAsync(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_PauseAsync(This,value) \
    ( (This)->lpVtbl->PauseAsync(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_GetMuteAsync(This,value) \
    ( (This)->lpVtbl->GetMuteAsync(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_SetMuteAsync(This,mute,value) \
    ( (This)->lpVtbl->SetMuteAsync(This,mute,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_GetVolumeAsync(This,value) \
    ( (This)->lpVtbl->GetVolumeAsync(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_SetVolumeAsync(This,volume,value) \
    ( (This)->lpVtbl->SetVolumeAsync(This,volume,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_SeekAsync(This,target,value) \
    ( (This)->lpVtbl->SeekAsync(This,target,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_GetTransportInformationAsync(This,value) \
    ( (This)->lpVtbl->GetTransportInformationAsync(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_GetPositionInformationAsync(This,value) \
    ( (This)->lpVtbl->GetPositionInformationAsync(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_add_TransportParametersUpdate(This,handler,token) \
    ( (This)->lpVtbl->add_TransportParametersUpdate(This,handler,token) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_remove_TransportParametersUpdate(This,token) \
    ( (This)->lpVtbl->remove_TransportParametersUpdate(This,token) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_add_RenderingParametersUpdate(This,handler,token) \
    ( (This)->lpVtbl->add_RenderingParametersUpdate(This,handler,token) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_remove_RenderingParametersUpdate(This,token) \
    ( (This)->lpVtbl->remove_RenderingParametersUpdate(This,token) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_NextAsync(This,value) \
    ( (This)->lpVtbl->NextAsync(This,value) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IMediaRendererActionInformation
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IMediaRendererActionInformation[] = L"Windows.Media.Streaming.IMediaRendererActionInformation";
/* [object, uuid("66FBBFEE-5AB0-4A4F-8D15-E5056B26BEDA"), contract] */
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformationVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IsMuteAvailable )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IsPauseAvailable )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IsPlayAvailable )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IsSeekAvailable )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IsSetNextSourceAvailable )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IsStopAvailable )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IsVolumeAvailable )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_PlaySpeeds )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation * This,
        /* [retval, out] */__RPC__deref_out_opt __FIVector_1_Windows__CMedia__CStreaming__CPlaySpeed * * value
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformationVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformationVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_get_IsMuteAvailable(This,value) \
    ( (This)->lpVtbl->get_IsMuteAvailable(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_get_IsPauseAvailable(This,value) \
    ( (This)->lpVtbl->get_IsPauseAvailable(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_get_IsPlayAvailable(This,value) \
    ( (This)->lpVtbl->get_IsPlayAvailable(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_get_IsSeekAvailable(This,value) \
    ( (This)->lpVtbl->get_IsSeekAvailable(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_get_IsSetNextSourceAvailable(This,value) \
    ( (This)->lpVtbl->get_IsSetNextSourceAvailable(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_get_IsStopAvailable(This,value) \
    ( (This)->lpVtbl->get_IsStopAvailable(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_get_IsVolumeAvailable(This,value) \
    ( (This)->lpVtbl->get_IsVolumeAvailable(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_get_PlaySpeeds(This,value) \
    ( (This)->lpVtbl->get_PlaySpeeds(This,value) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IMediaRendererFactory
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IMediaRendererFactory[] = L"Windows.Media.Streaming.IMediaRendererFactory";
/* [object, uuid("657AB43D-B909-42B2-94D0-E3A0B134E8C9"), contract] */
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactoryVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
HRESULT ( STDMETHODCALLTYPE *CreateMediaRendererAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory * This,
        /* [in] */__RPC__in HSTRING deviceIdentifier,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer * * value
        );
    HRESULT ( STDMETHODCALLTYPE *CreateMediaRendererFromBasicDeviceAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * basicDevice,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer * * value
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactoryVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_CreateMediaRendererAsync(This,deviceIdentifier,value) \
    ( (This)->lpVtbl->CreateMediaRendererAsync(This,deviceIdentifier,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_CreateMediaRendererFromBasicDeviceAsync(This,basicDevice,value) \
    ( (This)->lpVtbl->CreateMediaRendererFromBasicDeviceAsync(This,basicDevice,value) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.IStreamSelectorStatics
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_IStreamSelectorStatics[] = L"Windows.Media.Streaming.IStreamSelectorStatics";
/* [object, uuid("8A4CD4A1-ED85-4E0F-BD68-8A6862E4636D"), contract] */
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStaticsVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
HRESULT ( STDMETHODCALLTYPE *SelectBestStreamAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CStorage_CIStorageFile * storageFile,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet * selectorProperties,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType * * value
        );
    HRESULT ( STDMETHODCALLTYPE *GetStreamPropertiesAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CStorage_CIStorageFile * storageFile,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet * selectorProperties,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet * * value
        );
    HRESULT ( STDMETHODCALLTYPE *SelectBestStreamFromStreamAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream * stream,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet * selectorProperties,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamWithContentType * * value
        );
    HRESULT ( STDMETHODCALLTYPE *GetStreamPropertiesFromStreamAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream * stream,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet * selectorProperties,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1___FIVectorView_1___F__CIPropertySet * * value
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStaticsVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_SelectBestStreamAsync(This,storageFile,selectorProperties,value) \
    ( (This)->lpVtbl->SelectBestStreamAsync(This,storageFile,selectorProperties,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_GetStreamPropertiesAsync(This,storageFile,selectorProperties,value) \
    ( (This)->lpVtbl->GetStreamPropertiesAsync(This,storageFile,selectorProperties,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_SelectBestStreamFromStreamAsync(This,stream,selectorProperties,value) \
    ( (This)->lpVtbl->SelectBestStreamFromStreamAsync(This,stream,selectorProperties,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_GetStreamPropertiesFromStreamAsync(This,stream,selectorProperties,value) \
    ( (This)->lpVtbl->GetStreamPropertiesFromStreamAsync(This,stream,selectorProperties,value) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CIStreamSelectorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Media.Streaming.ITransportParameters
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_ITransportParameters[] = L"Windows.Media.Streaming.ITransportParameters";
/* [object, uuid("EB0C4E24-2283-438D-8FFF-DBE9DF1CB2CC"), contract] */
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ActionInformation )(
        __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters * This,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CWindows_CMedia_CStreaming_CIMediaRendererActionInformation * * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_TrackInformation )(
        __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters * This,
        /* [retval, out] */__RPC__out __x_ABI_CWindows_CMedia_CStreaming_CTrackInformation * value
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_TransportInformation )(
        __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters * This,
        /* [retval, out] */__RPC__out __x_ABI_CWindows_CMedia_CStreaming_CTransportInformation * value
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CITransportParametersVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_get_ActionInformation(This,value) \
    ( (This)->lpVtbl->get_ActionInformation(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_get_TrackInformation(This,value) \
    ( (This)->lpVtbl->get_TrackInformation(This,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_get_TransportInformation(This,value) \
    ( (This)->lpVtbl->get_TransportInformation(This,value) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CITransportParameters;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CITransportParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.Media.Streaming.ActiveBasicDevice
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Media.Streaming.StreamingContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Streaming.IActiveBasicDeviceStatics interface starting with version 1.0 of the Windows.Media.Streaming.StreamingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.IActiveBasicDevice ** Default Interface **
 *    Windows.Media.Streaming.IBasicDevice
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Media_Streaming_ActiveBasicDevice_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_ActiveBasicDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_ActiveBasicDevice[] = L"Windows.Media.Streaming.ActiveBasicDevice";
#endif
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.Media.Streaming.BasicDevice
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Media.Streaming.StreamingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.IBasicDevice ** Default Interface **
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Media_Streaming_BasicDevice_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_BasicDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_BasicDevice[] = L"Windows.Media.Streaming.BasicDevice";
#endif
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.Media.Streaming.CreateMediaRendererOperation
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncOperation_1_Windows.Media.Streaming.MediaRenderer ** Default Interface **
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Media_Streaming_CreateMediaRendererOperation_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_CreateMediaRendererOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_CreateMediaRendererOperation[] = L"Windows.Media.Streaming.CreateMediaRendererOperation";
#endif
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.Media.Streaming.DeviceController
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Media.Streaming.StreamingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.IDeviceController ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Media_Streaming_DeviceController_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_DeviceController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_DeviceController[] = L"Windows.Media.Streaming.DeviceController";
#endif
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.Media.Streaming.DevicePair
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.IDevicePair ** Default Interface **
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Media_Streaming_DevicePair_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_DevicePair_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_DevicePair[] = L"Windows.Media.Streaming.DevicePair";
#endif
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.Media.Streaming.MediaRenderer
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Media.Streaming.StreamingContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Streaming.IMediaRendererFactory interface starting with version 1.0 of the Windows.Media.Streaming.StreamingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.IMediaRenderer ** Default Interface **
 *    Windows.Media.Streaming.IBasicDevice
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Media_Streaming_MediaRenderer_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_MediaRenderer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_MediaRenderer[] = L"Windows.Media.Streaming.MediaRenderer";
#endif
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.Media.Streaming.StreamSelector
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Streaming.IStreamSelectorStatics interface starting with version 1.0 of the Windows.Media.Streaming.StreamingContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_StreamSelector_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_StreamSelector_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_StreamSelector[] = L"Windows.Media.Streaming.StreamSelector";
#endif
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000




#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Emedia2Estreaming_p_h__

#endif // __windows2Emedia2Estreaming_h__
