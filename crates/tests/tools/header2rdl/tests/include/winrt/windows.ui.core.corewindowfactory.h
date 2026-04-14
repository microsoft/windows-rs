/* Header file automatically generated from windows.ui.core.corewindowfactory.idl */
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
#ifndef __windows2Eui2Ecore2Ecorewindowfactory_h__
#define __windows2Eui2Ecore2Ecorewindowfactory_h__
#ifndef __windows2Eui2Ecore2Ecorewindowfactory_p_h__
#define __windows2Eui2Ecore2Ecorewindowfactory_p_h__


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

#if !defined(WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION)
#define WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION)

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
#include "Windows.UI.Core.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindowFactory;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory ABI::Windows::UI::Core::ICoreWindowFactory

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_FWD_DEFINED__




namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class HostedCoreWindowFactory;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */


namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class ImmersiveCoreWindowFactory;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */


namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class UAPCoreWindowFactory;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */







/*
 *
 * Interface Windows.UI.Core.ICoreWindowFactory
 *
 * Introduced to Windows.UI.Core.CoreWindowFactoryContract in version 1.0
 *
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowFactory[] = L"Windows.UI.Core.ICoreWindowFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                /* [object, uuid("CD292360-2763-4085-8A9F-74B224A29175"), contract] */
                MIDL_INTERFACE("CD292360-2763-4085-8A9F-74B224A29175")
                ICoreWindowFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateCoreWindow(
                        /* [in] */__RPC__in HSTRING windowTitle,
                        /* [retval, out] */__RPC__deref_out_opt ABI::Windows::UI::Core::ICoreWindow * * window
                        ) = 0;
                    /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_WindowReuseAllowed(
                        /* [retval, out] */__RPC__out ::boolean * value
                        ) = 0;
                    
                };

                MIDL_CONST_ID IID & IID_ICoreWindowFactory=__uuidof(ICoreWindowFactory);
                
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.UI.Core.HostedCoreWindowFactory
 *
 * Introduced to Windows.UI.Core.CoreWindowFactoryContract in version 1.0
 *
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindowFactory ** Default Interface **
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_UI_Core_HostedCoreWindowFactory_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_HostedCoreWindowFactory_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_HostedCoreWindowFactory[] = L"Windows.UI.Core.HostedCoreWindowFactory";
#endif
#endif // WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.UI.Core.ImmersiveCoreWindowFactory
 *
 * Introduced to Windows.UI.Core.CoreWindowFactoryContract in version 1.0
 *
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindowFactory ** Default Interface **
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_UI_Core_ImmersiveCoreWindowFactory_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_ImmersiveCoreWindowFactory_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_ImmersiveCoreWindowFactory[] = L"Windows.UI.Core.ImmersiveCoreWindowFactory";
#endif
#endif // WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.UI.Core.UAPCoreWindowFactory
 *
 * Introduced to Windows.UI.Core.CoreWindowFactoryContract in version 1.0
 *
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindowFactory ** Default Interface **
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_UI_Core_UAPCoreWindowFactory_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_UAPCoreWindowFactory_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_UAPCoreWindowFactory[] = L"Windows.UI.Core.UAPCoreWindowFactory";
#endif
#endif // WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION >= 0x10000




#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_FWD_DEFINED__











/*
 *
 * Interface Windows.UI.Core.ICoreWindowFactory
 *
 * Introduced to Windows.UI.Core.CoreWindowFactoryContract in version 1.0
 *
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowFactory[] = L"Windows.UI.Core.ICoreWindowFactory";
/* [object, uuid("CD292360-2763-4085-8A9F-74B224A29175"), contract] */
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreWindowFactoryVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
HRESULT ( STDMETHODCALLTYPE *CreateCoreWindow )(
        __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory * This,
        /* [in] */__RPC__in HSTRING windowTitle,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CWindows_CUI_CCore_CICoreWindow * * window
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_WindowReuseAllowed )(
        __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory * This,
        /* [retval, out] */__RPC__out boolean * value
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CUI_CCore_CICoreWindowFactoryVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreWindowFactoryVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_CreateCoreWindow(This,windowTitle,window) \
    ( (This)->lpVtbl->CreateCoreWindow(This,windowTitle,window) )

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_get_WindowReuseAllowed(This,value) \
    ( (This)->lpVtbl->get_WindowReuseAllowed(This,value) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.UI.Core.HostedCoreWindowFactory
 *
 * Introduced to Windows.UI.Core.CoreWindowFactoryContract in version 1.0
 *
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindowFactory ** Default Interface **
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_UI_Core_HostedCoreWindowFactory_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_HostedCoreWindowFactory_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_HostedCoreWindowFactory[] = L"Windows.UI.Core.HostedCoreWindowFactory";
#endif
#endif // WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.UI.Core.ImmersiveCoreWindowFactory
 *
 * Introduced to Windows.UI.Core.CoreWindowFactoryContract in version 1.0
 *
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindowFactory ** Default Interface **
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_UI_Core_ImmersiveCoreWindowFactory_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_ImmersiveCoreWindowFactory_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_ImmersiveCoreWindowFactory[] = L"Windows.UI.Core.ImmersiveCoreWindowFactory";
#endif
#endif // WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION >= 0x10000


/*
 *
 * Class Windows.UI.Core.UAPCoreWindowFactory
 *
 * Introduced to Windows.UI.Core.CoreWindowFactoryContract in version 1.0
 *
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindowFactory ** Default Interface **
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_UI_Core_UAPCoreWindowFactory_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_UAPCoreWindowFactory_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_UAPCoreWindowFactory[] = L"Windows.UI.Core.UAPCoreWindowFactory";
#endif
#endif // WINDOWS_UI_CORE_COREWINDOWFACTORYCONTRACT_VERSION >= 0x10000




#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Ecore2Ecorewindowfactory_p_h__

#endif // __windows2Eui2Ecore2Ecorewindowfactory_h__
