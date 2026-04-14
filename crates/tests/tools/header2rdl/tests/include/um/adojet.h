

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


#ifndef __adojet_h__
#define __adojet_h__

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

#ifndef __IReplica_FWD_DEFINED__
#define __IReplica_FWD_DEFINED__
typedef interface IReplica IReplica;

#endif 	/* __IReplica_FWD_DEFINED__ */


#ifndef __Filter_FWD_DEFINED__
#define __Filter_FWD_DEFINED__
typedef interface Filter Filter;

#endif 	/* __Filter_FWD_DEFINED__ */


#ifndef __Filters_FWD_DEFINED__
#define __Filters_FWD_DEFINED__
typedef interface Filters Filters;

#endif 	/* __Filters_FWD_DEFINED__ */


#ifndef __IJetEngine_FWD_DEFINED__
#define __IJetEngine_FWD_DEFINED__
typedef interface IJetEngine IJetEngine;

#endif 	/* __IJetEngine_FWD_DEFINED__ */


#ifndef __Replica_FWD_DEFINED__
#define __Replica_FWD_DEFINED__

#ifdef __cplusplus
typedef class Replica Replica;
#else
typedef struct Replica Replica;
#endif /* __cplusplus */

#endif 	/* __Replica_FWD_DEFINED__ */


#ifndef __JetEngine_FWD_DEFINED__
#define __JetEngine_FWD_DEFINED__

#ifdef __cplusplus
typedef class JetEngine JetEngine;
#else
typedef struct JetEngine JetEngine;
#endif /* __cplusplus */

#endif 	/* __JetEngine_FWD_DEFINED__ */


#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_adojet_0000_0000 */
/* [local] */ 







#define TARGET_IS_NT40_OR_LATER   1


extern RPC_IF_HANDLE __MIDL_itf_adojet_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_adojet_0000_0000_v0_0_s_ifspec;


#ifndef __JRO_LIBRARY_DEFINED__
#define __JRO_LIBRARY_DEFINED__

/* library JRO */
/* [helpstring][version][uuid] */ 

typedef /* [uuid] */  DECLSPEC_UUID("D2D139DF-B6CA-11d1-9F31-00C04FC29D52") 
enum ReplicaTypeEnum
    {
        jrRepTypeNotReplicable	= 0,
        jrRepTypeDesignMaster	= 0x1,
        jrRepTypeFull	= 0x2,
        jrRepTypePartial	= 0x3
    } 	ReplicaTypeEnum;

typedef /* [uuid] */  DECLSPEC_UUID("6877D21A-B6CE-11d1-9F31-00C04FC29D52") 
enum VisibilityEnum
    {
        jrRepVisibilityGlobal	= 0x1,
        jrRepVisibilityLocal	= 0x2,
        jrRepVisibilityAnon	= 0x4
    } 	VisibilityEnum;

typedef /* [uuid] */  DECLSPEC_UUID("B42FBFF6-B6CF-11d1-9F31-00C04FC29D52") 
enum UpdatabilityEnum
    {
        jrRepUpdFull	= 0,
        jrRepUpdReadOnly	= 0x2
    } 	UpdatabilityEnum;

typedef /* [uuid] */  DECLSPEC_UUID("60C05416-B6D0-11d1-9F31-00C04FC29D52") 
enum SyncTypeEnum
    {
        jrSyncTypeExport	= 0x1,
        jrSyncTypeImport	= 0x2,
        jrSyncTypeImpExp	= 0x3
    } 	SyncTypeEnum;

typedef /* [uuid] */  DECLSPEC_UUID("5EBA3970-061E-11d2-BB77-00C04FAE22DA") 
enum SyncModeEnum
    {
        jrSyncModeIndirect	= 0x1,
        jrSyncModeDirect	= 0x2,
        jrSyncModeInternet	= 0x3
    } 	SyncModeEnum;

typedef /* [uuid] */  DECLSPEC_UUID("72769F94-BF78-11d1-AC4D-00C04FC29F8F") 
enum FilterTypeEnum
    {
        jrFilterTypeTable	= 0x1,
        jrFilterTypeRelationship	= 0x2
    } 	FilterTypeEnum;


EXTERN_C const IID LIBID_JRO;

#ifndef __IReplica_INTERFACE_DEFINED__
#define __IReplica_INTERFACE_DEFINED__

/* interface IReplica */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IReplica;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D2D139E0-B6CA-11d1-9F31-00C04FC29D52")
    IReplica : public IDispatch
    {
    public:
        virtual /* [helpcontext][propputref] */ HRESULT STDMETHODCALLTYPE putref_ActiveConnection( 
            /* [in] */ __RPC__in_opt IDispatch *pconn) = 0;
        
        virtual /* [helpcontext][propput] */ HRESULT STDMETHODCALLTYPE put_ActiveConnection( 
            /* [in] */ VARIANT vConn) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_ActiveConnection( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppconn) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_ConflictFunction( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propput] */ HRESULT STDMETHODCALLTYPE put_ConflictFunction( 
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_ConflictTables( 
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ _Recordset **pprset) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_DesignMasterId( 
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [helpcontext][propput] */ HRESULT STDMETHODCALLTYPE put_DesignMasterId( 
            /* [in] */ VARIANT var) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Priority( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_ReplicaId( 
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_ReplicaType( 
            /* [retval][out] */ __RPC__out ReplicaTypeEnum *pl) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_RetentionPeriod( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
        virtual /* [helpcontext][propput] */ HRESULT STDMETHODCALLTYPE put_RetentionPeriod( 
            /* [in] */ long l) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Visibility( 
            /* [retval][out] */ __RPC__out VisibilityEnum *pl) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE CreateReplica( 
            /* [in] */ __RPC__in BSTR replicaName,
            /* [in] */ __RPC__in BSTR description,
            /* [defaultvalue][in] */ ReplicaTypeEnum replicaType = jrRepTypeFull,
            /* [defaultvalue][in] */ VisibilityEnum visibility = jrRepVisibilityGlobal,
            /* [defaultvalue][in] */ long priority = -1,
            /* [defaultvalue][in] */ UpdatabilityEnum updatability = jrRepUpdFull) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE GetObjectReplicability( 
            /* [in] */ __RPC__in BSTR objectName,
            /* [in] */ __RPC__in BSTR objectType,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *replicability) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE SetObjectReplicability( 
            /* [in] */ __RPC__in BSTR objectName,
            /* [in] */ __RPC__in BSTR objectType,
            /* [in] */ VARIANT_BOOL replicability) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE MakeReplicable( 
            /* [defaultvalue][in] */ __RPC__in BSTR connectionString = (BSTR)L"",
            /* [defaultvalue][in] */ VARIANT_BOOL columnTracking = -1) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE PopulatePartial( 
            /* [in] */ __RPC__in BSTR FullReplica) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Synchronize( 
            /* [in] */ __RPC__in BSTR target,
            /* [defaultvalue][in] */ SyncTypeEnum syncType = jrSyncTypeImpExp,
            /* [defaultvalue][in] */ SyncModeEnum syncMode = jrSyncModeIndirect) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Filters( 
            /* [retval][out] */ __RPC__deref_out_opt Filters **ppFilters) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IReplicaVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IReplica * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IReplica * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IReplica * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IReplica * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IReplica * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IReplica * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IReplica * This,
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
        
        DECLSPEC_XFGVIRT(IReplica, putref_ActiveConnection)
        /* [helpcontext][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_ActiveConnection )( 
            __RPC__in IReplica * This,
            /* [in] */ __RPC__in_opt IDispatch *pconn);
        
        DECLSPEC_XFGVIRT(IReplica, put_ActiveConnection)
        /* [helpcontext][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ActiveConnection )( 
            __RPC__in IReplica * This,
            /* [in] */ VARIANT vConn);
        
        DECLSPEC_XFGVIRT(IReplica, get_ActiveConnection)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveConnection )( 
            __RPC__in IReplica * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppconn);
        
        DECLSPEC_XFGVIRT(IReplica, get_ConflictFunction)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConflictFunction )( 
            __RPC__in IReplica * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(IReplica, put_ConflictFunction)
        /* [helpcontext][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ConflictFunction )( 
            __RPC__in IReplica * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        DECLSPEC_XFGVIRT(IReplica, get_ConflictTables)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConflictTables )( 
            __RPC__in IReplica * This,
            /* [retval][out] */ __RPC__deref_out_opt /* external definition not present */ _Recordset **pprset);
        
        DECLSPEC_XFGVIRT(IReplica, get_DesignMasterId)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DesignMasterId )( 
            __RPC__in IReplica * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        DECLSPEC_XFGVIRT(IReplica, put_DesignMasterId)
        /* [helpcontext][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DesignMasterId )( 
            __RPC__in IReplica * This,
            /* [in] */ VARIANT var);
        
        DECLSPEC_XFGVIRT(IReplica, get_Priority)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Priority )( 
            __RPC__in IReplica * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(IReplica, get_ReplicaId)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReplicaId )( 
            __RPC__in IReplica * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        DECLSPEC_XFGVIRT(IReplica, get_ReplicaType)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReplicaType )( 
            __RPC__in IReplica * This,
            /* [retval][out] */ __RPC__out ReplicaTypeEnum *pl);
        
        DECLSPEC_XFGVIRT(IReplica, get_RetentionPeriod)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RetentionPeriod )( 
            __RPC__in IReplica * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(IReplica, put_RetentionPeriod)
        /* [helpcontext][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RetentionPeriod )( 
            __RPC__in IReplica * This,
            /* [in] */ long l);
        
        DECLSPEC_XFGVIRT(IReplica, get_Visibility)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Visibility )( 
            __RPC__in IReplica * This,
            /* [retval][out] */ __RPC__out VisibilityEnum *pl);
        
        DECLSPEC_XFGVIRT(IReplica, CreateReplica)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *CreateReplica )( 
            __RPC__in IReplica * This,
            /* [in] */ __RPC__in BSTR replicaName,
            /* [in] */ __RPC__in BSTR description,
            /* [defaultvalue][in] */ ReplicaTypeEnum replicaType,
            /* [defaultvalue][in] */ VisibilityEnum visibility,
            /* [defaultvalue][in] */ long priority,
            /* [defaultvalue][in] */ UpdatabilityEnum updatability);
        
        DECLSPEC_XFGVIRT(IReplica, GetObjectReplicability)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *GetObjectReplicability )( 
            __RPC__in IReplica * This,
            /* [in] */ __RPC__in BSTR objectName,
            /* [in] */ __RPC__in BSTR objectType,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *replicability);
        
        DECLSPEC_XFGVIRT(IReplica, SetObjectReplicability)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *SetObjectReplicability )( 
            __RPC__in IReplica * This,
            /* [in] */ __RPC__in BSTR objectName,
            /* [in] */ __RPC__in BSTR objectType,
            /* [in] */ VARIANT_BOOL replicability);
        
        DECLSPEC_XFGVIRT(IReplica, MakeReplicable)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *MakeReplicable )( 
            __RPC__in IReplica * This,
            /* [defaultvalue][in] */ __RPC__in BSTR connectionString,
            /* [defaultvalue][in] */ VARIANT_BOOL columnTracking);
        
        DECLSPEC_XFGVIRT(IReplica, PopulatePartial)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *PopulatePartial )( 
            __RPC__in IReplica * This,
            /* [in] */ __RPC__in BSTR FullReplica);
        
        DECLSPEC_XFGVIRT(IReplica, Synchronize)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Synchronize )( 
            __RPC__in IReplica * This,
            /* [in] */ __RPC__in BSTR target,
            /* [defaultvalue][in] */ SyncTypeEnum syncType,
            /* [defaultvalue][in] */ SyncModeEnum syncMode);
        
        DECLSPEC_XFGVIRT(IReplica, get_Filters)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Filters )( 
            __RPC__in IReplica * This,
            /* [retval][out] */ __RPC__deref_out_opt Filters **ppFilters);
        
        END_INTERFACE
    } IReplicaVtbl;

    interface IReplica
    {
        CONST_VTBL struct IReplicaVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IReplica_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IReplica_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IReplica_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IReplica_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IReplica_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IReplica_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IReplica_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IReplica_putref_ActiveConnection(This,pconn)	\
    ( (This)->lpVtbl -> putref_ActiveConnection(This,pconn) ) 

#define IReplica_put_ActiveConnection(This,vConn)	\
    ( (This)->lpVtbl -> put_ActiveConnection(This,vConn) ) 

#define IReplica_get_ActiveConnection(This,ppconn)	\
    ( (This)->lpVtbl -> get_ActiveConnection(This,ppconn) ) 

#define IReplica_get_ConflictFunction(This,pbstr)	\
    ( (This)->lpVtbl -> get_ConflictFunction(This,pbstr) ) 

#define IReplica_put_ConflictFunction(This,bstr)	\
    ( (This)->lpVtbl -> put_ConflictFunction(This,bstr) ) 

#define IReplica_get_ConflictTables(This,pprset)	\
    ( (This)->lpVtbl -> get_ConflictTables(This,pprset) ) 

#define IReplica_get_DesignMasterId(This,pvar)	\
    ( (This)->lpVtbl -> get_DesignMasterId(This,pvar) ) 

#define IReplica_put_DesignMasterId(This,var)	\
    ( (This)->lpVtbl -> put_DesignMasterId(This,var) ) 

#define IReplica_get_Priority(This,pl)	\
    ( (This)->lpVtbl -> get_Priority(This,pl) ) 

#define IReplica_get_ReplicaId(This,pvar)	\
    ( (This)->lpVtbl -> get_ReplicaId(This,pvar) ) 

#define IReplica_get_ReplicaType(This,pl)	\
    ( (This)->lpVtbl -> get_ReplicaType(This,pl) ) 

#define IReplica_get_RetentionPeriod(This,pl)	\
    ( (This)->lpVtbl -> get_RetentionPeriod(This,pl) ) 

#define IReplica_put_RetentionPeriod(This,l)	\
    ( (This)->lpVtbl -> put_RetentionPeriod(This,l) ) 

#define IReplica_get_Visibility(This,pl)	\
    ( (This)->lpVtbl -> get_Visibility(This,pl) ) 

#define IReplica_CreateReplica(This,replicaName,description,replicaType,visibility,priority,updatability)	\
    ( (This)->lpVtbl -> CreateReplica(This,replicaName,description,replicaType,visibility,priority,updatability) ) 

#define IReplica_GetObjectReplicability(This,objectName,objectType,replicability)	\
    ( (This)->lpVtbl -> GetObjectReplicability(This,objectName,objectType,replicability) ) 

#define IReplica_SetObjectReplicability(This,objectName,objectType,replicability)	\
    ( (This)->lpVtbl -> SetObjectReplicability(This,objectName,objectType,replicability) ) 

#define IReplica_MakeReplicable(This,connectionString,columnTracking)	\
    ( (This)->lpVtbl -> MakeReplicable(This,connectionString,columnTracking) ) 

#define IReplica_PopulatePartial(This,FullReplica)	\
    ( (This)->lpVtbl -> PopulatePartial(This,FullReplica) ) 

#define IReplica_Synchronize(This,target,syncType,syncMode)	\
    ( (This)->lpVtbl -> Synchronize(This,target,syncType,syncMode) ) 

#define IReplica_get_Filters(This,ppFilters)	\
    ( (This)->lpVtbl -> get_Filters(This,ppFilters) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IReplica_INTERFACE_DEFINED__ */


#ifndef __Filter_INTERFACE_DEFINED__
#define __Filter_INTERFACE_DEFINED__

/* interface Filter */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_Filter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D2D139E1-B6CA-11d1-9F31-00C04FC29D52")
    Filter : public IDispatch
    {
    public:
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_TableName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_FilterType( 
            /* [retval][out] */ __RPC__out FilterTypeEnum *ptype) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_FilterCriteria( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct FilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Filter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Filter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Filter * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Filter * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Filter * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Filter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Filter * This,
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
        
        DECLSPEC_XFGVIRT(Filter, get_TableName)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TableName )( 
            __RPC__in Filter * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        DECLSPEC_XFGVIRT(Filter, get_FilterType)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FilterType )( 
            __RPC__in Filter * This,
            /* [retval][out] */ __RPC__out FilterTypeEnum *ptype);
        
        DECLSPEC_XFGVIRT(Filter, get_FilterCriteria)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FilterCriteria )( 
            __RPC__in Filter * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        END_INTERFACE
    } FilterVtbl;

    interface Filter
    {
        CONST_VTBL struct FilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Filter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Filter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Filter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Filter_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Filter_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Filter_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Filter_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Filter_get_TableName(This,pbstr)	\
    ( (This)->lpVtbl -> get_TableName(This,pbstr) ) 

#define Filter_get_FilterType(This,ptype)	\
    ( (This)->lpVtbl -> get_FilterType(This,ptype) ) 

#define Filter_get_FilterCriteria(This,pbstr)	\
    ( (This)->lpVtbl -> get_FilterCriteria(This,pbstr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Filter_INTERFACE_DEFINED__ */


#ifndef __Filters_INTERFACE_DEFINED__
#define __Filters_INTERFACE_DEFINED__

/* interface Filters */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_Filters;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D2D139E2-B6CA-11d1-9F31-00C04FC29D52")
    Filters : public IDispatch
    {
    public:
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [id][restricted] */ HRESULT STDMETHODCALLTYPE _NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject) = 0;
        
        virtual /* [helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *c) = 0;
        
        virtual /* [id][helpcontext][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt Filter **ppvObject) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in BSTR tableName,
            /* [in] */ FilterTypeEnum filterType,
            /* [in] */ __RPC__in BSTR filterCriteria) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ VARIANT Index) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct FiltersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Filters * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Filters * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Filters * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Filters * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Filters * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Filters * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Filters * This,
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
        
        DECLSPEC_XFGVIRT(Filters, Refresh)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in Filters * This);
        
        DECLSPEC_XFGVIRT(Filters, _NewEnum)
        /* [id][restricted] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in Filters * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(Filters, get_Count)
        /* [helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in Filters * This,
            /* [retval][out] */ __RPC__out long *c);
        
        DECLSPEC_XFGVIRT(Filters, get_Item)
        /* [id][helpcontext][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in Filters * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt Filter **ppvObject);
        
        DECLSPEC_XFGVIRT(Filters, Append)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in Filters * This,
            /* [in] */ __RPC__in BSTR tableName,
            /* [in] */ FilterTypeEnum filterType,
            /* [in] */ __RPC__in BSTR filterCriteria);
        
        DECLSPEC_XFGVIRT(Filters, Delete)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in Filters * This,
            /* [in] */ VARIANT Index);
        
        END_INTERFACE
    } FiltersVtbl;

    interface Filters
    {
        CONST_VTBL struct FiltersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Filters_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Filters_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Filters_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Filters_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Filters_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Filters_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Filters_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Filters_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define Filters__NewEnum(This,ppvObject)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppvObject) ) 

#define Filters_get_Count(This,c)	\
    ( (This)->lpVtbl -> get_Count(This,c) ) 

#define Filters_get_Item(This,Index,ppvObject)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppvObject) ) 

#define Filters_Append(This,tableName,filterType,filterCriteria)	\
    ( (This)->lpVtbl -> Append(This,tableName,filterType,filterCriteria) ) 

#define Filters_Delete(This,Index)	\
    ( (This)->lpVtbl -> Delete(This,Index) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Filters_INTERFACE_DEFINED__ */


#ifndef __IJetEngine_INTERFACE_DEFINED__
#define __IJetEngine_INTERFACE_DEFINED__

/* interface IJetEngine */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IJetEngine;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9F63D980-FF25-11D1-BB6F-00C04FAE22DA")
    IJetEngine : public IDispatch
    {
    public:
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE CompactDatabase( 
            /* [in] */ __RPC__in BSTR SourceConnection,
            /* [in] */ __RPC__in BSTR Destconnection) = 0;
        
        virtual /* [helpcontext] */ HRESULT STDMETHODCALLTYPE RefreshCache( 
            /* [in] */ __RPC__in /* external definition not present */ _Connection *Connection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IJetEngineVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IJetEngine * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IJetEngine * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IJetEngine * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IJetEngine * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IJetEngine * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IJetEngine * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IJetEngine * This,
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
        
        DECLSPEC_XFGVIRT(IJetEngine, CompactDatabase)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *CompactDatabase )( 
            __RPC__in IJetEngine * This,
            /* [in] */ __RPC__in BSTR SourceConnection,
            /* [in] */ __RPC__in BSTR Destconnection);
        
        DECLSPEC_XFGVIRT(IJetEngine, RefreshCache)
        /* [helpcontext] */ HRESULT ( STDMETHODCALLTYPE *RefreshCache )( 
            __RPC__in IJetEngine * This,
            /* [in] */ __RPC__in /* external definition not present */ _Connection *Connection);
        
        END_INTERFACE
    } IJetEngineVtbl;

    interface IJetEngine
    {
        CONST_VTBL struct IJetEngineVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IJetEngine_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IJetEngine_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IJetEngine_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IJetEngine_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IJetEngine_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IJetEngine_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IJetEngine_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IJetEngine_CompactDatabase(This,SourceConnection,Destconnection)	\
    ( (This)->lpVtbl -> CompactDatabase(This,SourceConnection,Destconnection) ) 

#define IJetEngine_RefreshCache(This,Connection)	\
    ( (This)->lpVtbl -> RefreshCache(This,Connection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IJetEngine_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_Replica;

#ifdef __cplusplus

class DECLSPEC_UUID("D2D139E3-B6CA-11d1-9F31-00C04FC29D52")
Replica;
#endif

EXTERN_C const CLSID CLSID_JetEngine;

#ifdef __cplusplus

class DECLSPEC_UUID("DE88C160-FF2C-11D1-BB6F-00C04FAE22DA")
JetEngine;
#endif
#endif /* __JRO_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


