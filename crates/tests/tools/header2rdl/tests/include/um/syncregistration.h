

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __syncregistration_h__
#define __syncregistration_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

#ifndef __ISyncProviderRegistration_FWD_DEFINED__
#define __ISyncProviderRegistration_FWD_DEFINED__
typedef interface ISyncProviderRegistration ISyncProviderRegistration;

#endif 	/* __ISyncProviderRegistration_FWD_DEFINED__ */


#ifndef __IEnumSyncProviderConfigUIInfos_FWD_DEFINED__
#define __IEnumSyncProviderConfigUIInfos_FWD_DEFINED__
typedef interface IEnumSyncProviderConfigUIInfos IEnumSyncProviderConfigUIInfos;

#endif 	/* __IEnumSyncProviderConfigUIInfos_FWD_DEFINED__ */


#ifndef __IEnumSyncProviderInfos_FWD_DEFINED__
#define __IEnumSyncProviderInfos_FWD_DEFINED__
typedef interface IEnumSyncProviderInfos IEnumSyncProviderInfos;

#endif 	/* __IEnumSyncProviderInfos_FWD_DEFINED__ */


#ifndef __ISyncProviderInfo_FWD_DEFINED__
#define __ISyncProviderInfo_FWD_DEFINED__
typedef interface ISyncProviderInfo ISyncProviderInfo;

#endif 	/* __ISyncProviderInfo_FWD_DEFINED__ */


#ifndef __ISyncProviderConfigUIInfo_FWD_DEFINED__
#define __ISyncProviderConfigUIInfo_FWD_DEFINED__
typedef interface ISyncProviderConfigUIInfo ISyncProviderConfigUIInfo;

#endif 	/* __ISyncProviderConfigUIInfo_FWD_DEFINED__ */


#ifndef __ISyncProviderConfigUI_FWD_DEFINED__
#define __ISyncProviderConfigUI_FWD_DEFINED__
typedef interface ISyncProviderConfigUI ISyncProviderConfigUI;

#endif 	/* __ISyncProviderConfigUI_FWD_DEFINED__ */


#ifndef __IRegisteredSyncProvider_FWD_DEFINED__
#define __IRegisteredSyncProvider_FWD_DEFINED__
typedef interface IRegisteredSyncProvider IRegisteredSyncProvider;

#endif 	/* __IRegisteredSyncProvider_FWD_DEFINED__ */


#ifndef __ISyncRegistrationChange_FWD_DEFINED__
#define __ISyncRegistrationChange_FWD_DEFINED__
typedef interface ISyncRegistrationChange ISyncRegistrationChange;

#endif 	/* __ISyncRegistrationChange_FWD_DEFINED__ */


#ifndef __ISyncProviderRegistration_FWD_DEFINED__
#define __ISyncProviderRegistration_FWD_DEFINED__
typedef interface ISyncProviderRegistration ISyncProviderRegistration;

#endif 	/* __ISyncProviderRegistration_FWD_DEFINED__ */


#ifndef __IEnumSyncProviderConfigUIInfos_FWD_DEFINED__
#define __IEnumSyncProviderConfigUIInfos_FWD_DEFINED__
typedef interface IEnumSyncProviderConfigUIInfos IEnumSyncProviderConfigUIInfos;

#endif 	/* __IEnumSyncProviderConfigUIInfos_FWD_DEFINED__ */


#ifndef __IEnumSyncProviderInfos_FWD_DEFINED__
#define __IEnumSyncProviderInfos_FWD_DEFINED__
typedef interface IEnumSyncProviderInfos IEnumSyncProviderInfos;

#endif 	/* __IEnumSyncProviderInfos_FWD_DEFINED__ */


#ifndef __ISyncProviderInfo_FWD_DEFINED__
#define __ISyncProviderInfo_FWD_DEFINED__
typedef interface ISyncProviderInfo ISyncProviderInfo;

#endif 	/* __ISyncProviderInfo_FWD_DEFINED__ */


#ifndef __ISyncProviderConfigUIInfo_FWD_DEFINED__
#define __ISyncProviderConfigUIInfo_FWD_DEFINED__
typedef interface ISyncProviderConfigUIInfo ISyncProviderConfigUIInfo;

#endif 	/* __ISyncProviderConfigUIInfo_FWD_DEFINED__ */


#ifndef __ISyncProviderConfigUI_FWD_DEFINED__
#define __ISyncProviderConfigUI_FWD_DEFINED__
typedef interface ISyncProviderConfigUI ISyncProviderConfigUI;

#endif 	/* __ISyncProviderConfigUI_FWD_DEFINED__ */


#ifndef __ISyncRegistrationChange_FWD_DEFINED__
#define __ISyncRegistrationChange_FWD_DEFINED__
typedef interface ISyncRegistrationChange ISyncRegistrationChange;

#endif 	/* __ISyncRegistrationChange_FWD_DEFINED__ */


#ifndef __SyncProviderRegistration_FWD_DEFINED__
#define __SyncProviderRegistration_FWD_DEFINED__

#ifdef __cplusplus
typedef class SyncProviderRegistration SyncProviderRegistration;
#else
typedef struct SyncProviderRegistration SyncProviderRegistration;
#endif /* __cplusplus */

#endif 	/* __SyncProviderRegistration_FWD_DEFINED__ */


/* header files for imported files */
#include "oleidl.h"
#include "propsys.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_syncregistration_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)







#define SCC_DEFAULT ((DWORD)0x00000000)
#define SCC_CAN_CREATE_WITHOUT_UI ((DWORD)0x00000001)
#define SCC_CAN_MODIFY_WITHOUT_UI ((DWORD)0x00000002)
#define SCC_CREATE_NOT_SUPPORTED ((DWORD)0x00000004)
#define SCC_MODIFY_NOT_SUPPORTED ((DWORD)0x00000008)
#define SPC_DEFAULT ((DWORD)0x00000000)
#define SYNC_PROVIDER_STATE_ENABLED  ((DWORD)0x00000001)
#define SYNC_PROVIDER_STATE_DIRTY    ((DWORD)0x00000002)
#define SYNC_PROVIDER_CONFIGURATION_VERSION ((DWORD)0x00000001)
#define SYNC_PROVIDER_CONFIGUI_CONFIGURATION_VERSION ((DWORD)0x00000001)
#define SYNC_32_BIT_SUPPORTED ((DWORD)0x00000001)
#define SYNC_64_BIT_SUPPORTED ((DWORD)0x00000002)
DEFINE_PROPERTYKEY(PKEY_PROVIDER_INSTANCEID, 0x84179e61, 0x60f6, 0x4c1c, 0x88, 0xed, 0xf1, 0xc5, 0x31, 0xb3, 0x2b, 0xda, 2);
DEFINE_PROPERTYKEY(PKEY_PROVIDER_CLSID, 0x84179e61, 0x60f6, 0x4c1c, 0x88, 0xed, 0xf1, 0xc5, 0x31, 0xb3, 0x2b, 0xda, 3);
DEFINE_PROPERTYKEY(PKEY_PROVIDER_CONFIGUI, 0x84179e61, 0x60f6, 0x4c1c, 0x88, 0xed, 0xf1, 0xc5, 0x31, 0xb3, 0x2b, 0xda, 4);
DEFINE_PROPERTYKEY(PKEY_PROVIDER_CONTENTTYPE, 0x84179e61, 0x60f6, 0x4c1c, 0x88, 0xed, 0xf1, 0xc5, 0x31, 0xb3, 0x2b, 0xda, 5);
DEFINE_PROPERTYKEY(PKEY_PROVIDER_CAPABILITIES, 0x84179e61, 0x60f6, 0x4c1c, 0x88, 0xed, 0xf1, 0xc5, 0x31, 0xb3, 0x2b, 0xda, 6);
DEFINE_PROPERTYKEY(PKEY_PROVIDER_SUPPORTED_ARCHITECTURE, 0x84179e61, 0x60f6, 0x4c1c, 0x88, 0xed, 0xf1, 0xc5, 0x31, 0xb3, 0x2b, 0xda, 7);
DEFINE_PROPERTYKEY(PKEY_PROVIDER_NAME, 0x84179e61, 0x60f6, 0x4c1c, 0x88, 0xed, 0xf1, 0xc5, 0x31, 0xb3, 0x2b, 0xda, 8);
DEFINE_PROPERTYKEY(PKEY_PROVIDER_DESCRIPTION, 0x84179e61, 0x60f6, 0x4c1c, 0x88, 0xed, 0xf1, 0xc5, 0x31, 0xb3, 0x2b, 0xda, 9);
DEFINE_PROPERTYKEY(PKEY_PROVIDER_TOOLTIPS, 0x84179e61, 0x60f6, 0x4c1c, 0x88, 0xed, 0xf1, 0xc5, 0x31, 0xb3, 0x2b, 0xda, 10);
DEFINE_PROPERTYKEY(PKEY_PROVIDER_ICON, 0x84179e61, 0x60f6, 0x4c1c, 0x88, 0xed, 0xf1, 0xc5, 0x31, 0xb3, 0x2b, 0xda, 11);
DEFINE_PROPERTYKEY(PKEY_CONFIGUI_INSTANCEID, 0x554b24ea, 0xe8e3, 0x45ba, 0x93, 0x52, 0xdf, 0xb5, 0x61, 0xe1, 0x71, 0xe4, 2);
DEFINE_PROPERTYKEY(PKEY_CONFIGUI_CLSID, 0x554b24ea, 0xe8e3, 0x45ba, 0x93, 0x52, 0xdf, 0xb5, 0x61, 0xe1, 0x71, 0xe4, 3);
DEFINE_PROPERTYKEY(PKEY_CONFIGUI_CONTENTTYPE, 0x554b24ea, 0xe8e3, 0x45ba, 0x93, 0x52, 0xdf, 0xb5, 0x61, 0xe1, 0x71, 0xe4, 4);
DEFINE_PROPERTYKEY(PKEY_CONFIGUI_CAPABILITIES, 0x554b24ea, 0xe8e3, 0x45ba, 0x93, 0x52, 0xdf, 0xb5, 0x61, 0xe1, 0x71, 0xe4, 5);
DEFINE_PROPERTYKEY(PKEY_CONFIGUI_SUPPORTED_ARCHITECTURE, 0x554b24ea, 0xe8e3, 0x45ba, 0x93, 0x52, 0xdf, 0xb5, 0x61, 0xe1, 0x71, 0xe4, 6);
DEFINE_PROPERTYKEY(PKEY_CONFIGUI_IS_GLOBAL, 0x554b24ea, 0xe8e3, 0x45ba, 0x93, 0x52, 0xdf, 0xb5, 0x61, 0xe1, 0x71, 0xe4, 7);
DEFINE_PROPERTYKEY(PKEY_CONFIGUI_NAME, 0x554b24ea, 0xe8e3, 0x45ba, 0x93, 0x52, 0xdf, 0xb5, 0x61, 0xe1, 0x71, 0xe4, 8);
DEFINE_PROPERTYKEY(PKEY_CONFIGUI_DESCRIPTION, 0x554b24ea, 0xe8e3, 0x45ba, 0x93, 0x52, 0xdf, 0xb5, 0x61, 0xe1, 0x71, 0xe4, 9);
DEFINE_PROPERTYKEY(PKEY_CONFIGUI_TOOLTIPS, 0x554b24ea, 0xe8e3, 0x45ba, 0x93, 0x52, 0xdf, 0xb5, 0x61, 0xe1, 0x71, 0xe4, 10);
DEFINE_PROPERTYKEY(PKEY_CONFIGUI_ICON, 0x554b24ea, 0xe8e3, 0x45ba, 0x93, 0x52, 0xdf, 0xb5, 0x61, 0xe1, 0x71, 0xe4, 11);
DEFINE_PROPERTYKEY(PKEY_CONFIGUI_MENUITEM_NOUI, 0x554b24ea, 0xe8e3, 0x45ba, 0x93, 0x52, 0xdf, 0xb5, 0x61, 0xe1, 0x71, 0xe4, 12);
DEFINE_PROPERTYKEY(PKEY_CONFIGUI_MENUITEM, 0x554b24ea, 0xe8e3, 0x45ba, 0x93, 0x52, 0xdf, 0xb5, 0x61, 0xe1, 0x71, 0xe4, 13);
typedef struct _SyncProviderConfiguration
    {
    DWORD dwVersion;
    GUID guidInstanceId;
    CLSID clsidProvider;
    GUID guidConfigUIInstanceId;
    GUID guidContentType;
    DWORD dwCapabilities;
    DWORD dwSupportedArchitecture;
    } 	SyncProviderConfiguration;

typedef struct _SyncProviderConfigUIConfiguration
    {
    DWORD dwVersion;
    GUID guidInstanceId;
    CLSID clsidConfigUI;
    GUID guidContentType;
    DWORD dwCapabilities;
    DWORD dwSupportedArchitecture;
    BOOL fIsGlobal;
    } 	SyncProviderConfigUIConfiguration;



extern RPC_IF_HANDLE __MIDL_itf_syncregistration_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_syncregistration_0000_0000_v0_0_s_ifspec;

#ifndef __ISyncProviderRegistration_INTERFACE_DEFINED__
#define __ISyncProviderRegistration_INTERFACE_DEFINED__

/* interface ISyncProviderRegistration */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISyncProviderRegistration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("cb45953b-7624-47bc-a472-eb8cac6b222e")
    ISyncProviderRegistration : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateSyncProviderConfigUIRegistrationInstance( 
            /* [in] */ __RPC__in const SyncProviderConfigUIConfiguration *pConfigUIConfig,
            /* [retval][out] */ __RPC__deref_out_opt ISyncProviderConfigUIInfo **ppConfigUIInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterSyncProviderConfigUI( 
            /* [in] */ __RPC__in LPCGUID pguidInstanceId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateSyncProviderConfigUIs( 
            /* [unique][in] */ __RPC__in_opt LPCGUID pguidContentType,
            /* [in] */ DWORD dwSupportedArchitecture,
            /* [retval][out] */ __RPC__deref_out_opt IEnumSyncProviderConfigUIInfos **ppEnumSyncProviderConfigUIInfos) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSyncProviderRegistrationInstance( 
            /* [in] */ __RPC__in const SyncProviderConfiguration *pProviderConfiguration,
            /* [retval][out] */ __RPC__deref_out_opt ISyncProviderInfo **ppProviderInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterSyncProvider( 
            /* [in] */ __RPC__in LPCGUID pguidInstanceId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSyncProviderConfigUIInfoforProvider( 
            /* [in] */ __RPC__in LPCGUID pguidProviderInstanceId,
            /* [retval][out] */ __RPC__deref_out_opt ISyncProviderConfigUIInfo **ppProviderConfigUIInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateSyncProviders( 
            /* [unique][in] */ __RPC__in_opt LPCGUID pguidContentType,
            /* [in] */ DWORD dwStateFlagsToFilterMask,
            /* [in] */ DWORD dwStateFlagsToFilter,
            /* [in] */ __RPC__in REFCLSID refProviderClsId,
            /* [in] */ DWORD dwSupportedArchitecture,
            /* [retval][out] */ __RPC__deref_out_opt IEnumSyncProviderInfos **ppEnumSyncProviderInfos) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSyncProviderInfo( 
            /* [in] */ __RPC__in LPCGUID pguidInstanceId,
            /* [retval][out] */ __RPC__deref_out_opt ISyncProviderInfo **ppProviderInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSyncProviderFromInstanceId( 
            /* [in] */ __RPC__in LPCGUID pguidInstanceId,
            /* [in] */ DWORD dwClsContext,
            /* [retval][out] */ __RPC__deref_out_opt IRegisteredSyncProvider **ppSyncProvider) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSyncProviderConfigUIInfo( 
            /* [in] */ __RPC__in LPCGUID pguidInstanceId,
            /* [retval][out] */ __RPC__deref_out_opt ISyncProviderConfigUIInfo **ppConfigUIInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSyncProviderConfigUIFromInstanceId( 
            /* [in] */ __RPC__in LPCGUID pguidInstanceId,
            /* [in] */ DWORD dwClsContext,
            /* [retval][out] */ __RPC__deref_out_opt ISyncProviderConfigUI **ppConfigUI) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSyncProviderState( 
            /* [in] */ __RPC__in LPCGUID pguidInstanceId,
            /* [retval][out] */ __RPC__out DWORD *pdwStateFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSyncProviderState( 
            /* [in] */ __RPC__in LPCGUID pguidInstanceId,
            /* [in] */ DWORD dwStateFlagsMask,
            /* [in] */ DWORD dwStateFlags) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE RegisterForEvent( 
            /* [retval][out] */ HANDLE *phEvent) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE RevokeEvent( 
            /* [in] */ HANDLE hEvent) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetChange( 
            /* [in] */ HANDLE hEvent,
            /* [retval][out] */ ISyncRegistrationChange **ppChange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISyncProviderRegistrationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISyncProviderRegistration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISyncProviderRegistration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISyncProviderRegistration * This);
        
        DECLSPEC_XFGVIRT(ISyncProviderRegistration, CreateSyncProviderConfigUIRegistrationInstance)
        HRESULT ( STDMETHODCALLTYPE *CreateSyncProviderConfigUIRegistrationInstance )( 
            __RPC__in ISyncProviderRegistration * This,
            /* [in] */ __RPC__in const SyncProviderConfigUIConfiguration *pConfigUIConfig,
            /* [retval][out] */ __RPC__deref_out_opt ISyncProviderConfigUIInfo **ppConfigUIInfo);
        
        DECLSPEC_XFGVIRT(ISyncProviderRegistration, UnregisterSyncProviderConfigUI)
        HRESULT ( STDMETHODCALLTYPE *UnregisterSyncProviderConfigUI )( 
            __RPC__in ISyncProviderRegistration * This,
            /* [in] */ __RPC__in LPCGUID pguidInstanceId);
        
        DECLSPEC_XFGVIRT(ISyncProviderRegistration, EnumerateSyncProviderConfigUIs)
        HRESULT ( STDMETHODCALLTYPE *EnumerateSyncProviderConfigUIs )( 
            __RPC__in ISyncProviderRegistration * This,
            /* [unique][in] */ __RPC__in_opt LPCGUID pguidContentType,
            /* [in] */ DWORD dwSupportedArchitecture,
            /* [retval][out] */ __RPC__deref_out_opt IEnumSyncProviderConfigUIInfos **ppEnumSyncProviderConfigUIInfos);
        
        DECLSPEC_XFGVIRT(ISyncProviderRegistration, CreateSyncProviderRegistrationInstance)
        HRESULT ( STDMETHODCALLTYPE *CreateSyncProviderRegistrationInstance )( 
            __RPC__in ISyncProviderRegistration * This,
            /* [in] */ __RPC__in const SyncProviderConfiguration *pProviderConfiguration,
            /* [retval][out] */ __RPC__deref_out_opt ISyncProviderInfo **ppProviderInfo);
        
        DECLSPEC_XFGVIRT(ISyncProviderRegistration, UnregisterSyncProvider)
        HRESULT ( STDMETHODCALLTYPE *UnregisterSyncProvider )( 
            __RPC__in ISyncProviderRegistration * This,
            /* [in] */ __RPC__in LPCGUID pguidInstanceId);
        
        DECLSPEC_XFGVIRT(ISyncProviderRegistration, GetSyncProviderConfigUIInfoforProvider)
        HRESULT ( STDMETHODCALLTYPE *GetSyncProviderConfigUIInfoforProvider )( 
            __RPC__in ISyncProviderRegistration * This,
            /* [in] */ __RPC__in LPCGUID pguidProviderInstanceId,
            /* [retval][out] */ __RPC__deref_out_opt ISyncProviderConfigUIInfo **ppProviderConfigUIInfo);
        
        DECLSPEC_XFGVIRT(ISyncProviderRegistration, EnumerateSyncProviders)
        HRESULT ( STDMETHODCALLTYPE *EnumerateSyncProviders )( 
            __RPC__in ISyncProviderRegistration * This,
            /* [unique][in] */ __RPC__in_opt LPCGUID pguidContentType,
            /* [in] */ DWORD dwStateFlagsToFilterMask,
            /* [in] */ DWORD dwStateFlagsToFilter,
            /* [in] */ __RPC__in REFCLSID refProviderClsId,
            /* [in] */ DWORD dwSupportedArchitecture,
            /* [retval][out] */ __RPC__deref_out_opt IEnumSyncProviderInfos **ppEnumSyncProviderInfos);
        
        DECLSPEC_XFGVIRT(ISyncProviderRegistration, GetSyncProviderInfo)
        HRESULT ( STDMETHODCALLTYPE *GetSyncProviderInfo )( 
            __RPC__in ISyncProviderRegistration * This,
            /* [in] */ __RPC__in LPCGUID pguidInstanceId,
            /* [retval][out] */ __RPC__deref_out_opt ISyncProviderInfo **ppProviderInfo);
        
        DECLSPEC_XFGVIRT(ISyncProviderRegistration, GetSyncProviderFromInstanceId)
        HRESULT ( STDMETHODCALLTYPE *GetSyncProviderFromInstanceId )( 
            __RPC__in ISyncProviderRegistration * This,
            /* [in] */ __RPC__in LPCGUID pguidInstanceId,
            /* [in] */ DWORD dwClsContext,
            /* [retval][out] */ __RPC__deref_out_opt IRegisteredSyncProvider **ppSyncProvider);
        
        DECLSPEC_XFGVIRT(ISyncProviderRegistration, GetSyncProviderConfigUIInfo)
        HRESULT ( STDMETHODCALLTYPE *GetSyncProviderConfigUIInfo )( 
            __RPC__in ISyncProviderRegistration * This,
            /* [in] */ __RPC__in LPCGUID pguidInstanceId,
            /* [retval][out] */ __RPC__deref_out_opt ISyncProviderConfigUIInfo **ppConfigUIInfo);
        
        DECLSPEC_XFGVIRT(ISyncProviderRegistration, GetSyncProviderConfigUIFromInstanceId)
        HRESULT ( STDMETHODCALLTYPE *GetSyncProviderConfigUIFromInstanceId )( 
            __RPC__in ISyncProviderRegistration * This,
            /* [in] */ __RPC__in LPCGUID pguidInstanceId,
            /* [in] */ DWORD dwClsContext,
            /* [retval][out] */ __RPC__deref_out_opt ISyncProviderConfigUI **ppConfigUI);
        
        DECLSPEC_XFGVIRT(ISyncProviderRegistration, GetSyncProviderState)
        HRESULT ( STDMETHODCALLTYPE *GetSyncProviderState )( 
            __RPC__in ISyncProviderRegistration * This,
            /* [in] */ __RPC__in LPCGUID pguidInstanceId,
            /* [retval][out] */ __RPC__out DWORD *pdwStateFlags);
        
        DECLSPEC_XFGVIRT(ISyncProviderRegistration, SetSyncProviderState)
        HRESULT ( STDMETHODCALLTYPE *SetSyncProviderState )( 
            __RPC__in ISyncProviderRegistration * This,
            /* [in] */ __RPC__in LPCGUID pguidInstanceId,
            /* [in] */ DWORD dwStateFlagsMask,
            /* [in] */ DWORD dwStateFlags);
        
        DECLSPEC_XFGVIRT(ISyncProviderRegistration, RegisterForEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *RegisterForEvent )( 
            ISyncProviderRegistration * This,
            /* [retval][out] */ HANDLE *phEvent);
        
        DECLSPEC_XFGVIRT(ISyncProviderRegistration, RevokeEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *RevokeEvent )( 
            ISyncProviderRegistration * This,
            /* [in] */ HANDLE hEvent);
        
        DECLSPEC_XFGVIRT(ISyncProviderRegistration, GetChange)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetChange )( 
            ISyncProviderRegistration * This,
            /* [in] */ HANDLE hEvent,
            /* [retval][out] */ ISyncRegistrationChange **ppChange);
        
        END_INTERFACE
    } ISyncProviderRegistrationVtbl;

    interface ISyncProviderRegistration
    {
        CONST_VTBL struct ISyncProviderRegistrationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISyncProviderRegistration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISyncProviderRegistration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISyncProviderRegistration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISyncProviderRegistration_CreateSyncProviderConfigUIRegistrationInstance(This,pConfigUIConfig,ppConfigUIInfo)	\
    ( (This)->lpVtbl -> CreateSyncProviderConfigUIRegistrationInstance(This,pConfigUIConfig,ppConfigUIInfo) ) 

#define ISyncProviderRegistration_UnregisterSyncProviderConfigUI(This,pguidInstanceId)	\
    ( (This)->lpVtbl -> UnregisterSyncProviderConfigUI(This,pguidInstanceId) ) 

#define ISyncProviderRegistration_EnumerateSyncProviderConfigUIs(This,pguidContentType,dwSupportedArchitecture,ppEnumSyncProviderConfigUIInfos)	\
    ( (This)->lpVtbl -> EnumerateSyncProviderConfigUIs(This,pguidContentType,dwSupportedArchitecture,ppEnumSyncProviderConfigUIInfos) ) 

#define ISyncProviderRegistration_CreateSyncProviderRegistrationInstance(This,pProviderConfiguration,ppProviderInfo)	\
    ( (This)->lpVtbl -> CreateSyncProviderRegistrationInstance(This,pProviderConfiguration,ppProviderInfo) ) 

#define ISyncProviderRegistration_UnregisterSyncProvider(This,pguidInstanceId)	\
    ( (This)->lpVtbl -> UnregisterSyncProvider(This,pguidInstanceId) ) 

#define ISyncProviderRegistration_GetSyncProviderConfigUIInfoforProvider(This,pguidProviderInstanceId,ppProviderConfigUIInfo)	\
    ( (This)->lpVtbl -> GetSyncProviderConfigUIInfoforProvider(This,pguidProviderInstanceId,ppProviderConfigUIInfo) ) 

#define ISyncProviderRegistration_EnumerateSyncProviders(This,pguidContentType,dwStateFlagsToFilterMask,dwStateFlagsToFilter,refProviderClsId,dwSupportedArchitecture,ppEnumSyncProviderInfos)	\
    ( (This)->lpVtbl -> EnumerateSyncProviders(This,pguidContentType,dwStateFlagsToFilterMask,dwStateFlagsToFilter,refProviderClsId,dwSupportedArchitecture,ppEnumSyncProviderInfos) ) 

#define ISyncProviderRegistration_GetSyncProviderInfo(This,pguidInstanceId,ppProviderInfo)	\
    ( (This)->lpVtbl -> GetSyncProviderInfo(This,pguidInstanceId,ppProviderInfo) ) 

#define ISyncProviderRegistration_GetSyncProviderFromInstanceId(This,pguidInstanceId,dwClsContext,ppSyncProvider)	\
    ( (This)->lpVtbl -> GetSyncProviderFromInstanceId(This,pguidInstanceId,dwClsContext,ppSyncProvider) ) 

#define ISyncProviderRegistration_GetSyncProviderConfigUIInfo(This,pguidInstanceId,ppConfigUIInfo)	\
    ( (This)->lpVtbl -> GetSyncProviderConfigUIInfo(This,pguidInstanceId,ppConfigUIInfo) ) 

#define ISyncProviderRegistration_GetSyncProviderConfigUIFromInstanceId(This,pguidInstanceId,dwClsContext,ppConfigUI)	\
    ( (This)->lpVtbl -> GetSyncProviderConfigUIFromInstanceId(This,pguidInstanceId,dwClsContext,ppConfigUI) ) 

#define ISyncProviderRegistration_GetSyncProviderState(This,pguidInstanceId,pdwStateFlags)	\
    ( (This)->lpVtbl -> GetSyncProviderState(This,pguidInstanceId,pdwStateFlags) ) 

#define ISyncProviderRegistration_SetSyncProviderState(This,pguidInstanceId,dwStateFlagsMask,dwStateFlags)	\
    ( (This)->lpVtbl -> SetSyncProviderState(This,pguidInstanceId,dwStateFlagsMask,dwStateFlags) ) 

#define ISyncProviderRegistration_RegisterForEvent(This,phEvent)	\
    ( (This)->lpVtbl -> RegisterForEvent(This,phEvent) ) 

#define ISyncProviderRegistration_RevokeEvent(This,hEvent)	\
    ( (This)->lpVtbl -> RevokeEvent(This,hEvent) ) 

#define ISyncProviderRegistration_GetChange(This,hEvent,ppChange)	\
    ( (This)->lpVtbl -> GetChange(This,hEvent,ppChange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISyncProviderRegistration_INTERFACE_DEFINED__ */


#ifndef __IEnumSyncProviderConfigUIInfos_INTERFACE_DEFINED__
#define __IEnumSyncProviderConfigUIInfos_INTERFACE_DEFINED__

/* interface IEnumSyncProviderConfigUIInfos */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumSyncProviderConfigUIInfos;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f6be2602-17c6-4658-a2d7-68ed3330f641")
    IEnumSyncProviderConfigUIInfos : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [range][in] */ __RPC__in_range(0,1) ULONG cFactories,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cFactories, *pcFetched) ISyncProviderConfigUIInfo **ppSyncProviderConfigUIInfo,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG cFactories) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumSyncProviderConfigUIInfos **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumSyncProviderConfigUIInfosVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumSyncProviderConfigUIInfos * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumSyncProviderConfigUIInfos * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumSyncProviderConfigUIInfos * This);
        
        DECLSPEC_XFGVIRT(IEnumSyncProviderConfigUIInfos, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumSyncProviderConfigUIInfos * This,
            /* [range][in] */ __RPC__in_range(0,1) ULONG cFactories,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cFactories, *pcFetched) ISyncProviderConfigUIInfo **ppSyncProviderConfigUIInfo,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumSyncProviderConfigUIInfos, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumSyncProviderConfigUIInfos * This,
            /* [in] */ ULONG cFactories);
        
        DECLSPEC_XFGVIRT(IEnumSyncProviderConfigUIInfos, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumSyncProviderConfigUIInfos * This);
        
        DECLSPEC_XFGVIRT(IEnumSyncProviderConfigUIInfos, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumSyncProviderConfigUIInfos * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumSyncProviderConfigUIInfos **ppEnum);
        
        END_INTERFACE
    } IEnumSyncProviderConfigUIInfosVtbl;

    interface IEnumSyncProviderConfigUIInfos
    {
        CONST_VTBL struct IEnumSyncProviderConfigUIInfosVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumSyncProviderConfigUIInfos_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumSyncProviderConfigUIInfos_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumSyncProviderConfigUIInfos_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumSyncProviderConfigUIInfos_Next(This,cFactories,ppSyncProviderConfigUIInfo,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,cFactories,ppSyncProviderConfigUIInfo,pcFetched) ) 

#define IEnumSyncProviderConfigUIInfos_Skip(This,cFactories)	\
    ( (This)->lpVtbl -> Skip(This,cFactories) ) 

#define IEnumSyncProviderConfigUIInfos_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumSyncProviderConfigUIInfos_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumSyncProviderConfigUIInfos_INTERFACE_DEFINED__ */


#ifndef __IEnumSyncProviderInfos_INTERFACE_DEFINED__
#define __IEnumSyncProviderInfos_INTERFACE_DEFINED__

/* interface IEnumSyncProviderInfos */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumSyncProviderInfos;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a04ba850-5eb1-460d-a973-393fcb608a11")
    IEnumSyncProviderInfos : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [range][in] */ __RPC__in_range(0,1) ULONG cInstances,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cInstances, *pcFetched) ISyncProviderInfo **ppSyncProviderInfo,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG cInstances) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumSyncProviderInfos **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumSyncProviderInfosVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumSyncProviderInfos * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumSyncProviderInfos * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumSyncProviderInfos * This);
        
        DECLSPEC_XFGVIRT(IEnumSyncProviderInfos, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumSyncProviderInfos * This,
            /* [range][in] */ __RPC__in_range(0,1) ULONG cInstances,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cInstances, *pcFetched) ISyncProviderInfo **ppSyncProviderInfo,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumSyncProviderInfos, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumSyncProviderInfos * This,
            /* [in] */ ULONG cInstances);
        
        DECLSPEC_XFGVIRT(IEnumSyncProviderInfos, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumSyncProviderInfos * This);
        
        DECLSPEC_XFGVIRT(IEnumSyncProviderInfos, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumSyncProviderInfos * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumSyncProviderInfos **ppEnum);
        
        END_INTERFACE
    } IEnumSyncProviderInfosVtbl;

    interface IEnumSyncProviderInfos
    {
        CONST_VTBL struct IEnumSyncProviderInfosVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumSyncProviderInfos_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumSyncProviderInfos_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumSyncProviderInfos_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumSyncProviderInfos_Next(This,cInstances,ppSyncProviderInfo,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,cInstances,ppSyncProviderInfo,pcFetched) ) 

#define IEnumSyncProviderInfos_Skip(This,cInstances)	\
    ( (This)->lpVtbl -> Skip(This,cInstances) ) 

#define IEnumSyncProviderInfos_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumSyncProviderInfos_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumSyncProviderInfos_INTERFACE_DEFINED__ */


#ifndef __ISyncProviderInfo_INTERFACE_DEFINED__
#define __ISyncProviderInfo_INTERFACE_DEFINED__

/* interface ISyncProviderInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISyncProviderInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1ee135de-88a4-4504-b0d0-f7920d7e5ba6")
    ISyncProviderInfo : public IPropertyStore
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSyncProvider( 
            /* [in] */ DWORD dwClsContext,
            /* [retval][out] */ __RPC__deref_out_opt IRegisteredSyncProvider **ppSyncProvider) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISyncProviderInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISyncProviderInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISyncProviderInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISyncProviderInfo * This);
        
        DECLSPEC_XFGVIRT(IPropertyStore, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ISyncProviderInfo * This,
            /* [out] */ __RPC__out DWORD *cProps);
        
        DECLSPEC_XFGVIRT(IPropertyStore, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in ISyncProviderInfo * This,
            /* [in] */ DWORD iProp,
            /* [out] */ __RPC__out PROPERTYKEY *pkey);
        
        DECLSPEC_XFGVIRT(IPropertyStore, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in ISyncProviderInfo * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [out] */ __RPC__out PROPVARIANT *pv);
        
        DECLSPEC_XFGVIRT(IPropertyStore, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in ISyncProviderInfo * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [in] */ __RPC__in REFPROPVARIANT propvar);
        
        DECLSPEC_XFGVIRT(IPropertyStore, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in ISyncProviderInfo * This);
        
        DECLSPEC_XFGVIRT(ISyncProviderInfo, GetSyncProvider)
        HRESULT ( STDMETHODCALLTYPE *GetSyncProvider )( 
            __RPC__in ISyncProviderInfo * This,
            /* [in] */ DWORD dwClsContext,
            /* [retval][out] */ __RPC__deref_out_opt IRegisteredSyncProvider **ppSyncProvider);
        
        END_INTERFACE
    } ISyncProviderInfoVtbl;

    interface ISyncProviderInfo
    {
        CONST_VTBL struct ISyncProviderInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISyncProviderInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISyncProviderInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISyncProviderInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISyncProviderInfo_GetCount(This,cProps)	\
    ( (This)->lpVtbl -> GetCount(This,cProps) ) 

#define ISyncProviderInfo_GetAt(This,iProp,pkey)	\
    ( (This)->lpVtbl -> GetAt(This,iProp,pkey) ) 

#define ISyncProviderInfo_GetValue(This,key,pv)	\
    ( (This)->lpVtbl -> GetValue(This,key,pv) ) 

#define ISyncProviderInfo_SetValue(This,key,propvar)	\
    ( (This)->lpVtbl -> SetValue(This,key,propvar) ) 

#define ISyncProviderInfo_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 


#define ISyncProviderInfo_GetSyncProvider(This,dwClsContext,ppSyncProvider)	\
    ( (This)->lpVtbl -> GetSyncProvider(This,dwClsContext,ppSyncProvider) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISyncProviderInfo_INTERFACE_DEFINED__ */


#ifndef __ISyncProviderConfigUIInfo_INTERFACE_DEFINED__
#define __ISyncProviderConfigUIInfo_INTERFACE_DEFINED__

/* interface ISyncProviderConfigUIInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISyncProviderConfigUIInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("214141ae-33d7-4d8d-8e37-f227e880ce50")
    ISyncProviderConfigUIInfo : public IPropertyStore
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSyncProviderConfigUI( 
            /* [in] */ DWORD dwClsContext,
            /* [retval][out] */ __RPC__deref_out_opt ISyncProviderConfigUI **ppSyncProviderConfigUI) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISyncProviderConfigUIInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISyncProviderConfigUIInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISyncProviderConfigUIInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISyncProviderConfigUIInfo * This);
        
        DECLSPEC_XFGVIRT(IPropertyStore, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ISyncProviderConfigUIInfo * This,
            /* [out] */ __RPC__out DWORD *cProps);
        
        DECLSPEC_XFGVIRT(IPropertyStore, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in ISyncProviderConfigUIInfo * This,
            /* [in] */ DWORD iProp,
            /* [out] */ __RPC__out PROPERTYKEY *pkey);
        
        DECLSPEC_XFGVIRT(IPropertyStore, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in ISyncProviderConfigUIInfo * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [out] */ __RPC__out PROPVARIANT *pv);
        
        DECLSPEC_XFGVIRT(IPropertyStore, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in ISyncProviderConfigUIInfo * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [in] */ __RPC__in REFPROPVARIANT propvar);
        
        DECLSPEC_XFGVIRT(IPropertyStore, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in ISyncProviderConfigUIInfo * This);
        
        DECLSPEC_XFGVIRT(ISyncProviderConfigUIInfo, GetSyncProviderConfigUI)
        HRESULT ( STDMETHODCALLTYPE *GetSyncProviderConfigUI )( 
            __RPC__in ISyncProviderConfigUIInfo * This,
            /* [in] */ DWORD dwClsContext,
            /* [retval][out] */ __RPC__deref_out_opt ISyncProviderConfigUI **ppSyncProviderConfigUI);
        
        END_INTERFACE
    } ISyncProviderConfigUIInfoVtbl;

    interface ISyncProviderConfigUIInfo
    {
        CONST_VTBL struct ISyncProviderConfigUIInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISyncProviderConfigUIInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISyncProviderConfigUIInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISyncProviderConfigUIInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISyncProviderConfigUIInfo_GetCount(This,cProps)	\
    ( (This)->lpVtbl -> GetCount(This,cProps) ) 

#define ISyncProviderConfigUIInfo_GetAt(This,iProp,pkey)	\
    ( (This)->lpVtbl -> GetAt(This,iProp,pkey) ) 

#define ISyncProviderConfigUIInfo_GetValue(This,key,pv)	\
    ( (This)->lpVtbl -> GetValue(This,key,pv) ) 

#define ISyncProviderConfigUIInfo_SetValue(This,key,propvar)	\
    ( (This)->lpVtbl -> SetValue(This,key,propvar) ) 

#define ISyncProviderConfigUIInfo_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 


#define ISyncProviderConfigUIInfo_GetSyncProviderConfigUI(This,dwClsContext,ppSyncProviderConfigUI)	\
    ( (This)->lpVtbl -> GetSyncProviderConfigUI(This,dwClsContext,ppSyncProviderConfigUI) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISyncProviderConfigUIInfo_INTERFACE_DEFINED__ */


#ifndef __ISyncProviderConfigUI_INTERFACE_DEFINED__
#define __ISyncProviderConfigUI_INTERFACE_DEFINED__

/* interface ISyncProviderConfigUI */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISyncProviderConfigUI;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7b0705f6-cbcd-4071-ab05-3bdc364d4a0c")
    ISyncProviderConfigUI : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Init( 
            /* [in] */ __RPC__in LPCGUID pguidInstanceId,
            /* [in] */ __RPC__in LPCGUID pguidContentType,
            /* [in] */ __RPC__in_opt IPropertyStore *pConfigurationProperties) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRegisteredProperties( 
            /* [retval][out] */ __RPC__deref_out_opt IPropertyStore **ppConfigUIProperties) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateAndRegisterNewSyncProvider( 
            /* [unique][in] */ __RPC__in_opt HWND hwndParent,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkContext,
            /* [retval][out] */ __RPC__deref_out_opt ISyncProviderInfo **ppProviderInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ModifySyncProvider( 
            /* [unique][in] */ __RPC__in_opt HWND hwndParent,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkContext,
            /* [in] */ __RPC__in_opt ISyncProviderInfo *pProviderInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISyncProviderConfigUIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISyncProviderConfigUI * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISyncProviderConfigUI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISyncProviderConfigUI * This);
        
        DECLSPEC_XFGVIRT(ISyncProviderConfigUI, Init)
        HRESULT ( STDMETHODCALLTYPE *Init )( 
            __RPC__in ISyncProviderConfigUI * This,
            /* [in] */ __RPC__in LPCGUID pguidInstanceId,
            /* [in] */ __RPC__in LPCGUID pguidContentType,
            /* [in] */ __RPC__in_opt IPropertyStore *pConfigurationProperties);
        
        DECLSPEC_XFGVIRT(ISyncProviderConfigUI, GetRegisteredProperties)
        HRESULT ( STDMETHODCALLTYPE *GetRegisteredProperties )( 
            __RPC__in ISyncProviderConfigUI * This,
            /* [retval][out] */ __RPC__deref_out_opt IPropertyStore **ppConfigUIProperties);
        
        DECLSPEC_XFGVIRT(ISyncProviderConfigUI, CreateAndRegisterNewSyncProvider)
        HRESULT ( STDMETHODCALLTYPE *CreateAndRegisterNewSyncProvider )( 
            __RPC__in ISyncProviderConfigUI * This,
            /* [unique][in] */ __RPC__in_opt HWND hwndParent,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkContext,
            /* [retval][out] */ __RPC__deref_out_opt ISyncProviderInfo **ppProviderInfo);
        
        DECLSPEC_XFGVIRT(ISyncProviderConfigUI, ModifySyncProvider)
        HRESULT ( STDMETHODCALLTYPE *ModifySyncProvider )( 
            __RPC__in ISyncProviderConfigUI * This,
            /* [unique][in] */ __RPC__in_opt HWND hwndParent,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkContext,
            /* [in] */ __RPC__in_opt ISyncProviderInfo *pProviderInfo);
        
        END_INTERFACE
    } ISyncProviderConfigUIVtbl;

    interface ISyncProviderConfigUI
    {
        CONST_VTBL struct ISyncProviderConfigUIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISyncProviderConfigUI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISyncProviderConfigUI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISyncProviderConfigUI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISyncProviderConfigUI_Init(This,pguidInstanceId,pguidContentType,pConfigurationProperties)	\
    ( (This)->lpVtbl -> Init(This,pguidInstanceId,pguidContentType,pConfigurationProperties) ) 

#define ISyncProviderConfigUI_GetRegisteredProperties(This,ppConfigUIProperties)	\
    ( (This)->lpVtbl -> GetRegisteredProperties(This,ppConfigUIProperties) ) 

#define ISyncProviderConfigUI_CreateAndRegisterNewSyncProvider(This,hwndParent,pUnkContext,ppProviderInfo)	\
    ( (This)->lpVtbl -> CreateAndRegisterNewSyncProvider(This,hwndParent,pUnkContext,ppProviderInfo) ) 

#define ISyncProviderConfigUI_ModifySyncProvider(This,hwndParent,pUnkContext,pProviderInfo)	\
    ( (This)->lpVtbl -> ModifySyncProvider(This,hwndParent,pUnkContext,pProviderInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISyncProviderConfigUI_INTERFACE_DEFINED__ */


#ifndef __IRegisteredSyncProvider_INTERFACE_DEFINED__
#define __IRegisteredSyncProvider_INTERFACE_DEFINED__

/* interface IRegisteredSyncProvider */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IRegisteredSyncProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("913bcf76-47c1-40b5-a896-5e8a9c414c14")
    IRegisteredSyncProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Init( 
            /* [in] */ __RPC__in LPCGUID pguidInstanceId,
            /* [in] */ __RPC__in LPCGUID pguidContentType,
            /* [in] */ __RPC__in_opt IPropertyStore *pContextPropertyStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInstanceId( 
            /* [retval][out] */ __RPC__out GUID *pguidInstanceId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRegisteredSyncProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRegisteredSyncProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRegisteredSyncProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRegisteredSyncProvider * This);
        
        DECLSPEC_XFGVIRT(IRegisteredSyncProvider, Init)
        HRESULT ( STDMETHODCALLTYPE *Init )( 
            __RPC__in IRegisteredSyncProvider * This,
            /* [in] */ __RPC__in LPCGUID pguidInstanceId,
            /* [in] */ __RPC__in LPCGUID pguidContentType,
            /* [in] */ __RPC__in_opt IPropertyStore *pContextPropertyStore);
        
        DECLSPEC_XFGVIRT(IRegisteredSyncProvider, GetInstanceId)
        HRESULT ( STDMETHODCALLTYPE *GetInstanceId )( 
            __RPC__in IRegisteredSyncProvider * This,
            /* [retval][out] */ __RPC__out GUID *pguidInstanceId);
        
        DECLSPEC_XFGVIRT(IRegisteredSyncProvider, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IRegisteredSyncProvider * This);
        
        END_INTERFACE
    } IRegisteredSyncProviderVtbl;

    interface IRegisteredSyncProvider
    {
        CONST_VTBL struct IRegisteredSyncProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRegisteredSyncProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRegisteredSyncProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRegisteredSyncProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRegisteredSyncProvider_Init(This,pguidInstanceId,pguidContentType,pContextPropertyStore)	\
    ( (This)->lpVtbl -> Init(This,pguidInstanceId,pguidContentType,pContextPropertyStore) ) 

#define IRegisteredSyncProvider_GetInstanceId(This,pguidInstanceId)	\
    ( (This)->lpVtbl -> GetInstanceId(This,pguidInstanceId) ) 

#define IRegisteredSyncProvider_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRegisteredSyncProvider_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_syncregistration_0000_0007 */
/* [local] */ 

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_syncregistration_0000_0007_0001
    {
        SRE_PROVIDER_ADDED	= 0,
        SRE_PROVIDER_REMOVED	= ( SRE_PROVIDER_ADDED + 1 ) ,
        SRE_PROVIDER_UPDATED	= ( SRE_PROVIDER_REMOVED + 1 ) ,
        SRE_PROVIDER_STATE_CHANGED	= ( SRE_PROVIDER_UPDATED + 1 ) ,
        SRE_CONFIGUI_ADDED	= ( SRE_PROVIDER_STATE_CHANGED + 1 ) ,
        SRE_CONFIGUI_REMOVED	= ( SRE_CONFIGUI_ADDED + 1 ) ,
        SRE_CONFIGUI_UPDATED	= ( SRE_CONFIGUI_REMOVED + 1 ) 
    } 	SYNC_REGISTRATION_EVENT;



extern RPC_IF_HANDLE __MIDL_itf_syncregistration_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_syncregistration_0000_0007_v0_0_s_ifspec;

#ifndef __ISyncRegistrationChange_INTERFACE_DEFINED__
#define __ISyncRegistrationChange_INTERFACE_DEFINED__

/* interface ISyncRegistrationChange */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISyncRegistrationChange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eea0d9ae-6b29-43b4-9e70-e3ae33bb2c3b")
    ISyncRegistrationChange : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetEvent( 
            /* [retval][out] */ __RPC__out SYNC_REGISTRATION_EVENT *psreEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInstanceId( 
            /* [retval][out] */ __RPC__out LPGUID pguidInstanceId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISyncRegistrationChangeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISyncRegistrationChange * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISyncRegistrationChange * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISyncRegistrationChange * This);
        
        DECLSPEC_XFGVIRT(ISyncRegistrationChange, GetEvent)
        HRESULT ( STDMETHODCALLTYPE *GetEvent )( 
            __RPC__in ISyncRegistrationChange * This,
            /* [retval][out] */ __RPC__out SYNC_REGISTRATION_EVENT *psreEvent);
        
        DECLSPEC_XFGVIRT(ISyncRegistrationChange, GetInstanceId)
        HRESULT ( STDMETHODCALLTYPE *GetInstanceId )( 
            __RPC__in ISyncRegistrationChange * This,
            /* [retval][out] */ __RPC__out LPGUID pguidInstanceId);
        
        END_INTERFACE
    } ISyncRegistrationChangeVtbl;

    interface ISyncRegistrationChange
    {
        CONST_VTBL struct ISyncRegistrationChangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISyncRegistrationChange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISyncRegistrationChange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISyncRegistrationChange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISyncRegistrationChange_GetEvent(This,psreEvent)	\
    ( (This)->lpVtbl -> GetEvent(This,psreEvent) ) 

#define ISyncRegistrationChange_GetInstanceId(This,pguidInstanceId)	\
    ( (This)->lpVtbl -> GetInstanceId(This,pguidInstanceId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISyncRegistrationChange_INTERFACE_DEFINED__ */



#ifndef __SyncRegistration_LIBRARY_DEFINED__
#define __SyncRegistration_LIBRARY_DEFINED__

/* library SyncRegistration */
/* [helpstring][version][uuid] */ 









EXTERN_C const IID LIBID_SyncRegistration;

EXTERN_C const CLSID CLSID_SyncProviderRegistration;

#ifdef __cplusplus

class DECLSPEC_UUID("F82B4EF1-93A9-4dde-8015-F7950A1A6E31")
SyncProviderRegistration;
#endif
#endif /* __SyncRegistration_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_syncregistration_0000_0009 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_syncregistration_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_syncregistration_0000_0009_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


