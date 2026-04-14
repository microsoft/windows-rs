/* Header file automatically generated from windows.media.streaming.internal.idl */
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
#ifndef __windows2Emedia2Estreaming2Einternal_h__
#define __windows2Emedia2Estreaming2Einternal_h__
#ifndef __windows2Emedia2Estreaming2Einternal_p_h__
#define __windows2Emedia2Estreaming2Einternal_p_h__


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
#include "Windows.Media.Streaming.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Internal {
                    interface IMediaRendererBrokerStatics;
                } /* Internal */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics ABI::Windows::Media::Streaming::Internal::IMediaRendererBrokerStatics

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                class MediaRenderer;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

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
                class CreateMediaRendererOperation;
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */


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













/*
 *
 * Interface Windows.Media.Streaming.Internal.IMediaRendererBrokerStatics
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Internal_IMediaRendererBrokerStatics[] = L"Windows.Media.Streaming.Internal.IMediaRendererBrokerStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Internal {
                    /* [object, uuid("19974CB1-9A1D-45C0-87CF-2C1E835A981C"), contract] */
                    MIDL_INTERFACE("19974CB1-9A1D-45C0-87CF-2C1E835A981C")
                    IMediaRendererBrokerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateMediaRendererAsync(
                            /* [in] */__RPC__in HSTRING deviceIdentifier,
                            /* [in] */__RPC__in_opt IInspectable * mediaSessionFactoryMF,
                            /* [in] */__RPC__in HSTRING applicationUserModelId,
                            /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer * * value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateMediaRendererFromBasicDeviceAsync(
                            /* [in] */__RPC__in_opt ABI::Windows::Media::Streaming::IBasicDevice * basicDevice,
                            /* [in] */__RPC__in_opt IInspectable * mediaSessionFactoryMF,
                            /* [in] */__RPC__in HSTRING applicationUserModelId,
                            /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer * * value
                            ) = 0;
                        
                    };

                    MIDL_CONST_ID IID & IID_IMediaRendererBrokerStatics=__uuidof(IMediaRendererBrokerStatics);
                    
                } /* Internal */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.Media.Streaming.Internal.MediaRendererBroker
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Streaming.Internal.IMediaRendererBrokerStatics interface starting with version 1.0 of the Windows.Media.Streaming.StreamingContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Internal_MediaRendererBroker_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Internal_MediaRendererBroker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Internal_MediaRendererBroker[] = L"Windows.Media.Streaming.Internal.MediaRendererBroker";
#endif
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000





#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions
#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer __x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIMediaRenderer_FWD_DEFINED__


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



#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice_FWD_DEFINED__













/*
 *
 * Interface Windows.Media.Streaming.Internal.IMediaRendererBrokerStatics
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Internal_IMediaRendererBrokerStatics[] = L"Windows.Media.Streaming.Internal.IMediaRendererBrokerStatics";
/* [object, uuid("19974CB1-9A1D-45C0-87CF-2C1E835A981C"), contract] */
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStaticsVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
HRESULT ( STDMETHODCALLTYPE *CreateMediaRendererAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics * This,
        /* [in] */__RPC__in HSTRING deviceIdentifier,
        /* [in] */__RPC__in_opt IInspectable * mediaSessionFactoryMF,
        /* [in] */__RPC__in HSTRING applicationUserModelId,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer * * value
        );
    HRESULT ( STDMETHODCALLTYPE *CreateMediaRendererFromBasicDeviceAsync )(
        __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics * This,
        /* [in] */__RPC__in_opt __x_ABI_CWindows_CMedia_CStreaming_CIBasicDevice * basicDevice,
        /* [in] */__RPC__in_opt IInspectable * mediaSessionFactoryMF,
        /* [in] */__RPC__in HSTRING applicationUserModelId,
        /* [retval, out] */__RPC__deref_out_opt __FIAsyncOperation_1_Windows__CMedia__CStreaming__CMediaRenderer * * value
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStaticsVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_CreateMediaRendererAsync(This,deviceIdentifier,mediaSessionFactoryMF,applicationUserModelId,value) \
    ( (This)->lpVtbl->CreateMediaRendererAsync(This,deviceIdentifier,mediaSessionFactoryMF,applicationUserModelId,value) )

#define __x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_CreateMediaRendererFromBasicDeviceAsync(This,basicDevice,mediaSessionFactoryMF,applicationUserModelId,value) \
    ( (This)->lpVtbl->CreateMediaRendererFromBasicDeviceAsync(This,basicDevice,mediaSessionFactoryMF,applicationUserModelId,value) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CInternal_CIMediaRendererBrokerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.Media.Streaming.Internal.MediaRendererBroker
 *
 * Introduced to Windows.Media.Streaming.StreamingContract in version 1.0
 *
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Streaming.Internal.IMediaRendererBrokerStatics interface starting with version 1.0 of the Windows.Media.Streaming.StreamingContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 */
#if WINDOWS_MEDIA_STREAMING_STREAMINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Internal_MediaRendererBroker_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Internal_MediaRendererBroker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Internal_MediaRendererBroker[] = L"Windows.Media.Streaming.Internal.MediaRendererBroker";
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
#endif // __windows2Emedia2Estreaming2Einternal_p_h__

#endif // __windows2Emedia2Estreaming2Einternal_h__
