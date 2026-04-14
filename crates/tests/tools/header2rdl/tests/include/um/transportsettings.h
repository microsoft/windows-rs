

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
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

#ifndef __transportsettings_h__
#define __transportsettings_h__

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

#ifndef __ITransportSettingsInternal_FWD_DEFINED__
#define __ITransportSettingsInternal_FWD_DEFINED__
typedef interface ITransportSettingsInternal ITransportSettingsInternal;

#endif 	/* __ITransportSettingsInternal_FWD_DEFINED__ */


#ifndef __INetworkTransportSettings_FWD_DEFINED__
#define __INetworkTransportSettings_FWD_DEFINED__
typedef interface INetworkTransportSettings INetworkTransportSettings;

#endif 	/* __INetworkTransportSettings_FWD_DEFINED__ */


#ifndef __INotificationTransportSync_FWD_DEFINED__
#define __INotificationTransportSync_FWD_DEFINED__
typedef interface INotificationTransportSync INotificationTransportSync;

#endif 	/* __INotificationTransportSync_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "transportsettingcommon.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_transportsettings_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef struct TRANSPORT_SETTING
    {
    TRANSPORT_SETTING_ID SettingId;
    ULONG *Length;
    /* [size_is] */ BYTE *Value;
    } 	TRANSPORT_SETTING;



extern RPC_IF_HANDLE __MIDL_itf_transportsettings_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_transportsettings_0000_0000_v0_0_s_ifspec;

#ifndef __ITransportSettingsInternal_INTERFACE_DEFINED__
#define __ITransportSettingsInternal_INTERFACE_DEFINED__

/* interface ITransportSettingsInternal */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_ITransportSettingsInternal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5123e076-29e3-4bfd-84fe-0192d411e3e8")
    ITransportSettingsInternal : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ApplySetting( 
            /* [out][in] */ TRANSPORT_SETTING *Setting) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QuerySetting( 
            /* [out][in] */ TRANSPORT_SETTING *Setting) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransportSettingsInternalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITransportSettingsInternal * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITransportSettingsInternal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITransportSettingsInternal * This);
        
        DECLSPEC_XFGVIRT(ITransportSettingsInternal, ApplySetting)
        HRESULT ( STDMETHODCALLTYPE *ApplySetting )( 
            ITransportSettingsInternal * This,
            /* [out][in] */ TRANSPORT_SETTING *Setting);
        
        DECLSPEC_XFGVIRT(ITransportSettingsInternal, QuerySetting)
        HRESULT ( STDMETHODCALLTYPE *QuerySetting )( 
            ITransportSettingsInternal * This,
            /* [out][in] */ TRANSPORT_SETTING *Setting);
        
        END_INTERFACE
    } ITransportSettingsInternalVtbl;

    interface ITransportSettingsInternal
    {
        CONST_VTBL struct ITransportSettingsInternalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransportSettingsInternal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransportSettingsInternal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransportSettingsInternal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransportSettingsInternal_ApplySetting(This,Setting)	\
    ( (This)->lpVtbl -> ApplySetting(This,Setting) ) 

#define ITransportSettingsInternal_QuerySetting(This,Setting)	\
    ( (This)->lpVtbl -> QuerySetting(This,Setting) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransportSettingsInternal_INTERFACE_DEFINED__ */


#ifndef __INetworkTransportSettings_INTERFACE_DEFINED__
#define __INetworkTransportSettings_INTERFACE_DEFINED__

/* interface INetworkTransportSettings */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_INetworkTransportSettings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5e7abb2c-f2c1-4a61-bd35-deb7a08ab0f1")
    INetworkTransportSettings : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ApplySetting( 
            /* [in] */ __RPC__in const TRANSPORT_SETTING_ID *SettingId,
            /* [in] */ ULONG LengthIn,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(LengthIn) const BYTE *ValueIn,
            /* [out] */ __RPC__out ULONG *LengthOut,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*LengthOut) BYTE **ValueOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QuerySetting( 
            /* [in] */ __RPC__in const TRANSPORT_SETTING_ID *SettingId,
            /* [in] */ ULONG LengthIn,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(LengthIn) const BYTE *ValueIn,
            /* [out] */ __RPC__out ULONG *LengthOut,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*LengthOut) BYTE **ValueOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetworkTransportSettingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INetworkTransportSettings * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INetworkTransportSettings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INetworkTransportSettings * This);
        
        DECLSPEC_XFGVIRT(INetworkTransportSettings, ApplySetting)
        HRESULT ( STDMETHODCALLTYPE *ApplySetting )( 
            __RPC__in INetworkTransportSettings * This,
            /* [in] */ __RPC__in const TRANSPORT_SETTING_ID *SettingId,
            /* [in] */ ULONG LengthIn,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(LengthIn) const BYTE *ValueIn,
            /* [out] */ __RPC__out ULONG *LengthOut,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*LengthOut) BYTE **ValueOut);
        
        DECLSPEC_XFGVIRT(INetworkTransportSettings, QuerySetting)
        HRESULT ( STDMETHODCALLTYPE *QuerySetting )( 
            __RPC__in INetworkTransportSettings * This,
            /* [in] */ __RPC__in const TRANSPORT_SETTING_ID *SettingId,
            /* [in] */ ULONG LengthIn,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(LengthIn) const BYTE *ValueIn,
            /* [out] */ __RPC__out ULONG *LengthOut,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*LengthOut) BYTE **ValueOut);
        
        END_INTERFACE
    } INetworkTransportSettingsVtbl;

    interface INetworkTransportSettings
    {
        CONST_VTBL struct INetworkTransportSettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetworkTransportSettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetworkTransportSettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetworkTransportSettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetworkTransportSettings_ApplySetting(This,SettingId,LengthIn,ValueIn,LengthOut,ValueOut)	\
    ( (This)->lpVtbl -> ApplySetting(This,SettingId,LengthIn,ValueIn,LengthOut,ValueOut) ) 

#define INetworkTransportSettings_QuerySetting(This,SettingId,LengthIn,ValueIn,LengthOut,ValueOut)	\
    ( (This)->lpVtbl -> QuerySetting(This,SettingId,LengthIn,ValueIn,LengthOut,ValueOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetworkTransportSettings_INTERFACE_DEFINED__ */


#ifndef __INotificationTransportSync_INTERFACE_DEFINED__
#define __INotificationTransportSync_INTERFACE_DEFINED__

/* interface INotificationTransportSync */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_INotificationTransportSync;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eb1402-0ab8-49c0-9e14-a1ae4ba93058")
    INotificationTransportSync : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CompleteDelivery( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Flush( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INotificationTransportSyncVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INotificationTransportSync * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INotificationTransportSync * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INotificationTransportSync * This);
        
        DECLSPEC_XFGVIRT(INotificationTransportSync, CompleteDelivery)
        HRESULT ( STDMETHODCALLTYPE *CompleteDelivery )( 
            __RPC__in INotificationTransportSync * This);
        
        DECLSPEC_XFGVIRT(INotificationTransportSync, Flush)
        HRESULT ( STDMETHODCALLTYPE *Flush )( 
            __RPC__in INotificationTransportSync * This);
        
        END_INTERFACE
    } INotificationTransportSyncVtbl;

    interface INotificationTransportSync
    {
        CONST_VTBL struct INotificationTransportSyncVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INotificationTransportSync_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INotificationTransportSync_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INotificationTransportSync_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INotificationTransportSync_CompleteDelivery(This)	\
    ( (This)->lpVtbl -> CompleteDelivery(This) ) 

#define INotificationTransportSync_Flush(This)	\
    ( (This)->lpVtbl -> Flush(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INotificationTransportSync_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_transportsettings_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_transportsettings_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_transportsettings_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


