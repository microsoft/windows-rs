

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

#ifndef __rdpappcontainerclient_h__
#define __rdpappcontainerclient_h__

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

#ifndef __IRemoteDesktopClientSettings_FWD_DEFINED__
#define __IRemoteDesktopClientSettings_FWD_DEFINED__
typedef interface IRemoteDesktopClientSettings IRemoteDesktopClientSettings;

#endif 	/* __IRemoteDesktopClientSettings_FWD_DEFINED__ */


#ifndef __IRemoteDesktopClientActions_FWD_DEFINED__
#define __IRemoteDesktopClientActions_FWD_DEFINED__
typedef interface IRemoteDesktopClientActions IRemoteDesktopClientActions;

#endif 	/* __IRemoteDesktopClientActions_FWD_DEFINED__ */


#ifndef __IRemoteDesktopClientTouchPointer_FWD_DEFINED__
#define __IRemoteDesktopClientTouchPointer_FWD_DEFINED__
typedef interface IRemoteDesktopClientTouchPointer IRemoteDesktopClientTouchPointer;

#endif 	/* __IRemoteDesktopClientTouchPointer_FWD_DEFINED__ */


#ifndef __IRemoteDesktopClient_FWD_DEFINED__
#define __IRemoteDesktopClient_FWD_DEFINED__
typedef interface IRemoteDesktopClient IRemoteDesktopClient;

#endif 	/* __IRemoteDesktopClient_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_rdpappcontainerclient_0000_0000 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define	DISPID_METHOD_REMOTEDESKTOPCLIENT_CONNECT	( 701 )

#define	DISPID_METHOD_REMOTEDESKTOPCLIENT_DISCONNECT	( 702 )

#define	DISPID_METHOD_REMOTEDESKTOPCLIENT_RECONNECT	( 703 )

#define	DISPID_METHOD_REMOTEDESKTOPCLIENT_DELETE_SAVED_CREDENTIALS	( 704 )

#define	DISPID_METHOD_REMOTEDESKTOPCLIENT_UPDATE_SESSION_DISPLAYSETTINGS	( 705 )

#define	DISPID_METHOD_REMOTEDESKTOPCLIENT_ATTACH_EVENT	( 706 )

#define	DISPID_METHOD_REMOTEDESKTOPCLIENT_DETACH_EVENT	( 707 )

#define	DISPID_PROP_REMOTEDESKTOPCLIENT_SETTINGS	( 710 )

#define	DISPID_PROP_REMOTEDESKTOPCLIENT_ACTIONS	( 711 )

#define	DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCH_POINTER	( 712 )

#define	DISPID_METHOD_REMOTEDESKTOPCLIENT_SET_RDPPROPERTY	( 720 )

#define	DISPID_METHOD_REMOTEDESKTOPCLIENT_GET_RDPPROPERTY	( 721 )

#define	DISPID_METHOD_REMOTEDESKTOPCLIENT_APPLY_SETTINGS	( 722 )

#define	DISPID_METHOD_REMOTEDESKTOPCLIENT_RETRIEVE_SETTINGS	( 723 )

#define	DISPID_METHOD_REMOTEDESKTOPCLIENT_SUSPEND_SCREEN_UPDATES	( 730 )

#define	DISPID_METHOD_REMOTEDESKTOPCLIENT_RESUME_SCREEN_UPDATES	( 731 )

#define	DISPID_METHOD_REMOTEDESKTOPCLIENT_EXECUTE_REMOTE_ACTION	( 732 )

#define	DISPID_METHOD_REMOTEDESKTOPCLIENT_GET_SNAPSHOT	( 733 )

#define	DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_ENABLED	( 740 )

#define	DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_EVENTSENABLED	( 741 )

#define	DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_POINTERSPEED	( 742 )

#define	DISPID_AX_CONNECTING	( 750 )

#define	DISPID_AX_CONNECTED	( 751 )

#define	DISPID_AX_LOGINCOMPLETED	( 752 )

#define	DISPID_AX_DISCONNECTED	( 753 )

#define	DISPID_AX_STATUSCHANGED	( 754 )

#define	DISPID_AX_AUTORECONNECTING	( 755 )

#define	DISPID_AX_AUTORECONNECTED	( 756 )

#define	DISPID_AX_DIALOGDISPLAYING	( 757 )

#define	DISPID_AX_DIALOGDISMISSED	( 758 )

#define	DISPID_AX_NETWORKSTATUSCHANGED	( 759 )

#define	DISPID_AX_ADMINMESSAGERECEIVED	( 760 )

#define	DISPID_AX_KEYCOMBINATIONPRESSED	( 761 )

#define	DISPID_AX_REMOTEDESKTOPSIZECHANGED	( 762 )

#define	DISPID_AX_TOUCHPOINTERCURSORMOVED	( 800 )



extern RPC_IF_HANDLE __MIDL_itf_rdpappcontainerclient_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_rdpappcontainerclient_0000_0000_v0_0_s_ifspec;

#ifndef __IRemoteDesktopClientSettings_INTERFACE_DEFINED__
#define __IRemoteDesktopClientSettings_INTERFACE_DEFINED__

/* interface IRemoteDesktopClientSettings */
/* [unique][dual][uuid][object] */ 

typedef /* [public] */ 
enum __MIDL_IRemoteDesktopClientSettings_0001
    {
        PasswordEncodingUTF8	= 0,
        PasswordEncodingUTF16LE	= 1,
        PasswordEncodingUTF16BE	= 2
    } 	PasswordEncodingType;


EXTERN_C const IID IID_IRemoteDesktopClientSettings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("48A0F2A7-2713-431f-BBAC-6F4558E7D64D")
    IRemoteDesktopClientSettings : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ApplySettings( 
            /* [in] */ __RPC__in BSTR rdpFileContents) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE RetrieveSettings( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *rdpFileContents) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetRdpProperty( 
            /* [in] */ __RPC__in BSTR propertyName,
            /* [retval][out] */ __RPC__out VARIANT *value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetRdpProperty( 
            /* [in] */ __RPC__in BSTR propertyName,
            /* [in] */ VARIANT value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRemoteDesktopClientSettingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRemoteDesktopClientSettings * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRemoteDesktopClientSettings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRemoteDesktopClientSettings * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRemoteDesktopClientSettings * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRemoteDesktopClientSettings * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRemoteDesktopClientSettings * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRemoteDesktopClientSettings * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClientSettings, ApplySettings)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ApplySettings )( 
            __RPC__in IRemoteDesktopClientSettings * This,
            /* [in] */ __RPC__in BSTR rdpFileContents);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClientSettings, RetrieveSettings)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RetrieveSettings )( 
            __RPC__in IRemoteDesktopClientSettings * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *rdpFileContents);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClientSettings, GetRdpProperty)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetRdpProperty )( 
            __RPC__in IRemoteDesktopClientSettings * This,
            /* [in] */ __RPC__in BSTR propertyName,
            /* [retval][out] */ __RPC__out VARIANT *value);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClientSettings, SetRdpProperty)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetRdpProperty )( 
            __RPC__in IRemoteDesktopClientSettings * This,
            /* [in] */ __RPC__in BSTR propertyName,
            /* [in] */ VARIANT value);
        
        END_INTERFACE
    } IRemoteDesktopClientSettingsVtbl;

    interface IRemoteDesktopClientSettings
    {
        CONST_VTBL struct IRemoteDesktopClientSettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRemoteDesktopClientSettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRemoteDesktopClientSettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRemoteDesktopClientSettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRemoteDesktopClientSettings_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRemoteDesktopClientSettings_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRemoteDesktopClientSettings_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRemoteDesktopClientSettings_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRemoteDesktopClientSettings_ApplySettings(This,rdpFileContents)	\
    ( (This)->lpVtbl -> ApplySettings(This,rdpFileContents) ) 

#define IRemoteDesktopClientSettings_RetrieveSettings(This,rdpFileContents)	\
    ( (This)->lpVtbl -> RetrieveSettings(This,rdpFileContents) ) 

#define IRemoteDesktopClientSettings_GetRdpProperty(This,propertyName,value)	\
    ( (This)->lpVtbl -> GetRdpProperty(This,propertyName,value) ) 

#define IRemoteDesktopClientSettings_SetRdpProperty(This,propertyName,value)	\
    ( (This)->lpVtbl -> SetRdpProperty(This,propertyName,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRemoteDesktopClientSettings_INTERFACE_DEFINED__ */


#ifndef __IRemoteDesktopClientActions_INTERFACE_DEFINED__
#define __IRemoteDesktopClientActions_INTERFACE_DEFINED__

/* interface IRemoteDesktopClientActions */
/* [unique][dual][uuid][object] */ 

typedef /* [public][public] */ 
enum __MIDL_IRemoteDesktopClientActions_0001
    {
        RemoteActionCharms	= 0,
        RemoteActionAppbar	= 1,
        RemoteActionSnap	= 2,
        RemoteActionStartScreen	= 3,
        RemoteActionAppSwitch	= 4
    } 	RemoteActionType;

typedef /* [public][public] */ 
enum __MIDL_IRemoteDesktopClientActions_0002
    {
        SnapshotEncodingDataUri	= 0
    } 	SnapshotEncodingType;

typedef /* [public][public] */ 
enum __MIDL_IRemoteDesktopClientActions_0003
    {
        SnapshotFormatPng	= 0,
        SnapshotFormatJpeg	= 1,
        SnapshotFormatBmp	= 2
    } 	SnapshotFormatType;


EXTERN_C const IID IID_IRemoteDesktopClientActions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7D54BC4E-1028-45d4-8B0A-B9B6BFFBA176")
    IRemoteDesktopClientActions : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SuspendScreenUpdates( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ResumeScreenUpdates( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ExecuteRemoteAction( 
            /* [in] */ RemoteActionType remoteAction) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetSnapshot( 
            /* [in] */ SnapshotEncodingType snapshotEncoding,
            /* [in] */ SnapshotFormatType snapshotFormat,
            /* [in] */ ULONG snapshotWidth,
            /* [in] */ ULONG snapshotHeight,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *snapshotData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRemoteDesktopClientActionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRemoteDesktopClientActions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRemoteDesktopClientActions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRemoteDesktopClientActions * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRemoteDesktopClientActions * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRemoteDesktopClientActions * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRemoteDesktopClientActions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRemoteDesktopClientActions * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClientActions, SuspendScreenUpdates)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SuspendScreenUpdates )( 
            __RPC__in IRemoteDesktopClientActions * This);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClientActions, ResumeScreenUpdates)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ResumeScreenUpdates )( 
            __RPC__in IRemoteDesktopClientActions * This);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClientActions, ExecuteRemoteAction)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ExecuteRemoteAction )( 
            __RPC__in IRemoteDesktopClientActions * This,
            /* [in] */ RemoteActionType remoteAction);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClientActions, GetSnapshot)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetSnapshot )( 
            __RPC__in IRemoteDesktopClientActions * This,
            /* [in] */ SnapshotEncodingType snapshotEncoding,
            /* [in] */ SnapshotFormatType snapshotFormat,
            /* [in] */ ULONG snapshotWidth,
            /* [in] */ ULONG snapshotHeight,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *snapshotData);
        
        END_INTERFACE
    } IRemoteDesktopClientActionsVtbl;

    interface IRemoteDesktopClientActions
    {
        CONST_VTBL struct IRemoteDesktopClientActionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRemoteDesktopClientActions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRemoteDesktopClientActions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRemoteDesktopClientActions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRemoteDesktopClientActions_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRemoteDesktopClientActions_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRemoteDesktopClientActions_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRemoteDesktopClientActions_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRemoteDesktopClientActions_SuspendScreenUpdates(This)	\
    ( (This)->lpVtbl -> SuspendScreenUpdates(This) ) 

#define IRemoteDesktopClientActions_ResumeScreenUpdates(This)	\
    ( (This)->lpVtbl -> ResumeScreenUpdates(This) ) 

#define IRemoteDesktopClientActions_ExecuteRemoteAction(This,remoteAction)	\
    ( (This)->lpVtbl -> ExecuteRemoteAction(This,remoteAction) ) 

#define IRemoteDesktopClientActions_GetSnapshot(This,snapshotEncoding,snapshotFormat,snapshotWidth,snapshotHeight,snapshotData)	\
    ( (This)->lpVtbl -> GetSnapshot(This,snapshotEncoding,snapshotFormat,snapshotWidth,snapshotHeight,snapshotData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRemoteDesktopClientActions_INTERFACE_DEFINED__ */


#ifndef __IRemoteDesktopClientTouchPointer_INTERFACE_DEFINED__
#define __IRemoteDesktopClientTouchPointer_INTERFACE_DEFINED__

/* interface IRemoteDesktopClientTouchPointer */
/* [unique][dual][uuid][object] */ 


EXTERN_C const IID IID_IRemoteDesktopClientTouchPointer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("260ec22d-8cbc-44b5-9e88-2a37f6c93ae9")
    IRemoteDesktopClientTouchPointer : public IDispatch
    {
    public:
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Enabled( 
            /* [in] */ VARIANT_BOOL enabled) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *enabled) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_EventsEnabled( 
            /* [in] */ VARIANT_BOOL eventsEnabled) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_EventsEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *eventsEnabled) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PointerSpeed( 
            /* [in] */ ULONG pointerSpeed) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PointerSpeed( 
            /* [retval][out] */ __RPC__out ULONG *pointerSpeed) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRemoteDesktopClientTouchPointerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRemoteDesktopClientTouchPointer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRemoteDesktopClientTouchPointer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRemoteDesktopClientTouchPointer * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRemoteDesktopClientTouchPointer * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRemoteDesktopClientTouchPointer * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRemoteDesktopClientTouchPointer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRemoteDesktopClientTouchPointer * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClientTouchPointer, put_Enabled)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            __RPC__in IRemoteDesktopClientTouchPointer * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClientTouchPointer, get_Enabled)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in IRemoteDesktopClientTouchPointer * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *enabled);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClientTouchPointer, put_EventsEnabled)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EventsEnabled )( 
            __RPC__in IRemoteDesktopClientTouchPointer * This,
            /* [in] */ VARIANT_BOOL eventsEnabled);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClientTouchPointer, get_EventsEnabled)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventsEnabled )( 
            __RPC__in IRemoteDesktopClientTouchPointer * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *eventsEnabled);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClientTouchPointer, put_PointerSpeed)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PointerSpeed )( 
            __RPC__in IRemoteDesktopClientTouchPointer * This,
            /* [in] */ ULONG pointerSpeed);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClientTouchPointer, get_PointerSpeed)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PointerSpeed )( 
            __RPC__in IRemoteDesktopClientTouchPointer * This,
            /* [retval][out] */ __RPC__out ULONG *pointerSpeed);
        
        END_INTERFACE
    } IRemoteDesktopClientTouchPointerVtbl;

    interface IRemoteDesktopClientTouchPointer
    {
        CONST_VTBL struct IRemoteDesktopClientTouchPointerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRemoteDesktopClientTouchPointer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRemoteDesktopClientTouchPointer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRemoteDesktopClientTouchPointer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRemoteDesktopClientTouchPointer_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRemoteDesktopClientTouchPointer_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRemoteDesktopClientTouchPointer_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRemoteDesktopClientTouchPointer_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRemoteDesktopClientTouchPointer_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 

#define IRemoteDesktopClientTouchPointer_get_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,enabled) ) 

#define IRemoteDesktopClientTouchPointer_put_EventsEnabled(This,eventsEnabled)	\
    ( (This)->lpVtbl -> put_EventsEnabled(This,eventsEnabled) ) 

#define IRemoteDesktopClientTouchPointer_get_EventsEnabled(This,eventsEnabled)	\
    ( (This)->lpVtbl -> get_EventsEnabled(This,eventsEnabled) ) 

#define IRemoteDesktopClientTouchPointer_put_PointerSpeed(This,pointerSpeed)	\
    ( (This)->lpVtbl -> put_PointerSpeed(This,pointerSpeed) ) 

#define IRemoteDesktopClientTouchPointer_get_PointerSpeed(This,pointerSpeed)	\
    ( (This)->lpVtbl -> get_PointerSpeed(This,pointerSpeed) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRemoteDesktopClientTouchPointer_INTERFACE_DEFINED__ */


#ifndef __IRemoteDesktopClient_INTERFACE_DEFINED__
#define __IRemoteDesktopClient_INTERFACE_DEFINED__

/* interface IRemoteDesktopClient */
/* [unique][dual][uuid][object] */ 

typedef /* [public] */ 
enum __MIDL_IRemoteDesktopClient_0001
    {
        KeyCombinationHome	= 0,
        KeyCombinationLeft	= 1,
        KeyCombinationUp	= 2,
        KeyCombinationRight	= 3,
        KeyCombinationDown	= 4,
        KeyCombinationScroll	= 5
    } 	KeyCombinationType;


EXTERN_C const IID IID_IRemoteDesktopClient;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("57D25668-625A-4905-BE4E-304CAA13F89C")
    IRemoteDesktopClient : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Connect( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Disconnect( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Reconnect( 
            /* [in] */ ULONG width,
            /* [in] */ ULONG height) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Settings( 
            /* [retval][out] */ __RPC__deref_out_opt IRemoteDesktopClientSettings **settings) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Actions( 
            /* [retval][out] */ __RPC__deref_out_opt IRemoteDesktopClientActions **actions) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_TouchPointer( 
            /* [retval][out] */ __RPC__deref_out_opt IRemoteDesktopClientTouchPointer **touchPointer) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DeleteSavedCredentials( 
            /* [in] */ __RPC__in BSTR serverName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE UpdateSessionDisplaySettings( 
            /* [in] */ ULONG width,
            /* [in] */ ULONG height) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE attachEvent( 
            /* [in] */ __RPC__in BSTR eventName,
            /* [in] */ __RPC__in_opt IDispatch *callback) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE detachEvent( 
            /* [in] */ __RPC__in BSTR eventName,
            /* [in] */ __RPC__in_opt IDispatch *callback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRemoteDesktopClientVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRemoteDesktopClient * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRemoteDesktopClient * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRemoteDesktopClient * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRemoteDesktopClient * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRemoteDesktopClient * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRemoteDesktopClient * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRemoteDesktopClient * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClient, Connect)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in IRemoteDesktopClient * This);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClient, Disconnect)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            __RPC__in IRemoteDesktopClient * This);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClient, Reconnect)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Reconnect )( 
            __RPC__in IRemoteDesktopClient * This,
            /* [in] */ ULONG width,
            /* [in] */ ULONG height);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClient, get_Settings)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Settings )( 
            __RPC__in IRemoteDesktopClient * This,
            /* [retval][out] */ __RPC__deref_out_opt IRemoteDesktopClientSettings **settings);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClient, get_Actions)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Actions )( 
            __RPC__in IRemoteDesktopClient * This,
            /* [retval][out] */ __RPC__deref_out_opt IRemoteDesktopClientActions **actions);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClient, get_TouchPointer)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TouchPointer )( 
            __RPC__in IRemoteDesktopClient * This,
            /* [retval][out] */ __RPC__deref_out_opt IRemoteDesktopClientTouchPointer **touchPointer);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClient, DeleteSavedCredentials)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DeleteSavedCredentials )( 
            __RPC__in IRemoteDesktopClient * This,
            /* [in] */ __RPC__in BSTR serverName);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClient, UpdateSessionDisplaySettings)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *UpdateSessionDisplaySettings )( 
            __RPC__in IRemoteDesktopClient * This,
            /* [in] */ ULONG width,
            /* [in] */ ULONG height);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClient, attachEvent)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *attachEvent )( 
            __RPC__in IRemoteDesktopClient * This,
            /* [in] */ __RPC__in BSTR eventName,
            /* [in] */ __RPC__in_opt IDispatch *callback);
        
        DECLSPEC_XFGVIRT(IRemoteDesktopClient, detachEvent)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *detachEvent )( 
            __RPC__in IRemoteDesktopClient * This,
            /* [in] */ __RPC__in BSTR eventName,
            /* [in] */ __RPC__in_opt IDispatch *callback);
        
        END_INTERFACE
    } IRemoteDesktopClientVtbl;

    interface IRemoteDesktopClient
    {
        CONST_VTBL struct IRemoteDesktopClientVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRemoteDesktopClient_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRemoteDesktopClient_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRemoteDesktopClient_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRemoteDesktopClient_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRemoteDesktopClient_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRemoteDesktopClient_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRemoteDesktopClient_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRemoteDesktopClient_Connect(This)	\
    ( (This)->lpVtbl -> Connect(This) ) 

#define IRemoteDesktopClient_Disconnect(This)	\
    ( (This)->lpVtbl -> Disconnect(This) ) 

#define IRemoteDesktopClient_Reconnect(This,width,height)	\
    ( (This)->lpVtbl -> Reconnect(This,width,height) ) 

#define IRemoteDesktopClient_get_Settings(This,settings)	\
    ( (This)->lpVtbl -> get_Settings(This,settings) ) 

#define IRemoteDesktopClient_get_Actions(This,actions)	\
    ( (This)->lpVtbl -> get_Actions(This,actions) ) 

#define IRemoteDesktopClient_get_TouchPointer(This,touchPointer)	\
    ( (This)->lpVtbl -> get_TouchPointer(This,touchPointer) ) 

#define IRemoteDesktopClient_DeleteSavedCredentials(This,serverName)	\
    ( (This)->lpVtbl -> DeleteSavedCredentials(This,serverName) ) 

#define IRemoteDesktopClient_UpdateSessionDisplaySettings(This,width,height)	\
    ( (This)->lpVtbl -> UpdateSessionDisplaySettings(This,width,height) ) 

#define IRemoteDesktopClient_attachEvent(This,eventName,callback)	\
    ( (This)->lpVtbl -> attachEvent(This,eventName,callback) ) 

#define IRemoteDesktopClient_detachEvent(This,eventName,callback)	\
    ( (This)->lpVtbl -> detachEvent(This,eventName,callback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRemoteDesktopClient_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_rdpappcontainerclient_0000_0004 */
/* [local] */ 

#endif


extern RPC_IF_HANDLE __MIDL_itf_rdpappcontainerclient_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_rdpappcontainerclient_0000_0004_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


