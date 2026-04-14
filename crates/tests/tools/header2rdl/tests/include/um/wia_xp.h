

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

#ifndef __wia_xp_h__
#define __wia_xp_h__

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

#ifndef __IWiaDevMgr_FWD_DEFINED__
#define __IWiaDevMgr_FWD_DEFINED__
typedef interface IWiaDevMgr IWiaDevMgr;

#endif 	/* __IWiaDevMgr_FWD_DEFINED__ */


#ifndef __IEnumWIA_DEV_INFO_FWD_DEFINED__
#define __IEnumWIA_DEV_INFO_FWD_DEFINED__
typedef interface IEnumWIA_DEV_INFO IEnumWIA_DEV_INFO;

#endif 	/* __IEnumWIA_DEV_INFO_FWD_DEFINED__ */


#ifndef __IWiaEventCallback_FWD_DEFINED__
#define __IWiaEventCallback_FWD_DEFINED__
typedef interface IWiaEventCallback IWiaEventCallback;

#endif 	/* __IWiaEventCallback_FWD_DEFINED__ */


#ifndef __IWiaDataCallback_FWD_DEFINED__
#define __IWiaDataCallback_FWD_DEFINED__
typedef interface IWiaDataCallback IWiaDataCallback;

#endif 	/* __IWiaDataCallback_FWD_DEFINED__ */


#ifndef __IWiaDataTransfer_FWD_DEFINED__
#define __IWiaDataTransfer_FWD_DEFINED__
typedef interface IWiaDataTransfer IWiaDataTransfer;

#endif 	/* __IWiaDataTransfer_FWD_DEFINED__ */


#ifndef __IWiaItem_FWD_DEFINED__
#define __IWiaItem_FWD_DEFINED__
typedef interface IWiaItem IWiaItem;

#endif 	/* __IWiaItem_FWD_DEFINED__ */


#ifndef __IWiaPropertyStorage_FWD_DEFINED__
#define __IWiaPropertyStorage_FWD_DEFINED__
typedef interface IWiaPropertyStorage IWiaPropertyStorage;

#endif 	/* __IWiaPropertyStorage_FWD_DEFINED__ */


#ifndef __IEnumWiaItem_FWD_DEFINED__
#define __IEnumWiaItem_FWD_DEFINED__
typedef interface IEnumWiaItem IEnumWiaItem;

#endif 	/* __IEnumWiaItem_FWD_DEFINED__ */


#ifndef __IEnumWIA_DEV_CAPS_FWD_DEFINED__
#define __IEnumWIA_DEV_CAPS_FWD_DEFINED__
typedef interface IEnumWIA_DEV_CAPS IEnumWIA_DEV_CAPS;

#endif 	/* __IEnumWIA_DEV_CAPS_FWD_DEFINED__ */


#ifndef __IEnumWIA_FORMAT_INFO_FWD_DEFINED__
#define __IEnumWIA_FORMAT_INFO_FWD_DEFINED__
typedef interface IEnumWIA_FORMAT_INFO IEnumWIA_FORMAT_INFO;

#endif 	/* __IEnumWIA_FORMAT_INFO_FWD_DEFINED__ */


#ifndef __IWiaLog_FWD_DEFINED__
#define __IWiaLog_FWD_DEFINED__
typedef interface IWiaLog IWiaLog;

#endif 	/* __IWiaLog_FWD_DEFINED__ */


#ifndef __IWiaLogEx_FWD_DEFINED__
#define __IWiaLogEx_FWD_DEFINED__
typedef interface IWiaLogEx IWiaLogEx;

#endif 	/* __IWiaLogEx_FWD_DEFINED__ */


#ifndef __IWiaNotifyDevMgr_FWD_DEFINED__
#define __IWiaNotifyDevMgr_FWD_DEFINED__
typedef interface IWiaNotifyDevMgr IWiaNotifyDevMgr;

#endif 	/* __IWiaNotifyDevMgr_FWD_DEFINED__ */


#ifndef __IWiaItemExtras_FWD_DEFINED__
#define __IWiaItemExtras_FWD_DEFINED__
typedef interface IWiaItemExtras IWiaItemExtras;

#endif 	/* __IWiaItemExtras_FWD_DEFINED__ */


#ifndef __WiaDevMgr_FWD_DEFINED__
#define __WiaDevMgr_FWD_DEFINED__

#ifdef __cplusplus
typedef class WiaDevMgr WiaDevMgr;
#else
typedef struct WiaDevMgr WiaDevMgr;
#endif /* __cplusplus */

#endif 	/* __WiaDevMgr_FWD_DEFINED__ */


#ifndef __WiaLog_FWD_DEFINED__
#define __WiaLog_FWD_DEFINED__

#ifdef __cplusplus
typedef class WiaLog WiaLog;
#else
typedef struct WiaLog WiaLog;
#endif /* __cplusplus */

#endif 	/* __WiaLog_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "oaidl.h"
#include "propidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wia_xp_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)










typedef struct _WIA_DITHER_PATTERN_DATA
    {
    LONG lSize;
    BSTR bstrPatternName;
    LONG lPatternWidth;
    LONG lPatternLength;
    LONG cbPattern;
    /* [size_is] */ BYTE *pbPattern;
    } 	WIA_DITHER_PATTERN_DATA;

typedef struct _WIA_DITHER_PATTERN_DATA *PWIA_DITHER_PATTERN_DATA;

typedef struct _WIA_PROPID_TO_NAME
    {
    PROPID propid;
    LPOLESTR pszName;
    } 	WIA_PROPID_TO_NAME;

typedef struct _WIA_PROPID_TO_NAME *PWIA_PROPID_TO_NAME;

typedef struct _WIA_FORMAT_INFO
    {
    GUID guidFormatID;
    LONG lTymed;
    } 	WIA_FORMAT_INFO;

typedef struct _WIA_FORMAT_INFO *PWIA_FORMAT_INFO;

#include "wiadef.h"


extern RPC_IF_HANDLE __MIDL_itf_wia_xp_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wia_xp_0000_0000_v0_0_s_ifspec;

#ifndef __IWiaDevMgr_INTERFACE_DEFINED__
#define __IWiaDevMgr_INTERFACE_DEFINED__

/* interface IWiaDevMgr */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWiaDevMgr;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5eb2502a-8cf1-11d1-bf92-0060081ed811")
    IWiaDevMgr : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EnumDeviceInfo( 
            /* [in] */ LONG lFlag,
            /* [retval][out] */ __RPC__deref_out_opt IEnumWIA_DEV_INFO **ppIEnum) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE CreateDevice( 
            /* [in] */ BSTR bstrDeviceID,
            /* [out] */ IWiaItem **ppWiaItemRoot) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE SelectDeviceDlg( 
            /* [in] */ HWND hwndParent,
            /* [in] */ LONG lDeviceType,
            /* [in] */ LONG lFlags,
            /* [out][in] */ BSTR *pbstrDeviceID,
            /* [retval][out] */ IWiaItem **ppItemRoot) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE SelectDeviceDlgID( 
            /* [in] */ HWND hwndParent,
            /* [in] */ LONG lDeviceType,
            /* [in] */ LONG lFlags,
            /* [retval][out] */ BSTR *pbstrDeviceID) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetImageDlg( 
            /* [in] */ HWND hwndParent,
            /* [in] */ LONG lDeviceType,
            /* [in] */ LONG lFlags,
            /* [in] */ LONG lIntent,
            /* [in] */ IWiaItem *pItemRoot,
            /* [in] */ BSTR bstrFilename,
            /* [out][in] */ GUID *pguidFormat) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE RegisterEventCallbackProgram( 
            /* [in] */ LONG lFlags,
            /* [in] */ BSTR bstrDeviceID,
            /* [in] */ const GUID *pEventGUID,
            /* [in] */ BSTR bstrCommandline,
            /* [in] */ BSTR bstrName,
            /* [in] */ BSTR bstrDescription,
            /* [in] */ BSTR bstrIcon) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE RegisterEventCallbackInterface( 
            /* [in] */ LONG lFlags,
            /* [in] */ BSTR bstrDeviceID,
            /* [in] */ const GUID *pEventGUID,
            /* [unique][in] */ IWiaEventCallback *pIWiaEventCallback,
            /* [out] */ IUnknown **pEventObject) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE RegisterEventCallbackCLSID( 
            /* [in] */ LONG lFlags,
            /* [in] */ BSTR bstrDeviceID,
            /* [in] */ const GUID *pEventGUID,
            /* [unique][in] */ const GUID *pClsID,
            /* [in] */ BSTR bstrName,
            /* [in] */ BSTR bstrDescription,
            /* [in] */ BSTR bstrIcon) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddDeviceDlg( 
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ LONG lFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWiaDevMgrVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWiaDevMgr * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWiaDevMgr * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWiaDevMgr * This);
        
        DECLSPEC_XFGVIRT(IWiaDevMgr, EnumDeviceInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnumDeviceInfo )( 
            __RPC__in IWiaDevMgr * This,
            /* [in] */ LONG lFlag,
            /* [retval][out] */ __RPC__deref_out_opt IEnumWIA_DEV_INFO **ppIEnum);
        
        DECLSPEC_XFGVIRT(IWiaDevMgr, CreateDevice)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CreateDevice )( 
            IWiaDevMgr * This,
            /* [in] */ BSTR bstrDeviceID,
            /* [out] */ IWiaItem **ppWiaItemRoot);
        
        DECLSPEC_XFGVIRT(IWiaDevMgr, SelectDeviceDlg)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SelectDeviceDlg )( 
            IWiaDevMgr * This,
            /* [in] */ HWND hwndParent,
            /* [in] */ LONG lDeviceType,
            /* [in] */ LONG lFlags,
            /* [out][in] */ BSTR *pbstrDeviceID,
            /* [retval][out] */ IWiaItem **ppItemRoot);
        
        DECLSPEC_XFGVIRT(IWiaDevMgr, SelectDeviceDlgID)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SelectDeviceDlgID )( 
            IWiaDevMgr * This,
            /* [in] */ HWND hwndParent,
            /* [in] */ LONG lDeviceType,
            /* [in] */ LONG lFlags,
            /* [retval][out] */ BSTR *pbstrDeviceID);
        
        DECLSPEC_XFGVIRT(IWiaDevMgr, GetImageDlg)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetImageDlg )( 
            IWiaDevMgr * This,
            /* [in] */ HWND hwndParent,
            /* [in] */ LONG lDeviceType,
            /* [in] */ LONG lFlags,
            /* [in] */ LONG lIntent,
            /* [in] */ IWiaItem *pItemRoot,
            /* [in] */ BSTR bstrFilename,
            /* [out][in] */ GUID *pguidFormat);
        
        DECLSPEC_XFGVIRT(IWiaDevMgr, RegisterEventCallbackProgram)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *RegisterEventCallbackProgram )( 
            IWiaDevMgr * This,
            /* [in] */ LONG lFlags,
            /* [in] */ BSTR bstrDeviceID,
            /* [in] */ const GUID *pEventGUID,
            /* [in] */ BSTR bstrCommandline,
            /* [in] */ BSTR bstrName,
            /* [in] */ BSTR bstrDescription,
            /* [in] */ BSTR bstrIcon);
        
        DECLSPEC_XFGVIRT(IWiaDevMgr, RegisterEventCallbackInterface)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *RegisterEventCallbackInterface )( 
            IWiaDevMgr * This,
            /* [in] */ LONG lFlags,
            /* [in] */ BSTR bstrDeviceID,
            /* [in] */ const GUID *pEventGUID,
            /* [unique][in] */ IWiaEventCallback *pIWiaEventCallback,
            /* [out] */ IUnknown **pEventObject);
        
        DECLSPEC_XFGVIRT(IWiaDevMgr, RegisterEventCallbackCLSID)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *RegisterEventCallbackCLSID )( 
            IWiaDevMgr * This,
            /* [in] */ LONG lFlags,
            /* [in] */ BSTR bstrDeviceID,
            /* [in] */ const GUID *pEventGUID,
            /* [unique][in] */ const GUID *pClsID,
            /* [in] */ BSTR bstrName,
            /* [in] */ BSTR bstrDescription,
            /* [in] */ BSTR bstrIcon);
        
        DECLSPEC_XFGVIRT(IWiaDevMgr, AddDeviceDlg)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddDeviceDlg )( 
            __RPC__in IWiaDevMgr * This,
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ LONG lFlags);
        
        END_INTERFACE
    } IWiaDevMgrVtbl;

    interface IWiaDevMgr
    {
        CONST_VTBL struct IWiaDevMgrVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWiaDevMgr_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWiaDevMgr_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWiaDevMgr_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWiaDevMgr_EnumDeviceInfo(This,lFlag,ppIEnum)	\
    ( (This)->lpVtbl -> EnumDeviceInfo(This,lFlag,ppIEnum) ) 

#define IWiaDevMgr_CreateDevice(This,bstrDeviceID,ppWiaItemRoot)	\
    ( (This)->lpVtbl -> CreateDevice(This,bstrDeviceID,ppWiaItemRoot) ) 

#define IWiaDevMgr_SelectDeviceDlg(This,hwndParent,lDeviceType,lFlags,pbstrDeviceID,ppItemRoot)	\
    ( (This)->lpVtbl -> SelectDeviceDlg(This,hwndParent,lDeviceType,lFlags,pbstrDeviceID,ppItemRoot) ) 

#define IWiaDevMgr_SelectDeviceDlgID(This,hwndParent,lDeviceType,lFlags,pbstrDeviceID)	\
    ( (This)->lpVtbl -> SelectDeviceDlgID(This,hwndParent,lDeviceType,lFlags,pbstrDeviceID) ) 

#define IWiaDevMgr_GetImageDlg(This,hwndParent,lDeviceType,lFlags,lIntent,pItemRoot,bstrFilename,pguidFormat)	\
    ( (This)->lpVtbl -> GetImageDlg(This,hwndParent,lDeviceType,lFlags,lIntent,pItemRoot,bstrFilename,pguidFormat) ) 

#define IWiaDevMgr_RegisterEventCallbackProgram(This,lFlags,bstrDeviceID,pEventGUID,bstrCommandline,bstrName,bstrDescription,bstrIcon)	\
    ( (This)->lpVtbl -> RegisterEventCallbackProgram(This,lFlags,bstrDeviceID,pEventGUID,bstrCommandline,bstrName,bstrDescription,bstrIcon) ) 

#define IWiaDevMgr_RegisterEventCallbackInterface(This,lFlags,bstrDeviceID,pEventGUID,pIWiaEventCallback,pEventObject)	\
    ( (This)->lpVtbl -> RegisterEventCallbackInterface(This,lFlags,bstrDeviceID,pEventGUID,pIWiaEventCallback,pEventObject) ) 

#define IWiaDevMgr_RegisterEventCallbackCLSID(This,lFlags,bstrDeviceID,pEventGUID,pClsID,bstrName,bstrDescription,bstrIcon)	\
    ( (This)->lpVtbl -> RegisterEventCallbackCLSID(This,lFlags,bstrDeviceID,pEventGUID,pClsID,bstrName,bstrDescription,bstrIcon) ) 

#define IWiaDevMgr_AddDeviceDlg(This,hwndParent,lFlags)	\
    ( (This)->lpVtbl -> AddDeviceDlg(This,hwndParent,lFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_LocalCreateDevice_Proxy( 
    __RPC__in IWiaDevMgr * This,
    /* [in] */ __RPC__in BSTR bstrDeviceID,
    /* [out] */ __RPC__deref_out_opt IWiaItem **ppWiaItemRoot);


void __RPC_STUB IWiaDevMgr_LocalCreateDevice_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [nocode][helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_LocalSelectDeviceDlg_Proxy( 
    __RPC__in IWiaDevMgr * This,
    /* [in] */ __RPC__in HWND hwndParent,
    /* [in] */ LONG lDeviceType,
    /* [in] */ LONG lFlags,
    /* [out][in] */ __RPC__deref_inout_opt BSTR *pbstrDeviceID,
    /* [retval][out] */ __RPC__deref_out_opt IWiaItem **ppItemRoot);


void __RPC_STUB IWiaDevMgr_LocalSelectDeviceDlg_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [nocode][helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_LocalSelectDeviceDlgID_Proxy( 
    __RPC__in IWiaDevMgr * This,
    /* [in] */ __RPC__in HWND hwndParent,
    /* [in] */ LONG lDeviceType,
    /* [in] */ LONG lFlags,
    /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceID);


void __RPC_STUB IWiaDevMgr_LocalSelectDeviceDlgID_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [nocode][helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_LocalGetImageDlg_Proxy( 
    __RPC__in IWiaDevMgr * This,
    /* [in] */ __RPC__in HWND hwndParent,
    /* [in] */ LONG lDeviceType,
    /* [in] */ LONG lFlags,
    /* [in] */ LONG lIntent,
    /* [in] */ __RPC__in_opt IWiaItem *pItemRoot,
    /* [in] */ __RPC__in BSTR bstrFilename,
    /* [out][in] */ __RPC__inout GUID *pguidFormat);


void __RPC_STUB IWiaDevMgr_LocalGetImageDlg_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_LocalRegisterEventCallbackProgram_Proxy( 
    __RPC__in IWiaDevMgr * This,
    /* [in] */ LONG lFlags,
    /* [in] */ __RPC__in BSTR bstrDeviceID,
    /* [in] */ __RPC__in const GUID *pEventGUID,
    /* [in] */ __RPC__in BSTR bstrCommandline,
    /* [in] */ __RPC__in BSTR bstrName,
    /* [in] */ __RPC__in BSTR bstrDescription,
    /* [in] */ __RPC__in BSTR bstrIcon);


void __RPC_STUB IWiaDevMgr_LocalRegisterEventCallbackProgram_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_LocalRegisterEventCallbackInterface_Proxy( 
    __RPC__in IWiaDevMgr * This,
    /* [in] */ LONG lFlags,
    /* [in] */ __RPC__in BSTR bstrDeviceID,
    /* [in] */ __RPC__in const GUID *pEventGUID,
    /* [unique][in] */ __RPC__in_opt IWiaEventCallback *pIWiaEventCallback,
    /* [out] */ __RPC__deref_out_opt IUnknown **pEventObject);


void __RPC_STUB IWiaDevMgr_LocalRegisterEventCallbackInterface_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_LocalRegisterEventCallbackCLSID_Proxy( 
    __RPC__in IWiaDevMgr * This,
    /* [in] */ LONG lFlags,
    /* [in] */ __RPC__in BSTR bstrDeviceID,
    /* [in] */ __RPC__in const GUID *pEventGUID,
    /* [unique][in] */ __RPC__in_opt const GUID *pClsID,
    /* [in] */ __RPC__in BSTR bstrName,
    /* [in] */ __RPC__in BSTR bstrDescription,
    /* [in] */ __RPC__in BSTR bstrIcon);


void __RPC_STUB IWiaDevMgr_LocalRegisterEventCallbackCLSID_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IWiaDevMgr_INTERFACE_DEFINED__ */


#ifndef __IEnumWIA_DEV_INFO_INTERFACE_DEFINED__
#define __IEnumWIA_DEV_INFO_INTERFACE_DEFINED__

/* interface IEnumWIA_DEV_INFO */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEnumWIA_DEV_INFO;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5e38b83c-8cf1-11d1-bf92-0060081ed811")
    IEnumWIA_DEV_INFO : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ IWiaPropertyStorage **rgelt,
            /* [unique][out][in] */ ULONG *pceltFetched) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumWIA_DEV_INFO **ppIEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out ULONG *celt) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumWIA_DEV_INFOVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumWIA_DEV_INFO * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumWIA_DEV_INFO * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumWIA_DEV_INFO * This);
        
        DECLSPEC_XFGVIRT(IEnumWIA_DEV_INFO, Next)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumWIA_DEV_INFO * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ IWiaPropertyStorage **rgelt,
            /* [unique][out][in] */ ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumWIA_DEV_INFO, Skip)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumWIA_DEV_INFO * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumWIA_DEV_INFO, Reset)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumWIA_DEV_INFO * This);
        
        DECLSPEC_XFGVIRT(IEnumWIA_DEV_INFO, Clone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumWIA_DEV_INFO * This,
            /* [out] */ __RPC__deref_out_opt IEnumWIA_DEV_INFO **ppIEnum);
        
        DECLSPEC_XFGVIRT(IEnumWIA_DEV_INFO, GetCount)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IEnumWIA_DEV_INFO * This,
            /* [out] */ __RPC__out ULONG *celt);
        
        END_INTERFACE
    } IEnumWIA_DEV_INFOVtbl;

    interface IEnumWIA_DEV_INFO
    {
        CONST_VTBL struct IEnumWIA_DEV_INFOVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumWIA_DEV_INFO_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumWIA_DEV_INFO_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumWIA_DEV_INFO_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumWIA_DEV_INFO_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumWIA_DEV_INFO_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumWIA_DEV_INFO_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumWIA_DEV_INFO_Clone(This,ppIEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppIEnum) ) 

#define IEnumWIA_DEV_INFO_GetCount(This,celt)	\
    ( (This)->lpVtbl -> GetCount(This,celt) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IEnumWIA_DEV_INFO_RemoteNext_Proxy( 
    __RPC__in IEnumWIA_DEV_INFO * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IWiaPropertyStorage **rgelt,
    /* [unique][out][in] */ __RPC__inout_opt ULONG *pceltFetched);


void __RPC_STUB IEnumWIA_DEV_INFO_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumWIA_DEV_INFO_INTERFACE_DEFINED__ */


#ifndef __IWiaEventCallback_INTERFACE_DEFINED__
#define __IWiaEventCallback_INTERFACE_DEFINED__

/* interface IWiaEventCallback */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWiaEventCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ae6287b0-0084-11d2-973b-00a0c9068f2e")
    IWiaEventCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ImageEventCallback( 
            /* [in] */ __RPC__in const GUID *pEventGUID,
            /* [in] */ __RPC__in BSTR bstrEventDescription,
            /* [in] */ __RPC__in BSTR bstrDeviceID,
            /* [in] */ __RPC__in BSTR bstrDeviceDescription,
            /* [in] */ DWORD dwDeviceType,
            /* [in] */ __RPC__in BSTR bstrFullItemName,
            /* [out][in] */ __RPC__inout ULONG *pulEventType,
            /* [in] */ ULONG ulReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWiaEventCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWiaEventCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWiaEventCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWiaEventCallback * This);
        
        DECLSPEC_XFGVIRT(IWiaEventCallback, ImageEventCallback)
        HRESULT ( STDMETHODCALLTYPE *ImageEventCallback )( 
            __RPC__in IWiaEventCallback * This,
            /* [in] */ __RPC__in const GUID *pEventGUID,
            /* [in] */ __RPC__in BSTR bstrEventDescription,
            /* [in] */ __RPC__in BSTR bstrDeviceID,
            /* [in] */ __RPC__in BSTR bstrDeviceDescription,
            /* [in] */ DWORD dwDeviceType,
            /* [in] */ __RPC__in BSTR bstrFullItemName,
            /* [out][in] */ __RPC__inout ULONG *pulEventType,
            /* [in] */ ULONG ulReserved);
        
        END_INTERFACE
    } IWiaEventCallbackVtbl;

    interface IWiaEventCallback
    {
        CONST_VTBL struct IWiaEventCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWiaEventCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWiaEventCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWiaEventCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWiaEventCallback_ImageEventCallback(This,pEventGUID,bstrEventDescription,bstrDeviceID,bstrDeviceDescription,dwDeviceType,bstrFullItemName,pulEventType,ulReserved)	\
    ( (This)->lpVtbl -> ImageEventCallback(This,pEventGUID,bstrEventDescription,bstrDeviceID,bstrDeviceDescription,dwDeviceType,bstrFullItemName,pulEventType,ulReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWiaEventCallback_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wia_xp_0000_0003 */
/* [local] */ 

typedef struct _WIA_DATA_CALLBACK_HEADER
    {
    LONG lSize;
    GUID guidFormatID;
    LONG lBufferSize;
    LONG lPageCount;
    } 	WIA_DATA_CALLBACK_HEADER;

typedef struct _WIA_DATA_CALLBACK_HEADER *PWIA_DATA_CALLBACK_HEADER;



extern RPC_IF_HANDLE __MIDL_itf_wia_xp_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wia_xp_0000_0003_v0_0_s_ifspec;

#ifndef __IWiaDataCallback_INTERFACE_DEFINED__
#define __IWiaDataCallback_INTERFACE_DEFINED__

/* interface IWiaDataCallback */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWiaDataCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a558a866-a5b0-11d2-a08f-00c04f72dc3c")
    IWiaDataCallback : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE BandedDataCallback( 
            /* [in] */ LONG lMessage,
            /* [in] */ LONG lStatus,
            /* [in] */ LONG lPercentComplete,
            /* [in] */ LONG lOffset,
            /* [in] */ LONG lLength,
            /* [in] */ LONG lReserved,
            /* [in] */ LONG lResLength,
            /* [size_is][in] */ BYTE *pbBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWiaDataCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWiaDataCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWiaDataCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWiaDataCallback * This);
        
        DECLSPEC_XFGVIRT(IWiaDataCallback, BandedDataCallback)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BandedDataCallback )( 
            IWiaDataCallback * This,
            /* [in] */ LONG lMessage,
            /* [in] */ LONG lStatus,
            /* [in] */ LONG lPercentComplete,
            /* [in] */ LONG lOffset,
            /* [in] */ LONG lLength,
            /* [in] */ LONG lReserved,
            /* [in] */ LONG lResLength,
            /* [size_is][in] */ BYTE *pbBuffer);
        
        END_INTERFACE
    } IWiaDataCallbackVtbl;

    interface IWiaDataCallback
    {
        CONST_VTBL struct IWiaDataCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWiaDataCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWiaDataCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWiaDataCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWiaDataCallback_BandedDataCallback(This,lMessage,lStatus,lPercentComplete,lOffset,lLength,lReserved,lResLength,pbBuffer)	\
    ( (This)->lpVtbl -> BandedDataCallback(This,lMessage,lStatus,lPercentComplete,lOffset,lLength,lReserved,lResLength,pbBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDataCallback_RemoteBandedDataCallback_Proxy( 
    __RPC__in IWiaDataCallback * This,
    /* [in] */ LONG lMessage,
    /* [in] */ LONG lStatus,
    /* [in] */ LONG lPercentComplete,
    /* [in] */ LONG lOffset,
    /* [in] */ LONG lLength,
    /* [in] */ LONG lReserved,
    /* [in] */ LONG lResLength,
    /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(lResLength) BYTE *pbBuffer);


void __RPC_STUB IWiaDataCallback_RemoteBandedDataCallback_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IWiaDataCallback_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wia_xp_0000_0004 */
/* [local] */ 

typedef struct _WIA_DATA_TRANSFER_INFO
    {
    ULONG ulSize;
    ULONG ulSection;
    ULONG ulBufferSize;
    BOOL bDoubleBuffer;
    ULONG ulReserved1;
    ULONG ulReserved2;
    ULONG ulReserved3;
    } 	WIA_DATA_TRANSFER_INFO;

typedef struct _WIA_DATA_TRANSFER_INFO *PWIA_DATA_TRANSFER_INFO;

typedef struct _WIA_EXTENDED_TRANSFER_INFO
    {
    ULONG ulSize;
    ULONG ulMinBufferSize;
    ULONG ulOptimalBufferSize;
    ULONG ulMaxBufferSize;
    ULONG ulNumBuffers;
    } 	WIA_EXTENDED_TRANSFER_INFO;

typedef struct _WIA_EXTENDED_TRANSFER_INFO *PWIA_EXTENDED_TRANSFER_INFO;



extern RPC_IF_HANDLE __MIDL_itf_wia_xp_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wia_xp_0000_0004_v0_0_s_ifspec;

#ifndef __IWiaDataTransfer_INTERFACE_DEFINED__
#define __IWiaDataTransfer_INTERFACE_DEFINED__

/* interface IWiaDataTransfer */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWiaDataTransfer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a6cef998-a5b0-11d2-a08f-00c04f72dc3c")
    IWiaDataTransfer : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE idtGetData( 
            /* [out][in] */ LPSTGMEDIUM pMedium,
            /* [unique][in] */ IWiaDataCallback *pIWiaDataCallback) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE idtGetBandedData( 
            /* [unique][in] */ PWIA_DATA_TRANSFER_INFO pWiaDataTransInfo,
            /* [unique][in] */ IWiaDataCallback *pIWiaDataCallback) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE idtQueryGetData( 
            /* [unique][in] */ __RPC__in_opt WIA_FORMAT_INFO *pfe) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE idtEnumWIA_FORMAT_INFO( 
            /* [out] */ __RPC__deref_out_opt IEnumWIA_FORMAT_INFO **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE idtGetExtendedTransferInfo( 
            /* [out] */ __RPC__out PWIA_EXTENDED_TRANSFER_INFO pExtendedTransferInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWiaDataTransferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWiaDataTransfer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWiaDataTransfer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWiaDataTransfer * This);
        
        DECLSPEC_XFGVIRT(IWiaDataTransfer, idtGetData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *idtGetData )( 
            IWiaDataTransfer * This,
            /* [out][in] */ LPSTGMEDIUM pMedium,
            /* [unique][in] */ IWiaDataCallback *pIWiaDataCallback);
        
        DECLSPEC_XFGVIRT(IWiaDataTransfer, idtGetBandedData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *idtGetBandedData )( 
            IWiaDataTransfer * This,
            /* [unique][in] */ PWIA_DATA_TRANSFER_INFO pWiaDataTransInfo,
            /* [unique][in] */ IWiaDataCallback *pIWiaDataCallback);
        
        DECLSPEC_XFGVIRT(IWiaDataTransfer, idtQueryGetData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *idtQueryGetData )( 
            __RPC__in IWiaDataTransfer * This,
            /* [unique][in] */ __RPC__in_opt WIA_FORMAT_INFO *pfe);
        
        DECLSPEC_XFGVIRT(IWiaDataTransfer, idtEnumWIA_FORMAT_INFO)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *idtEnumWIA_FORMAT_INFO )( 
            __RPC__in IWiaDataTransfer * This,
            /* [out] */ __RPC__deref_out_opt IEnumWIA_FORMAT_INFO **ppEnum);
        
        DECLSPEC_XFGVIRT(IWiaDataTransfer, idtGetExtendedTransferInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *idtGetExtendedTransferInfo )( 
            __RPC__in IWiaDataTransfer * This,
            /* [out] */ __RPC__out PWIA_EXTENDED_TRANSFER_INFO pExtendedTransferInfo);
        
        END_INTERFACE
    } IWiaDataTransferVtbl;

    interface IWiaDataTransfer
    {
        CONST_VTBL struct IWiaDataTransferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWiaDataTransfer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWiaDataTransfer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWiaDataTransfer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWiaDataTransfer_idtGetData(This,pMedium,pIWiaDataCallback)	\
    ( (This)->lpVtbl -> idtGetData(This,pMedium,pIWiaDataCallback) ) 

#define IWiaDataTransfer_idtGetBandedData(This,pWiaDataTransInfo,pIWiaDataCallback)	\
    ( (This)->lpVtbl -> idtGetBandedData(This,pWiaDataTransInfo,pIWiaDataCallback) ) 

#define IWiaDataTransfer_idtQueryGetData(This,pfe)	\
    ( (This)->lpVtbl -> idtQueryGetData(This,pfe) ) 

#define IWiaDataTransfer_idtEnumWIA_FORMAT_INFO(This,ppEnum)	\
    ( (This)->lpVtbl -> idtEnumWIA_FORMAT_INFO(This,ppEnum) ) 

#define IWiaDataTransfer_idtGetExtendedTransferInfo(This,pExtendedTransferInfo)	\
    ( (This)->lpVtbl -> idtGetExtendedTransferInfo(This,pExtendedTransferInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDataTransfer_idtGetDataEx_Proxy( 
    __RPC__in IWiaDataTransfer * This,
    /* [out][in] */ __RPC__inout LPSTGMEDIUM pMedium,
    /* [unique][in] */ __RPC__in_opt IWiaDataCallback *pIWiaDataCallback);


void __RPC_STUB IWiaDataTransfer_idtGetDataEx_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDataTransfer_idtGetBandedDataEx_Proxy( 
    __RPC__in IWiaDataTransfer * This,
    /* [unique][in] */ __RPC__in_opt PWIA_DATA_TRANSFER_INFO pWiaDataTransInfo,
    /* [unique][in] */ __RPC__in_opt IWiaDataCallback *pIWiaDataCallback);


void __RPC_STUB IWiaDataTransfer_idtGetBandedDataEx_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IWiaDataTransfer_INTERFACE_DEFINED__ */


#ifndef __IWiaItem_INTERFACE_DEFINED__
#define __IWiaItem_INTERFACE_DEFINED__

/* interface IWiaItem */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWiaItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4db1ad10-3391-11d2-9a33-00c04fa36145")
    IWiaItem : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetItemType( 
            /* [out] */ __RPC__out LONG *pItemType) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AnalyzeItem( 
            /* [in] */ LONG lFlags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EnumChildItems( 
            /* [out] */ __RPC__deref_out_opt IEnumWiaItem **ppIEnumWiaItem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DeleteItem( 
            /* [in] */ LONG lFlags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateChildItem( 
            /* [in] */ LONG lFlags,
            /* [in] */ __RPC__in BSTR bstrItemName,
            /* [in] */ __RPC__in BSTR bstrFullItemName,
            /* [out] */ __RPC__deref_out_opt IWiaItem **ppIWiaItem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EnumRegisterEventInfo( 
            /* [in] */ LONG lFlags,
            /* [in] */ __RPC__in const GUID *pEventGUID,
            /* [out] */ __RPC__deref_out_opt IEnumWIA_DEV_CAPS **ppIEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE FindItemByName( 
            /* [in] */ LONG lFlags,
            /* [in] */ __RPC__in BSTR bstrFullItemName,
            /* [out] */ __RPC__deref_out_opt IWiaItem **ppIWiaItem) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE DeviceDlg( 
            /* [in] */ HWND hwndParent,
            /* [in] */ LONG lFlags,
            /* [in] */ LONG lIntent,
            /* [out] */ LONG *plItemCount,
            /* [out] */ IWiaItem ***ppIWiaItem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DeviceCommand( 
            /* [in] */ LONG lFlags,
            /* [in] */ __RPC__in const GUID *pCmdGUID,
            /* [out][in] */ __RPC__deref_inout_opt IWiaItem **pIWiaItem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetRootItem( 
            /* [out] */ __RPC__deref_out_opt IWiaItem **ppIWiaItem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EnumDeviceCapabilities( 
            /* [in] */ LONG lFlags,
            /* [out] */ __RPC__deref_out_opt IEnumWIA_DEV_CAPS **ppIEnumWIA_DEV_CAPS) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DumpItemData( 
            /* [out] */ __RPC__deref_out_opt BSTR *bstrData) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DumpDrvItemData( 
            /* [out] */ __RPC__deref_out_opt BSTR *bstrData) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DumpTreeItemData( 
            /* [out] */ __RPC__deref_out_opt BSTR *bstrData) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Diagnostic( 
            /* [in] */ ULONG ulSize,
            /* [size_is][in] */ __RPC__in_ecount_full(ulSize) BYTE *pBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWiaItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWiaItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWiaItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWiaItem * This);
        
        DECLSPEC_XFGVIRT(IWiaItem, GetItemType)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            __RPC__in IWiaItem * This,
            /* [out] */ __RPC__out LONG *pItemType);
        
        DECLSPEC_XFGVIRT(IWiaItem, AnalyzeItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AnalyzeItem )( 
            __RPC__in IWiaItem * This,
            /* [in] */ LONG lFlags);
        
        DECLSPEC_XFGVIRT(IWiaItem, EnumChildItems)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnumChildItems )( 
            __RPC__in IWiaItem * This,
            /* [out] */ __RPC__deref_out_opt IEnumWiaItem **ppIEnumWiaItem);
        
        DECLSPEC_XFGVIRT(IWiaItem, DeleteItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in IWiaItem * This,
            /* [in] */ LONG lFlags);
        
        DECLSPEC_XFGVIRT(IWiaItem, CreateChildItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateChildItem )( 
            __RPC__in IWiaItem * This,
            /* [in] */ LONG lFlags,
            /* [in] */ __RPC__in BSTR bstrItemName,
            /* [in] */ __RPC__in BSTR bstrFullItemName,
            /* [out] */ __RPC__deref_out_opt IWiaItem **ppIWiaItem);
        
        DECLSPEC_XFGVIRT(IWiaItem, EnumRegisterEventInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnumRegisterEventInfo )( 
            __RPC__in IWiaItem * This,
            /* [in] */ LONG lFlags,
            /* [in] */ __RPC__in const GUID *pEventGUID,
            /* [out] */ __RPC__deref_out_opt IEnumWIA_DEV_CAPS **ppIEnum);
        
        DECLSPEC_XFGVIRT(IWiaItem, FindItemByName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindItemByName )( 
            __RPC__in IWiaItem * This,
            /* [in] */ LONG lFlags,
            /* [in] */ __RPC__in BSTR bstrFullItemName,
            /* [out] */ __RPC__deref_out_opt IWiaItem **ppIWiaItem);
        
        DECLSPEC_XFGVIRT(IWiaItem, DeviceDlg)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *DeviceDlg )( 
            IWiaItem * This,
            /* [in] */ HWND hwndParent,
            /* [in] */ LONG lFlags,
            /* [in] */ LONG lIntent,
            /* [out] */ LONG *plItemCount,
            /* [out] */ IWiaItem ***ppIWiaItem);
        
        DECLSPEC_XFGVIRT(IWiaItem, DeviceCommand)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DeviceCommand )( 
            __RPC__in IWiaItem * This,
            /* [in] */ LONG lFlags,
            /* [in] */ __RPC__in const GUID *pCmdGUID,
            /* [out][in] */ __RPC__deref_inout_opt IWiaItem **pIWiaItem);
        
        DECLSPEC_XFGVIRT(IWiaItem, GetRootItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetRootItem )( 
            __RPC__in IWiaItem * This,
            /* [out] */ __RPC__deref_out_opt IWiaItem **ppIWiaItem);
        
        DECLSPEC_XFGVIRT(IWiaItem, EnumDeviceCapabilities)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnumDeviceCapabilities )( 
            __RPC__in IWiaItem * This,
            /* [in] */ LONG lFlags,
            /* [out] */ __RPC__deref_out_opt IEnumWIA_DEV_CAPS **ppIEnumWIA_DEV_CAPS);
        
        DECLSPEC_XFGVIRT(IWiaItem, DumpItemData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DumpItemData )( 
            __RPC__in IWiaItem * This,
            /* [out] */ __RPC__deref_out_opt BSTR *bstrData);
        
        DECLSPEC_XFGVIRT(IWiaItem, DumpDrvItemData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DumpDrvItemData )( 
            __RPC__in IWiaItem * This,
            /* [out] */ __RPC__deref_out_opt BSTR *bstrData);
        
        DECLSPEC_XFGVIRT(IWiaItem, DumpTreeItemData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DumpTreeItemData )( 
            __RPC__in IWiaItem * This,
            /* [out] */ __RPC__deref_out_opt BSTR *bstrData);
        
        DECLSPEC_XFGVIRT(IWiaItem, Diagnostic)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Diagnostic )( 
            __RPC__in IWiaItem * This,
            /* [in] */ ULONG ulSize,
            /* [size_is][in] */ __RPC__in_ecount_full(ulSize) BYTE *pBuffer);
        
        END_INTERFACE
    } IWiaItemVtbl;

    interface IWiaItem
    {
        CONST_VTBL struct IWiaItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWiaItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWiaItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWiaItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWiaItem_GetItemType(This,pItemType)	\
    ( (This)->lpVtbl -> GetItemType(This,pItemType) ) 

#define IWiaItem_AnalyzeItem(This,lFlags)	\
    ( (This)->lpVtbl -> AnalyzeItem(This,lFlags) ) 

#define IWiaItem_EnumChildItems(This,ppIEnumWiaItem)	\
    ( (This)->lpVtbl -> EnumChildItems(This,ppIEnumWiaItem) ) 

#define IWiaItem_DeleteItem(This,lFlags)	\
    ( (This)->lpVtbl -> DeleteItem(This,lFlags) ) 

#define IWiaItem_CreateChildItem(This,lFlags,bstrItemName,bstrFullItemName,ppIWiaItem)	\
    ( (This)->lpVtbl -> CreateChildItem(This,lFlags,bstrItemName,bstrFullItemName,ppIWiaItem) ) 

#define IWiaItem_EnumRegisterEventInfo(This,lFlags,pEventGUID,ppIEnum)	\
    ( (This)->lpVtbl -> EnumRegisterEventInfo(This,lFlags,pEventGUID,ppIEnum) ) 

#define IWiaItem_FindItemByName(This,lFlags,bstrFullItemName,ppIWiaItem)	\
    ( (This)->lpVtbl -> FindItemByName(This,lFlags,bstrFullItemName,ppIWiaItem) ) 

#define IWiaItem_DeviceDlg(This,hwndParent,lFlags,lIntent,plItemCount,ppIWiaItem)	\
    ( (This)->lpVtbl -> DeviceDlg(This,hwndParent,lFlags,lIntent,plItemCount,ppIWiaItem) ) 

#define IWiaItem_DeviceCommand(This,lFlags,pCmdGUID,pIWiaItem)	\
    ( (This)->lpVtbl -> DeviceCommand(This,lFlags,pCmdGUID,pIWiaItem) ) 

#define IWiaItem_GetRootItem(This,ppIWiaItem)	\
    ( (This)->lpVtbl -> GetRootItem(This,ppIWiaItem) ) 

#define IWiaItem_EnumDeviceCapabilities(This,lFlags,ppIEnumWIA_DEV_CAPS)	\
    ( (This)->lpVtbl -> EnumDeviceCapabilities(This,lFlags,ppIEnumWIA_DEV_CAPS) ) 

#define IWiaItem_DumpItemData(This,bstrData)	\
    ( (This)->lpVtbl -> DumpItemData(This,bstrData) ) 

#define IWiaItem_DumpDrvItemData(This,bstrData)	\
    ( (This)->lpVtbl -> DumpDrvItemData(This,bstrData) ) 

#define IWiaItem_DumpTreeItemData(This,bstrData)	\
    ( (This)->lpVtbl -> DumpTreeItemData(This,bstrData) ) 

#define IWiaItem_Diagnostic(This,ulSize,pBuffer)	\
    ( (This)->lpVtbl -> Diagnostic(This,ulSize,pBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [nocode][helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaItem_LocalDeviceDlg_Proxy( 
    __RPC__in IWiaItem * This,
    /* [in] */ __RPC__in HWND hwndParent,
    /* [in] */ LONG lFlags,
    /* [in] */ LONG lIntent,
    /* [out] */ __RPC__out LONG *plItemCount,
    /* [out] */ __RPC__deref_out_opt IWiaItem ***pIWiaItem);


void __RPC_STUB IWiaItem_LocalDeviceDlg_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IWiaItem_INTERFACE_DEFINED__ */


#ifndef __IWiaPropertyStorage_INTERFACE_DEFINED__
#define __IWiaPropertyStorage_INTERFACE_DEFINED__

/* interface IWiaPropertyStorage */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWiaPropertyStorage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("98B5E8A0-29CC-491a-AAC0-E6DB4FDCCEB6")
    IWiaPropertyStorage : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ReadMultiple( 
            /* [in] */ ULONG cpspec,
            /* [size_is][in] */ __RPC__in_ecount_full(cpspec) const PROPSPEC rgpspec[  ],
            /* [size_is][out] */ __RPC__out_ecount_full(cpspec) PROPVARIANT rgpropvar[  ]) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE WriteMultiple( 
            /* [in] */ ULONG cpspec,
            /* [size_is][in] */ const PROPSPEC rgpspec[  ],
            /* [size_is][in] */ const PROPVARIANT rgpropvar[  ],
            /* [in] */ PROPID propidNameFirst) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteMultiple( 
            /* [in] */ ULONG cpspec,
            /* [size_is][in] */ __RPC__in_ecount_full(cpspec) const PROPSPEC rgpspec[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReadPropertyNames( 
            /* [in] */ ULONG cpropid,
            /* [size_is][in] */ __RPC__in_ecount_full(cpropid) const PROPID rgpropid[  ],
            /* [size_is][out] */ __RPC__out_ecount_full(cpropid) LPOLESTR rglpwstrName[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WritePropertyNames( 
            /* [in] */ ULONG cpropid,
            /* [size_is][in] */ __RPC__in_ecount_full(cpropid) const PROPID rgpropid[  ],
            /* [size_is][in] */ __RPC__in_ecount_full(cpropid) const LPOLESTR rglpwstrName[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePropertyNames( 
            /* [in] */ ULONG cpropid,
            /* [size_is][in] */ __RPC__in_ecount_full(cpropid) const PROPID rgpropid[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Commit( 
            /* [in] */ DWORD grfCommitFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Revert( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Enum( 
            /* [out] */ __RPC__deref_out_opt IEnumSTATPROPSTG **ppenum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTimes( 
            /* [in] */ __RPC__in const FILETIME *pctime,
            /* [in] */ __RPC__in const FILETIME *patime,
            /* [in] */ __RPC__in const FILETIME *pmtime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetClass( 
            /* [in] */ __RPC__in REFCLSID clsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stat( 
            /* [out] */ __RPC__out STATPROPSETSTG *pstatpsstg) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPropertyAttributes( 
            /* [in] */ ULONG cpspec,
            /* [size_is][in] */ __RPC__in_ecount_full(cpspec) PROPSPEC rgpspec[  ],
            /* [size_is][out] */ __RPC__out_ecount_full(cpspec) ULONG rgflags[  ],
            /* [size_is][out] */ __RPC__out_ecount_full(cpspec) PROPVARIANT rgpropvar[  ]) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out ULONG *pulNumProps) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPropertyStream( 
            /* [out] */ __RPC__out GUID *pCompatibilityId,
            /* [out] */ __RPC__deref_out_opt IStream **ppIStream) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE SetPropertyStream( 
            /* [in] */ GUID *pCompatibilityId,
            /* [unique][in] */ IStream *pIStream) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWiaPropertyStorageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWiaPropertyStorage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWiaPropertyStorage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWiaPropertyStorage * This);
        
        DECLSPEC_XFGVIRT(IWiaPropertyStorage, ReadMultiple)
        HRESULT ( STDMETHODCALLTYPE *ReadMultiple )( 
            __RPC__in IWiaPropertyStorage * This,
            /* [in] */ ULONG cpspec,
            /* [size_is][in] */ __RPC__in_ecount_full(cpspec) const PROPSPEC rgpspec[  ],
            /* [size_is][out] */ __RPC__out_ecount_full(cpspec) PROPVARIANT rgpropvar[  ]);
        
        DECLSPEC_XFGVIRT(IWiaPropertyStorage, WriteMultiple)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *WriteMultiple )( 
            IWiaPropertyStorage * This,
            /* [in] */ ULONG cpspec,
            /* [size_is][in] */ const PROPSPEC rgpspec[  ],
            /* [size_is][in] */ const PROPVARIANT rgpropvar[  ],
            /* [in] */ PROPID propidNameFirst);
        
        DECLSPEC_XFGVIRT(IWiaPropertyStorage, DeleteMultiple)
        HRESULT ( STDMETHODCALLTYPE *DeleteMultiple )( 
            __RPC__in IWiaPropertyStorage * This,
            /* [in] */ ULONG cpspec,
            /* [size_is][in] */ __RPC__in_ecount_full(cpspec) const PROPSPEC rgpspec[  ]);
        
        DECLSPEC_XFGVIRT(IWiaPropertyStorage, ReadPropertyNames)
        HRESULT ( STDMETHODCALLTYPE *ReadPropertyNames )( 
            __RPC__in IWiaPropertyStorage * This,
            /* [in] */ ULONG cpropid,
            /* [size_is][in] */ __RPC__in_ecount_full(cpropid) const PROPID rgpropid[  ],
            /* [size_is][out] */ __RPC__out_ecount_full(cpropid) LPOLESTR rglpwstrName[  ]);
        
        DECLSPEC_XFGVIRT(IWiaPropertyStorage, WritePropertyNames)
        HRESULT ( STDMETHODCALLTYPE *WritePropertyNames )( 
            __RPC__in IWiaPropertyStorage * This,
            /* [in] */ ULONG cpropid,
            /* [size_is][in] */ __RPC__in_ecount_full(cpropid) const PROPID rgpropid[  ],
            /* [size_is][in] */ __RPC__in_ecount_full(cpropid) const LPOLESTR rglpwstrName[  ]);
        
        DECLSPEC_XFGVIRT(IWiaPropertyStorage, DeletePropertyNames)
        HRESULT ( STDMETHODCALLTYPE *DeletePropertyNames )( 
            __RPC__in IWiaPropertyStorage * This,
            /* [in] */ ULONG cpropid,
            /* [size_is][in] */ __RPC__in_ecount_full(cpropid) const PROPID rgpropid[  ]);
        
        DECLSPEC_XFGVIRT(IWiaPropertyStorage, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IWiaPropertyStorage * This,
            /* [in] */ DWORD grfCommitFlags);
        
        DECLSPEC_XFGVIRT(IWiaPropertyStorage, Revert)
        HRESULT ( STDMETHODCALLTYPE *Revert )( 
            __RPC__in IWiaPropertyStorage * This);
        
        DECLSPEC_XFGVIRT(IWiaPropertyStorage, Enum)
        HRESULT ( STDMETHODCALLTYPE *Enum )( 
            __RPC__in IWiaPropertyStorage * This,
            /* [out] */ __RPC__deref_out_opt IEnumSTATPROPSTG **ppenum);
        
        DECLSPEC_XFGVIRT(IWiaPropertyStorage, SetTimes)
        HRESULT ( STDMETHODCALLTYPE *SetTimes )( 
            __RPC__in IWiaPropertyStorage * This,
            /* [in] */ __RPC__in const FILETIME *pctime,
            /* [in] */ __RPC__in const FILETIME *patime,
            /* [in] */ __RPC__in const FILETIME *pmtime);
        
        DECLSPEC_XFGVIRT(IWiaPropertyStorage, SetClass)
        HRESULT ( STDMETHODCALLTYPE *SetClass )( 
            __RPC__in IWiaPropertyStorage * This,
            /* [in] */ __RPC__in REFCLSID clsid);
        
        DECLSPEC_XFGVIRT(IWiaPropertyStorage, Stat)
        HRESULT ( STDMETHODCALLTYPE *Stat )( 
            __RPC__in IWiaPropertyStorage * This,
            /* [out] */ __RPC__out STATPROPSETSTG *pstatpsstg);
        
        DECLSPEC_XFGVIRT(IWiaPropertyStorage, GetPropertyAttributes)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyAttributes )( 
            __RPC__in IWiaPropertyStorage * This,
            /* [in] */ ULONG cpspec,
            /* [size_is][in] */ __RPC__in_ecount_full(cpspec) PROPSPEC rgpspec[  ],
            /* [size_is][out] */ __RPC__out_ecount_full(cpspec) ULONG rgflags[  ],
            /* [size_is][out] */ __RPC__out_ecount_full(cpspec) PROPVARIANT rgpropvar[  ]);
        
        DECLSPEC_XFGVIRT(IWiaPropertyStorage, GetCount)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IWiaPropertyStorage * This,
            /* [out] */ __RPC__out ULONG *pulNumProps);
        
        DECLSPEC_XFGVIRT(IWiaPropertyStorage, GetPropertyStream)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyStream )( 
            __RPC__in IWiaPropertyStorage * This,
            /* [out] */ __RPC__out GUID *pCompatibilityId,
            /* [out] */ __RPC__deref_out_opt IStream **ppIStream);
        
        DECLSPEC_XFGVIRT(IWiaPropertyStorage, SetPropertyStream)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetPropertyStream )( 
            IWiaPropertyStorage * This,
            /* [in] */ GUID *pCompatibilityId,
            /* [unique][in] */ IStream *pIStream);
        
        END_INTERFACE
    } IWiaPropertyStorageVtbl;

    interface IWiaPropertyStorage
    {
        CONST_VTBL struct IWiaPropertyStorageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWiaPropertyStorage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWiaPropertyStorage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWiaPropertyStorage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWiaPropertyStorage_ReadMultiple(This,cpspec,rgpspec,rgpropvar)	\
    ( (This)->lpVtbl -> ReadMultiple(This,cpspec,rgpspec,rgpropvar) ) 

#define IWiaPropertyStorage_WriteMultiple(This,cpspec,rgpspec,rgpropvar,propidNameFirst)	\
    ( (This)->lpVtbl -> WriteMultiple(This,cpspec,rgpspec,rgpropvar,propidNameFirst) ) 

#define IWiaPropertyStorage_DeleteMultiple(This,cpspec,rgpspec)	\
    ( (This)->lpVtbl -> DeleteMultiple(This,cpspec,rgpspec) ) 

#define IWiaPropertyStorage_ReadPropertyNames(This,cpropid,rgpropid,rglpwstrName)	\
    ( (This)->lpVtbl -> ReadPropertyNames(This,cpropid,rgpropid,rglpwstrName) ) 

#define IWiaPropertyStorage_WritePropertyNames(This,cpropid,rgpropid,rglpwstrName)	\
    ( (This)->lpVtbl -> WritePropertyNames(This,cpropid,rgpropid,rglpwstrName) ) 

#define IWiaPropertyStorage_DeletePropertyNames(This,cpropid,rgpropid)	\
    ( (This)->lpVtbl -> DeletePropertyNames(This,cpropid,rgpropid) ) 

#define IWiaPropertyStorage_Commit(This,grfCommitFlags)	\
    ( (This)->lpVtbl -> Commit(This,grfCommitFlags) ) 

#define IWiaPropertyStorage_Revert(This)	\
    ( (This)->lpVtbl -> Revert(This) ) 

#define IWiaPropertyStorage_Enum(This,ppenum)	\
    ( (This)->lpVtbl -> Enum(This,ppenum) ) 

#define IWiaPropertyStorage_SetTimes(This,pctime,patime,pmtime)	\
    ( (This)->lpVtbl -> SetTimes(This,pctime,patime,pmtime) ) 

#define IWiaPropertyStorage_SetClass(This,clsid)	\
    ( (This)->lpVtbl -> SetClass(This,clsid) ) 

#define IWiaPropertyStorage_Stat(This,pstatpsstg)	\
    ( (This)->lpVtbl -> Stat(This,pstatpsstg) ) 

#define IWiaPropertyStorage_GetPropertyAttributes(This,cpspec,rgpspec,rgflags,rgpropvar)	\
    ( (This)->lpVtbl -> GetPropertyAttributes(This,cpspec,rgpspec,rgflags,rgpropvar) ) 

#define IWiaPropertyStorage_GetCount(This,pulNumProps)	\
    ( (This)->lpVtbl -> GetCount(This,pulNumProps) ) 

#define IWiaPropertyStorage_GetPropertyStream(This,pCompatibilityId,ppIStream)	\
    ( (This)->lpVtbl -> GetPropertyStream(This,pCompatibilityId,ppIStream) ) 

#define IWiaPropertyStorage_SetPropertyStream(This,pCompatibilityId,pIStream)	\
    ( (This)->lpVtbl -> SetPropertyStream(This,pCompatibilityId,pIStream) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IWiaPropertyStorage_RemoteWriteMultiple_Proxy( 
    __RPC__in IWiaPropertyStorage * This,
    /* [in] */ ULONG cpspec,
    /* [size_is][in] */ __RPC__in_ecount_full(cpspec) const PROPSPEC *rgpspec,
    /* [size_is][in] */ __RPC__in_ecount_full(cpspec) const PROPVARIANT *rgpropvar,
    /* [in] */ PROPID propidNameFirst);


void __RPC_STUB IWiaPropertyStorage_RemoteWriteMultiple_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaPropertyStorage_RemoteSetPropertyStream_Proxy( 
    __RPC__in IWiaPropertyStorage * This,
    /* [in] */ __RPC__in GUID *pCompatibilityId,
    /* [unique][in] */ __RPC__in_opt IStream *pIStream);


void __RPC_STUB IWiaPropertyStorage_RemoteSetPropertyStream_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IWiaPropertyStorage_INTERFACE_DEFINED__ */


#ifndef __IEnumWiaItem_INTERFACE_DEFINED__
#define __IEnumWiaItem_INTERFACE_DEFINED__

/* interface IEnumWiaItem */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEnumWiaItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5e8383fc-3391-11d2-9a33-00c04fa36145")
    IEnumWiaItem : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ IWiaItem **ppIWiaItem,
            /* [unique][out][in] */ ULONG *pceltFetched) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumWiaItem **ppIEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out ULONG *celt) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumWiaItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumWiaItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumWiaItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumWiaItem * This);
        
        DECLSPEC_XFGVIRT(IEnumWiaItem, Next)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumWiaItem * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ IWiaItem **ppIWiaItem,
            /* [unique][out][in] */ ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumWiaItem, Skip)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumWiaItem * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumWiaItem, Reset)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumWiaItem * This);
        
        DECLSPEC_XFGVIRT(IEnumWiaItem, Clone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumWiaItem * This,
            /* [out] */ __RPC__deref_out_opt IEnumWiaItem **ppIEnum);
        
        DECLSPEC_XFGVIRT(IEnumWiaItem, GetCount)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IEnumWiaItem * This,
            /* [out] */ __RPC__out ULONG *celt);
        
        END_INTERFACE
    } IEnumWiaItemVtbl;

    interface IEnumWiaItem
    {
        CONST_VTBL struct IEnumWiaItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumWiaItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumWiaItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumWiaItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumWiaItem_Next(This,celt,ppIWiaItem,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppIWiaItem,pceltFetched) ) 

#define IEnumWiaItem_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumWiaItem_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumWiaItem_Clone(This,ppIEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppIEnum) ) 

#define IEnumWiaItem_GetCount(This,celt)	\
    ( (This)->lpVtbl -> GetCount(This,celt) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IEnumWiaItem_RemoteNext_Proxy( 
    __RPC__in IEnumWiaItem * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IWiaItem **ppIWiaItem,
    /* [unique][out][in] */ __RPC__inout_opt ULONG *pceltFetched);


void __RPC_STUB IEnumWiaItem_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumWiaItem_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wia_xp_0000_0008 */
/* [local] */ 

typedef struct _WIA_DEV_CAP
    {
    GUID guid;
    ULONG ulFlags;
    BSTR bstrName;
    BSTR bstrDescription;
    BSTR bstrIcon;
    BSTR bstrCommandline;
    } 	WIA_DEV_CAP;

typedef struct _WIA_DEV_CAP *PWIA_DEV_CAP;

typedef struct _WIA_DEV_CAP WIA_EVENT_HANDLER;

typedef struct _WIA_DEV_CAP *PWIA_EVENT_HANDLER;



extern RPC_IF_HANDLE __MIDL_itf_wia_xp_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wia_xp_0000_0008_v0_0_s_ifspec;

#ifndef __IEnumWIA_DEV_CAPS_INTERFACE_DEFINED__
#define __IEnumWIA_DEV_CAPS_INTERFACE_DEFINED__

/* interface IEnumWIA_DEV_CAPS */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEnumWIA_DEV_CAPS;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1fcc4287-aca6-11d2-a093-00c04f72dc3c")
    IEnumWIA_DEV_CAPS : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ WIA_DEV_CAP *rgelt,
            /* [unique][out][in] */ ULONG *pceltFetched) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumWIA_DEV_CAPS **ppIEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out ULONG *pcelt) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumWIA_DEV_CAPSVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumWIA_DEV_CAPS * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumWIA_DEV_CAPS * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumWIA_DEV_CAPS * This);
        
        DECLSPEC_XFGVIRT(IEnumWIA_DEV_CAPS, Next)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumWIA_DEV_CAPS * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ WIA_DEV_CAP *rgelt,
            /* [unique][out][in] */ ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumWIA_DEV_CAPS, Skip)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumWIA_DEV_CAPS * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumWIA_DEV_CAPS, Reset)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumWIA_DEV_CAPS * This);
        
        DECLSPEC_XFGVIRT(IEnumWIA_DEV_CAPS, Clone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumWIA_DEV_CAPS * This,
            /* [out] */ __RPC__deref_out_opt IEnumWIA_DEV_CAPS **ppIEnum);
        
        DECLSPEC_XFGVIRT(IEnumWIA_DEV_CAPS, GetCount)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IEnumWIA_DEV_CAPS * This,
            /* [out] */ __RPC__out ULONG *pcelt);
        
        END_INTERFACE
    } IEnumWIA_DEV_CAPSVtbl;

    interface IEnumWIA_DEV_CAPS
    {
        CONST_VTBL struct IEnumWIA_DEV_CAPSVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumWIA_DEV_CAPS_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumWIA_DEV_CAPS_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumWIA_DEV_CAPS_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumWIA_DEV_CAPS_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumWIA_DEV_CAPS_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumWIA_DEV_CAPS_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumWIA_DEV_CAPS_Clone(This,ppIEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppIEnum) ) 

#define IEnumWIA_DEV_CAPS_GetCount(This,pcelt)	\
    ( (This)->lpVtbl -> GetCount(This,pcelt) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IEnumWIA_DEV_CAPS_RemoteNext_Proxy( 
    __RPC__in IEnumWIA_DEV_CAPS * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) WIA_DEV_CAP *rgelt,
    /* [unique][out][in] */ __RPC__inout_opt ULONG *pceltFetched);


void __RPC_STUB IEnumWIA_DEV_CAPS_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumWIA_DEV_CAPS_INTERFACE_DEFINED__ */


#ifndef __IEnumWIA_FORMAT_INFO_INTERFACE_DEFINED__
#define __IEnumWIA_FORMAT_INFO_INTERFACE_DEFINED__

/* interface IEnumWIA_FORMAT_INFO */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEnumWIA_FORMAT_INFO;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("81BEFC5B-656D-44f1-B24C-D41D51B4DC81")
    IEnumWIA_FORMAT_INFO : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ WIA_FORMAT_INFO *rgelt,
            /* [unique][out][in] */ ULONG *pceltFetched) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumWIA_FORMAT_INFO **ppIEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out ULONG *pcelt) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumWIA_FORMAT_INFOVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumWIA_FORMAT_INFO * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumWIA_FORMAT_INFO * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumWIA_FORMAT_INFO * This);
        
        DECLSPEC_XFGVIRT(IEnumWIA_FORMAT_INFO, Next)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumWIA_FORMAT_INFO * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ WIA_FORMAT_INFO *rgelt,
            /* [unique][out][in] */ ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumWIA_FORMAT_INFO, Skip)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumWIA_FORMAT_INFO * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumWIA_FORMAT_INFO, Reset)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumWIA_FORMAT_INFO * This);
        
        DECLSPEC_XFGVIRT(IEnumWIA_FORMAT_INFO, Clone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumWIA_FORMAT_INFO * This,
            /* [out] */ __RPC__deref_out_opt IEnumWIA_FORMAT_INFO **ppIEnum);
        
        DECLSPEC_XFGVIRT(IEnumWIA_FORMAT_INFO, GetCount)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IEnumWIA_FORMAT_INFO * This,
            /* [out] */ __RPC__out ULONG *pcelt);
        
        END_INTERFACE
    } IEnumWIA_FORMAT_INFOVtbl;

    interface IEnumWIA_FORMAT_INFO
    {
        CONST_VTBL struct IEnumWIA_FORMAT_INFOVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumWIA_FORMAT_INFO_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumWIA_FORMAT_INFO_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumWIA_FORMAT_INFO_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumWIA_FORMAT_INFO_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumWIA_FORMAT_INFO_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumWIA_FORMAT_INFO_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumWIA_FORMAT_INFO_Clone(This,ppIEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppIEnum) ) 

#define IEnumWIA_FORMAT_INFO_GetCount(This,pcelt)	\
    ( (This)->lpVtbl -> GetCount(This,pcelt) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IEnumWIA_FORMAT_INFO_RemoteNext_Proxy( 
    __RPC__in IEnumWIA_FORMAT_INFO * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) WIA_FORMAT_INFO *rgelt,
    /* [unique][out][in] */ __RPC__inout_opt ULONG *pceltFetched);


void __RPC_STUB IEnumWIA_FORMAT_INFO_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumWIA_FORMAT_INFO_INTERFACE_DEFINED__ */


#ifndef __IWiaLog_INTERFACE_DEFINED__
#define __IWiaLog_INTERFACE_DEFINED__

/* interface IWiaLog */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWiaLog;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A00C10B6-82A1-452f-8B6C-86062AAD6890")
    IWiaLog : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE InitializeLog( 
            /* [in] */ LONG hInstance) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE hResult( 
            /* [in] */ HRESULT hResult) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Log( 
            /* [in] */ LONG lFlags,
            /* [in] */ LONG lResID,
            /* [in] */ LONG lDetail,
            /* [in] */ __RPC__in BSTR bstrText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWiaLogVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWiaLog * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWiaLog * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWiaLog * This);
        
        DECLSPEC_XFGVIRT(IWiaLog, InitializeLog)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InitializeLog )( 
            __RPC__in IWiaLog * This,
            /* [in] */ LONG hInstance);
        
        DECLSPEC_XFGVIRT(IWiaLog, hResult)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *hResult )( 
            __RPC__in IWiaLog * This,
            /* [in] */ HRESULT hResult);
        
        DECLSPEC_XFGVIRT(IWiaLog, Log)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Log )( 
            __RPC__in IWiaLog * This,
            /* [in] */ LONG lFlags,
            /* [in] */ LONG lResID,
            /* [in] */ LONG lDetail,
            /* [in] */ __RPC__in BSTR bstrText);
        
        END_INTERFACE
    } IWiaLogVtbl;

    interface IWiaLog
    {
        CONST_VTBL struct IWiaLogVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWiaLog_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWiaLog_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWiaLog_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWiaLog_InitializeLog(This,hInstance)	\
    ( (This)->lpVtbl -> InitializeLog(This,hInstance) ) 

#define IWiaLog_hResult(This,hResult)	\
    ( (This)->lpVtbl -> hResult(This,hResult) ) 

#define IWiaLog_Log(This,lFlags,lResID,lDetail,bstrText)	\
    ( (This)->lpVtbl -> Log(This,lFlags,lResID,lDetail,bstrText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWiaLog_INTERFACE_DEFINED__ */


#ifndef __IWiaLogEx_INTERFACE_DEFINED__
#define __IWiaLogEx_INTERFACE_DEFINED__

/* interface IWiaLogEx */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWiaLogEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AF1F22AC-7A40-4787-B421-AEb47A1FBD0B")
    IWiaLogEx : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE InitializeLogEx( 
            /* [in] */ __RPC__in BYTE *hInstance) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE hResult( 
            /* [in] */ HRESULT hResult) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Log( 
            /* [in] */ LONG lFlags,
            /* [in] */ LONG lResID,
            /* [in] */ LONG lDetail,
            /* [in] */ __RPC__in BSTR bstrText) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE hResultEx( 
            /* [in] */ LONG lMethodId,
            /* [in] */ HRESULT hResult) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE LogEx( 
            /* [in] */ LONG lMethodId,
            /* [in] */ LONG lFlags,
            /* [in] */ LONG lResID,
            /* [in] */ LONG lDetail,
            /* [in] */ __RPC__in BSTR bstrText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWiaLogExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWiaLogEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWiaLogEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWiaLogEx * This);
        
        DECLSPEC_XFGVIRT(IWiaLogEx, InitializeLogEx)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InitializeLogEx )( 
            __RPC__in IWiaLogEx * This,
            /* [in] */ __RPC__in BYTE *hInstance);
        
        DECLSPEC_XFGVIRT(IWiaLogEx, hResult)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *hResult )( 
            __RPC__in IWiaLogEx * This,
            /* [in] */ HRESULT hResult);
        
        DECLSPEC_XFGVIRT(IWiaLogEx, Log)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Log )( 
            __RPC__in IWiaLogEx * This,
            /* [in] */ LONG lFlags,
            /* [in] */ LONG lResID,
            /* [in] */ LONG lDetail,
            /* [in] */ __RPC__in BSTR bstrText);
        
        DECLSPEC_XFGVIRT(IWiaLogEx, hResultEx)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *hResultEx )( 
            __RPC__in IWiaLogEx * This,
            /* [in] */ LONG lMethodId,
            /* [in] */ HRESULT hResult);
        
        DECLSPEC_XFGVIRT(IWiaLogEx, LogEx)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *LogEx )( 
            __RPC__in IWiaLogEx * This,
            /* [in] */ LONG lMethodId,
            /* [in] */ LONG lFlags,
            /* [in] */ LONG lResID,
            /* [in] */ LONG lDetail,
            /* [in] */ __RPC__in BSTR bstrText);
        
        END_INTERFACE
    } IWiaLogExVtbl;

    interface IWiaLogEx
    {
        CONST_VTBL struct IWiaLogExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWiaLogEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWiaLogEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWiaLogEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWiaLogEx_InitializeLogEx(This,hInstance)	\
    ( (This)->lpVtbl -> InitializeLogEx(This,hInstance) ) 

#define IWiaLogEx_hResult(This,hResult)	\
    ( (This)->lpVtbl -> hResult(This,hResult) ) 

#define IWiaLogEx_Log(This,lFlags,lResID,lDetail,bstrText)	\
    ( (This)->lpVtbl -> Log(This,lFlags,lResID,lDetail,bstrText) ) 

#define IWiaLogEx_hResultEx(This,lMethodId,hResult)	\
    ( (This)->lpVtbl -> hResultEx(This,lMethodId,hResult) ) 

#define IWiaLogEx_LogEx(This,lMethodId,lFlags,lResID,lDetail,bstrText)	\
    ( (This)->lpVtbl -> LogEx(This,lMethodId,lFlags,lResID,lDetail,bstrText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWiaLogEx_INTERFACE_DEFINED__ */


#ifndef __IWiaNotifyDevMgr_INTERFACE_DEFINED__
#define __IWiaNotifyDevMgr_INTERFACE_DEFINED__

/* interface IWiaNotifyDevMgr */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWiaNotifyDevMgr;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70681EA0-E7BF-4291-9FB1-4E8813A3F78E")
    IWiaNotifyDevMgr : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE NewDeviceArrival( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWiaNotifyDevMgrVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWiaNotifyDevMgr * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWiaNotifyDevMgr * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWiaNotifyDevMgr * This);
        
        DECLSPEC_XFGVIRT(IWiaNotifyDevMgr, NewDeviceArrival)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *NewDeviceArrival )( 
            __RPC__in IWiaNotifyDevMgr * This);
        
        END_INTERFACE
    } IWiaNotifyDevMgrVtbl;

    interface IWiaNotifyDevMgr
    {
        CONST_VTBL struct IWiaNotifyDevMgrVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWiaNotifyDevMgr_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWiaNotifyDevMgr_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWiaNotifyDevMgr_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWiaNotifyDevMgr_NewDeviceArrival(This)	\
    ( (This)->lpVtbl -> NewDeviceArrival(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWiaNotifyDevMgr_INTERFACE_DEFINED__ */


#ifndef __IWiaItemExtras_INTERFACE_DEFINED__
#define __IWiaItemExtras_INTERFACE_DEFINED__

/* interface IWiaItemExtras */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWiaItemExtras;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6291ef2c-36ef-4532-876a-8e132593778d")
    IWiaItemExtras : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetExtendedErrorInfo( 
            /* [out] */ __RPC__deref_out_opt BSTR *bstrErrorText) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Escape( 
            /* [in] */ DWORD dwEscapeCode,
            /* [size_is][in] */ __RPC__in_ecount_full(cbInDataSize) BYTE *lpInData,
            /* [in] */ DWORD cbInDataSize,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(dwOutDataSize, pdwActualDataSize ? *pdwActualDataSize : dwOutDataSize) BYTE *pOutData,
            /* [in] */ DWORD dwOutDataSize,
            /* [out] */ __RPC__out DWORD *pdwActualDataSize) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CancelPendingIO( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWiaItemExtrasVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWiaItemExtras * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWiaItemExtras * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWiaItemExtras * This);
        
        DECLSPEC_XFGVIRT(IWiaItemExtras, GetExtendedErrorInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetExtendedErrorInfo )( 
            __RPC__in IWiaItemExtras * This,
            /* [out] */ __RPC__deref_out_opt BSTR *bstrErrorText);
        
        DECLSPEC_XFGVIRT(IWiaItemExtras, Escape)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Escape )( 
            __RPC__in IWiaItemExtras * This,
            /* [in] */ DWORD dwEscapeCode,
            /* [size_is][in] */ __RPC__in_ecount_full(cbInDataSize) BYTE *lpInData,
            /* [in] */ DWORD cbInDataSize,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(dwOutDataSize, pdwActualDataSize ? *pdwActualDataSize : dwOutDataSize) BYTE *pOutData,
            /* [in] */ DWORD dwOutDataSize,
            /* [out] */ __RPC__out DWORD *pdwActualDataSize);
        
        DECLSPEC_XFGVIRT(IWiaItemExtras, CancelPendingIO)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CancelPendingIO )( 
            __RPC__in IWiaItemExtras * This);
        
        END_INTERFACE
    } IWiaItemExtrasVtbl;

    interface IWiaItemExtras
    {
        CONST_VTBL struct IWiaItemExtrasVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWiaItemExtras_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWiaItemExtras_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWiaItemExtras_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWiaItemExtras_GetExtendedErrorInfo(This,bstrErrorText)	\
    ( (This)->lpVtbl -> GetExtendedErrorInfo(This,bstrErrorText) ) 

#define IWiaItemExtras_Escape(This,dwEscapeCode,lpInData,cbInDataSize,pOutData,dwOutDataSize,pdwActualDataSize)	\
    ( (This)->lpVtbl -> Escape(This,dwEscapeCode,lpInData,cbInDataSize,pOutData,dwOutDataSize,pdwActualDataSize) ) 

#define IWiaItemExtras_CancelPendingIO(This)	\
    ( (This)->lpVtbl -> CancelPendingIO(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWiaItemExtras_INTERFACE_DEFINED__ */



#ifndef __WiaDevMgr_LIBRARY_DEFINED__
#define __WiaDevMgr_LIBRARY_DEFINED__

/* library WiaDevMgr */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_WiaDevMgr;

EXTERN_C const CLSID CLSID_WiaDevMgr;

#ifdef __cplusplus

class DECLSPEC_UUID("a1f4e726-8cf1-11d1-bf92-0060081ed811")
WiaDevMgr;
#endif

EXTERN_C const CLSID CLSID_WiaLog;

#ifdef __cplusplus

class DECLSPEC_UUID("A1E75357-881A-419e-83E2-BB16DB197C68")
WiaLog;
#endif
#endif /* __WiaDevMgr_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_wia_xp_0000_0015 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wia_xp_0000_0015_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wia_xp_0000_0015_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  STGMEDIUM_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in STGMEDIUM * ); 
unsigned char * __RPC_USER  STGMEDIUM_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in STGMEDIUM * ); 
unsigned char * __RPC_USER  STGMEDIUM_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out STGMEDIUM * ); 
void                      __RPC_USER  STGMEDIUM_UserFree(     __RPC__in unsigned long *, __RPC__in STGMEDIUM * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  STGMEDIUM_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in STGMEDIUM * ); 
unsigned char * __RPC_USER  STGMEDIUM_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in STGMEDIUM * ); 
unsigned char * __RPC_USER  STGMEDIUM_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out STGMEDIUM * ); 
void                      __RPC_USER  STGMEDIUM_UserFree64(     __RPC__in unsigned long *, __RPC__in STGMEDIUM * ); 

/* [local] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_CreateDevice_Proxy( 
    IWiaDevMgr * This,
    /* [in] */ BSTR bstrDeviceID,
    /* [out] */ IWiaItem **ppWiaItemRoot);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_CreateDevice_Stub( 
    __RPC__in IWiaDevMgr * This,
    /* [in] */ __RPC__in BSTR bstrDeviceID,
    /* [out] */ __RPC__deref_out_opt IWiaItem **ppWiaItemRoot);

/* [local] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_SelectDeviceDlg_Proxy( 
    IWiaDevMgr * This,
    /* [in] */ HWND hwndParent,
    /* [in] */ LONG lDeviceType,
    /* [in] */ LONG lFlags,
    /* [out][in] */ BSTR *pbstrDeviceID,
    /* [retval][out] */ IWiaItem **ppItemRoot);


/* [nocode][helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_SelectDeviceDlg_Stub( 
    __RPC__in IWiaDevMgr * This,
    /* [in] */ __RPC__in HWND hwndParent,
    /* [in] */ LONG lDeviceType,
    /* [in] */ LONG lFlags,
    /* [out][in] */ __RPC__deref_inout_opt BSTR *pbstrDeviceID,
    /* [retval][out] */ __RPC__deref_out_opt IWiaItem **ppItemRoot);

/* [local] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_SelectDeviceDlgID_Proxy( 
    IWiaDevMgr * This,
    /* [in] */ HWND hwndParent,
    /* [in] */ LONG lDeviceType,
    /* [in] */ LONG lFlags,
    /* [retval][out] */ BSTR *pbstrDeviceID);


/* [nocode][helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_SelectDeviceDlgID_Stub( 
    __RPC__in IWiaDevMgr * This,
    /* [in] */ __RPC__in HWND hwndParent,
    /* [in] */ LONG lDeviceType,
    /* [in] */ LONG lFlags,
    /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceID);

/* [local] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_GetImageDlg_Proxy( 
    IWiaDevMgr * This,
    /* [in] */ HWND hwndParent,
    /* [in] */ LONG lDeviceType,
    /* [in] */ LONG lFlags,
    /* [in] */ LONG lIntent,
    /* [in] */ IWiaItem *pItemRoot,
    /* [in] */ BSTR bstrFilename,
    /* [out][in] */ GUID *pguidFormat);


/* [nocode][helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_GetImageDlg_Stub( 
    __RPC__in IWiaDevMgr * This,
    /* [in] */ __RPC__in HWND hwndParent,
    /* [in] */ LONG lDeviceType,
    /* [in] */ LONG lFlags,
    /* [in] */ LONG lIntent,
    /* [in] */ __RPC__in_opt IWiaItem *pItemRoot,
    /* [in] */ __RPC__in BSTR bstrFilename,
    /* [out][in] */ __RPC__inout GUID *pguidFormat);

/* [local] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_RegisterEventCallbackProgram_Proxy( 
    IWiaDevMgr * This,
    /* [in] */ LONG lFlags,
    /* [in] */ BSTR bstrDeviceID,
    /* [in] */ const GUID *pEventGUID,
    /* [in] */ BSTR bstrCommandline,
    /* [in] */ BSTR bstrName,
    /* [in] */ BSTR bstrDescription,
    /* [in] */ BSTR bstrIcon);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_RegisterEventCallbackProgram_Stub( 
    __RPC__in IWiaDevMgr * This,
    /* [in] */ LONG lFlags,
    /* [in] */ __RPC__in BSTR bstrDeviceID,
    /* [in] */ __RPC__in const GUID *pEventGUID,
    /* [in] */ __RPC__in BSTR bstrCommandline,
    /* [in] */ __RPC__in BSTR bstrName,
    /* [in] */ __RPC__in BSTR bstrDescription,
    /* [in] */ __RPC__in BSTR bstrIcon);

/* [local] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_RegisterEventCallbackInterface_Proxy( 
    IWiaDevMgr * This,
    /* [in] */ LONG lFlags,
    /* [in] */ BSTR bstrDeviceID,
    /* [in] */ const GUID *pEventGUID,
    /* [unique][in] */ IWiaEventCallback *pIWiaEventCallback,
    /* [out] */ IUnknown **pEventObject);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_RegisterEventCallbackInterface_Stub( 
    __RPC__in IWiaDevMgr * This,
    /* [in] */ LONG lFlags,
    /* [in] */ __RPC__in BSTR bstrDeviceID,
    /* [in] */ __RPC__in const GUID *pEventGUID,
    /* [unique][in] */ __RPC__in_opt IWiaEventCallback *pIWiaEventCallback,
    /* [out] */ __RPC__deref_out_opt IUnknown **pEventObject);

/* [local] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_RegisterEventCallbackCLSID_Proxy( 
    IWiaDevMgr * This,
    /* [in] */ LONG lFlags,
    /* [in] */ BSTR bstrDeviceID,
    /* [in] */ const GUID *pEventGUID,
    /* [unique][in] */ const GUID *pClsID,
    /* [in] */ BSTR bstrName,
    /* [in] */ BSTR bstrDescription,
    /* [in] */ BSTR bstrIcon);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDevMgr_RegisterEventCallbackCLSID_Stub( 
    __RPC__in IWiaDevMgr * This,
    /* [in] */ LONG lFlags,
    /* [in] */ __RPC__in BSTR bstrDeviceID,
    /* [in] */ __RPC__in const GUID *pEventGUID,
    /* [unique][in] */ __RPC__in_opt const GUID *pClsID,
    /* [in] */ __RPC__in BSTR bstrName,
    /* [in] */ __RPC__in BSTR bstrDescription,
    /* [in] */ __RPC__in BSTR bstrIcon);

/* [local] */ HRESULT STDMETHODCALLTYPE IEnumWIA_DEV_INFO_Next_Proxy( 
    IEnumWIA_DEV_INFO * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ IWiaPropertyStorage **rgelt,
    /* [unique][out][in] */ ULONG *pceltFetched);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IEnumWIA_DEV_INFO_Next_Stub( 
    __RPC__in IEnumWIA_DEV_INFO * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IWiaPropertyStorage **rgelt,
    /* [unique][out][in] */ __RPC__inout_opt ULONG *pceltFetched);

/* [local] */ HRESULT STDMETHODCALLTYPE IWiaDataCallback_BandedDataCallback_Proxy( 
    IWiaDataCallback * This,
    /* [in] */ LONG lMessage,
    /* [in] */ LONG lStatus,
    /* [in] */ LONG lPercentComplete,
    /* [in] */ LONG lOffset,
    /* [in] */ LONG lLength,
    /* [in] */ LONG lReserved,
    /* [in] */ LONG lResLength,
    /* [size_is][in] */ BYTE *pbBuffer);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDataCallback_BandedDataCallback_Stub( 
    __RPC__in IWiaDataCallback * This,
    /* [in] */ LONG lMessage,
    /* [in] */ LONG lStatus,
    /* [in] */ LONG lPercentComplete,
    /* [in] */ LONG lOffset,
    /* [in] */ LONG lLength,
    /* [in] */ LONG lReserved,
    /* [in] */ LONG lResLength,
    /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(lResLength) BYTE *pbBuffer);

/* [local] */ HRESULT STDMETHODCALLTYPE IWiaDataTransfer_idtGetData_Proxy( 
    IWiaDataTransfer * This,
    /* [out][in] */ LPSTGMEDIUM pMedium,
    /* [unique][in] */ IWiaDataCallback *pIWiaDataCallback);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDataTransfer_idtGetData_Stub( 
    __RPC__in IWiaDataTransfer * This,
    /* [out][in] */ __RPC__inout LPSTGMEDIUM pMedium,
    /* [unique][in] */ __RPC__in_opt IWiaDataCallback *pIWiaDataCallback);

/* [local] */ HRESULT STDMETHODCALLTYPE IWiaDataTransfer_idtGetBandedData_Proxy( 
    IWiaDataTransfer * This,
    /* [unique][in] */ PWIA_DATA_TRANSFER_INFO pWiaDataTransInfo,
    /* [unique][in] */ IWiaDataCallback *pIWiaDataCallback);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaDataTransfer_idtGetBandedData_Stub( 
    __RPC__in IWiaDataTransfer * This,
    /* [unique][in] */ __RPC__in_opt PWIA_DATA_TRANSFER_INFO pWiaDataTransInfo,
    /* [unique][in] */ __RPC__in_opt IWiaDataCallback *pIWiaDataCallback);

/* [local] */ HRESULT STDMETHODCALLTYPE IWiaItem_DeviceDlg_Proxy( 
    IWiaItem * This,
    /* [in] */ HWND hwndParent,
    /* [in] */ LONG lFlags,
    /* [in] */ LONG lIntent,
    /* [out] */ LONG *plItemCount,
    /* [out] */ IWiaItem ***ppIWiaItem);


/* [nocode][helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaItem_DeviceDlg_Stub( 
    __RPC__in IWiaItem * This,
    /* [in] */ __RPC__in HWND hwndParent,
    /* [in] */ LONG lFlags,
    /* [in] */ LONG lIntent,
    /* [out] */ __RPC__out LONG *plItemCount,
    /* [out] */ __RPC__deref_out_opt IWiaItem ***pIWiaItem);

/* [local] */ HRESULT STDMETHODCALLTYPE IWiaPropertyStorage_WriteMultiple_Proxy( 
    IWiaPropertyStorage * This,
    /* [in] */ ULONG cpspec,
    /* [size_is][in] */ const PROPSPEC rgpspec[  ],
    /* [size_is][in] */ const PROPVARIANT rgpropvar[  ],
    /* [in] */ PROPID propidNameFirst);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IWiaPropertyStorage_WriteMultiple_Stub( 
    __RPC__in IWiaPropertyStorage * This,
    /* [in] */ ULONG cpspec,
    /* [size_is][in] */ __RPC__in_ecount_full(cpspec) const PROPSPEC *rgpspec,
    /* [size_is][in] */ __RPC__in_ecount_full(cpspec) const PROPVARIANT *rgpropvar,
    /* [in] */ PROPID propidNameFirst);

/* [local] */ HRESULT STDMETHODCALLTYPE IWiaPropertyStorage_SetPropertyStream_Proxy( 
    IWiaPropertyStorage * This,
    /* [in] */ GUID *pCompatibilityId,
    /* [unique][in] */ IStream *pIStream);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IWiaPropertyStorage_SetPropertyStream_Stub( 
    __RPC__in IWiaPropertyStorage * This,
    /* [in] */ __RPC__in GUID *pCompatibilityId,
    /* [unique][in] */ __RPC__in_opt IStream *pIStream);

/* [local] */ HRESULT STDMETHODCALLTYPE IEnumWiaItem_Next_Proxy( 
    IEnumWiaItem * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ IWiaItem **ppIWiaItem,
    /* [unique][out][in] */ ULONG *pceltFetched);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IEnumWiaItem_Next_Stub( 
    __RPC__in IEnumWiaItem * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IWiaItem **ppIWiaItem,
    /* [unique][out][in] */ __RPC__inout_opt ULONG *pceltFetched);

/* [local] */ HRESULT STDMETHODCALLTYPE IEnumWIA_DEV_CAPS_Next_Proxy( 
    IEnumWIA_DEV_CAPS * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ WIA_DEV_CAP *rgelt,
    /* [unique][out][in] */ ULONG *pceltFetched);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IEnumWIA_DEV_CAPS_Next_Stub( 
    __RPC__in IEnumWIA_DEV_CAPS * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) WIA_DEV_CAP *rgelt,
    /* [unique][out][in] */ __RPC__inout_opt ULONG *pceltFetched);

/* [local] */ HRESULT STDMETHODCALLTYPE IEnumWIA_FORMAT_INFO_Next_Proxy( 
    IEnumWIA_FORMAT_INFO * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ WIA_FORMAT_INFO *rgelt,
    /* [unique][out][in] */ ULONG *pceltFetched);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IEnumWIA_FORMAT_INFO_Next_Stub( 
    __RPC__in IEnumWIA_FORMAT_INFO * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) WIA_FORMAT_INFO *rgelt,
    /* [unique][out][in] */ __RPC__inout_opt ULONG *pceltFetched);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


