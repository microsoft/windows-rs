

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


#ifndef __sensevts_h__
#define __sensevts_h__

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

#ifndef __ISensNetwork_FWD_DEFINED__
#define __ISensNetwork_FWD_DEFINED__
typedef interface ISensNetwork ISensNetwork;

#endif 	/* __ISensNetwork_FWD_DEFINED__ */


#ifndef __ISensOnNow_FWD_DEFINED__
#define __ISensOnNow_FWD_DEFINED__
typedef interface ISensOnNow ISensOnNow;

#endif 	/* __ISensOnNow_FWD_DEFINED__ */


#ifndef __ISensLogon_FWD_DEFINED__
#define __ISensLogon_FWD_DEFINED__
typedef interface ISensLogon ISensLogon;

#endif 	/* __ISensLogon_FWD_DEFINED__ */


#ifndef __ISensLogon2_FWD_DEFINED__
#define __ISensLogon2_FWD_DEFINED__
typedef interface ISensLogon2 ISensLogon2;

#endif 	/* __ISensLogon2_FWD_DEFINED__ */


#ifndef __SENS_FWD_DEFINED__
#define __SENS_FWD_DEFINED__

#ifdef __cplusplus
typedef class SENS SENS;
#else
typedef struct SENS SENS;
#endif /* __cplusplus */

#endif 	/* __SENS_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_sensevts_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_sensevts_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_sensevts_0000_0000_v0_0_s_ifspec;


#ifndef __SensEvents_LIBRARY_DEFINED__
#define __SensEvents_LIBRARY_DEFINED__

/* library SensEvents */
/* [helpstring][version][uuid] */ 

typedef /* [uuid] */  DECLSPEC_UUID("d597fad1-5b9f-11d1-8dd2-00aa004abd5e") struct SENS_QOCINFO
    {
    DWORD dwSize;
    DWORD dwFlags;
    DWORD dwOutSpeed;
    DWORD dwInSpeed;
    } 	SENS_QOCINFO;

typedef SENS_QOCINFO *LPSENS_QOCINFO;


EXTERN_C const IID LIBID_SensEvents;

#ifndef __ISensNetwork_INTERFACE_DEFINED__
#define __ISensNetwork_INTERFACE_DEFINED__

/* interface ISensNetwork */
/* [dual][helpstring][version][uuid][object] */ 


EXTERN_C const IID IID_ISensNetwork;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d597bab1-5b9f-11d1-8dd2-00aa004abd5e")
    ISensNetwork : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ConnectionMade( 
            /* [in] */ __RPC__in BSTR bstrConnection,
            /* [in] */ ULONG ulType,
            /* [in] */ __RPC__in LPSENS_QOCINFO lpQOCInfo) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ConnectionMadeNoQOCInfo( 
            /* [in] */ __RPC__in BSTR bstrConnection,
            /* [in] */ ULONG ulType) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ConnectionLost( 
            /* [in] */ __RPC__in BSTR bstrConnection,
            /* [in] */ ULONG ulType) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DestinationReachable( 
            /* [in] */ __RPC__in BSTR bstrDestination,
            /* [in] */ __RPC__in BSTR bstrConnection,
            /* [in] */ ULONG ulType,
            /* [in] */ __RPC__in LPSENS_QOCINFO lpQOCInfo) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DestinationReachableNoQOCInfo( 
            /* [in] */ __RPC__in BSTR bstrDestination,
            /* [in] */ __RPC__in BSTR bstrConnection,
            /* [in] */ ULONG ulType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISensNetworkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISensNetwork * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISensNetwork * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISensNetwork * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISensNetwork * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISensNetwork * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISensNetwork * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISensNetwork * This,
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
        
        DECLSPEC_XFGVIRT(ISensNetwork, ConnectionMade)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ConnectionMade )( 
            __RPC__in ISensNetwork * This,
            /* [in] */ __RPC__in BSTR bstrConnection,
            /* [in] */ ULONG ulType,
            /* [in] */ __RPC__in LPSENS_QOCINFO lpQOCInfo);
        
        DECLSPEC_XFGVIRT(ISensNetwork, ConnectionMadeNoQOCInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ConnectionMadeNoQOCInfo )( 
            __RPC__in ISensNetwork * This,
            /* [in] */ __RPC__in BSTR bstrConnection,
            /* [in] */ ULONG ulType);
        
        DECLSPEC_XFGVIRT(ISensNetwork, ConnectionLost)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ConnectionLost )( 
            __RPC__in ISensNetwork * This,
            /* [in] */ __RPC__in BSTR bstrConnection,
            /* [in] */ ULONG ulType);
        
        DECLSPEC_XFGVIRT(ISensNetwork, DestinationReachable)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DestinationReachable )( 
            __RPC__in ISensNetwork * This,
            /* [in] */ __RPC__in BSTR bstrDestination,
            /* [in] */ __RPC__in BSTR bstrConnection,
            /* [in] */ ULONG ulType,
            /* [in] */ __RPC__in LPSENS_QOCINFO lpQOCInfo);
        
        DECLSPEC_XFGVIRT(ISensNetwork, DestinationReachableNoQOCInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DestinationReachableNoQOCInfo )( 
            __RPC__in ISensNetwork * This,
            /* [in] */ __RPC__in BSTR bstrDestination,
            /* [in] */ __RPC__in BSTR bstrConnection,
            /* [in] */ ULONG ulType);
        
        END_INTERFACE
    } ISensNetworkVtbl;

    interface ISensNetwork
    {
        CONST_VTBL struct ISensNetworkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISensNetwork_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISensNetwork_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISensNetwork_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISensNetwork_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISensNetwork_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISensNetwork_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISensNetwork_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISensNetwork_ConnectionMade(This,bstrConnection,ulType,lpQOCInfo)	\
    ( (This)->lpVtbl -> ConnectionMade(This,bstrConnection,ulType,lpQOCInfo) ) 

#define ISensNetwork_ConnectionMadeNoQOCInfo(This,bstrConnection,ulType)	\
    ( (This)->lpVtbl -> ConnectionMadeNoQOCInfo(This,bstrConnection,ulType) ) 

#define ISensNetwork_ConnectionLost(This,bstrConnection,ulType)	\
    ( (This)->lpVtbl -> ConnectionLost(This,bstrConnection,ulType) ) 

#define ISensNetwork_DestinationReachable(This,bstrDestination,bstrConnection,ulType,lpQOCInfo)	\
    ( (This)->lpVtbl -> DestinationReachable(This,bstrDestination,bstrConnection,ulType,lpQOCInfo) ) 

#define ISensNetwork_DestinationReachableNoQOCInfo(This,bstrDestination,bstrConnection,ulType)	\
    ( (This)->lpVtbl -> DestinationReachableNoQOCInfo(This,bstrDestination,bstrConnection,ulType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISensNetwork_INTERFACE_DEFINED__ */


#ifndef __ISensOnNow_INTERFACE_DEFINED__
#define __ISensOnNow_INTERFACE_DEFINED__

/* interface ISensOnNow */
/* [dual][helpstring][version][uuid][object] */ 


EXTERN_C const IID IID_ISensOnNow;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d597bab2-5b9f-11d1-8dd2-00aa004abd5e")
    ISensOnNow : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE OnACPower( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE OnBatteryPower( 
            /* [in] */ DWORD dwBatteryLifePercent) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE BatteryLow( 
            /* [in] */ DWORD dwBatteryLifePercent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISensOnNowVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISensOnNow * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISensOnNow * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISensOnNow * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISensOnNow * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISensOnNow * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISensOnNow * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISensOnNow * This,
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
        
        DECLSPEC_XFGVIRT(ISensOnNow, OnACPower)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *OnACPower )( 
            __RPC__in ISensOnNow * This);
        
        DECLSPEC_XFGVIRT(ISensOnNow, OnBatteryPower)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *OnBatteryPower )( 
            __RPC__in ISensOnNow * This,
            /* [in] */ DWORD dwBatteryLifePercent);
        
        DECLSPEC_XFGVIRT(ISensOnNow, BatteryLow)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BatteryLow )( 
            __RPC__in ISensOnNow * This,
            /* [in] */ DWORD dwBatteryLifePercent);
        
        END_INTERFACE
    } ISensOnNowVtbl;

    interface ISensOnNow
    {
        CONST_VTBL struct ISensOnNowVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISensOnNow_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISensOnNow_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISensOnNow_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISensOnNow_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISensOnNow_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISensOnNow_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISensOnNow_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISensOnNow_OnACPower(This)	\
    ( (This)->lpVtbl -> OnACPower(This) ) 

#define ISensOnNow_OnBatteryPower(This,dwBatteryLifePercent)	\
    ( (This)->lpVtbl -> OnBatteryPower(This,dwBatteryLifePercent) ) 

#define ISensOnNow_BatteryLow(This,dwBatteryLifePercent)	\
    ( (This)->lpVtbl -> BatteryLow(This,dwBatteryLifePercent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISensOnNow_INTERFACE_DEFINED__ */


#ifndef __ISensLogon_INTERFACE_DEFINED__
#define __ISensLogon_INTERFACE_DEFINED__

/* interface ISensLogon */
/* [dual][helpstring][version][uuid][object] */ 


EXTERN_C const IID IID_ISensLogon;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d597bab3-5b9f-11d1-8dd2-00aa004abd5e")
    ISensLogon : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Logon( 
            /* [in] */ __RPC__in BSTR bstrUserName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Logoff( 
            /* [in] */ __RPC__in BSTR bstrUserName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE StartShell( 
            /* [in] */ __RPC__in BSTR bstrUserName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DisplayLock( 
            /* [in] */ __RPC__in BSTR bstrUserName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DisplayUnlock( 
            /* [in] */ __RPC__in BSTR bstrUserName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE StartScreenSaver( 
            /* [in] */ __RPC__in BSTR bstrUserName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE StopScreenSaver( 
            /* [in] */ __RPC__in BSTR bstrUserName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISensLogonVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISensLogon * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISensLogon * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISensLogon * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISensLogon * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISensLogon * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISensLogon * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISensLogon * This,
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
        
        DECLSPEC_XFGVIRT(ISensLogon, Logon)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Logon )( 
            __RPC__in ISensLogon * This,
            /* [in] */ __RPC__in BSTR bstrUserName);
        
        DECLSPEC_XFGVIRT(ISensLogon, Logoff)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Logoff )( 
            __RPC__in ISensLogon * This,
            /* [in] */ __RPC__in BSTR bstrUserName);
        
        DECLSPEC_XFGVIRT(ISensLogon, StartShell)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *StartShell )( 
            __RPC__in ISensLogon * This,
            /* [in] */ __RPC__in BSTR bstrUserName);
        
        DECLSPEC_XFGVIRT(ISensLogon, DisplayLock)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DisplayLock )( 
            __RPC__in ISensLogon * This,
            /* [in] */ __RPC__in BSTR bstrUserName);
        
        DECLSPEC_XFGVIRT(ISensLogon, DisplayUnlock)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DisplayUnlock )( 
            __RPC__in ISensLogon * This,
            /* [in] */ __RPC__in BSTR bstrUserName);
        
        DECLSPEC_XFGVIRT(ISensLogon, StartScreenSaver)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *StartScreenSaver )( 
            __RPC__in ISensLogon * This,
            /* [in] */ __RPC__in BSTR bstrUserName);
        
        DECLSPEC_XFGVIRT(ISensLogon, StopScreenSaver)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *StopScreenSaver )( 
            __RPC__in ISensLogon * This,
            /* [in] */ __RPC__in BSTR bstrUserName);
        
        END_INTERFACE
    } ISensLogonVtbl;

    interface ISensLogon
    {
        CONST_VTBL struct ISensLogonVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISensLogon_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISensLogon_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISensLogon_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISensLogon_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISensLogon_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISensLogon_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISensLogon_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISensLogon_Logon(This,bstrUserName)	\
    ( (This)->lpVtbl -> Logon(This,bstrUserName) ) 

#define ISensLogon_Logoff(This,bstrUserName)	\
    ( (This)->lpVtbl -> Logoff(This,bstrUserName) ) 

#define ISensLogon_StartShell(This,bstrUserName)	\
    ( (This)->lpVtbl -> StartShell(This,bstrUserName) ) 

#define ISensLogon_DisplayLock(This,bstrUserName)	\
    ( (This)->lpVtbl -> DisplayLock(This,bstrUserName) ) 

#define ISensLogon_DisplayUnlock(This,bstrUserName)	\
    ( (This)->lpVtbl -> DisplayUnlock(This,bstrUserName) ) 

#define ISensLogon_StartScreenSaver(This,bstrUserName)	\
    ( (This)->lpVtbl -> StartScreenSaver(This,bstrUserName) ) 

#define ISensLogon_StopScreenSaver(This,bstrUserName)	\
    ( (This)->lpVtbl -> StopScreenSaver(This,bstrUserName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISensLogon_INTERFACE_DEFINED__ */


#ifndef __ISensLogon2_INTERFACE_DEFINED__
#define __ISensLogon2_INTERFACE_DEFINED__

/* interface ISensLogon2 */
/* [dual][helpstring][version][uuid][object] */ 


EXTERN_C const IID IID_ISensLogon2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d597bab4-5b9f-11d1-8dd2-00aa004abd5e")
    ISensLogon2 : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Logon( 
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ DWORD dwSessionId) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Logoff( 
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ DWORD dwSessionId) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionDisconnect( 
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ DWORD dwSessionId) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SessionReconnect( 
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ DWORD dwSessionId) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE PostShell( 
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ DWORD dwSessionId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISensLogon2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISensLogon2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISensLogon2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISensLogon2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISensLogon2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISensLogon2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISensLogon2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISensLogon2 * This,
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
        
        DECLSPEC_XFGVIRT(ISensLogon2, Logon)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Logon )( 
            __RPC__in ISensLogon2 * This,
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ DWORD dwSessionId);
        
        DECLSPEC_XFGVIRT(ISensLogon2, Logoff)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Logoff )( 
            __RPC__in ISensLogon2 * This,
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ DWORD dwSessionId);
        
        DECLSPEC_XFGVIRT(ISensLogon2, SessionDisconnect)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionDisconnect )( 
            __RPC__in ISensLogon2 * This,
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ DWORD dwSessionId);
        
        DECLSPEC_XFGVIRT(ISensLogon2, SessionReconnect)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SessionReconnect )( 
            __RPC__in ISensLogon2 * This,
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ DWORD dwSessionId);
        
        DECLSPEC_XFGVIRT(ISensLogon2, PostShell)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PostShell )( 
            __RPC__in ISensLogon2 * This,
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ DWORD dwSessionId);
        
        END_INTERFACE
    } ISensLogon2Vtbl;

    interface ISensLogon2
    {
        CONST_VTBL struct ISensLogon2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISensLogon2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISensLogon2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISensLogon2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISensLogon2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISensLogon2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISensLogon2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISensLogon2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISensLogon2_Logon(This,bstrUserName,dwSessionId)	\
    ( (This)->lpVtbl -> Logon(This,bstrUserName,dwSessionId) ) 

#define ISensLogon2_Logoff(This,bstrUserName,dwSessionId)	\
    ( (This)->lpVtbl -> Logoff(This,bstrUserName,dwSessionId) ) 

#define ISensLogon2_SessionDisconnect(This,bstrUserName,dwSessionId)	\
    ( (This)->lpVtbl -> SessionDisconnect(This,bstrUserName,dwSessionId) ) 

#define ISensLogon2_SessionReconnect(This,bstrUserName,dwSessionId)	\
    ( (This)->lpVtbl -> SessionReconnect(This,bstrUserName,dwSessionId) ) 

#define ISensLogon2_PostShell(This,bstrUserName,dwSessionId)	\
    ( (This)->lpVtbl -> PostShell(This,bstrUserName,dwSessionId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISensLogon2_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_SENS;

#ifdef __cplusplus

class DECLSPEC_UUID("d597cafe-5b9f-11d1-8dd2-00aa004abd5e")
SENS;
#endif
#endif /* __SensEvents_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_sensevts_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_sensevts_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_sensevts_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


