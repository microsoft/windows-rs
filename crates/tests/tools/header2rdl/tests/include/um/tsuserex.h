

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


#ifndef __tsuserex_h__
#define __tsuserex_h__

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

#ifndef __TSUserExInterfaces_FWD_DEFINED__
#define __TSUserExInterfaces_FWD_DEFINED__

#ifdef __cplusplus
typedef class TSUserExInterfaces TSUserExInterfaces;
#else
typedef struct TSUserExInterfaces TSUserExInterfaces;
#endif /* __cplusplus */

#endif 	/* __TSUserExInterfaces_FWD_DEFINED__ */


#ifndef __IADsTSUserEx_FWD_DEFINED__
#define __IADsTSUserEx_FWD_DEFINED__
typedef interface IADsTSUserEx IADsTSUserEx;

#endif 	/* __IADsTSUserEx_FWD_DEFINED__ */


#ifndef __ADsTSUserEx_FWD_DEFINED__
#define __ADsTSUserEx_FWD_DEFINED__

#ifdef __cplusplus
typedef class ADsTSUserEx ADsTSUserEx;
#else
typedef struct ADsTSUserEx ADsTSUserEx;
#endif /* __cplusplus */

#endif 	/* __ADsTSUserEx_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "mmc.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_tsuserex_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_tsuserex_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tsuserex_0000_0000_v0_0_s_ifspec;


#ifndef __TSUSEREXLib_LIBRARY_DEFINED__
#define __TSUSEREXLib_LIBRARY_DEFINED__

/* library TSUSEREXLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_TSUSEREXLib;

EXTERN_C const CLSID CLSID_TSUserExInterfaces;

#ifdef __cplusplus

class DECLSPEC_UUID("0910dd01-df8c-11d1-ae27-00c04fa35813")
TSUserExInterfaces;
#endif

#ifndef __IADsTSUserEx_INTERFACE_DEFINED__
#define __IADsTSUserEx_INTERFACE_DEFINED__

/* interface IADsTSUserEx */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IADsTSUserEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C4930E79-2989-4462-8A60-2FCF2F2955EF")
    IADsTSUserEx : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_TerminalServicesProfilePath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_TerminalServicesProfilePath( 
            /* [in] */ __RPC__in BSTR pNewVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_TerminalServicesHomeDirectory( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_TerminalServicesHomeDirectory( 
            /* [in] */ __RPC__in BSTR pNewVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_TerminalServicesHomeDrive( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_TerminalServicesHomeDrive( 
            /* [in] */ __RPC__in BSTR pNewVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AllowLogon( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_AllowLogon( 
            /* [in] */ LONG NewVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_EnableRemoteControl( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_EnableRemoteControl( 
            /* [in] */ LONG NewVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxDisconnectionTime( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_MaxDisconnectionTime( 
            /* [in] */ LONG NewVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxConnectionTime( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_MaxConnectionTime( 
            /* [in] */ LONG NewVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxIdleTime( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_MaxIdleTime( 
            /* [in] */ LONG NewVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ReconnectionAction( 
            /* [retval][out] */ __RPC__out LONG *pNewVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ReconnectionAction( 
            /* [in] */ LONG NewVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_BrokenConnectionAction( 
            /* [retval][out] */ __RPC__out LONG *pNewVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_BrokenConnectionAction( 
            /* [in] */ LONG NewVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ConnectClientDrivesAtLogon( 
            /* [retval][out] */ __RPC__out LONG *pNewVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ConnectClientDrivesAtLogon( 
            /* [in] */ LONG NewVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ConnectClientPrintersAtLogon( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ConnectClientPrintersAtLogon( 
            /* [in] */ LONG NewVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DefaultToMainPrinter( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_DefaultToMainPrinter( 
            /* [in] */ LONG NewVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_TerminalServicesWorkDirectory( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_TerminalServicesWorkDirectory( 
            /* [in] */ __RPC__in BSTR pNewVal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_TerminalServicesInitialProgram( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_TerminalServicesInitialProgram( 
            /* [in] */ __RPC__in BSTR pNewVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsTSUserExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsTSUserEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsTSUserEx * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsTSUserEx * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsTSUserEx * This,
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
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, get_TerminalServicesProfilePath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TerminalServicesProfilePath )( 
            __RPC__in IADsTSUserEx * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, put_TerminalServicesProfilePath)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TerminalServicesProfilePath )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ __RPC__in BSTR pNewVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, get_TerminalServicesHomeDirectory)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TerminalServicesHomeDirectory )( 
            __RPC__in IADsTSUserEx * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, put_TerminalServicesHomeDirectory)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TerminalServicesHomeDirectory )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ __RPC__in BSTR pNewVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, get_TerminalServicesHomeDrive)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TerminalServicesHomeDrive )( 
            __RPC__in IADsTSUserEx * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, put_TerminalServicesHomeDrive)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TerminalServicesHomeDrive )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ __RPC__in BSTR pNewVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, get_AllowLogon)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AllowLogon )( 
            __RPC__in IADsTSUserEx * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, put_AllowLogon)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AllowLogon )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ LONG NewVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, get_EnableRemoteControl)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnableRemoteControl )( 
            __RPC__in IADsTSUserEx * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, put_EnableRemoteControl)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EnableRemoteControl )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ LONG NewVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, get_MaxDisconnectionTime)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxDisconnectionTime )( 
            __RPC__in IADsTSUserEx * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, put_MaxDisconnectionTime)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxDisconnectionTime )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ LONG NewVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, get_MaxConnectionTime)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxConnectionTime )( 
            __RPC__in IADsTSUserEx * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, put_MaxConnectionTime)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxConnectionTime )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ LONG NewVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, get_MaxIdleTime)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxIdleTime )( 
            __RPC__in IADsTSUserEx * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, put_MaxIdleTime)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxIdleTime )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ LONG NewVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, get_ReconnectionAction)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReconnectionAction )( 
            __RPC__in IADsTSUserEx * This,
            /* [retval][out] */ __RPC__out LONG *pNewVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, put_ReconnectionAction)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReconnectionAction )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ LONG NewVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, get_BrokenConnectionAction)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BrokenConnectionAction )( 
            __RPC__in IADsTSUserEx * This,
            /* [retval][out] */ __RPC__out LONG *pNewVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, put_BrokenConnectionAction)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BrokenConnectionAction )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ LONG NewVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, get_ConnectClientDrivesAtLogon)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConnectClientDrivesAtLogon )( 
            __RPC__in IADsTSUserEx * This,
            /* [retval][out] */ __RPC__out LONG *pNewVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, put_ConnectClientDrivesAtLogon)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ConnectClientDrivesAtLogon )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ LONG NewVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, get_ConnectClientPrintersAtLogon)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConnectClientPrintersAtLogon )( 
            __RPC__in IADsTSUserEx * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, put_ConnectClientPrintersAtLogon)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ConnectClientPrintersAtLogon )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ LONG NewVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, get_DefaultToMainPrinter)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultToMainPrinter )( 
            __RPC__in IADsTSUserEx * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, put_DefaultToMainPrinter)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultToMainPrinter )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ LONG NewVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, get_TerminalServicesWorkDirectory)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TerminalServicesWorkDirectory )( 
            __RPC__in IADsTSUserEx * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, put_TerminalServicesWorkDirectory)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TerminalServicesWorkDirectory )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ __RPC__in BSTR pNewVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, get_TerminalServicesInitialProgram)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TerminalServicesInitialProgram )( 
            __RPC__in IADsTSUserEx * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IADsTSUserEx, put_TerminalServicesInitialProgram)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TerminalServicesInitialProgram )( 
            __RPC__in IADsTSUserEx * This,
            /* [in] */ __RPC__in BSTR pNewVal);
        
        END_INTERFACE
    } IADsTSUserExVtbl;

    interface IADsTSUserEx
    {
        CONST_VTBL struct IADsTSUserExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsTSUserEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsTSUserEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsTSUserEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsTSUserEx_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsTSUserEx_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsTSUserEx_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsTSUserEx_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsTSUserEx_get_TerminalServicesProfilePath(This,pVal)	\
    ( (This)->lpVtbl -> get_TerminalServicesProfilePath(This,pVal) ) 

#define IADsTSUserEx_put_TerminalServicesProfilePath(This,pNewVal)	\
    ( (This)->lpVtbl -> put_TerminalServicesProfilePath(This,pNewVal) ) 

#define IADsTSUserEx_get_TerminalServicesHomeDirectory(This,pVal)	\
    ( (This)->lpVtbl -> get_TerminalServicesHomeDirectory(This,pVal) ) 

#define IADsTSUserEx_put_TerminalServicesHomeDirectory(This,pNewVal)	\
    ( (This)->lpVtbl -> put_TerminalServicesHomeDirectory(This,pNewVal) ) 

#define IADsTSUserEx_get_TerminalServicesHomeDrive(This,pVal)	\
    ( (This)->lpVtbl -> get_TerminalServicesHomeDrive(This,pVal) ) 

#define IADsTSUserEx_put_TerminalServicesHomeDrive(This,pNewVal)	\
    ( (This)->lpVtbl -> put_TerminalServicesHomeDrive(This,pNewVal) ) 

#define IADsTSUserEx_get_AllowLogon(This,pVal)	\
    ( (This)->lpVtbl -> get_AllowLogon(This,pVal) ) 

#define IADsTSUserEx_put_AllowLogon(This,NewVal)	\
    ( (This)->lpVtbl -> put_AllowLogon(This,NewVal) ) 

#define IADsTSUserEx_get_EnableRemoteControl(This,pVal)	\
    ( (This)->lpVtbl -> get_EnableRemoteControl(This,pVal) ) 

#define IADsTSUserEx_put_EnableRemoteControl(This,NewVal)	\
    ( (This)->lpVtbl -> put_EnableRemoteControl(This,NewVal) ) 

#define IADsTSUserEx_get_MaxDisconnectionTime(This,pVal)	\
    ( (This)->lpVtbl -> get_MaxDisconnectionTime(This,pVal) ) 

#define IADsTSUserEx_put_MaxDisconnectionTime(This,NewVal)	\
    ( (This)->lpVtbl -> put_MaxDisconnectionTime(This,NewVal) ) 

#define IADsTSUserEx_get_MaxConnectionTime(This,pVal)	\
    ( (This)->lpVtbl -> get_MaxConnectionTime(This,pVal) ) 

#define IADsTSUserEx_put_MaxConnectionTime(This,NewVal)	\
    ( (This)->lpVtbl -> put_MaxConnectionTime(This,NewVal) ) 

#define IADsTSUserEx_get_MaxIdleTime(This,pVal)	\
    ( (This)->lpVtbl -> get_MaxIdleTime(This,pVal) ) 

#define IADsTSUserEx_put_MaxIdleTime(This,NewVal)	\
    ( (This)->lpVtbl -> put_MaxIdleTime(This,NewVal) ) 

#define IADsTSUserEx_get_ReconnectionAction(This,pNewVal)	\
    ( (This)->lpVtbl -> get_ReconnectionAction(This,pNewVal) ) 

#define IADsTSUserEx_put_ReconnectionAction(This,NewVal)	\
    ( (This)->lpVtbl -> put_ReconnectionAction(This,NewVal) ) 

#define IADsTSUserEx_get_BrokenConnectionAction(This,pNewVal)	\
    ( (This)->lpVtbl -> get_BrokenConnectionAction(This,pNewVal) ) 

#define IADsTSUserEx_put_BrokenConnectionAction(This,NewVal)	\
    ( (This)->lpVtbl -> put_BrokenConnectionAction(This,NewVal) ) 

#define IADsTSUserEx_get_ConnectClientDrivesAtLogon(This,pNewVal)	\
    ( (This)->lpVtbl -> get_ConnectClientDrivesAtLogon(This,pNewVal) ) 

#define IADsTSUserEx_put_ConnectClientDrivesAtLogon(This,NewVal)	\
    ( (This)->lpVtbl -> put_ConnectClientDrivesAtLogon(This,NewVal) ) 

#define IADsTSUserEx_get_ConnectClientPrintersAtLogon(This,pVal)	\
    ( (This)->lpVtbl -> get_ConnectClientPrintersAtLogon(This,pVal) ) 

#define IADsTSUserEx_put_ConnectClientPrintersAtLogon(This,NewVal)	\
    ( (This)->lpVtbl -> put_ConnectClientPrintersAtLogon(This,NewVal) ) 

#define IADsTSUserEx_get_DefaultToMainPrinter(This,pVal)	\
    ( (This)->lpVtbl -> get_DefaultToMainPrinter(This,pVal) ) 

#define IADsTSUserEx_put_DefaultToMainPrinter(This,NewVal)	\
    ( (This)->lpVtbl -> put_DefaultToMainPrinter(This,NewVal) ) 

#define IADsTSUserEx_get_TerminalServicesWorkDirectory(This,pVal)	\
    ( (This)->lpVtbl -> get_TerminalServicesWorkDirectory(This,pVal) ) 

#define IADsTSUserEx_put_TerminalServicesWorkDirectory(This,pNewVal)	\
    ( (This)->lpVtbl -> put_TerminalServicesWorkDirectory(This,pNewVal) ) 

#define IADsTSUserEx_get_TerminalServicesInitialProgram(This,pVal)	\
    ( (This)->lpVtbl -> get_TerminalServicesInitialProgram(This,pVal) ) 

#define IADsTSUserEx_put_TerminalServicesInitialProgram(This,pNewVal)	\
    ( (This)->lpVtbl -> put_TerminalServicesInitialProgram(This,pNewVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsTSUserEx_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_ADsTSUserEx;

#ifdef __cplusplus

class DECLSPEC_UUID("E2E9CAE6-1E7B-4B8E-BABD-E9BF6292AC29")
ADsTSUserEx;
#endif
#endif /* __TSUSEREXLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_tsuserex_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_tsuserex_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tsuserex_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


