

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

#ifndef __netcfgn_h__
#define __netcfgn_h__

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

#ifndef __INetCfgPnpReconfigCallback_FWD_DEFINED__
#define __INetCfgPnpReconfigCallback_FWD_DEFINED__
typedef interface INetCfgPnpReconfigCallback INetCfgPnpReconfigCallback;

#endif 	/* __INetCfgPnpReconfigCallback_FWD_DEFINED__ */


#ifndef __INetCfgComponentControl_FWD_DEFINED__
#define __INetCfgComponentControl_FWD_DEFINED__
typedef interface INetCfgComponentControl INetCfgComponentControl;

#endif 	/* __INetCfgComponentControl_FWD_DEFINED__ */


#ifndef __INetCfgComponentSetup_FWD_DEFINED__
#define __INetCfgComponentSetup_FWD_DEFINED__
typedef interface INetCfgComponentSetup INetCfgComponentSetup;

#endif 	/* __INetCfgComponentSetup_FWD_DEFINED__ */


#ifndef __INetCfgComponentPropertyUi_FWD_DEFINED__
#define __INetCfgComponentPropertyUi_FWD_DEFINED__
typedef interface INetCfgComponentPropertyUi INetCfgComponentPropertyUi;

#endif 	/* __INetCfgComponentPropertyUi_FWD_DEFINED__ */


#ifndef __INetCfgComponentNotifyBinding_FWD_DEFINED__
#define __INetCfgComponentNotifyBinding_FWD_DEFINED__
typedef interface INetCfgComponentNotifyBinding INetCfgComponentNotifyBinding;

#endif 	/* __INetCfgComponentNotifyBinding_FWD_DEFINED__ */


#ifndef __INetCfgComponentNotifyGlobal_FWD_DEFINED__
#define __INetCfgComponentNotifyGlobal_FWD_DEFINED__
typedef interface INetCfgComponentNotifyGlobal INetCfgComponentNotifyGlobal;

#endif 	/* __INetCfgComponentNotifyGlobal_FWD_DEFINED__ */


#ifndef __INetCfgComponentUpperEdge_FWD_DEFINED__
#define __INetCfgComponentUpperEdge_FWD_DEFINED__
typedef interface INetCfgComponentUpperEdge INetCfgComponentUpperEdge;

#endif 	/* __INetCfgComponentUpperEdge_FWD_DEFINED__ */


#ifndef __INetLanConnectionUiInfo_FWD_DEFINED__
#define __INetLanConnectionUiInfo_FWD_DEFINED__
typedef interface INetLanConnectionUiInfo INetLanConnectionUiInfo;

#endif 	/* __INetLanConnectionUiInfo_FWD_DEFINED__ */


#ifndef __INetRasConnectionIpUiInfo_FWD_DEFINED__
#define __INetRasConnectionIpUiInfo_FWD_DEFINED__
typedef interface INetRasConnectionIpUiInfo INetRasConnectionIpUiInfo;

#endif 	/* __INetRasConnectionIpUiInfo_FWD_DEFINED__ */


#ifndef __INetCfgComponentSysPrep_FWD_DEFINED__
#define __INetCfgComponentSysPrep_FWD_DEFINED__
typedef interface INetCfgComponentSysPrep INetCfgComponentSysPrep;

#endif 	/* __INetCfgComponentSysPrep_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "netcfgx.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_netcfgn_0000_0000 */
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
#if ( _MSC_VER >= 800 )
#pragma warning(disable:4201)
#endif


extern RPC_IF_HANDLE __MIDL_itf_netcfgn_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_netcfgn_0000_0000_v0_0_s_ifspec;

#ifndef __INetCfgPnpReconfigCallback_INTERFACE_DEFINED__
#define __INetCfgPnpReconfigCallback_INTERFACE_DEFINED__

/* interface INetCfgPnpReconfigCallback */
/* [unique][uuid][object][local] */ 

typedef /* [v1_enum] */ 
enum tagNCPNP_RECONFIG_LAYER
    {
        NCRL_NDIS	= 1,
        NCRL_TDI	= 2
    } 	NCPNP_RECONFIG_LAYER;


EXTERN_C const IID IID_INetCfgPnpReconfigCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8d84bd35-e227-11d2-b700-00a0c98a6a85")
    INetCfgPnpReconfigCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SendPnpReconfig( 
            /* [annotation][in] */ 
            _In_  NCPNP_RECONFIG_LAYER Layer,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszwUpper,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszwLower,
            /* [annotation][in] */ 
            _In_reads_bytes_(dwSizeOfData)  PVOID pvData,
            /* [annotation][in] */ 
            _In_  DWORD dwSizeOfData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgPnpReconfigCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfgPnpReconfigCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfgPnpReconfigCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfgPnpReconfigCallback * This);
        
        DECLSPEC_XFGVIRT(INetCfgPnpReconfigCallback, SendPnpReconfig)
        HRESULT ( STDMETHODCALLTYPE *SendPnpReconfig )( 
            INetCfgPnpReconfigCallback * This,
            /* [annotation][in] */ 
            _In_  NCPNP_RECONFIG_LAYER Layer,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszwUpper,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszwLower,
            /* [annotation][in] */ 
            _In_reads_bytes_(dwSizeOfData)  PVOID pvData,
            /* [annotation][in] */ 
            _In_  DWORD dwSizeOfData);
        
        END_INTERFACE
    } INetCfgPnpReconfigCallbackVtbl;

    interface INetCfgPnpReconfigCallback
    {
        CONST_VTBL struct INetCfgPnpReconfigCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfgPnpReconfigCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfgPnpReconfigCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfgPnpReconfigCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfgPnpReconfigCallback_SendPnpReconfig(This,Layer,pszwUpper,pszwLower,pvData,dwSizeOfData)	\
    ( (This)->lpVtbl -> SendPnpReconfig(This,Layer,pszwUpper,pszwLower,pvData,dwSizeOfData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfgPnpReconfigCallback_INTERFACE_DEFINED__ */


#ifndef __INetCfgComponentControl_INTERFACE_DEFINED__
#define __INetCfgComponentControl_INTERFACE_DEFINED__

/* interface INetCfgComponentControl */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_INetCfgComponentControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("932238df-bea1-11d0-9298-00c04fc99dcf")
    INetCfgComponentControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pIComp,
            /* [annotation][in] */ 
            _In_  INetCfg *pINetCfg,
            /* [annotation][in] */ 
            _In_  BOOL fInstalling) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ApplyRegistryChanges( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ApplyPnpChanges( 
            /* [annotation][in] */ 
            _In_  INetCfgPnpReconfigCallback *pICallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelChanges( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgComponentControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfgComponentControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfgComponentControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfgComponentControl * This);
        
        DECLSPEC_XFGVIRT(INetCfgComponentControl, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            INetCfgComponentControl * This,
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pIComp,
            /* [annotation][in] */ 
            _In_  INetCfg *pINetCfg,
            /* [annotation][in] */ 
            _In_  BOOL fInstalling);
        
        DECLSPEC_XFGVIRT(INetCfgComponentControl, ApplyRegistryChanges)
        HRESULT ( STDMETHODCALLTYPE *ApplyRegistryChanges )( 
            INetCfgComponentControl * This);
        
        DECLSPEC_XFGVIRT(INetCfgComponentControl, ApplyPnpChanges)
        HRESULT ( STDMETHODCALLTYPE *ApplyPnpChanges )( 
            INetCfgComponentControl * This,
            /* [annotation][in] */ 
            _In_  INetCfgPnpReconfigCallback *pICallback);
        
        DECLSPEC_XFGVIRT(INetCfgComponentControl, CancelChanges)
        HRESULT ( STDMETHODCALLTYPE *CancelChanges )( 
            INetCfgComponentControl * This);
        
        END_INTERFACE
    } INetCfgComponentControlVtbl;

    interface INetCfgComponentControl
    {
        CONST_VTBL struct INetCfgComponentControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfgComponentControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfgComponentControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfgComponentControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfgComponentControl_Initialize(This,pIComp,pINetCfg,fInstalling)	\
    ( (This)->lpVtbl -> Initialize(This,pIComp,pINetCfg,fInstalling) ) 

#define INetCfgComponentControl_ApplyRegistryChanges(This)	\
    ( (This)->lpVtbl -> ApplyRegistryChanges(This) ) 

#define INetCfgComponentControl_ApplyPnpChanges(This,pICallback)	\
    ( (This)->lpVtbl -> ApplyPnpChanges(This,pICallback) ) 

#define INetCfgComponentControl_CancelChanges(This)	\
    ( (This)->lpVtbl -> CancelChanges(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfgComponentControl_INTERFACE_DEFINED__ */


#ifndef __INetCfgComponentSetup_INTERFACE_DEFINED__
#define __INetCfgComponentSetup_INTERFACE_DEFINED__

/* interface INetCfgComponentSetup */
/* [unique][uuid][object][local] */ 

typedef /* [v1_enum] */ 
enum tagNETWORK_INSTALL_TIME
    {
        NSF_PRIMARYINSTALL	= 0x1,
        NSF_POSTSYSINSTALL	= 0x2
    } 	NETWORK_INSTALL_TIME;

typedef /* [v1_enum] */ 
enum tagNETWORK_UPGRADE_TYPE
    {
        NSF_WIN16_UPGRADE	= 0x10,
        NSF_WIN95_UPGRADE	= 0x20,
        NSF_WINNT_WKS_UPGRADE	= 0x40,
        NSF_WINNT_SVR_UPGRADE	= 0x80,
        NSF_WINNT_SBS_UPGRADE	= 0x100,
        NSF_COMPONENT_UPDATE	= 0x200
    } 	NETWORK_UPGRADE_TYPE;


EXTERN_C const IID IID_INetCfgComponentSetup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("932238e3-bea1-11d0-9298-00c04fc99dcf")
    INetCfgComponentSetup : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Install( 
            /* [annotation][in] */ 
            _In_  DWORD dwSetupFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Upgrade( 
            /* [annotation][in] */ 
            _In_  DWORD dwSetupFlags,
            /* [annotation][in] */ 
            _In_  DWORD dwUpgradeFomBuildNo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReadAnswerFile( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszwAnswerFile,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszwAnswerSections) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Removing( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgComponentSetupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfgComponentSetup * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfgComponentSetup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfgComponentSetup * This);
        
        DECLSPEC_XFGVIRT(INetCfgComponentSetup, Install)
        HRESULT ( STDMETHODCALLTYPE *Install )( 
            INetCfgComponentSetup * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSetupFlags);
        
        DECLSPEC_XFGVIRT(INetCfgComponentSetup, Upgrade)
        HRESULT ( STDMETHODCALLTYPE *Upgrade )( 
            INetCfgComponentSetup * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSetupFlags,
            /* [annotation][in] */ 
            _In_  DWORD dwUpgradeFomBuildNo);
        
        DECLSPEC_XFGVIRT(INetCfgComponentSetup, ReadAnswerFile)
        HRESULT ( STDMETHODCALLTYPE *ReadAnswerFile )( 
            INetCfgComponentSetup * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszwAnswerFile,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszwAnswerSections);
        
        DECLSPEC_XFGVIRT(INetCfgComponentSetup, Removing)
        HRESULT ( STDMETHODCALLTYPE *Removing )( 
            INetCfgComponentSetup * This);
        
        END_INTERFACE
    } INetCfgComponentSetupVtbl;

    interface INetCfgComponentSetup
    {
        CONST_VTBL struct INetCfgComponentSetupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfgComponentSetup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfgComponentSetup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfgComponentSetup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfgComponentSetup_Install(This,dwSetupFlags)	\
    ( (This)->lpVtbl -> Install(This,dwSetupFlags) ) 

#define INetCfgComponentSetup_Upgrade(This,dwSetupFlags,dwUpgradeFomBuildNo)	\
    ( (This)->lpVtbl -> Upgrade(This,dwSetupFlags,dwUpgradeFomBuildNo) ) 

#define INetCfgComponentSetup_ReadAnswerFile(This,pszwAnswerFile,pszwAnswerSections)	\
    ( (This)->lpVtbl -> ReadAnswerFile(This,pszwAnswerFile,pszwAnswerSections) ) 

#define INetCfgComponentSetup_Removing(This)	\
    ( (This)->lpVtbl -> Removing(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfgComponentSetup_INTERFACE_DEFINED__ */


#ifndef __INetCfgComponentPropertyUi_INTERFACE_DEFINED__
#define __INetCfgComponentPropertyUi_INTERFACE_DEFINED__

/* interface INetCfgComponentPropertyUi */
/* [unique][uuid][object][local] */ 

typedef /* [v1_enum] */ 
enum tagDEFAULT_PAGES
    {
        DPP_ADVANCED	= 1
    } 	DEFAULT_PAGES;


EXTERN_C const IID IID_INetCfgComponentPropertyUi;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("932238e0-bea1-11d0-9298-00c04fc99dcf")
    INetCfgComponentPropertyUi : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueryPropertyUi( 
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetContext( 
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MergePropPages( 
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pdwDefPages,
            /* [annotation][out] */ 
            _Out_  BYTE **pahpspPrivate,
            /* [annotation][out] */ 
            _Out_  UINT *pcPages,
            /* [annotation][in] */ 
            _In_  HWND hwndParent,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR *pszStartPage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ValidateProperties( 
            /* [annotation][in] */ 
            _In_  HWND hwndSheet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ApplyProperties( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelProperties( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgComponentPropertyUiVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfgComponentPropertyUi * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfgComponentPropertyUi * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfgComponentPropertyUi * This);
        
        DECLSPEC_XFGVIRT(INetCfgComponentPropertyUi, QueryPropertyUi)
        HRESULT ( STDMETHODCALLTYPE *QueryPropertyUi )( 
            INetCfgComponentPropertyUi * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkReserved);
        
        DECLSPEC_XFGVIRT(INetCfgComponentPropertyUi, SetContext)
        HRESULT ( STDMETHODCALLTYPE *SetContext )( 
            INetCfgComponentPropertyUi * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkReserved);
        
        DECLSPEC_XFGVIRT(INetCfgComponentPropertyUi, MergePropPages)
        HRESULT ( STDMETHODCALLTYPE *MergePropPages )( 
            INetCfgComponentPropertyUi * This,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pdwDefPages,
            /* [annotation][out] */ 
            _Out_  BYTE **pahpspPrivate,
            /* [annotation][out] */ 
            _Out_  UINT *pcPages,
            /* [annotation][in] */ 
            _In_  HWND hwndParent,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR *pszStartPage);
        
        DECLSPEC_XFGVIRT(INetCfgComponentPropertyUi, ValidateProperties)
        HRESULT ( STDMETHODCALLTYPE *ValidateProperties )( 
            INetCfgComponentPropertyUi * This,
            /* [annotation][in] */ 
            _In_  HWND hwndSheet);
        
        DECLSPEC_XFGVIRT(INetCfgComponentPropertyUi, ApplyProperties)
        HRESULT ( STDMETHODCALLTYPE *ApplyProperties )( 
            INetCfgComponentPropertyUi * This);
        
        DECLSPEC_XFGVIRT(INetCfgComponentPropertyUi, CancelProperties)
        HRESULT ( STDMETHODCALLTYPE *CancelProperties )( 
            INetCfgComponentPropertyUi * This);
        
        END_INTERFACE
    } INetCfgComponentPropertyUiVtbl;

    interface INetCfgComponentPropertyUi
    {
        CONST_VTBL struct INetCfgComponentPropertyUiVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfgComponentPropertyUi_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfgComponentPropertyUi_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfgComponentPropertyUi_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfgComponentPropertyUi_QueryPropertyUi(This,pUnkReserved)	\
    ( (This)->lpVtbl -> QueryPropertyUi(This,pUnkReserved) ) 

#define INetCfgComponentPropertyUi_SetContext(This,pUnkReserved)	\
    ( (This)->lpVtbl -> SetContext(This,pUnkReserved) ) 

#define INetCfgComponentPropertyUi_MergePropPages(This,pdwDefPages,pahpspPrivate,pcPages,hwndParent,pszStartPage)	\
    ( (This)->lpVtbl -> MergePropPages(This,pdwDefPages,pahpspPrivate,pcPages,hwndParent,pszStartPage) ) 

#define INetCfgComponentPropertyUi_ValidateProperties(This,hwndSheet)	\
    ( (This)->lpVtbl -> ValidateProperties(This,hwndSheet) ) 

#define INetCfgComponentPropertyUi_ApplyProperties(This)	\
    ( (This)->lpVtbl -> ApplyProperties(This) ) 

#define INetCfgComponentPropertyUi_CancelProperties(This)	\
    ( (This)->lpVtbl -> CancelProperties(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfgComponentPropertyUi_INTERFACE_DEFINED__ */


#ifndef __INetCfgComponentNotifyBinding_INTERFACE_DEFINED__
#define __INetCfgComponentNotifyBinding_INTERFACE_DEFINED__

/* interface INetCfgComponentNotifyBinding */
/* [unique][uuid][object][local] */ 

typedef /* [v1_enum] */ 
enum tagBIND_FLAGS1
    {
        NCN_ADD	= 0x1,
        NCN_REMOVE	= 0x2,
        NCN_UPDATE	= 0x4,
        NCN_ENABLE	= 0x10,
        NCN_DISABLE	= 0x20,
        NCN_BINDING_PATH	= 0x100,
        NCN_PROPERTYCHANGE	= 0x200,
        NCN_NET	= 0x10000,
        NCN_NETTRANS	= 0x20000,
        NCN_NETCLIENT	= 0x40000,
        NCN_NETSERVICE	= 0x80000
    } 	BIND_FLAGS1;


EXTERN_C const IID IID_INetCfgComponentNotifyBinding;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("932238e1-bea1-11d0-9298-00c04fc99dcf")
    INetCfgComponentNotifyBinding : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueryBindingPath( 
            /* [annotation][in] */ 
            _In_  DWORD dwChangeFlag,
            /* [annotation][in] */ 
            _In_  INetCfgBindingPath *pIPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyBindingPath( 
            /* [annotation][in] */ 
            _In_  DWORD dwChangeFlag,
            /* [annotation][in] */ 
            _In_  INetCfgBindingPath *pIPath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgComponentNotifyBindingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfgComponentNotifyBinding * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfgComponentNotifyBinding * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfgComponentNotifyBinding * This);
        
        DECLSPEC_XFGVIRT(INetCfgComponentNotifyBinding, QueryBindingPath)
        HRESULT ( STDMETHODCALLTYPE *QueryBindingPath )( 
            INetCfgComponentNotifyBinding * This,
            /* [annotation][in] */ 
            _In_  DWORD dwChangeFlag,
            /* [annotation][in] */ 
            _In_  INetCfgBindingPath *pIPath);
        
        DECLSPEC_XFGVIRT(INetCfgComponentNotifyBinding, NotifyBindingPath)
        HRESULT ( STDMETHODCALLTYPE *NotifyBindingPath )( 
            INetCfgComponentNotifyBinding * This,
            /* [annotation][in] */ 
            _In_  DWORD dwChangeFlag,
            /* [annotation][in] */ 
            _In_  INetCfgBindingPath *pIPath);
        
        END_INTERFACE
    } INetCfgComponentNotifyBindingVtbl;

    interface INetCfgComponentNotifyBinding
    {
        CONST_VTBL struct INetCfgComponentNotifyBindingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfgComponentNotifyBinding_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfgComponentNotifyBinding_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfgComponentNotifyBinding_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfgComponentNotifyBinding_QueryBindingPath(This,dwChangeFlag,pIPath)	\
    ( (This)->lpVtbl -> QueryBindingPath(This,dwChangeFlag,pIPath) ) 

#define INetCfgComponentNotifyBinding_NotifyBindingPath(This,dwChangeFlag,pIPath)	\
    ( (This)->lpVtbl -> NotifyBindingPath(This,dwChangeFlag,pIPath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfgComponentNotifyBinding_INTERFACE_DEFINED__ */


#ifndef __INetCfgComponentNotifyGlobal_INTERFACE_DEFINED__
#define __INetCfgComponentNotifyGlobal_INTERFACE_DEFINED__

/* interface INetCfgComponentNotifyGlobal */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_INetCfgComponentNotifyGlobal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("932238e2-bea1-11d0-9298-00c04fc99dcf")
    INetCfgComponentNotifyGlobal : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSupportedNotifications( 
            /* [annotation][out] */ 
            _Out_  DWORD *dwNotifications) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SysQueryBindingPath( 
            /* [annotation][in] */ 
            _In_  DWORD dwChangeFlag,
            /* [annotation][in] */ 
            _In_  INetCfgBindingPath *pIPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SysNotifyBindingPath( 
            /* [annotation][in] */ 
            _In_  DWORD dwChangeFlag,
            /* [annotation][in] */ 
            _In_  INetCfgBindingPath *pIPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SysNotifyComponent( 
            /* [annotation][in] */ 
            _In_  DWORD dwChangeFlag,
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pIComp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgComponentNotifyGlobalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfgComponentNotifyGlobal * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfgComponentNotifyGlobal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfgComponentNotifyGlobal * This);
        
        DECLSPEC_XFGVIRT(INetCfgComponentNotifyGlobal, GetSupportedNotifications)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedNotifications )( 
            INetCfgComponentNotifyGlobal * This,
            /* [annotation][out] */ 
            _Out_  DWORD *dwNotifications);
        
        DECLSPEC_XFGVIRT(INetCfgComponentNotifyGlobal, SysQueryBindingPath)
        HRESULT ( STDMETHODCALLTYPE *SysQueryBindingPath )( 
            INetCfgComponentNotifyGlobal * This,
            /* [annotation][in] */ 
            _In_  DWORD dwChangeFlag,
            /* [annotation][in] */ 
            _In_  INetCfgBindingPath *pIPath);
        
        DECLSPEC_XFGVIRT(INetCfgComponentNotifyGlobal, SysNotifyBindingPath)
        HRESULT ( STDMETHODCALLTYPE *SysNotifyBindingPath )( 
            INetCfgComponentNotifyGlobal * This,
            /* [annotation][in] */ 
            _In_  DWORD dwChangeFlag,
            /* [annotation][in] */ 
            _In_  INetCfgBindingPath *pIPath);
        
        DECLSPEC_XFGVIRT(INetCfgComponentNotifyGlobal, SysNotifyComponent)
        HRESULT ( STDMETHODCALLTYPE *SysNotifyComponent )( 
            INetCfgComponentNotifyGlobal * This,
            /* [annotation][in] */ 
            _In_  DWORD dwChangeFlag,
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pIComp);
        
        END_INTERFACE
    } INetCfgComponentNotifyGlobalVtbl;

    interface INetCfgComponentNotifyGlobal
    {
        CONST_VTBL struct INetCfgComponentNotifyGlobalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfgComponentNotifyGlobal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfgComponentNotifyGlobal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfgComponentNotifyGlobal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfgComponentNotifyGlobal_GetSupportedNotifications(This,dwNotifications)	\
    ( (This)->lpVtbl -> GetSupportedNotifications(This,dwNotifications) ) 

#define INetCfgComponentNotifyGlobal_SysQueryBindingPath(This,dwChangeFlag,pIPath)	\
    ( (This)->lpVtbl -> SysQueryBindingPath(This,dwChangeFlag,pIPath) ) 

#define INetCfgComponentNotifyGlobal_SysNotifyBindingPath(This,dwChangeFlag,pIPath)	\
    ( (This)->lpVtbl -> SysNotifyBindingPath(This,dwChangeFlag,pIPath) ) 

#define INetCfgComponentNotifyGlobal_SysNotifyComponent(This,dwChangeFlag,pIComp)	\
    ( (This)->lpVtbl -> SysNotifyComponent(This,dwChangeFlag,pIComp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfgComponentNotifyGlobal_INTERFACE_DEFINED__ */


#ifndef __INetCfgComponentUpperEdge_INTERFACE_DEFINED__
#define __INetCfgComponentUpperEdge_INTERFACE_DEFINED__

/* interface INetCfgComponentUpperEdge */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_INetCfgComponentUpperEdge;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("932238e4-bea1-11d0-9298-00c04fc99dcf")
    INetCfgComponentUpperEdge : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetInterfaceIdsForAdapter( 
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pAdapter,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwNumInterfaces,
            /* [annotation][out] */ 
            _Outptr_opt_result_buffer_all_(*pdwNumInterfaces)  GUID **ppguidInterfaceIds) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddInterfacesToAdapter( 
            /* [annotation][in] */ 
            _In_reads_(dwNumInterfaces)  INetCfgComponent *pAdapter,
            /* [annotation][in] */ 
            _In_  DWORD dwNumInterfaces) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveInterfacesFromAdapter( 
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pAdapter,
            /* [annotation][in] */ 
            _In_  DWORD dwNumInterfaces,
            /* [annotation][in] */ 
            _In_reads_(dwNumInterfaces)  const GUID *pguidInterfaceIds) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgComponentUpperEdgeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfgComponentUpperEdge * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfgComponentUpperEdge * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfgComponentUpperEdge * This);
        
        DECLSPEC_XFGVIRT(INetCfgComponentUpperEdge, GetInterfaceIdsForAdapter)
        HRESULT ( STDMETHODCALLTYPE *GetInterfaceIdsForAdapter )( 
            INetCfgComponentUpperEdge * This,
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pAdapter,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwNumInterfaces,
            /* [annotation][out] */ 
            _Outptr_opt_result_buffer_all_(*pdwNumInterfaces)  GUID **ppguidInterfaceIds);
        
        DECLSPEC_XFGVIRT(INetCfgComponentUpperEdge, AddInterfacesToAdapter)
        HRESULT ( STDMETHODCALLTYPE *AddInterfacesToAdapter )( 
            INetCfgComponentUpperEdge * This,
            /* [annotation][in] */ 
            _In_reads_(dwNumInterfaces)  INetCfgComponent *pAdapter,
            /* [annotation][in] */ 
            _In_  DWORD dwNumInterfaces);
        
        DECLSPEC_XFGVIRT(INetCfgComponentUpperEdge, RemoveInterfacesFromAdapter)
        HRESULT ( STDMETHODCALLTYPE *RemoveInterfacesFromAdapter )( 
            INetCfgComponentUpperEdge * This,
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pAdapter,
            /* [annotation][in] */ 
            _In_  DWORD dwNumInterfaces,
            /* [annotation][in] */ 
            _In_reads_(dwNumInterfaces)  const GUID *pguidInterfaceIds);
        
        END_INTERFACE
    } INetCfgComponentUpperEdgeVtbl;

    interface INetCfgComponentUpperEdge
    {
        CONST_VTBL struct INetCfgComponentUpperEdgeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfgComponentUpperEdge_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfgComponentUpperEdge_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfgComponentUpperEdge_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfgComponentUpperEdge_GetInterfaceIdsForAdapter(This,pAdapter,pdwNumInterfaces,ppguidInterfaceIds)	\
    ( (This)->lpVtbl -> GetInterfaceIdsForAdapter(This,pAdapter,pdwNumInterfaces,ppguidInterfaceIds) ) 

#define INetCfgComponentUpperEdge_AddInterfacesToAdapter(This,pAdapter,dwNumInterfaces)	\
    ( (This)->lpVtbl -> AddInterfacesToAdapter(This,pAdapter,dwNumInterfaces) ) 

#define INetCfgComponentUpperEdge_RemoveInterfacesFromAdapter(This,pAdapter,dwNumInterfaces,pguidInterfaceIds)	\
    ( (This)->lpVtbl -> RemoveInterfacesFromAdapter(This,pAdapter,dwNumInterfaces,pguidInterfaceIds) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfgComponentUpperEdge_INTERFACE_DEFINED__ */


#ifndef __INetLanConnectionUiInfo_INTERFACE_DEFINED__
#define __INetLanConnectionUiInfo_INTERFACE_DEFINED__

/* interface INetLanConnectionUiInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_INetLanConnectionUiInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C08956A6-1CD3-11D1-B1C5-00805FC1270E")
    INetLanConnectionUiInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDeviceGuid( 
            /* [out] */ __RPC__out GUID *pguid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetLanConnectionUiInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INetLanConnectionUiInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INetLanConnectionUiInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INetLanConnectionUiInfo * This);
        
        DECLSPEC_XFGVIRT(INetLanConnectionUiInfo, GetDeviceGuid)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceGuid )( 
            __RPC__in INetLanConnectionUiInfo * This,
            /* [out] */ __RPC__out GUID *pguid);
        
        END_INTERFACE
    } INetLanConnectionUiInfoVtbl;

    interface INetLanConnectionUiInfo
    {
        CONST_VTBL struct INetLanConnectionUiInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetLanConnectionUiInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetLanConnectionUiInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetLanConnectionUiInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetLanConnectionUiInfo_GetDeviceGuid(This,pguid)	\
    ( (This)->lpVtbl -> GetDeviceGuid(This,pguid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetLanConnectionUiInfo_INTERFACE_DEFINED__ */


#ifndef __INetRasConnectionIpUiInfo_INTERFACE_DEFINED__
#define __INetRasConnectionIpUiInfo_INTERFACE_DEFINED__

/* interface INetRasConnectionIpUiInfo */
/* [unique][uuid][object] */ 

typedef 
enum tagRASCON_IPUI_FLAGS
    {
        RCUIF_VPN	= 0x1,
        RCUIF_DEMAND_DIAL	= 0x2,
        RCUIF_NOT_ADMIN	= 0x4,
        RCUIF_USE_IPv4_STATICADDRESS	= 0x8,
        RCUIF_USE_IPv4_NAME_SERVERS	= 0x10,
        RCUIF_USE_IPv4_REMOTE_GATEWAY	= 0x20,
        RCUIF_USE_IPv4_EXPLICIT_METRIC	= 0x40,
        RCUIF_USE_HEADER_COMPRESSION	= 0x80,
        RCUIF_USE_DISABLE_REGISTER_DNS	= 0x100,
        RCUIF_USE_PRIVATE_DNS_SUFFIX	= 0x200,
        RCUIF_ENABLE_NBT	= 0x400,
        RCUIF_USE_IPv6_STATICADDRESS	= 0x800,
        RCUIF_USE_IPv6_NAME_SERVERS	= 0x1000,
        RCUIF_USE_IPv6_REMOTE_GATEWAY	= 0x2000,
        RCUIF_USE_IPv6_EXPLICIT_METRIC	= 0x4000,
        RCUIF_DISABLE_CLASS_BASED_ROUTE	= 0x8000
    } 	RASCON_UIINFO_FLAGS;

typedef struct tagRASCON_IPUI
    {
    GUID guidConnection;
    BOOL fIPv6Cfg;
    DWORD dwFlags;
    WCHAR pszwIpAddr[ 16 ];
    WCHAR pszwDnsAddr[ 16 ];
    WCHAR pszwDns2Addr[ 16 ];
    WCHAR pszwWinsAddr[ 16 ];
    WCHAR pszwWins2Addr[ 16 ];
    WCHAR pszwDnsSuffix[ 256 ];
    WCHAR pszwIpv6Addr[ 65 ];
    DWORD dwIpv6PrefixLength;
    WCHAR pszwIpv6DnsAddr[ 65 ];
    WCHAR pszwIpv6Dns2Addr[ 65 ];
    DWORD dwIPv4InfMetric;
    DWORD dwIPv6InfMetric;
    } 	RASCON_IPUI;


EXTERN_C const IID IID_INetRasConnectionIpUiInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FAEDCF58-31FE-11D1-AAD2-00805FC1270E")
    INetRasConnectionIpUiInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetUiInfo( 
            /* [out] */ __RPC__out RASCON_IPUI *pInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetRasConnectionIpUiInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INetRasConnectionIpUiInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INetRasConnectionIpUiInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INetRasConnectionIpUiInfo * This);
        
        DECLSPEC_XFGVIRT(INetRasConnectionIpUiInfo, GetUiInfo)
        HRESULT ( STDMETHODCALLTYPE *GetUiInfo )( 
            __RPC__in INetRasConnectionIpUiInfo * This,
            /* [out] */ __RPC__out RASCON_IPUI *pInfo);
        
        END_INTERFACE
    } INetRasConnectionIpUiInfoVtbl;

    interface INetRasConnectionIpUiInfo
    {
        CONST_VTBL struct INetRasConnectionIpUiInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetRasConnectionIpUiInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetRasConnectionIpUiInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetRasConnectionIpUiInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetRasConnectionIpUiInfo_GetUiInfo(This,pInfo)	\
    ( (This)->lpVtbl -> GetUiInfo(This,pInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetRasConnectionIpUiInfo_INTERFACE_DEFINED__ */


#ifndef __INetCfgComponentSysPrep_INTERFACE_DEFINED__
#define __INetCfgComponentSysPrep_INTERFACE_DEFINED__

/* interface INetCfgComponentSysPrep */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_INetCfgComponentSysPrep;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0E8AE9A-306E-11D1-AACF-00805FC1270E")
    INetCfgComponentSysPrep : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SaveAdapterParameters( 
            /* [annotation][in] */ 
            _In_  INetCfgSysPrep *pncsp,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszwAnswerSections,
            /* [annotation][in] */ 
            _In_  GUID *pAdapterInstanceGuid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RestoreAdapterParameters( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszwAnswerFile,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszwAnswerSection,
            /* [annotation][in] */ 
            _In_  GUID *pAdapterInstanceGuid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgComponentSysPrepVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfgComponentSysPrep * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfgComponentSysPrep * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfgComponentSysPrep * This);
        
        DECLSPEC_XFGVIRT(INetCfgComponentSysPrep, SaveAdapterParameters)
        HRESULT ( STDMETHODCALLTYPE *SaveAdapterParameters )( 
            INetCfgComponentSysPrep * This,
            /* [annotation][in] */ 
            _In_  INetCfgSysPrep *pncsp,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszwAnswerSections,
            /* [annotation][in] */ 
            _In_  GUID *pAdapterInstanceGuid);
        
        DECLSPEC_XFGVIRT(INetCfgComponentSysPrep, RestoreAdapterParameters)
        HRESULT ( STDMETHODCALLTYPE *RestoreAdapterParameters )( 
            INetCfgComponentSysPrep * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszwAnswerFile,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszwAnswerSection,
            /* [annotation][in] */ 
            _In_  GUID *pAdapterInstanceGuid);
        
        END_INTERFACE
    } INetCfgComponentSysPrepVtbl;

    interface INetCfgComponentSysPrep
    {
        CONST_VTBL struct INetCfgComponentSysPrepVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfgComponentSysPrep_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfgComponentSysPrep_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfgComponentSysPrep_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfgComponentSysPrep_SaveAdapterParameters(This,pncsp,pszwAnswerSections,pAdapterInstanceGuid)	\
    ( (This)->lpVtbl -> SaveAdapterParameters(This,pncsp,pszwAnswerSections,pAdapterInstanceGuid) ) 

#define INetCfgComponentSysPrep_RestoreAdapterParameters(This,pszwAnswerFile,pszwAnswerSection,pAdapterInstanceGuid)	\
    ( (This)->lpVtbl -> RestoreAdapterParameters(This,pszwAnswerFile,pszwAnswerSection,pAdapterInstanceGuid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfgComponentSysPrep_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_netcfgn_0000_0010 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_netcfgn_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_netcfgn_0000_0010_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


