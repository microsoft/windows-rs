

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

#ifndef __tapi3cc_h__
#define __tapi3cc_h__

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

#ifndef __ITAgent_FWD_DEFINED__
#define __ITAgent_FWD_DEFINED__
typedef interface ITAgent ITAgent;

#endif 	/* __ITAgent_FWD_DEFINED__ */


#ifndef __ITAgentSession_FWD_DEFINED__
#define __ITAgentSession_FWD_DEFINED__
typedef interface ITAgentSession ITAgentSession;

#endif 	/* __ITAgentSession_FWD_DEFINED__ */


#ifndef __ITACDGroup_FWD_DEFINED__
#define __ITACDGroup_FWD_DEFINED__
typedef interface ITACDGroup ITACDGroup;

#endif 	/* __ITACDGroup_FWD_DEFINED__ */


#ifndef __ITQueue_FWD_DEFINED__
#define __ITQueue_FWD_DEFINED__
typedef interface ITQueue ITQueue;

#endif 	/* __ITQueue_FWD_DEFINED__ */


#ifndef __ITAgentEvent_FWD_DEFINED__
#define __ITAgentEvent_FWD_DEFINED__
typedef interface ITAgentEvent ITAgentEvent;

#endif 	/* __ITAgentEvent_FWD_DEFINED__ */


#ifndef __ITAgentSessionEvent_FWD_DEFINED__
#define __ITAgentSessionEvent_FWD_DEFINED__
typedef interface ITAgentSessionEvent ITAgentSessionEvent;

#endif 	/* __ITAgentSessionEvent_FWD_DEFINED__ */


#ifndef __ITACDGroupEvent_FWD_DEFINED__
#define __ITACDGroupEvent_FWD_DEFINED__
typedef interface ITACDGroupEvent ITACDGroupEvent;

#endif 	/* __ITACDGroupEvent_FWD_DEFINED__ */


#ifndef __ITQueueEvent_FWD_DEFINED__
#define __ITQueueEvent_FWD_DEFINED__
typedef interface ITQueueEvent ITQueueEvent;

#endif 	/* __ITQueueEvent_FWD_DEFINED__ */


#ifndef __ITAgentHandlerEvent_FWD_DEFINED__
#define __ITAgentHandlerEvent_FWD_DEFINED__
typedef interface ITAgentHandlerEvent ITAgentHandlerEvent;

#endif 	/* __ITAgentHandlerEvent_FWD_DEFINED__ */


#ifndef __ITTAPICallCenter_FWD_DEFINED__
#define __ITTAPICallCenter_FWD_DEFINED__
typedef interface ITTAPICallCenter ITTAPICallCenter;

#endif 	/* __ITTAPICallCenter_FWD_DEFINED__ */


#ifndef __ITAgentHandler_FWD_DEFINED__
#define __ITAgentHandler_FWD_DEFINED__
typedef interface ITAgentHandler ITAgentHandler;

#endif 	/* __ITAgentHandler_FWD_DEFINED__ */


#ifndef __IEnumAgent_FWD_DEFINED__
#define __IEnumAgent_FWD_DEFINED__
typedef interface IEnumAgent IEnumAgent;

#endif 	/* __IEnumAgent_FWD_DEFINED__ */


#ifndef __IEnumAgentSession_FWD_DEFINED__
#define __IEnumAgentSession_FWD_DEFINED__
typedef interface IEnumAgentSession IEnumAgentSession;

#endif 	/* __IEnumAgentSession_FWD_DEFINED__ */


#ifndef __IEnumQueue_FWD_DEFINED__
#define __IEnumQueue_FWD_DEFINED__
typedef interface IEnumQueue IEnumQueue;

#endif 	/* __IEnumQueue_FWD_DEFINED__ */


#ifndef __IEnumACDGroup_FWD_DEFINED__
#define __IEnumACDGroup_FWD_DEFINED__
typedef interface IEnumACDGroup IEnumACDGroup;

#endif 	/* __IEnumACDGroup_FWD_DEFINED__ */


#ifndef __IEnumAgentHandler_FWD_DEFINED__
#define __IEnumAgentHandler_FWD_DEFINED__
typedef interface IEnumAgentHandler IEnumAgentHandler;

#endif 	/* __IEnumAgentHandler_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "tapi3if.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_tapi3cc_0000_0000 */
/* [local] */ 

/* Copyright (c) Microsoft Corporation. All rights reserved. */
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef 
enum AGENT_EVENT
    {
        AE_NOT_READY	= 0,
        AE_READY	= ( AE_NOT_READY + 1 ) ,
        AE_BUSY_ACD	= ( AE_READY + 1 ) ,
        AE_BUSY_INCOMING	= ( AE_BUSY_ACD + 1 ) ,
        AE_BUSY_OUTGOING	= ( AE_BUSY_INCOMING + 1 ) ,
        AE_UNKNOWN	= ( AE_BUSY_OUTGOING + 1 ) 
    } 	AGENT_EVENT;

typedef 
enum AGENT_STATE
    {
        AS_NOT_READY	= 0,
        AS_READY	= ( AS_NOT_READY + 1 ) ,
        AS_BUSY_ACD	= ( AS_READY + 1 ) ,
        AS_BUSY_INCOMING	= ( AS_BUSY_ACD + 1 ) ,
        AS_BUSY_OUTGOING	= ( AS_BUSY_INCOMING + 1 ) ,
        AS_UNKNOWN	= ( AS_BUSY_OUTGOING + 1 ) 
    } 	AGENT_STATE;

typedef 
enum AGENT_SESSION_EVENT
    {
        ASE_NEW_SESSION	= 0,
        ASE_NOT_READY	= ( ASE_NEW_SESSION + 1 ) ,
        ASE_READY	= ( ASE_NOT_READY + 1 ) ,
        ASE_BUSY	= ( ASE_READY + 1 ) ,
        ASE_WRAPUP	= ( ASE_BUSY + 1 ) ,
        ASE_END	= ( ASE_WRAPUP + 1 ) 
    } 	AGENT_SESSION_EVENT;

typedef 
enum AGENT_SESSION_STATE
    {
        ASST_NOT_READY	= 0,
        ASST_READY	= ( ASST_NOT_READY + 1 ) ,
        ASST_BUSY_ON_CALL	= ( ASST_READY + 1 ) ,
        ASST_BUSY_WRAPUP	= ( ASST_BUSY_ON_CALL + 1 ) ,
        ASST_SESSION_ENDED	= ( ASST_BUSY_WRAPUP + 1 ) 
    } 	AGENT_SESSION_STATE;

typedef 
enum AGENTHANDLER_EVENT
    {
        AHE_NEW_AGENTHANDLER	= 0,
        AHE_AGENTHANDLER_REMOVED	= ( AHE_NEW_AGENTHANDLER + 1 ) 
    } 	AGENTHANDLER_EVENT;

typedef 
enum ACDGROUP_EVENT
    {
        ACDGE_NEW_GROUP	= 0,
        ACDGE_GROUP_REMOVED	= ( ACDGE_NEW_GROUP + 1 ) 
    } 	ACDGROUP_EVENT;

typedef 
enum ACDQUEUE_EVENT
    {
        ACDQE_NEW_QUEUE	= 0,
        ACDQE_QUEUE_REMOVED	= ( ACDQE_NEW_QUEUE + 1 ) 
    } 	ACDQUEUE_EVENT;


















extern RPC_IF_HANDLE __MIDL_itf_tapi3cc_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tapi3cc_0000_0000_v0_0_s_ifspec;

#ifndef __ITAgent_INTERFACE_DEFINED__
#define __ITAgent_INTERFACE_DEFINED__

/* interface ITAgent */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITAgent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5770ECE5-4B27-11d1-BF80-00805FC147D3")
    ITAgent : public IDispatch
    {
    public:
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateAgentSessions( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumAgentSession **ppEnumAgentSession) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateSession( 
            /* [in] */ __RPC__in_opt ITACDGroup *pACDGroup,
            /* [in] */ __RPC__in_opt ITAddress *pAddress,
            /* [retval][out] */ __RPC__deref_out_opt ITAgentSession **ppAgentSession) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateSessionWithPIN( 
            /* [in] */ __RPC__in_opt ITACDGroup *pACDGroup,
            /* [in] */ __RPC__in_opt ITAddress *pAddress,
            /* [in] */ __RPC__in BSTR pPIN,
            /* [retval][out] */ __RPC__deref_out_opt ITAgentSession **ppAgentSession) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_User( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppUser) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_State( 
            /* [in] */ AGENT_STATE AgentState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out AGENT_STATE *pAgentState) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MeasurementPeriod( 
            /* [in] */ long lPeriod) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MeasurementPeriod( 
            /* [retval][out] */ __RPC__out long *plPeriod) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OverallCallRate( 
            /* [retval][out] */ __RPC__out CURRENCY *pcyCallrate) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NumberOfACDCalls( 
            /* [retval][out] */ __RPC__out long *plCalls) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NumberOfIncomingCalls( 
            /* [retval][out] */ __RPC__out long *plCalls) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NumberOfOutgoingCalls( 
            /* [retval][out] */ __RPC__out long *plCalls) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalACDTalkTime( 
            /* [retval][out] */ __RPC__out long *plTalkTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalACDCallTime( 
            /* [retval][out] */ __RPC__out long *plCallTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalWrapUpTime( 
            /* [retval][out] */ __RPC__out long *plWrapUpTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AgentSessions( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITAgentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITAgent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITAgent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITAgent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITAgent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITAgent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITAgent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITAgent * This,
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
        
        DECLSPEC_XFGVIRT(ITAgent, EnumerateAgentSessions)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateAgentSessions )( 
            __RPC__in ITAgent * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumAgentSession **ppEnumAgentSession);
        
        DECLSPEC_XFGVIRT(ITAgent, CreateSession)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateSession )( 
            __RPC__in ITAgent * This,
            /* [in] */ __RPC__in_opt ITACDGroup *pACDGroup,
            /* [in] */ __RPC__in_opt ITAddress *pAddress,
            /* [retval][out] */ __RPC__deref_out_opt ITAgentSession **ppAgentSession);
        
        DECLSPEC_XFGVIRT(ITAgent, CreateSessionWithPIN)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateSessionWithPIN )( 
            __RPC__in ITAgent * This,
            /* [in] */ __RPC__in_opt ITACDGroup *pACDGroup,
            /* [in] */ __RPC__in_opt ITAddress *pAddress,
            /* [in] */ __RPC__in BSTR pPIN,
            /* [retval][out] */ __RPC__deref_out_opt ITAgentSession **ppAgentSession);
        
        DECLSPEC_XFGVIRT(ITAgent, get_ID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ID )( 
            __RPC__in ITAgent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppID);
        
        DECLSPEC_XFGVIRT(ITAgent, get_User)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_User )( 
            __RPC__in ITAgent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppUser);
        
        DECLSPEC_XFGVIRT(ITAgent, put_State)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_State )( 
            __RPC__in ITAgent * This,
            /* [in] */ AGENT_STATE AgentState);
        
        DECLSPEC_XFGVIRT(ITAgent, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in ITAgent * This,
            /* [retval][out] */ __RPC__out AGENT_STATE *pAgentState);
        
        DECLSPEC_XFGVIRT(ITAgent, put_MeasurementPeriod)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MeasurementPeriod )( 
            __RPC__in ITAgent * This,
            /* [in] */ long lPeriod);
        
        DECLSPEC_XFGVIRT(ITAgent, get_MeasurementPeriod)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MeasurementPeriod )( 
            __RPC__in ITAgent * This,
            /* [retval][out] */ __RPC__out long *plPeriod);
        
        DECLSPEC_XFGVIRT(ITAgent, get_OverallCallRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OverallCallRate )( 
            __RPC__in ITAgent * This,
            /* [retval][out] */ __RPC__out CURRENCY *pcyCallrate);
        
        DECLSPEC_XFGVIRT(ITAgent, get_NumberOfACDCalls)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NumberOfACDCalls )( 
            __RPC__in ITAgent * This,
            /* [retval][out] */ __RPC__out long *plCalls);
        
        DECLSPEC_XFGVIRT(ITAgent, get_NumberOfIncomingCalls)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NumberOfIncomingCalls )( 
            __RPC__in ITAgent * This,
            /* [retval][out] */ __RPC__out long *plCalls);
        
        DECLSPEC_XFGVIRT(ITAgent, get_NumberOfOutgoingCalls)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NumberOfOutgoingCalls )( 
            __RPC__in ITAgent * This,
            /* [retval][out] */ __RPC__out long *plCalls);
        
        DECLSPEC_XFGVIRT(ITAgent, get_TotalACDTalkTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalACDTalkTime )( 
            __RPC__in ITAgent * This,
            /* [retval][out] */ __RPC__out long *plTalkTime);
        
        DECLSPEC_XFGVIRT(ITAgent, get_TotalACDCallTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalACDCallTime )( 
            __RPC__in ITAgent * This,
            /* [retval][out] */ __RPC__out long *plCallTime);
        
        DECLSPEC_XFGVIRT(ITAgent, get_TotalWrapUpTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalWrapUpTime )( 
            __RPC__in ITAgent * This,
            /* [retval][out] */ __RPC__out long *plWrapUpTime);
        
        DECLSPEC_XFGVIRT(ITAgent, get_AgentSessions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AgentSessions )( 
            __RPC__in ITAgent * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        END_INTERFACE
    } ITAgentVtbl;

    interface ITAgent
    {
        CONST_VTBL struct ITAgentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITAgent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITAgent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITAgent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITAgent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITAgent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITAgent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITAgent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITAgent_EnumerateAgentSessions(This,ppEnumAgentSession)	\
    ( (This)->lpVtbl -> EnumerateAgentSessions(This,ppEnumAgentSession) ) 

#define ITAgent_CreateSession(This,pACDGroup,pAddress,ppAgentSession)	\
    ( (This)->lpVtbl -> CreateSession(This,pACDGroup,pAddress,ppAgentSession) ) 

#define ITAgent_CreateSessionWithPIN(This,pACDGroup,pAddress,pPIN,ppAgentSession)	\
    ( (This)->lpVtbl -> CreateSessionWithPIN(This,pACDGroup,pAddress,pPIN,ppAgentSession) ) 

#define ITAgent_get_ID(This,ppID)	\
    ( (This)->lpVtbl -> get_ID(This,ppID) ) 

#define ITAgent_get_User(This,ppUser)	\
    ( (This)->lpVtbl -> get_User(This,ppUser) ) 

#define ITAgent_put_State(This,AgentState)	\
    ( (This)->lpVtbl -> put_State(This,AgentState) ) 

#define ITAgent_get_State(This,pAgentState)	\
    ( (This)->lpVtbl -> get_State(This,pAgentState) ) 

#define ITAgent_put_MeasurementPeriod(This,lPeriod)	\
    ( (This)->lpVtbl -> put_MeasurementPeriod(This,lPeriod) ) 

#define ITAgent_get_MeasurementPeriod(This,plPeriod)	\
    ( (This)->lpVtbl -> get_MeasurementPeriod(This,plPeriod) ) 

#define ITAgent_get_OverallCallRate(This,pcyCallrate)	\
    ( (This)->lpVtbl -> get_OverallCallRate(This,pcyCallrate) ) 

#define ITAgent_get_NumberOfACDCalls(This,plCalls)	\
    ( (This)->lpVtbl -> get_NumberOfACDCalls(This,plCalls) ) 

#define ITAgent_get_NumberOfIncomingCalls(This,plCalls)	\
    ( (This)->lpVtbl -> get_NumberOfIncomingCalls(This,plCalls) ) 

#define ITAgent_get_NumberOfOutgoingCalls(This,plCalls)	\
    ( (This)->lpVtbl -> get_NumberOfOutgoingCalls(This,plCalls) ) 

#define ITAgent_get_TotalACDTalkTime(This,plTalkTime)	\
    ( (This)->lpVtbl -> get_TotalACDTalkTime(This,plTalkTime) ) 

#define ITAgent_get_TotalACDCallTime(This,plCallTime)	\
    ( (This)->lpVtbl -> get_TotalACDCallTime(This,plCallTime) ) 

#define ITAgent_get_TotalWrapUpTime(This,plWrapUpTime)	\
    ( (This)->lpVtbl -> get_TotalWrapUpTime(This,plWrapUpTime) ) 

#define ITAgent_get_AgentSessions(This,pVariant)	\
    ( (This)->lpVtbl -> get_AgentSessions(This,pVariant) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITAgent_INTERFACE_DEFINED__ */


#ifndef __ITAgentSession_INTERFACE_DEFINED__
#define __ITAgentSession_INTERFACE_DEFINED__

/* interface ITAgentSession */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITAgentSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5AFC3147-4BCC-11d1-BF80-00805FC147D3")
    ITAgentSession : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Agent( 
            /* [retval][out] */ __RPC__deref_out_opt ITAgent **ppAgent) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Address( 
            /* [retval][out] */ __RPC__deref_out_opt ITAddress **ppAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ACDGroup( 
            /* [retval][out] */ __RPC__deref_out_opt ITACDGroup **ppACDGroup) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_State( 
            /* [in] */ AGENT_SESSION_STATE SessionState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out AGENT_SESSION_STATE *pSessionState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SessionStartTime( 
            /* [retval][out] */ __RPC__out DATE *pdateSessionStart) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SessionDuration( 
            /* [retval][out] */ __RPC__out long *plDuration) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NumberOfCalls( 
            /* [retval][out] */ __RPC__out long *plCalls) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalTalkTime( 
            /* [retval][out] */ __RPC__out long *plTalkTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AverageTalkTime( 
            /* [retval][out] */ __RPC__out long *plTalkTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalCallTime( 
            /* [retval][out] */ __RPC__out long *plCallTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AverageCallTime( 
            /* [retval][out] */ __RPC__out long *plCallTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalWrapUpTime( 
            /* [retval][out] */ __RPC__out long *plWrapUpTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AverageWrapUpTime( 
            /* [retval][out] */ __RPC__out long *plWrapUpTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ACDCallRate( 
            /* [retval][out] */ __RPC__out CURRENCY *pcyCallrate) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LongestTimeToAnswer( 
            /* [retval][out] */ __RPC__out long *plAnswerTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AverageTimeToAnswer( 
            /* [retval][out] */ __RPC__out long *plAnswerTime) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITAgentSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITAgentSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITAgentSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITAgentSession * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITAgentSession * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITAgentSession * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITAgentSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITAgentSession * This,
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
        
        DECLSPEC_XFGVIRT(ITAgentSession, get_Agent)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Agent )( 
            __RPC__in ITAgentSession * This,
            /* [retval][out] */ __RPC__deref_out_opt ITAgent **ppAgent);
        
        DECLSPEC_XFGVIRT(ITAgentSession, get_Address)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Address )( 
            __RPC__in ITAgentSession * This,
            /* [retval][out] */ __RPC__deref_out_opt ITAddress **ppAddress);
        
        DECLSPEC_XFGVIRT(ITAgentSession, get_ACDGroup)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ACDGroup )( 
            __RPC__in ITAgentSession * This,
            /* [retval][out] */ __RPC__deref_out_opt ITACDGroup **ppACDGroup);
        
        DECLSPEC_XFGVIRT(ITAgentSession, put_State)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_State )( 
            __RPC__in ITAgentSession * This,
            /* [in] */ AGENT_SESSION_STATE SessionState);
        
        DECLSPEC_XFGVIRT(ITAgentSession, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in ITAgentSession * This,
            /* [retval][out] */ __RPC__out AGENT_SESSION_STATE *pSessionState);
        
        DECLSPEC_XFGVIRT(ITAgentSession, get_SessionStartTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SessionStartTime )( 
            __RPC__in ITAgentSession * This,
            /* [retval][out] */ __RPC__out DATE *pdateSessionStart);
        
        DECLSPEC_XFGVIRT(ITAgentSession, get_SessionDuration)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SessionDuration )( 
            __RPC__in ITAgentSession * This,
            /* [retval][out] */ __RPC__out long *plDuration);
        
        DECLSPEC_XFGVIRT(ITAgentSession, get_NumberOfCalls)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NumberOfCalls )( 
            __RPC__in ITAgentSession * This,
            /* [retval][out] */ __RPC__out long *plCalls);
        
        DECLSPEC_XFGVIRT(ITAgentSession, get_TotalTalkTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalTalkTime )( 
            __RPC__in ITAgentSession * This,
            /* [retval][out] */ __RPC__out long *plTalkTime);
        
        DECLSPEC_XFGVIRT(ITAgentSession, get_AverageTalkTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AverageTalkTime )( 
            __RPC__in ITAgentSession * This,
            /* [retval][out] */ __RPC__out long *plTalkTime);
        
        DECLSPEC_XFGVIRT(ITAgentSession, get_TotalCallTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalCallTime )( 
            __RPC__in ITAgentSession * This,
            /* [retval][out] */ __RPC__out long *plCallTime);
        
        DECLSPEC_XFGVIRT(ITAgentSession, get_AverageCallTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AverageCallTime )( 
            __RPC__in ITAgentSession * This,
            /* [retval][out] */ __RPC__out long *plCallTime);
        
        DECLSPEC_XFGVIRT(ITAgentSession, get_TotalWrapUpTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalWrapUpTime )( 
            __RPC__in ITAgentSession * This,
            /* [retval][out] */ __RPC__out long *plWrapUpTime);
        
        DECLSPEC_XFGVIRT(ITAgentSession, get_AverageWrapUpTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AverageWrapUpTime )( 
            __RPC__in ITAgentSession * This,
            /* [retval][out] */ __RPC__out long *plWrapUpTime);
        
        DECLSPEC_XFGVIRT(ITAgentSession, get_ACDCallRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ACDCallRate )( 
            __RPC__in ITAgentSession * This,
            /* [retval][out] */ __RPC__out CURRENCY *pcyCallrate);
        
        DECLSPEC_XFGVIRT(ITAgentSession, get_LongestTimeToAnswer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LongestTimeToAnswer )( 
            __RPC__in ITAgentSession * This,
            /* [retval][out] */ __RPC__out long *plAnswerTime);
        
        DECLSPEC_XFGVIRT(ITAgentSession, get_AverageTimeToAnswer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AverageTimeToAnswer )( 
            __RPC__in ITAgentSession * This,
            /* [retval][out] */ __RPC__out long *plAnswerTime);
        
        END_INTERFACE
    } ITAgentSessionVtbl;

    interface ITAgentSession
    {
        CONST_VTBL struct ITAgentSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITAgentSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITAgentSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITAgentSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITAgentSession_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITAgentSession_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITAgentSession_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITAgentSession_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITAgentSession_get_Agent(This,ppAgent)	\
    ( (This)->lpVtbl -> get_Agent(This,ppAgent) ) 

#define ITAgentSession_get_Address(This,ppAddress)	\
    ( (This)->lpVtbl -> get_Address(This,ppAddress) ) 

#define ITAgentSession_get_ACDGroup(This,ppACDGroup)	\
    ( (This)->lpVtbl -> get_ACDGroup(This,ppACDGroup) ) 

#define ITAgentSession_put_State(This,SessionState)	\
    ( (This)->lpVtbl -> put_State(This,SessionState) ) 

#define ITAgentSession_get_State(This,pSessionState)	\
    ( (This)->lpVtbl -> get_State(This,pSessionState) ) 

#define ITAgentSession_get_SessionStartTime(This,pdateSessionStart)	\
    ( (This)->lpVtbl -> get_SessionStartTime(This,pdateSessionStart) ) 

#define ITAgentSession_get_SessionDuration(This,plDuration)	\
    ( (This)->lpVtbl -> get_SessionDuration(This,plDuration) ) 

#define ITAgentSession_get_NumberOfCalls(This,plCalls)	\
    ( (This)->lpVtbl -> get_NumberOfCalls(This,plCalls) ) 

#define ITAgentSession_get_TotalTalkTime(This,plTalkTime)	\
    ( (This)->lpVtbl -> get_TotalTalkTime(This,plTalkTime) ) 

#define ITAgentSession_get_AverageTalkTime(This,plTalkTime)	\
    ( (This)->lpVtbl -> get_AverageTalkTime(This,plTalkTime) ) 

#define ITAgentSession_get_TotalCallTime(This,plCallTime)	\
    ( (This)->lpVtbl -> get_TotalCallTime(This,plCallTime) ) 

#define ITAgentSession_get_AverageCallTime(This,plCallTime)	\
    ( (This)->lpVtbl -> get_AverageCallTime(This,plCallTime) ) 

#define ITAgentSession_get_TotalWrapUpTime(This,plWrapUpTime)	\
    ( (This)->lpVtbl -> get_TotalWrapUpTime(This,plWrapUpTime) ) 

#define ITAgentSession_get_AverageWrapUpTime(This,plWrapUpTime)	\
    ( (This)->lpVtbl -> get_AverageWrapUpTime(This,plWrapUpTime) ) 

#define ITAgentSession_get_ACDCallRate(This,pcyCallrate)	\
    ( (This)->lpVtbl -> get_ACDCallRate(This,pcyCallrate) ) 

#define ITAgentSession_get_LongestTimeToAnswer(This,plAnswerTime)	\
    ( (This)->lpVtbl -> get_LongestTimeToAnswer(This,plAnswerTime) ) 

#define ITAgentSession_get_AverageTimeToAnswer(This,plAnswerTime)	\
    ( (This)->lpVtbl -> get_AverageTimeToAnswer(This,plAnswerTime) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITAgentSession_INTERFACE_DEFINED__ */


#ifndef __ITACDGroup_INTERFACE_DEFINED__
#define __ITACDGroup_INTERFACE_DEFINED__

/* interface ITACDGroup */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITACDGroup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5AFC3148-4BCC-11d1-BF80-00805FC147D3")
    ITACDGroup : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppName) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateQueues( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumQueue **ppEnumQueue) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Queues( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITACDGroupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITACDGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITACDGroup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITACDGroup * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITACDGroup * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITACDGroup * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITACDGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITACDGroup * This,
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
        
        DECLSPEC_XFGVIRT(ITACDGroup, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ITACDGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppName);
        
        DECLSPEC_XFGVIRT(ITACDGroup, EnumerateQueues)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateQueues )( 
            __RPC__in ITACDGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumQueue **ppEnumQueue);
        
        DECLSPEC_XFGVIRT(ITACDGroup, get_Queues)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Queues )( 
            __RPC__in ITACDGroup * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        END_INTERFACE
    } ITACDGroupVtbl;

    interface ITACDGroup
    {
        CONST_VTBL struct ITACDGroupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITACDGroup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITACDGroup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITACDGroup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITACDGroup_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITACDGroup_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITACDGroup_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITACDGroup_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITACDGroup_get_Name(This,ppName)	\
    ( (This)->lpVtbl -> get_Name(This,ppName) ) 

#define ITACDGroup_EnumerateQueues(This,ppEnumQueue)	\
    ( (This)->lpVtbl -> EnumerateQueues(This,ppEnumQueue) ) 

#define ITACDGroup_get_Queues(This,pVariant)	\
    ( (This)->lpVtbl -> get_Queues(This,pVariant) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITACDGroup_INTERFACE_DEFINED__ */


#ifndef __ITQueue_INTERFACE_DEFINED__
#define __ITQueue_INTERFACE_DEFINED__

/* interface ITQueue */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITQueue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5AFC3149-4BCC-11d1-BF80-00805FC147D3")
    ITQueue : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MeasurementPeriod( 
            /* [in] */ long lPeriod) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MeasurementPeriod( 
            /* [retval][out] */ __RPC__out long *plPeriod) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalCallsQueued( 
            /* [retval][out] */ __RPC__out long *plCalls) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentCallsQueued( 
            /* [retval][out] */ __RPC__out long *plCalls) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalCallsAbandoned( 
            /* [retval][out] */ __RPC__out long *plCalls) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalCallsFlowedIn( 
            /* [retval][out] */ __RPC__out long *plCalls) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalCallsFlowedOut( 
            /* [retval][out] */ __RPC__out long *plCalls) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LongestEverWaitTime( 
            /* [retval][out] */ __RPC__out long *plWaitTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentLongestWaitTime( 
            /* [retval][out] */ __RPC__out long *plWaitTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AverageWaitTime( 
            /* [retval][out] */ __RPC__out long *plWaitTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FinalDisposition( 
            /* [retval][out] */ __RPC__out long *plCalls) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITQueueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITQueue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITQueue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITQueue * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITQueue * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITQueue * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITQueue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITQueue * This,
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
        
        DECLSPEC_XFGVIRT(ITQueue, put_MeasurementPeriod)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MeasurementPeriod )( 
            __RPC__in ITQueue * This,
            /* [in] */ long lPeriod);
        
        DECLSPEC_XFGVIRT(ITQueue, get_MeasurementPeriod)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MeasurementPeriod )( 
            __RPC__in ITQueue * This,
            /* [retval][out] */ __RPC__out long *plPeriod);
        
        DECLSPEC_XFGVIRT(ITQueue, get_TotalCallsQueued)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalCallsQueued )( 
            __RPC__in ITQueue * This,
            /* [retval][out] */ __RPC__out long *plCalls);
        
        DECLSPEC_XFGVIRT(ITQueue, get_CurrentCallsQueued)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentCallsQueued )( 
            __RPC__in ITQueue * This,
            /* [retval][out] */ __RPC__out long *plCalls);
        
        DECLSPEC_XFGVIRT(ITQueue, get_TotalCallsAbandoned)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalCallsAbandoned )( 
            __RPC__in ITQueue * This,
            /* [retval][out] */ __RPC__out long *plCalls);
        
        DECLSPEC_XFGVIRT(ITQueue, get_TotalCallsFlowedIn)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalCallsFlowedIn )( 
            __RPC__in ITQueue * This,
            /* [retval][out] */ __RPC__out long *plCalls);
        
        DECLSPEC_XFGVIRT(ITQueue, get_TotalCallsFlowedOut)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalCallsFlowedOut )( 
            __RPC__in ITQueue * This,
            /* [retval][out] */ __RPC__out long *plCalls);
        
        DECLSPEC_XFGVIRT(ITQueue, get_LongestEverWaitTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LongestEverWaitTime )( 
            __RPC__in ITQueue * This,
            /* [retval][out] */ __RPC__out long *plWaitTime);
        
        DECLSPEC_XFGVIRT(ITQueue, get_CurrentLongestWaitTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentLongestWaitTime )( 
            __RPC__in ITQueue * This,
            /* [retval][out] */ __RPC__out long *plWaitTime);
        
        DECLSPEC_XFGVIRT(ITQueue, get_AverageWaitTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AverageWaitTime )( 
            __RPC__in ITQueue * This,
            /* [retval][out] */ __RPC__out long *plWaitTime);
        
        DECLSPEC_XFGVIRT(ITQueue, get_FinalDisposition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FinalDisposition )( 
            __RPC__in ITQueue * This,
            /* [retval][out] */ __RPC__out long *plCalls);
        
        DECLSPEC_XFGVIRT(ITQueue, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ITQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppName);
        
        END_INTERFACE
    } ITQueueVtbl;

    interface ITQueue
    {
        CONST_VTBL struct ITQueueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITQueue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITQueue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITQueue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITQueue_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITQueue_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITQueue_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITQueue_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITQueue_put_MeasurementPeriod(This,lPeriod)	\
    ( (This)->lpVtbl -> put_MeasurementPeriod(This,lPeriod) ) 

#define ITQueue_get_MeasurementPeriod(This,plPeriod)	\
    ( (This)->lpVtbl -> get_MeasurementPeriod(This,plPeriod) ) 

#define ITQueue_get_TotalCallsQueued(This,plCalls)	\
    ( (This)->lpVtbl -> get_TotalCallsQueued(This,plCalls) ) 

#define ITQueue_get_CurrentCallsQueued(This,plCalls)	\
    ( (This)->lpVtbl -> get_CurrentCallsQueued(This,plCalls) ) 

#define ITQueue_get_TotalCallsAbandoned(This,plCalls)	\
    ( (This)->lpVtbl -> get_TotalCallsAbandoned(This,plCalls) ) 

#define ITQueue_get_TotalCallsFlowedIn(This,plCalls)	\
    ( (This)->lpVtbl -> get_TotalCallsFlowedIn(This,plCalls) ) 

#define ITQueue_get_TotalCallsFlowedOut(This,plCalls)	\
    ( (This)->lpVtbl -> get_TotalCallsFlowedOut(This,plCalls) ) 

#define ITQueue_get_LongestEverWaitTime(This,plWaitTime)	\
    ( (This)->lpVtbl -> get_LongestEverWaitTime(This,plWaitTime) ) 

#define ITQueue_get_CurrentLongestWaitTime(This,plWaitTime)	\
    ( (This)->lpVtbl -> get_CurrentLongestWaitTime(This,plWaitTime) ) 

#define ITQueue_get_AverageWaitTime(This,plWaitTime)	\
    ( (This)->lpVtbl -> get_AverageWaitTime(This,plWaitTime) ) 

#define ITQueue_get_FinalDisposition(This,plCalls)	\
    ( (This)->lpVtbl -> get_FinalDisposition(This,plCalls) ) 

#define ITQueue_get_Name(This,ppName)	\
    ( (This)->lpVtbl -> get_Name(This,ppName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITQueue_INTERFACE_DEFINED__ */


#ifndef __ITAgentEvent_INTERFACE_DEFINED__
#define __ITAgentEvent_INTERFACE_DEFINED__

/* interface ITAgentEvent */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITAgentEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5AFC314A-4BCC-11d1-BF80-00805FC147D3")
    ITAgentEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Agent( 
            /* [retval][out] */ __RPC__deref_out_opt ITAgent **ppAgent) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Event( 
            /* [retval][out] */ __RPC__out AGENT_EVENT *pEvent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITAgentEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITAgentEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITAgentEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITAgentEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITAgentEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITAgentEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITAgentEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITAgentEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITAgentEvent, get_Agent)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Agent )( 
            __RPC__in ITAgentEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITAgent **ppAgent);
        
        DECLSPEC_XFGVIRT(ITAgentEvent, get_Event)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Event )( 
            __RPC__in ITAgentEvent * This,
            /* [retval][out] */ __RPC__out AGENT_EVENT *pEvent);
        
        END_INTERFACE
    } ITAgentEventVtbl;

    interface ITAgentEvent
    {
        CONST_VTBL struct ITAgentEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITAgentEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITAgentEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITAgentEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITAgentEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITAgentEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITAgentEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITAgentEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITAgentEvent_get_Agent(This,ppAgent)	\
    ( (This)->lpVtbl -> get_Agent(This,ppAgent) ) 

#define ITAgentEvent_get_Event(This,pEvent)	\
    ( (This)->lpVtbl -> get_Event(This,pEvent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITAgentEvent_INTERFACE_DEFINED__ */


#ifndef __ITAgentSessionEvent_INTERFACE_DEFINED__
#define __ITAgentSessionEvent_INTERFACE_DEFINED__

/* interface ITAgentSessionEvent */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITAgentSessionEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5AFC314B-4BCC-11d1-BF80-00805FC147D3")
    ITAgentSessionEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Session( 
            /* [retval][out] */ __RPC__deref_out_opt ITAgentSession **ppSession) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Event( 
            /* [retval][out] */ __RPC__out AGENT_SESSION_EVENT *pEvent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITAgentSessionEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITAgentSessionEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITAgentSessionEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITAgentSessionEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITAgentSessionEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITAgentSessionEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITAgentSessionEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITAgentSessionEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITAgentSessionEvent, get_Session)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Session )( 
            __RPC__in ITAgentSessionEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITAgentSession **ppSession);
        
        DECLSPEC_XFGVIRT(ITAgentSessionEvent, get_Event)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Event )( 
            __RPC__in ITAgentSessionEvent * This,
            /* [retval][out] */ __RPC__out AGENT_SESSION_EVENT *pEvent);
        
        END_INTERFACE
    } ITAgentSessionEventVtbl;

    interface ITAgentSessionEvent
    {
        CONST_VTBL struct ITAgentSessionEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITAgentSessionEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITAgentSessionEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITAgentSessionEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITAgentSessionEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITAgentSessionEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITAgentSessionEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITAgentSessionEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITAgentSessionEvent_get_Session(This,ppSession)	\
    ( (This)->lpVtbl -> get_Session(This,ppSession) ) 

#define ITAgentSessionEvent_get_Event(This,pEvent)	\
    ( (This)->lpVtbl -> get_Event(This,pEvent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITAgentSessionEvent_INTERFACE_DEFINED__ */


#ifndef __ITACDGroupEvent_INTERFACE_DEFINED__
#define __ITACDGroupEvent_INTERFACE_DEFINED__

/* interface ITACDGroupEvent */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITACDGroupEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("297F3032-BD11-11d1-A0A7-00805FC147D3")
    ITACDGroupEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Group( 
            /* [retval][out] */ __RPC__deref_out_opt ITACDGroup **ppGroup) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Event( 
            /* [retval][out] */ __RPC__out ACDGROUP_EVENT *pEvent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITACDGroupEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITACDGroupEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITACDGroupEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITACDGroupEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITACDGroupEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITACDGroupEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITACDGroupEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITACDGroupEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITACDGroupEvent, get_Group)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Group )( 
            __RPC__in ITACDGroupEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITACDGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(ITACDGroupEvent, get_Event)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Event )( 
            __RPC__in ITACDGroupEvent * This,
            /* [retval][out] */ __RPC__out ACDGROUP_EVENT *pEvent);
        
        END_INTERFACE
    } ITACDGroupEventVtbl;

    interface ITACDGroupEvent
    {
        CONST_VTBL struct ITACDGroupEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITACDGroupEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITACDGroupEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITACDGroupEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITACDGroupEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITACDGroupEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITACDGroupEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITACDGroupEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITACDGroupEvent_get_Group(This,ppGroup)	\
    ( (This)->lpVtbl -> get_Group(This,ppGroup) ) 

#define ITACDGroupEvent_get_Event(This,pEvent)	\
    ( (This)->lpVtbl -> get_Event(This,pEvent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITACDGroupEvent_INTERFACE_DEFINED__ */


#ifndef __ITQueueEvent_INTERFACE_DEFINED__
#define __ITQueueEvent_INTERFACE_DEFINED__

/* interface ITQueueEvent */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITQueueEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("297F3033-BD11-11d1-A0A7-00805FC147D3")
    ITQueueEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Queue( 
            /* [retval][out] */ __RPC__deref_out_opt ITQueue **ppQueue) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Event( 
            /* [retval][out] */ __RPC__out ACDQUEUE_EVENT *pEvent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITQueueEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITQueueEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITQueueEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITQueueEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITQueueEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITQueueEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITQueueEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITQueueEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITQueueEvent, get_Queue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Queue )( 
            __RPC__in ITQueueEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITQueue **ppQueue);
        
        DECLSPEC_XFGVIRT(ITQueueEvent, get_Event)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Event )( 
            __RPC__in ITQueueEvent * This,
            /* [retval][out] */ __RPC__out ACDQUEUE_EVENT *pEvent);
        
        END_INTERFACE
    } ITQueueEventVtbl;

    interface ITQueueEvent
    {
        CONST_VTBL struct ITQueueEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITQueueEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITQueueEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITQueueEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITQueueEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITQueueEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITQueueEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITQueueEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITQueueEvent_get_Queue(This,ppQueue)	\
    ( (This)->lpVtbl -> get_Queue(This,ppQueue) ) 

#define ITQueueEvent_get_Event(This,pEvent)	\
    ( (This)->lpVtbl -> get_Event(This,pEvent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITQueueEvent_INTERFACE_DEFINED__ */


#ifndef __ITAgentHandlerEvent_INTERFACE_DEFINED__
#define __ITAgentHandlerEvent_INTERFACE_DEFINED__

/* interface ITAgentHandlerEvent */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITAgentHandlerEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("297F3034-BD11-11d1-A0A7-00805FC147D3")
    ITAgentHandlerEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AgentHandler( 
            /* [retval][out] */ __RPC__deref_out_opt ITAgentHandler **ppAgentHandler) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Event( 
            /* [retval][out] */ __RPC__out AGENTHANDLER_EVENT *pEvent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITAgentHandlerEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITAgentHandlerEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITAgentHandlerEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITAgentHandlerEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITAgentHandlerEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITAgentHandlerEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITAgentHandlerEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITAgentHandlerEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITAgentHandlerEvent, get_AgentHandler)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AgentHandler )( 
            __RPC__in ITAgentHandlerEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITAgentHandler **ppAgentHandler);
        
        DECLSPEC_XFGVIRT(ITAgentHandlerEvent, get_Event)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Event )( 
            __RPC__in ITAgentHandlerEvent * This,
            /* [retval][out] */ __RPC__out AGENTHANDLER_EVENT *pEvent);
        
        END_INTERFACE
    } ITAgentHandlerEventVtbl;

    interface ITAgentHandlerEvent
    {
        CONST_VTBL struct ITAgentHandlerEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITAgentHandlerEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITAgentHandlerEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITAgentHandlerEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITAgentHandlerEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITAgentHandlerEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITAgentHandlerEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITAgentHandlerEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITAgentHandlerEvent_get_AgentHandler(This,ppAgentHandler)	\
    ( (This)->lpVtbl -> get_AgentHandler(This,ppAgentHandler) ) 

#define ITAgentHandlerEvent_get_Event(This,pEvent)	\
    ( (This)->lpVtbl -> get_Event(This,pEvent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITAgentHandlerEvent_INTERFACE_DEFINED__ */


#ifndef __ITTAPICallCenter_INTERFACE_DEFINED__
#define __ITTAPICallCenter_INTERFACE_DEFINED__

/* interface ITTAPICallCenter */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITTAPICallCenter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5AFC3154-4BCC-11d1-BF80-00805FC147D3")
    ITTAPICallCenter : public IDispatch
    {
    public:
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateAgentHandlers( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumAgentHandler **ppEnumHandler) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AgentHandlers( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITTAPICallCenterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITTAPICallCenter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITTAPICallCenter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITTAPICallCenter * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITTAPICallCenter * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITTAPICallCenter * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITTAPICallCenter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITTAPICallCenter * This,
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
        
        DECLSPEC_XFGVIRT(ITTAPICallCenter, EnumerateAgentHandlers)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateAgentHandlers )( 
            __RPC__in ITTAPICallCenter * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumAgentHandler **ppEnumHandler);
        
        DECLSPEC_XFGVIRT(ITTAPICallCenter, get_AgentHandlers)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AgentHandlers )( 
            __RPC__in ITTAPICallCenter * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        END_INTERFACE
    } ITTAPICallCenterVtbl;

    interface ITTAPICallCenter
    {
        CONST_VTBL struct ITTAPICallCenterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITTAPICallCenter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITTAPICallCenter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITTAPICallCenter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITTAPICallCenter_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITTAPICallCenter_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITTAPICallCenter_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITTAPICallCenter_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITTAPICallCenter_EnumerateAgentHandlers(This,ppEnumHandler)	\
    ( (This)->lpVtbl -> EnumerateAgentHandlers(This,ppEnumHandler) ) 

#define ITTAPICallCenter_get_AgentHandlers(This,pVariant)	\
    ( (This)->lpVtbl -> get_AgentHandlers(This,pVariant) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITTAPICallCenter_INTERFACE_DEFINED__ */


#ifndef __ITAgentHandler_INTERFACE_DEFINED__
#define __ITAgentHandler_INTERFACE_DEFINED__

/* interface ITAgentHandler */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITAgentHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("587E8C22-9802-11d1-A0A4-00805FC147D3")
    ITAgentHandler : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateAgent( 
            /* [retval][out] */ __RPC__deref_out_opt ITAgent **ppAgent) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateAgentWithID( 
            /* [in] */ __RPC__in BSTR pID,
            /* [in] */ __RPC__in BSTR pPIN,
            /* [retval][out] */ __RPC__deref_out_opt ITAgent **ppAgent) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateACDGroups( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumACDGroup **ppEnumACDGroup) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateUsableAddresses( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumAddress **ppEnumAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ACDGroups( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UsableAddresses( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITAgentHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITAgentHandler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITAgentHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITAgentHandler * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITAgentHandler * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITAgentHandler * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITAgentHandler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITAgentHandler * This,
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
        
        DECLSPEC_XFGVIRT(ITAgentHandler, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ITAgentHandler * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppName);
        
        DECLSPEC_XFGVIRT(ITAgentHandler, CreateAgent)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateAgent )( 
            __RPC__in ITAgentHandler * This,
            /* [retval][out] */ __RPC__deref_out_opt ITAgent **ppAgent);
        
        DECLSPEC_XFGVIRT(ITAgentHandler, CreateAgentWithID)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateAgentWithID )( 
            __RPC__in ITAgentHandler * This,
            /* [in] */ __RPC__in BSTR pID,
            /* [in] */ __RPC__in BSTR pPIN,
            /* [retval][out] */ __RPC__deref_out_opt ITAgent **ppAgent);
        
        DECLSPEC_XFGVIRT(ITAgentHandler, EnumerateACDGroups)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateACDGroups )( 
            __RPC__in ITAgentHandler * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumACDGroup **ppEnumACDGroup);
        
        DECLSPEC_XFGVIRT(ITAgentHandler, EnumerateUsableAddresses)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateUsableAddresses )( 
            __RPC__in ITAgentHandler * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumAddress **ppEnumAddress);
        
        DECLSPEC_XFGVIRT(ITAgentHandler, get_ACDGroups)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ACDGroups )( 
            __RPC__in ITAgentHandler * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITAgentHandler, get_UsableAddresses)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UsableAddresses )( 
            __RPC__in ITAgentHandler * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        END_INTERFACE
    } ITAgentHandlerVtbl;

    interface ITAgentHandler
    {
        CONST_VTBL struct ITAgentHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITAgentHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITAgentHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITAgentHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITAgentHandler_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITAgentHandler_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITAgentHandler_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITAgentHandler_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITAgentHandler_get_Name(This,ppName)	\
    ( (This)->lpVtbl -> get_Name(This,ppName) ) 

#define ITAgentHandler_CreateAgent(This,ppAgent)	\
    ( (This)->lpVtbl -> CreateAgent(This,ppAgent) ) 

#define ITAgentHandler_CreateAgentWithID(This,pID,pPIN,ppAgent)	\
    ( (This)->lpVtbl -> CreateAgentWithID(This,pID,pPIN,ppAgent) ) 

#define ITAgentHandler_EnumerateACDGroups(This,ppEnumACDGroup)	\
    ( (This)->lpVtbl -> EnumerateACDGroups(This,ppEnumACDGroup) ) 

#define ITAgentHandler_EnumerateUsableAddresses(This,ppEnumAddress)	\
    ( (This)->lpVtbl -> EnumerateUsableAddresses(This,ppEnumAddress) ) 

#define ITAgentHandler_get_ACDGroups(This,pVariant)	\
    ( (This)->lpVtbl -> get_ACDGroups(This,pVariant) ) 

#define ITAgentHandler_get_UsableAddresses(This,pVariant)	\
    ( (This)->lpVtbl -> get_UsableAddresses(This,pVariant) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITAgentHandler_INTERFACE_DEFINED__ */


#ifndef __IEnumAgent_INTERFACE_DEFINED__
#define __IEnumAgent_INTERFACE_DEFINED__

/* interface IEnumAgent */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumAgent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5AFC314D-4BCC-11d1-BF80-00805FC147D3")
    IEnumAgent : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITAgent **ppElements,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumAgent **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumAgentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumAgent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumAgent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumAgent * This);
        
        DECLSPEC_XFGVIRT(IEnumAgent, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumAgent * This,
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITAgent **ppElements,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumAgent, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumAgent * This);
        
        DECLSPEC_XFGVIRT(IEnumAgent, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumAgent * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumAgent, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumAgent * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumAgent **ppEnum);
        
        END_INTERFACE
    } IEnumAgentVtbl;

    interface IEnumAgent
    {
        CONST_VTBL struct IEnumAgentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumAgent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumAgent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumAgent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumAgent_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IEnumAgent_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumAgent_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumAgent_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumAgent_INTERFACE_DEFINED__ */


#ifndef __IEnumAgentSession_INTERFACE_DEFINED__
#define __IEnumAgentSession_INTERFACE_DEFINED__

/* interface IEnumAgentSession */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumAgentSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5AFC314E-4BCC-11d1-BF80-00805FC147D3")
    IEnumAgentSession : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITAgentSession **ppElements,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumAgentSession **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumAgentSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumAgentSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumAgentSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumAgentSession * This);
        
        DECLSPEC_XFGVIRT(IEnumAgentSession, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumAgentSession * This,
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITAgentSession **ppElements,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumAgentSession, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumAgentSession * This);
        
        DECLSPEC_XFGVIRT(IEnumAgentSession, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumAgentSession * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumAgentSession, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumAgentSession * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumAgentSession **ppEnum);
        
        END_INTERFACE
    } IEnumAgentSessionVtbl;

    interface IEnumAgentSession
    {
        CONST_VTBL struct IEnumAgentSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumAgentSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumAgentSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumAgentSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumAgentSession_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IEnumAgentSession_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumAgentSession_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumAgentSession_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumAgentSession_INTERFACE_DEFINED__ */


#ifndef __IEnumQueue_INTERFACE_DEFINED__
#define __IEnumQueue_INTERFACE_DEFINED__

/* interface IEnumQueue */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumQueue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5AFC3158-4BCC-11d1-BF80-00805FC147D3")
    IEnumQueue : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITQueue **ppElements,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumQueue **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumQueueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumQueue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumQueue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumQueue * This);
        
        DECLSPEC_XFGVIRT(IEnumQueue, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumQueue * This,
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITQueue **ppElements,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumQueue, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumQueue * This);
        
        DECLSPEC_XFGVIRT(IEnumQueue, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumQueue * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumQueue, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumQueue **ppEnum);
        
        END_INTERFACE
    } IEnumQueueVtbl;

    interface IEnumQueue
    {
        CONST_VTBL struct IEnumQueueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumQueue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumQueue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumQueue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumQueue_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IEnumQueue_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumQueue_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumQueue_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumQueue_INTERFACE_DEFINED__ */


#ifndef __IEnumACDGroup_INTERFACE_DEFINED__
#define __IEnumACDGroup_INTERFACE_DEFINED__

/* interface IEnumACDGroup */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumACDGroup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5AFC3157-4BCC-11d1-BF80-00805FC147D3")
    IEnumACDGroup : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITACDGroup **ppElements,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumACDGroup **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumACDGroupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumACDGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumACDGroup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumACDGroup * This);
        
        DECLSPEC_XFGVIRT(IEnumACDGroup, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumACDGroup * This,
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITACDGroup **ppElements,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumACDGroup, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumACDGroup * This);
        
        DECLSPEC_XFGVIRT(IEnumACDGroup, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumACDGroup * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumACDGroup, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumACDGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumACDGroup **ppEnum);
        
        END_INTERFACE
    } IEnumACDGroupVtbl;

    interface IEnumACDGroup
    {
        CONST_VTBL struct IEnumACDGroupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumACDGroup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumACDGroup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumACDGroup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumACDGroup_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IEnumACDGroup_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumACDGroup_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumACDGroup_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumACDGroup_INTERFACE_DEFINED__ */


#ifndef __IEnumAgentHandler_INTERFACE_DEFINED__
#define __IEnumAgentHandler_INTERFACE_DEFINED__

/* interface IEnumAgentHandler */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumAgentHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("587E8C28-9802-11d1-A0A4-00805FC147D3")
    IEnumAgentHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITAgentHandler **ppElements,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumAgentHandler **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumAgentHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumAgentHandler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumAgentHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumAgentHandler * This);
        
        DECLSPEC_XFGVIRT(IEnumAgentHandler, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumAgentHandler * This,
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITAgentHandler **ppElements,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumAgentHandler, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumAgentHandler * This);
        
        DECLSPEC_XFGVIRT(IEnumAgentHandler, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumAgentHandler * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumAgentHandler, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumAgentHandler * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumAgentHandler **ppEnum);
        
        END_INTERFACE
    } IEnumAgentHandlerVtbl;

    interface IEnumAgentHandler
    {
        CONST_VTBL struct IEnumAgentHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumAgentHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumAgentHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumAgentHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumAgentHandler_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IEnumAgentHandler_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumAgentHandler_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumAgentHandler_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumAgentHandler_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_tapi3cc_0000_0016 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_tapi3cc_0000_0016_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tapi3cc_0000_0016_v0_0_s_ifspec;

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


