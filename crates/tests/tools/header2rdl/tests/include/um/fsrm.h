

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

#ifndef __fsrm_h__
#define __fsrm_h__

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

#ifndef __IFsrmObject_FWD_DEFINED__
#define __IFsrmObject_FWD_DEFINED__
typedef interface IFsrmObject IFsrmObject;

#endif 	/* __IFsrmObject_FWD_DEFINED__ */


#ifndef __IFsrmCollection_FWD_DEFINED__
#define __IFsrmCollection_FWD_DEFINED__
typedef interface IFsrmCollection IFsrmCollection;

#endif 	/* __IFsrmCollection_FWD_DEFINED__ */


#ifndef __IFsrmMutableCollection_FWD_DEFINED__
#define __IFsrmMutableCollection_FWD_DEFINED__
typedef interface IFsrmMutableCollection IFsrmMutableCollection;

#endif 	/* __IFsrmMutableCollection_FWD_DEFINED__ */


#ifndef __IFsrmCommittableCollection_FWD_DEFINED__
#define __IFsrmCommittableCollection_FWD_DEFINED__
typedef interface IFsrmCommittableCollection IFsrmCommittableCollection;

#endif 	/* __IFsrmCommittableCollection_FWD_DEFINED__ */


#ifndef __IFsrmAction_FWD_DEFINED__
#define __IFsrmAction_FWD_DEFINED__
typedef interface IFsrmAction IFsrmAction;

#endif 	/* __IFsrmAction_FWD_DEFINED__ */


#ifndef __IFsrmActionEmail_FWD_DEFINED__
#define __IFsrmActionEmail_FWD_DEFINED__
typedef interface IFsrmActionEmail IFsrmActionEmail;

#endif 	/* __IFsrmActionEmail_FWD_DEFINED__ */


#ifndef __IFsrmActionEmail2_FWD_DEFINED__
#define __IFsrmActionEmail2_FWD_DEFINED__
typedef interface IFsrmActionEmail2 IFsrmActionEmail2;

#endif 	/* __IFsrmActionEmail2_FWD_DEFINED__ */


#ifndef __IFsrmActionReport_FWD_DEFINED__
#define __IFsrmActionReport_FWD_DEFINED__
typedef interface IFsrmActionReport IFsrmActionReport;

#endif 	/* __IFsrmActionReport_FWD_DEFINED__ */


#ifndef __IFsrmActionEventLog_FWD_DEFINED__
#define __IFsrmActionEventLog_FWD_DEFINED__
typedef interface IFsrmActionEventLog IFsrmActionEventLog;

#endif 	/* __IFsrmActionEventLog_FWD_DEFINED__ */


#ifndef __IFsrmActionCommand_FWD_DEFINED__
#define __IFsrmActionCommand_FWD_DEFINED__
typedef interface IFsrmActionCommand IFsrmActionCommand;

#endif 	/* __IFsrmActionCommand_FWD_DEFINED__ */


#ifndef __IFsrmSetting_FWD_DEFINED__
#define __IFsrmSetting_FWD_DEFINED__
typedef interface IFsrmSetting IFsrmSetting;

#endif 	/* __IFsrmSetting_FWD_DEFINED__ */


#ifndef __IFsrmPathMapper_FWD_DEFINED__
#define __IFsrmPathMapper_FWD_DEFINED__
typedef interface IFsrmPathMapper IFsrmPathMapper;

#endif 	/* __IFsrmPathMapper_FWD_DEFINED__ */


#ifndef __IFsrmExportImport_FWD_DEFINED__
#define __IFsrmExportImport_FWD_DEFINED__
typedef interface IFsrmExportImport IFsrmExportImport;

#endif 	/* __IFsrmExportImport_FWD_DEFINED__ */


#ifndef __IFsrmDerivedObjectsResult_FWD_DEFINED__
#define __IFsrmDerivedObjectsResult_FWD_DEFINED__
typedef interface IFsrmDerivedObjectsResult IFsrmDerivedObjectsResult;

#endif 	/* __IFsrmDerivedObjectsResult_FWD_DEFINED__ */


#ifndef __IFsrmAccessDeniedRemediationClient_FWD_DEFINED__
#define __IFsrmAccessDeniedRemediationClient_FWD_DEFINED__
typedef interface IFsrmAccessDeniedRemediationClient IFsrmAccessDeniedRemediationClient;

#endif 	/* __IFsrmAccessDeniedRemediationClient_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "fsrmenums.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_fsrm_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)












#define	FSRM_DISPID_FEATURE_MASK	( 0xf000000 )

#define	FSRM_DISPID_INTERFACE_A_MASK	( 0xf00000 )

#define	FSRM_DISPID_INTERFACE_B_MASK	( 0xf0000 )

#define	FSRM_DISPID_INTERFACE_C_MASK	( 0xf000 )

#define	FSRM_DISPID_INTERFACE_D_MASK	( 0xf00 )

#define	FSRM_DISPID_INTERFACE_MASK	( ( ( ( FSRM_DISPID_INTERFACE_A_MASK | FSRM_DISPID_INTERFACE_B_MASK )  | FSRM_DISPID_INTERFACE_C_MASK )  | FSRM_DISPID_INTERFACE_D_MASK )  )

#define	FSRM_DISPID_IS_PROPERTY	( 0x80 )

#define	FSRM_DISPID_METHOD_NUM_MASK	( 0x7f )

#define	FSRM_DISPID_METHOD_MASK	( ( FSRM_DISPID_IS_PROPERTY | FSRM_DISPID_METHOD_NUM_MASK )  )

#define	FSRM_DISPID_FEATURE_GENERAL	( 0x1000000 )

#define	FSRM_DISPID_FEATURE_QUOTA	( 0x2000000 )

#define	FSRM_DISPID_FEATURE_FILESCREEN	( 0x3000000 )

#define	FSRM_DISPID_FEATURE_REPORTS	( 0x4000000 )

#define	FSRM_DISPID_FEATURE_CLASSIFICATION	( 0x5000000 )

#define	FSRM_DISPID_FEATURE_PIPELINE	( 0x6000000 )

#define	FSRM_DISPID_OBJECT	( ( FSRM_DISPID_FEATURE_GENERAL | 0x100000 )  )

#define	FSRM_DISPID_COLLECTION	( ( FSRM_DISPID_FEATURE_GENERAL | 0x200000 )  )

#define	FSRM_DISPID_COLLECTION_MUTABLE	( ( FSRM_DISPID_COLLECTION | 0x10000 )  )

#define	FSRM_DISPID_COLLECTION_COMMITTABLE	( ( FSRM_DISPID_COLLECTION_MUTABLE | 0x1000 )  )

#define	FSRM_DISPID_ACTION	( ( FSRM_DISPID_FEATURE_GENERAL | 0x300000 )  )

#define	FSRM_DISPID_ACTION_EMAIL	( ( FSRM_DISPID_ACTION | 0x10000 )  )

#define	FSRM_DISPID_ACTION_REPORT	( ( FSRM_DISPID_ACTION | 0x20000 )  )

#define	FSRM_DISPID_ACTION_EVENTLOG	( ( FSRM_DISPID_ACTION | 0x30000 )  )

#define	FSRM_DISPID_ACTION_COMMAND	( ( FSRM_DISPID_ACTION | 0x40000 )  )

#define	FSRM_DISPID_ACTION_EMAIL2	( ( FSRM_DISPID_ACTION | 0x50000 )  )

#define	FSRM_DISPID_SETTING	( ( FSRM_DISPID_FEATURE_GENERAL | 0x400000 )  )

#define	FSRM_DISPID_PATHMAPPER	( ( FSRM_DISPID_FEATURE_GENERAL | 0x500000 )  )

#define	FSRM_DISPID_EXPORTIMPORT	( ( FSRM_DISPID_FEATURE_GENERAL | 0x600000 )  )

#define	FSRM_DISPID_DERIVEDOBJECTSRESULT	( ( FSRM_DISPID_FEATURE_GENERAL | 0x700000 )  )

#define	FSRM_DISPID_ADR	( ( FSRM_DISPID_FEATURE_GENERAL | 0x800000 )  )



extern RPC_IF_HANDLE __MIDL_itf_fsrm_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_fsrm_0000_0000_v0_0_s_ifspec;

#ifndef __IFsrmObject_INTERFACE_DEFINED__
#define __IFsrmObject_INTERFACE_DEFINED__

/* interface IFsrmObject */
/* [unique][helpstring][nonextensible][hidden][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("22bcef93-4a3f-4183-89f9-2f8b8a628aee")
    IFsrmObject : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__out FSRM_OBJECT_ID *id) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR description) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Commit( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmObject * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmObject * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmObject * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmObject * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmObject, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IFsrmObject * This,
            /* [retval][out] */ __RPC__out FSRM_OBJECT_ID *id);
        
        DECLSPEC_XFGVIRT(IFsrmObject, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IFsrmObject * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IFsrmObject * This,
            /* [in] */ __RPC__in BSTR description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmObject * This);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Commit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IFsrmObject * This);
        
        END_INTERFACE
    } IFsrmObjectVtbl;

    interface IFsrmObject
    {
        CONST_VTBL struct IFsrmObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmObject_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmObject_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmObject_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmObject_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmObject_get_Id(This,id)	\
    ( (This)->lpVtbl -> get_Id(This,id) ) 

#define IFsrmObject_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#define IFsrmObject_put_Description(This,description)	\
    ( (This)->lpVtbl -> put_Description(This,description) ) 

#define IFsrmObject_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IFsrmObject_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmObject_INTERFACE_DEFINED__ */


#ifndef __IFsrmCollection_INTERFACE_DEFINED__
#define __IFsrmCollection_INTERFACE_DEFINED__

/* interface IFsrmCollection */
/* [unique][helpstring][nonextensible][hidden][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f76fbf3b-8ddd-4b42-b05a-cb1c3ff1fee8")
    IFsrmCollection : public IDispatch
    {
    public:
        virtual /* [restricted][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **unknown) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long index,
            /* [retval][out] */ __RPC__out VARIANT *item) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *count) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out FsrmCollectionState *state) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE WaitForCompletion( 
            /* [in] */ long waitSeconds,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *completed) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetById( 
            /* [in] */ FSRM_OBJECT_ID id,
            /* [retval][out] */ __RPC__out VARIANT *entry) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmCollection * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmCollection, get__NewEnum)
        /* [restricted][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFsrmCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **unknown);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFsrmCollection * This,
            /* [in] */ long index,
            /* [retval][out] */ __RPC__out VARIANT *item);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFsrmCollection * This,
            /* [retval][out] */ __RPC__out long *count);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IFsrmCollection * This,
            /* [retval][out] */ __RPC__out FsrmCollectionState *state);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, Cancel)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IFsrmCollection * This);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, WaitForCompletion)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *WaitForCompletion )( 
            __RPC__in IFsrmCollection * This,
            /* [in] */ long waitSeconds,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *completed);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, GetById)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetById )( 
            __RPC__in IFsrmCollection * This,
            /* [in] */ FSRM_OBJECT_ID id,
            /* [retval][out] */ __RPC__out VARIANT *entry);
        
        END_INTERFACE
    } IFsrmCollectionVtbl;

    interface IFsrmCollection
    {
        CONST_VTBL struct IFsrmCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmCollection_get__NewEnum(This,unknown)	\
    ( (This)->lpVtbl -> get__NewEnum(This,unknown) ) 

#define IFsrmCollection_get_Item(This,index,item)	\
    ( (This)->lpVtbl -> get_Item(This,index,item) ) 

#define IFsrmCollection_get_Count(This,count)	\
    ( (This)->lpVtbl -> get_Count(This,count) ) 

#define IFsrmCollection_get_State(This,state)	\
    ( (This)->lpVtbl -> get_State(This,state) ) 

#define IFsrmCollection_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IFsrmCollection_WaitForCompletion(This,waitSeconds,completed)	\
    ( (This)->lpVtbl -> WaitForCompletion(This,waitSeconds,completed) ) 

#define IFsrmCollection_GetById(This,id,entry)	\
    ( (This)->lpVtbl -> GetById(This,id,entry) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmCollection_INTERFACE_DEFINED__ */


#ifndef __IFsrmMutableCollection_INTERFACE_DEFINED__
#define __IFsrmMutableCollection_INTERFACE_DEFINED__

/* interface IFsrmMutableCollection */
/* [unique][helpstring][nonextensible][hidden][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmMutableCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1bb617b8-3886-49dc-af82-a6c90fa35dda")
    IFsrmMutableCollection : public IFsrmCollection
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ VARIANT item) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ long index) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveById( 
            /* [in] */ FSRM_OBJECT_ID id) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IFsrmMutableCollection **collection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmMutableCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmMutableCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmMutableCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmMutableCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmMutableCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmMutableCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmMutableCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmMutableCollection * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmCollection, get__NewEnum)
        /* [restricted][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFsrmMutableCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **unknown);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFsrmMutableCollection * This,
            /* [in] */ long index,
            /* [retval][out] */ __RPC__out VARIANT *item);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFsrmMutableCollection * This,
            /* [retval][out] */ __RPC__out long *count);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IFsrmMutableCollection * This,
            /* [retval][out] */ __RPC__out FsrmCollectionState *state);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, Cancel)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IFsrmMutableCollection * This);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, WaitForCompletion)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *WaitForCompletion )( 
            __RPC__in IFsrmMutableCollection * This,
            /* [in] */ long waitSeconds,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *completed);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, GetById)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetById )( 
            __RPC__in IFsrmMutableCollection * This,
            /* [in] */ FSRM_OBJECT_ID id,
            /* [retval][out] */ __RPC__out VARIANT *entry);
        
        DECLSPEC_XFGVIRT(IFsrmMutableCollection, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IFsrmMutableCollection * This,
            /* [in] */ VARIANT item);
        
        DECLSPEC_XFGVIRT(IFsrmMutableCollection, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IFsrmMutableCollection * This,
            /* [in] */ long index);
        
        DECLSPEC_XFGVIRT(IFsrmMutableCollection, RemoveById)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveById )( 
            __RPC__in IFsrmMutableCollection * This,
            /* [in] */ FSRM_OBJECT_ID id);
        
        DECLSPEC_XFGVIRT(IFsrmMutableCollection, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IFsrmMutableCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmMutableCollection **collection);
        
        END_INTERFACE
    } IFsrmMutableCollectionVtbl;

    interface IFsrmMutableCollection
    {
        CONST_VTBL struct IFsrmMutableCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmMutableCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmMutableCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmMutableCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmMutableCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmMutableCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmMutableCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmMutableCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmMutableCollection_get__NewEnum(This,unknown)	\
    ( (This)->lpVtbl -> get__NewEnum(This,unknown) ) 

#define IFsrmMutableCollection_get_Item(This,index,item)	\
    ( (This)->lpVtbl -> get_Item(This,index,item) ) 

#define IFsrmMutableCollection_get_Count(This,count)	\
    ( (This)->lpVtbl -> get_Count(This,count) ) 

#define IFsrmMutableCollection_get_State(This,state)	\
    ( (This)->lpVtbl -> get_State(This,state) ) 

#define IFsrmMutableCollection_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IFsrmMutableCollection_WaitForCompletion(This,waitSeconds,completed)	\
    ( (This)->lpVtbl -> WaitForCompletion(This,waitSeconds,completed) ) 

#define IFsrmMutableCollection_GetById(This,id,entry)	\
    ( (This)->lpVtbl -> GetById(This,id,entry) ) 


#define IFsrmMutableCollection_Add(This,item)	\
    ( (This)->lpVtbl -> Add(This,item) ) 

#define IFsrmMutableCollection_Remove(This,index)	\
    ( (This)->lpVtbl -> Remove(This,index) ) 

#define IFsrmMutableCollection_RemoveById(This,id)	\
    ( (This)->lpVtbl -> RemoveById(This,id) ) 

#define IFsrmMutableCollection_Clone(This,collection)	\
    ( (This)->lpVtbl -> Clone(This,collection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmMutableCollection_INTERFACE_DEFINED__ */


#ifndef __IFsrmCommittableCollection_INTERFACE_DEFINED__
#define __IFsrmCommittableCollection_INTERFACE_DEFINED__

/* interface IFsrmCommittableCollection */
/* [unique][helpstring][nonextensible][hidden][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmCommittableCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("96deb3b5-8b91-4a2a-9d93-80a35d8aa847")
    IFsrmCommittableCollection : public IFsrmMutableCollection
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Commit( 
            /* [in] */ FsrmCommitOptions options,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **results) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmCommittableCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmCommittableCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmCommittableCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmCommittableCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmCommittableCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmCommittableCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmCommittableCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmCommittableCollection * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmCollection, get__NewEnum)
        /* [restricted][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFsrmCommittableCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **unknown);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFsrmCommittableCollection * This,
            /* [in] */ long index,
            /* [retval][out] */ __RPC__out VARIANT *item);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFsrmCommittableCollection * This,
            /* [retval][out] */ __RPC__out long *count);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IFsrmCommittableCollection * This,
            /* [retval][out] */ __RPC__out FsrmCollectionState *state);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, Cancel)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IFsrmCommittableCollection * This);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, WaitForCompletion)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *WaitForCompletion )( 
            __RPC__in IFsrmCommittableCollection * This,
            /* [in] */ long waitSeconds,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *completed);
        
        DECLSPEC_XFGVIRT(IFsrmCollection, GetById)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetById )( 
            __RPC__in IFsrmCommittableCollection * This,
            /* [in] */ FSRM_OBJECT_ID id,
            /* [retval][out] */ __RPC__out VARIANT *entry);
        
        DECLSPEC_XFGVIRT(IFsrmMutableCollection, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IFsrmCommittableCollection * This,
            /* [in] */ VARIANT item);
        
        DECLSPEC_XFGVIRT(IFsrmMutableCollection, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IFsrmCommittableCollection * This,
            /* [in] */ long index);
        
        DECLSPEC_XFGVIRT(IFsrmMutableCollection, RemoveById)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveById )( 
            __RPC__in IFsrmCommittableCollection * This,
            /* [in] */ FSRM_OBJECT_ID id);
        
        DECLSPEC_XFGVIRT(IFsrmMutableCollection, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IFsrmCommittableCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmMutableCollection **collection);
        
        DECLSPEC_XFGVIRT(IFsrmCommittableCollection, Commit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IFsrmCommittableCollection * This,
            /* [in] */ FsrmCommitOptions options,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **results);
        
        END_INTERFACE
    } IFsrmCommittableCollectionVtbl;

    interface IFsrmCommittableCollection
    {
        CONST_VTBL struct IFsrmCommittableCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmCommittableCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmCommittableCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmCommittableCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmCommittableCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmCommittableCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmCommittableCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmCommittableCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmCommittableCollection_get__NewEnum(This,unknown)	\
    ( (This)->lpVtbl -> get__NewEnum(This,unknown) ) 

#define IFsrmCommittableCollection_get_Item(This,index,item)	\
    ( (This)->lpVtbl -> get_Item(This,index,item) ) 

#define IFsrmCommittableCollection_get_Count(This,count)	\
    ( (This)->lpVtbl -> get_Count(This,count) ) 

#define IFsrmCommittableCollection_get_State(This,state)	\
    ( (This)->lpVtbl -> get_State(This,state) ) 

#define IFsrmCommittableCollection_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IFsrmCommittableCollection_WaitForCompletion(This,waitSeconds,completed)	\
    ( (This)->lpVtbl -> WaitForCompletion(This,waitSeconds,completed) ) 

#define IFsrmCommittableCollection_GetById(This,id,entry)	\
    ( (This)->lpVtbl -> GetById(This,id,entry) ) 


#define IFsrmCommittableCollection_Add(This,item)	\
    ( (This)->lpVtbl -> Add(This,item) ) 

#define IFsrmCommittableCollection_Remove(This,index)	\
    ( (This)->lpVtbl -> Remove(This,index) ) 

#define IFsrmCommittableCollection_RemoveById(This,id)	\
    ( (This)->lpVtbl -> RemoveById(This,id) ) 

#define IFsrmCommittableCollection_Clone(This,collection)	\
    ( (This)->lpVtbl -> Clone(This,collection) ) 


#define IFsrmCommittableCollection_Commit(This,options,results)	\
    ( (This)->lpVtbl -> Commit(This,options,results) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmCommittableCollection_INTERFACE_DEFINED__ */


#ifndef __IFsrmAction_INTERFACE_DEFINED__
#define __IFsrmAction_INTERFACE_DEFINED__

/* interface IFsrmAction */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmAction;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6cd6408a-ae60-463b-9ef1-e117534d69dc")
    IFsrmAction : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__out FSRM_OBJECT_ID *id) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ActionType( 
            /* [retval][out] */ __RPC__out FsrmActionType *actionType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RunLimitInterval( 
            /* [retval][out] */ __RPC__out long *minutes) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RunLimitInterval( 
            /* [in] */ long minutes) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmActionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmAction * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmAction * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmAction * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmAction * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmAction * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmAction * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmAction * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IFsrmAction * This,
            /* [retval][out] */ __RPC__out FSRM_OBJECT_ID *id);
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_ActionType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActionType )( 
            __RPC__in IFsrmAction * This,
            /* [retval][out] */ __RPC__out FsrmActionType *actionType);
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_RunLimitInterval)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RunLimitInterval )( 
            __RPC__in IFsrmAction * This,
            /* [retval][out] */ __RPC__out long *minutes);
        
        DECLSPEC_XFGVIRT(IFsrmAction, put_RunLimitInterval)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RunLimitInterval )( 
            __RPC__in IFsrmAction * This,
            /* [in] */ long minutes);
        
        DECLSPEC_XFGVIRT(IFsrmAction, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmAction * This);
        
        END_INTERFACE
    } IFsrmActionVtbl;

    interface IFsrmAction
    {
        CONST_VTBL struct IFsrmActionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmAction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmAction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmAction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmAction_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmAction_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmAction_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmAction_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmAction_get_Id(This,id)	\
    ( (This)->lpVtbl -> get_Id(This,id) ) 

#define IFsrmAction_get_ActionType(This,actionType)	\
    ( (This)->lpVtbl -> get_ActionType(This,actionType) ) 

#define IFsrmAction_get_RunLimitInterval(This,minutes)	\
    ( (This)->lpVtbl -> get_RunLimitInterval(This,minutes) ) 

#define IFsrmAction_put_RunLimitInterval(This,minutes)	\
    ( (This)->lpVtbl -> put_RunLimitInterval(This,minutes) ) 

#define IFsrmAction_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmAction_INTERFACE_DEFINED__ */


#ifndef __IFsrmActionEmail_INTERFACE_DEFINED__
#define __IFsrmActionEmail_INTERFACE_DEFINED__

/* interface IFsrmActionEmail */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmActionEmail;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d646567d-26ae-4caa-9f84-4e0aad207fca")
    IFsrmActionEmail : public IFsrmAction
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MailFrom( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailFrom) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MailFrom( 
            /* [in] */ __RPC__in BSTR mailFrom) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MailReplyTo( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailReplyTo) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MailReplyTo( 
            /* [in] */ __RPC__in BSTR mailReplyTo) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MailTo( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailTo) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MailTo( 
            /* [in] */ __RPC__in BSTR mailTo) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MailCc( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailCc) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MailCc( 
            /* [in] */ __RPC__in BSTR mailCc) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MailBcc( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailBcc) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MailBcc( 
            /* [in] */ __RPC__in BSTR mailBcc) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MailSubject( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailSubject) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MailSubject( 
            /* [in] */ __RPC__in BSTR mailSubject) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MessageText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *messageText) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MessageText( 
            /* [in] */ __RPC__in BSTR messageText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmActionEmailVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmActionEmail * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmActionEmail * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmActionEmail * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmActionEmail * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmActionEmail * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmActionEmail * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmActionEmail * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IFsrmActionEmail * This,
            /* [retval][out] */ __RPC__out FSRM_OBJECT_ID *id);
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_ActionType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActionType )( 
            __RPC__in IFsrmActionEmail * This,
            /* [retval][out] */ __RPC__out FsrmActionType *actionType);
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_RunLimitInterval)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RunLimitInterval )( 
            __RPC__in IFsrmActionEmail * This,
            /* [retval][out] */ __RPC__out long *minutes);
        
        DECLSPEC_XFGVIRT(IFsrmAction, put_RunLimitInterval)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RunLimitInterval )( 
            __RPC__in IFsrmActionEmail * This,
            /* [in] */ long minutes);
        
        DECLSPEC_XFGVIRT(IFsrmAction, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmActionEmail * This);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, get_MailFrom)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MailFrom )( 
            __RPC__in IFsrmActionEmail * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailFrom);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, put_MailFrom)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MailFrom )( 
            __RPC__in IFsrmActionEmail * This,
            /* [in] */ __RPC__in BSTR mailFrom);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, get_MailReplyTo)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MailReplyTo )( 
            __RPC__in IFsrmActionEmail * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailReplyTo);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, put_MailReplyTo)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MailReplyTo )( 
            __RPC__in IFsrmActionEmail * This,
            /* [in] */ __RPC__in BSTR mailReplyTo);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, get_MailTo)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MailTo )( 
            __RPC__in IFsrmActionEmail * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailTo);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, put_MailTo)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MailTo )( 
            __RPC__in IFsrmActionEmail * This,
            /* [in] */ __RPC__in BSTR mailTo);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, get_MailCc)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MailCc )( 
            __RPC__in IFsrmActionEmail * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailCc);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, put_MailCc)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MailCc )( 
            __RPC__in IFsrmActionEmail * This,
            /* [in] */ __RPC__in BSTR mailCc);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, get_MailBcc)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MailBcc )( 
            __RPC__in IFsrmActionEmail * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailBcc);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, put_MailBcc)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MailBcc )( 
            __RPC__in IFsrmActionEmail * This,
            /* [in] */ __RPC__in BSTR mailBcc);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, get_MailSubject)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MailSubject )( 
            __RPC__in IFsrmActionEmail * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailSubject);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, put_MailSubject)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MailSubject )( 
            __RPC__in IFsrmActionEmail * This,
            /* [in] */ __RPC__in BSTR mailSubject);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, get_MessageText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MessageText )( 
            __RPC__in IFsrmActionEmail * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *messageText);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, put_MessageText)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MessageText )( 
            __RPC__in IFsrmActionEmail * This,
            /* [in] */ __RPC__in BSTR messageText);
        
        END_INTERFACE
    } IFsrmActionEmailVtbl;

    interface IFsrmActionEmail
    {
        CONST_VTBL struct IFsrmActionEmailVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmActionEmail_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmActionEmail_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmActionEmail_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmActionEmail_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmActionEmail_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmActionEmail_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmActionEmail_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmActionEmail_get_Id(This,id)	\
    ( (This)->lpVtbl -> get_Id(This,id) ) 

#define IFsrmActionEmail_get_ActionType(This,actionType)	\
    ( (This)->lpVtbl -> get_ActionType(This,actionType) ) 

#define IFsrmActionEmail_get_RunLimitInterval(This,minutes)	\
    ( (This)->lpVtbl -> get_RunLimitInterval(This,minutes) ) 

#define IFsrmActionEmail_put_RunLimitInterval(This,minutes)	\
    ( (This)->lpVtbl -> put_RunLimitInterval(This,minutes) ) 

#define IFsrmActionEmail_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 


#define IFsrmActionEmail_get_MailFrom(This,mailFrom)	\
    ( (This)->lpVtbl -> get_MailFrom(This,mailFrom) ) 

#define IFsrmActionEmail_put_MailFrom(This,mailFrom)	\
    ( (This)->lpVtbl -> put_MailFrom(This,mailFrom) ) 

#define IFsrmActionEmail_get_MailReplyTo(This,mailReplyTo)	\
    ( (This)->lpVtbl -> get_MailReplyTo(This,mailReplyTo) ) 

#define IFsrmActionEmail_put_MailReplyTo(This,mailReplyTo)	\
    ( (This)->lpVtbl -> put_MailReplyTo(This,mailReplyTo) ) 

#define IFsrmActionEmail_get_MailTo(This,mailTo)	\
    ( (This)->lpVtbl -> get_MailTo(This,mailTo) ) 

#define IFsrmActionEmail_put_MailTo(This,mailTo)	\
    ( (This)->lpVtbl -> put_MailTo(This,mailTo) ) 

#define IFsrmActionEmail_get_MailCc(This,mailCc)	\
    ( (This)->lpVtbl -> get_MailCc(This,mailCc) ) 

#define IFsrmActionEmail_put_MailCc(This,mailCc)	\
    ( (This)->lpVtbl -> put_MailCc(This,mailCc) ) 

#define IFsrmActionEmail_get_MailBcc(This,mailBcc)	\
    ( (This)->lpVtbl -> get_MailBcc(This,mailBcc) ) 

#define IFsrmActionEmail_put_MailBcc(This,mailBcc)	\
    ( (This)->lpVtbl -> put_MailBcc(This,mailBcc) ) 

#define IFsrmActionEmail_get_MailSubject(This,mailSubject)	\
    ( (This)->lpVtbl -> get_MailSubject(This,mailSubject) ) 

#define IFsrmActionEmail_put_MailSubject(This,mailSubject)	\
    ( (This)->lpVtbl -> put_MailSubject(This,mailSubject) ) 

#define IFsrmActionEmail_get_MessageText(This,messageText)	\
    ( (This)->lpVtbl -> get_MessageText(This,messageText) ) 

#define IFsrmActionEmail_put_MessageText(This,messageText)	\
    ( (This)->lpVtbl -> put_MessageText(This,messageText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmActionEmail_INTERFACE_DEFINED__ */


#ifndef __IFsrmActionEmail2_INTERFACE_DEFINED__
#define __IFsrmActionEmail2_INTERFACE_DEFINED__

/* interface IFsrmActionEmail2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmActionEmail2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8276702f-2532-4839-89bf-4872609a2ea4")
    IFsrmActionEmail2 : public IFsrmActionEmail
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AttachmentFileListSize( 
            /* [retval][out] */ __RPC__out long *attachmentFileListSize) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AttachmentFileListSize( 
            /* [in] */ long attachmentFileListSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmActionEmail2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmActionEmail2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmActionEmail2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmActionEmail2 * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [retval][out] */ __RPC__out FSRM_OBJECT_ID *id);
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_ActionType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActionType )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [retval][out] */ __RPC__out FsrmActionType *actionType);
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_RunLimitInterval)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RunLimitInterval )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [retval][out] */ __RPC__out long *minutes);
        
        DECLSPEC_XFGVIRT(IFsrmAction, put_RunLimitInterval)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RunLimitInterval )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [in] */ long minutes);
        
        DECLSPEC_XFGVIRT(IFsrmAction, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmActionEmail2 * This);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, get_MailFrom)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MailFrom )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailFrom);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, put_MailFrom)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MailFrom )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [in] */ __RPC__in BSTR mailFrom);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, get_MailReplyTo)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MailReplyTo )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailReplyTo);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, put_MailReplyTo)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MailReplyTo )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [in] */ __RPC__in BSTR mailReplyTo);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, get_MailTo)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MailTo )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailTo);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, put_MailTo)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MailTo )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [in] */ __RPC__in BSTR mailTo);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, get_MailCc)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MailCc )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailCc);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, put_MailCc)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MailCc )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [in] */ __RPC__in BSTR mailCc);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, get_MailBcc)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MailBcc )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailBcc);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, put_MailBcc)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MailBcc )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [in] */ __RPC__in BSTR mailBcc);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, get_MailSubject)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MailSubject )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailSubject);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, put_MailSubject)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MailSubject )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [in] */ __RPC__in BSTR mailSubject);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, get_MessageText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MessageText )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *messageText);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail, put_MessageText)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MessageText )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [in] */ __RPC__in BSTR messageText);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail2, get_AttachmentFileListSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AttachmentFileListSize )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [retval][out] */ __RPC__out long *attachmentFileListSize);
        
        DECLSPEC_XFGVIRT(IFsrmActionEmail2, put_AttachmentFileListSize)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AttachmentFileListSize )( 
            __RPC__in IFsrmActionEmail2 * This,
            /* [in] */ long attachmentFileListSize);
        
        END_INTERFACE
    } IFsrmActionEmail2Vtbl;

    interface IFsrmActionEmail2
    {
        CONST_VTBL struct IFsrmActionEmail2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmActionEmail2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmActionEmail2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmActionEmail2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmActionEmail2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmActionEmail2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmActionEmail2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmActionEmail2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmActionEmail2_get_Id(This,id)	\
    ( (This)->lpVtbl -> get_Id(This,id) ) 

#define IFsrmActionEmail2_get_ActionType(This,actionType)	\
    ( (This)->lpVtbl -> get_ActionType(This,actionType) ) 

#define IFsrmActionEmail2_get_RunLimitInterval(This,minutes)	\
    ( (This)->lpVtbl -> get_RunLimitInterval(This,minutes) ) 

#define IFsrmActionEmail2_put_RunLimitInterval(This,minutes)	\
    ( (This)->lpVtbl -> put_RunLimitInterval(This,minutes) ) 

#define IFsrmActionEmail2_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 


#define IFsrmActionEmail2_get_MailFrom(This,mailFrom)	\
    ( (This)->lpVtbl -> get_MailFrom(This,mailFrom) ) 

#define IFsrmActionEmail2_put_MailFrom(This,mailFrom)	\
    ( (This)->lpVtbl -> put_MailFrom(This,mailFrom) ) 

#define IFsrmActionEmail2_get_MailReplyTo(This,mailReplyTo)	\
    ( (This)->lpVtbl -> get_MailReplyTo(This,mailReplyTo) ) 

#define IFsrmActionEmail2_put_MailReplyTo(This,mailReplyTo)	\
    ( (This)->lpVtbl -> put_MailReplyTo(This,mailReplyTo) ) 

#define IFsrmActionEmail2_get_MailTo(This,mailTo)	\
    ( (This)->lpVtbl -> get_MailTo(This,mailTo) ) 

#define IFsrmActionEmail2_put_MailTo(This,mailTo)	\
    ( (This)->lpVtbl -> put_MailTo(This,mailTo) ) 

#define IFsrmActionEmail2_get_MailCc(This,mailCc)	\
    ( (This)->lpVtbl -> get_MailCc(This,mailCc) ) 

#define IFsrmActionEmail2_put_MailCc(This,mailCc)	\
    ( (This)->lpVtbl -> put_MailCc(This,mailCc) ) 

#define IFsrmActionEmail2_get_MailBcc(This,mailBcc)	\
    ( (This)->lpVtbl -> get_MailBcc(This,mailBcc) ) 

#define IFsrmActionEmail2_put_MailBcc(This,mailBcc)	\
    ( (This)->lpVtbl -> put_MailBcc(This,mailBcc) ) 

#define IFsrmActionEmail2_get_MailSubject(This,mailSubject)	\
    ( (This)->lpVtbl -> get_MailSubject(This,mailSubject) ) 

#define IFsrmActionEmail2_put_MailSubject(This,mailSubject)	\
    ( (This)->lpVtbl -> put_MailSubject(This,mailSubject) ) 

#define IFsrmActionEmail2_get_MessageText(This,messageText)	\
    ( (This)->lpVtbl -> get_MessageText(This,messageText) ) 

#define IFsrmActionEmail2_put_MessageText(This,messageText)	\
    ( (This)->lpVtbl -> put_MessageText(This,messageText) ) 


#define IFsrmActionEmail2_get_AttachmentFileListSize(This,attachmentFileListSize)	\
    ( (This)->lpVtbl -> get_AttachmentFileListSize(This,attachmentFileListSize) ) 

#define IFsrmActionEmail2_put_AttachmentFileListSize(This,attachmentFileListSize)	\
    ( (This)->lpVtbl -> put_AttachmentFileListSize(This,attachmentFileListSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmActionEmail2_INTERFACE_DEFINED__ */


#ifndef __IFsrmActionReport_INTERFACE_DEFINED__
#define __IFsrmActionReport_INTERFACE_DEFINED__

/* interface IFsrmActionReport */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmActionReport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2dbe63c4-b340-48a0-a5b0-158e07fc567e")
    IFsrmActionReport : public IFsrmAction
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReportTypes( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *reportTypes) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ReportTypes( 
            /* [in] */ __RPC__in SAFEARRAY * reportTypes) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MailTo( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailTo) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MailTo( 
            /* [in] */ __RPC__in BSTR mailTo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmActionReportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmActionReport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmActionReport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmActionReport * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmActionReport * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmActionReport * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmActionReport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmActionReport * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IFsrmActionReport * This,
            /* [retval][out] */ __RPC__out FSRM_OBJECT_ID *id);
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_ActionType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActionType )( 
            __RPC__in IFsrmActionReport * This,
            /* [retval][out] */ __RPC__out FsrmActionType *actionType);
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_RunLimitInterval)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RunLimitInterval )( 
            __RPC__in IFsrmActionReport * This,
            /* [retval][out] */ __RPC__out long *minutes);
        
        DECLSPEC_XFGVIRT(IFsrmAction, put_RunLimitInterval)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RunLimitInterval )( 
            __RPC__in IFsrmActionReport * This,
            /* [in] */ long minutes);
        
        DECLSPEC_XFGVIRT(IFsrmAction, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmActionReport * This);
        
        DECLSPEC_XFGVIRT(IFsrmActionReport, get_ReportTypes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReportTypes )( 
            __RPC__in IFsrmActionReport * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *reportTypes);
        
        DECLSPEC_XFGVIRT(IFsrmActionReport, put_ReportTypes)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReportTypes )( 
            __RPC__in IFsrmActionReport * This,
            /* [in] */ __RPC__in SAFEARRAY * reportTypes);
        
        DECLSPEC_XFGVIRT(IFsrmActionReport, get_MailTo)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MailTo )( 
            __RPC__in IFsrmActionReport * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailTo);
        
        DECLSPEC_XFGVIRT(IFsrmActionReport, put_MailTo)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MailTo )( 
            __RPC__in IFsrmActionReport * This,
            /* [in] */ __RPC__in BSTR mailTo);
        
        END_INTERFACE
    } IFsrmActionReportVtbl;

    interface IFsrmActionReport
    {
        CONST_VTBL struct IFsrmActionReportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmActionReport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmActionReport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmActionReport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmActionReport_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmActionReport_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmActionReport_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmActionReport_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmActionReport_get_Id(This,id)	\
    ( (This)->lpVtbl -> get_Id(This,id) ) 

#define IFsrmActionReport_get_ActionType(This,actionType)	\
    ( (This)->lpVtbl -> get_ActionType(This,actionType) ) 

#define IFsrmActionReport_get_RunLimitInterval(This,minutes)	\
    ( (This)->lpVtbl -> get_RunLimitInterval(This,minutes) ) 

#define IFsrmActionReport_put_RunLimitInterval(This,minutes)	\
    ( (This)->lpVtbl -> put_RunLimitInterval(This,minutes) ) 

#define IFsrmActionReport_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 


#define IFsrmActionReport_get_ReportTypes(This,reportTypes)	\
    ( (This)->lpVtbl -> get_ReportTypes(This,reportTypes) ) 

#define IFsrmActionReport_put_ReportTypes(This,reportTypes)	\
    ( (This)->lpVtbl -> put_ReportTypes(This,reportTypes) ) 

#define IFsrmActionReport_get_MailTo(This,mailTo)	\
    ( (This)->lpVtbl -> get_MailTo(This,mailTo) ) 

#define IFsrmActionReport_put_MailTo(This,mailTo)	\
    ( (This)->lpVtbl -> put_MailTo(This,mailTo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmActionReport_INTERFACE_DEFINED__ */


#ifndef __IFsrmActionEventLog_INTERFACE_DEFINED__
#define __IFsrmActionEventLog_INTERFACE_DEFINED__

/* interface IFsrmActionEventLog */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmActionEventLog;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4c8f96c3-5d94-4f37-a4f4-f56ab463546f")
    IFsrmActionEventLog : public IFsrmAction
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventType( 
            /* [retval][out] */ __RPC__out FsrmEventType *eventType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EventType( 
            /* [in] */ FsrmEventType eventType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MessageText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *messageText) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MessageText( 
            /* [in] */ __RPC__in BSTR messageText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmActionEventLogVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmActionEventLog * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmActionEventLog * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmActionEventLog * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmActionEventLog * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmActionEventLog * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmActionEventLog * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmActionEventLog * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IFsrmActionEventLog * This,
            /* [retval][out] */ __RPC__out FSRM_OBJECT_ID *id);
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_ActionType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActionType )( 
            __RPC__in IFsrmActionEventLog * This,
            /* [retval][out] */ __RPC__out FsrmActionType *actionType);
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_RunLimitInterval)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RunLimitInterval )( 
            __RPC__in IFsrmActionEventLog * This,
            /* [retval][out] */ __RPC__out long *minutes);
        
        DECLSPEC_XFGVIRT(IFsrmAction, put_RunLimitInterval)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RunLimitInterval )( 
            __RPC__in IFsrmActionEventLog * This,
            /* [in] */ long minutes);
        
        DECLSPEC_XFGVIRT(IFsrmAction, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmActionEventLog * This);
        
        DECLSPEC_XFGVIRT(IFsrmActionEventLog, get_EventType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventType )( 
            __RPC__in IFsrmActionEventLog * This,
            /* [retval][out] */ __RPC__out FsrmEventType *eventType);
        
        DECLSPEC_XFGVIRT(IFsrmActionEventLog, put_EventType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EventType )( 
            __RPC__in IFsrmActionEventLog * This,
            /* [in] */ FsrmEventType eventType);
        
        DECLSPEC_XFGVIRT(IFsrmActionEventLog, get_MessageText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MessageText )( 
            __RPC__in IFsrmActionEventLog * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *messageText);
        
        DECLSPEC_XFGVIRT(IFsrmActionEventLog, put_MessageText)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MessageText )( 
            __RPC__in IFsrmActionEventLog * This,
            /* [in] */ __RPC__in BSTR messageText);
        
        END_INTERFACE
    } IFsrmActionEventLogVtbl;

    interface IFsrmActionEventLog
    {
        CONST_VTBL struct IFsrmActionEventLogVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmActionEventLog_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmActionEventLog_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmActionEventLog_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmActionEventLog_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmActionEventLog_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmActionEventLog_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmActionEventLog_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmActionEventLog_get_Id(This,id)	\
    ( (This)->lpVtbl -> get_Id(This,id) ) 

#define IFsrmActionEventLog_get_ActionType(This,actionType)	\
    ( (This)->lpVtbl -> get_ActionType(This,actionType) ) 

#define IFsrmActionEventLog_get_RunLimitInterval(This,minutes)	\
    ( (This)->lpVtbl -> get_RunLimitInterval(This,minutes) ) 

#define IFsrmActionEventLog_put_RunLimitInterval(This,minutes)	\
    ( (This)->lpVtbl -> put_RunLimitInterval(This,minutes) ) 

#define IFsrmActionEventLog_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 


#define IFsrmActionEventLog_get_EventType(This,eventType)	\
    ( (This)->lpVtbl -> get_EventType(This,eventType) ) 

#define IFsrmActionEventLog_put_EventType(This,eventType)	\
    ( (This)->lpVtbl -> put_EventType(This,eventType) ) 

#define IFsrmActionEventLog_get_MessageText(This,messageText)	\
    ( (This)->lpVtbl -> get_MessageText(This,messageText) ) 

#define IFsrmActionEventLog_put_MessageText(This,messageText)	\
    ( (This)->lpVtbl -> put_MessageText(This,messageText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmActionEventLog_INTERFACE_DEFINED__ */


#ifndef __IFsrmActionCommand_INTERFACE_DEFINED__
#define __IFsrmActionCommand_INTERFACE_DEFINED__

/* interface IFsrmActionCommand */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmActionCommand;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("12937789-e247-4917-9c20-f3ee9c7ee783")
    IFsrmActionCommand : public IFsrmAction
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExecutablePath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *executablePath) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ExecutablePath( 
            /* [in] */ __RPC__in BSTR executablePath) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Arguments( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *arguments) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Arguments( 
            /* [in] */ __RPC__in BSTR arguments) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Account( 
            /* [retval][out] */ __RPC__out FsrmAccountType *account) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Account( 
            /* [in] */ FsrmAccountType account) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_WorkingDirectory( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *workingDirectory) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_WorkingDirectory( 
            /* [in] */ __RPC__in BSTR workingDirectory) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MonitorCommand( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *monitorCommand) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MonitorCommand( 
            /* [in] */ VARIANT_BOOL monitorCommand) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_KillTimeOut( 
            /* [retval][out] */ __RPC__out long *minutes) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_KillTimeOut( 
            /* [in] */ long minutes) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LogResult( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *logResults) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LogResult( 
            /* [in] */ VARIANT_BOOL logResults) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmActionCommandVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmActionCommand * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmActionCommand * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmActionCommand * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmActionCommand * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmActionCommand * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmActionCommand * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmActionCommand * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IFsrmActionCommand * This,
            /* [retval][out] */ __RPC__out FSRM_OBJECT_ID *id);
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_ActionType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActionType )( 
            __RPC__in IFsrmActionCommand * This,
            /* [retval][out] */ __RPC__out FsrmActionType *actionType);
        
        DECLSPEC_XFGVIRT(IFsrmAction, get_RunLimitInterval)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RunLimitInterval )( 
            __RPC__in IFsrmActionCommand * This,
            /* [retval][out] */ __RPC__out long *minutes);
        
        DECLSPEC_XFGVIRT(IFsrmAction, put_RunLimitInterval)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RunLimitInterval )( 
            __RPC__in IFsrmActionCommand * This,
            /* [in] */ long minutes);
        
        DECLSPEC_XFGVIRT(IFsrmAction, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmActionCommand * This);
        
        DECLSPEC_XFGVIRT(IFsrmActionCommand, get_ExecutablePath)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExecutablePath )( 
            __RPC__in IFsrmActionCommand * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *executablePath);
        
        DECLSPEC_XFGVIRT(IFsrmActionCommand, put_ExecutablePath)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ExecutablePath )( 
            __RPC__in IFsrmActionCommand * This,
            /* [in] */ __RPC__in BSTR executablePath);
        
        DECLSPEC_XFGVIRT(IFsrmActionCommand, get_Arguments)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Arguments )( 
            __RPC__in IFsrmActionCommand * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *arguments);
        
        DECLSPEC_XFGVIRT(IFsrmActionCommand, put_Arguments)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Arguments )( 
            __RPC__in IFsrmActionCommand * This,
            /* [in] */ __RPC__in BSTR arguments);
        
        DECLSPEC_XFGVIRT(IFsrmActionCommand, get_Account)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Account )( 
            __RPC__in IFsrmActionCommand * This,
            /* [retval][out] */ __RPC__out FsrmAccountType *account);
        
        DECLSPEC_XFGVIRT(IFsrmActionCommand, put_Account)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Account )( 
            __RPC__in IFsrmActionCommand * This,
            /* [in] */ FsrmAccountType account);
        
        DECLSPEC_XFGVIRT(IFsrmActionCommand, get_WorkingDirectory)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WorkingDirectory )( 
            __RPC__in IFsrmActionCommand * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *workingDirectory);
        
        DECLSPEC_XFGVIRT(IFsrmActionCommand, put_WorkingDirectory)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_WorkingDirectory )( 
            __RPC__in IFsrmActionCommand * This,
            /* [in] */ __RPC__in BSTR workingDirectory);
        
        DECLSPEC_XFGVIRT(IFsrmActionCommand, get_MonitorCommand)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MonitorCommand )( 
            __RPC__in IFsrmActionCommand * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *monitorCommand);
        
        DECLSPEC_XFGVIRT(IFsrmActionCommand, put_MonitorCommand)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MonitorCommand )( 
            __RPC__in IFsrmActionCommand * This,
            /* [in] */ VARIANT_BOOL monitorCommand);
        
        DECLSPEC_XFGVIRT(IFsrmActionCommand, get_KillTimeOut)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_KillTimeOut )( 
            __RPC__in IFsrmActionCommand * This,
            /* [retval][out] */ __RPC__out long *minutes);
        
        DECLSPEC_XFGVIRT(IFsrmActionCommand, put_KillTimeOut)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_KillTimeOut )( 
            __RPC__in IFsrmActionCommand * This,
            /* [in] */ long minutes);
        
        DECLSPEC_XFGVIRT(IFsrmActionCommand, get_LogResult)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogResult )( 
            __RPC__in IFsrmActionCommand * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *logResults);
        
        DECLSPEC_XFGVIRT(IFsrmActionCommand, put_LogResult)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogResult )( 
            __RPC__in IFsrmActionCommand * This,
            /* [in] */ VARIANT_BOOL logResults);
        
        END_INTERFACE
    } IFsrmActionCommandVtbl;

    interface IFsrmActionCommand
    {
        CONST_VTBL struct IFsrmActionCommandVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmActionCommand_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmActionCommand_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmActionCommand_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmActionCommand_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmActionCommand_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmActionCommand_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmActionCommand_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmActionCommand_get_Id(This,id)	\
    ( (This)->lpVtbl -> get_Id(This,id) ) 

#define IFsrmActionCommand_get_ActionType(This,actionType)	\
    ( (This)->lpVtbl -> get_ActionType(This,actionType) ) 

#define IFsrmActionCommand_get_RunLimitInterval(This,minutes)	\
    ( (This)->lpVtbl -> get_RunLimitInterval(This,minutes) ) 

#define IFsrmActionCommand_put_RunLimitInterval(This,minutes)	\
    ( (This)->lpVtbl -> put_RunLimitInterval(This,minutes) ) 

#define IFsrmActionCommand_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 


#define IFsrmActionCommand_get_ExecutablePath(This,executablePath)	\
    ( (This)->lpVtbl -> get_ExecutablePath(This,executablePath) ) 

#define IFsrmActionCommand_put_ExecutablePath(This,executablePath)	\
    ( (This)->lpVtbl -> put_ExecutablePath(This,executablePath) ) 

#define IFsrmActionCommand_get_Arguments(This,arguments)	\
    ( (This)->lpVtbl -> get_Arguments(This,arguments) ) 

#define IFsrmActionCommand_put_Arguments(This,arguments)	\
    ( (This)->lpVtbl -> put_Arguments(This,arguments) ) 

#define IFsrmActionCommand_get_Account(This,account)	\
    ( (This)->lpVtbl -> get_Account(This,account) ) 

#define IFsrmActionCommand_put_Account(This,account)	\
    ( (This)->lpVtbl -> put_Account(This,account) ) 

#define IFsrmActionCommand_get_WorkingDirectory(This,workingDirectory)	\
    ( (This)->lpVtbl -> get_WorkingDirectory(This,workingDirectory) ) 

#define IFsrmActionCommand_put_WorkingDirectory(This,workingDirectory)	\
    ( (This)->lpVtbl -> put_WorkingDirectory(This,workingDirectory) ) 

#define IFsrmActionCommand_get_MonitorCommand(This,monitorCommand)	\
    ( (This)->lpVtbl -> get_MonitorCommand(This,monitorCommand) ) 

#define IFsrmActionCommand_put_MonitorCommand(This,monitorCommand)	\
    ( (This)->lpVtbl -> put_MonitorCommand(This,monitorCommand) ) 

#define IFsrmActionCommand_get_KillTimeOut(This,minutes)	\
    ( (This)->lpVtbl -> get_KillTimeOut(This,minutes) ) 

#define IFsrmActionCommand_put_KillTimeOut(This,minutes)	\
    ( (This)->lpVtbl -> put_KillTimeOut(This,minutes) ) 

#define IFsrmActionCommand_get_LogResult(This,logResults)	\
    ( (This)->lpVtbl -> get_LogResult(This,logResults) ) 

#define IFsrmActionCommand_put_LogResult(This,logResults)	\
    ( (This)->lpVtbl -> put_LogResult(This,logResults) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmActionCommand_INTERFACE_DEFINED__ */


#ifndef __IFsrmSetting_INTERFACE_DEFINED__
#define __IFsrmSetting_INTERFACE_DEFINED__

/* interface IFsrmSetting */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmSetting;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f411d4fd-14be-4260-8c40-03b7c95e608a")
    IFsrmSetting : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SmtpServer( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *smtpServer) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SmtpServer( 
            /* [in] */ __RPC__in BSTR smtpServer) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MailFrom( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailFrom) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MailFrom( 
            /* [in] */ __RPC__in BSTR mailFrom) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AdminEmail( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *adminEmail) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AdminEmail( 
            /* [in] */ __RPC__in BSTR adminEmail) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DisableCommandLine( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *disableCommandLine) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DisableCommandLine( 
            /* [in] */ VARIANT_BOOL disableCommandLine) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EnableScreeningAudit( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *enableScreeningAudit) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EnableScreeningAudit( 
            /* [in] */ VARIANT_BOOL enableScreeningAudit) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EmailTest( 
            /* [in] */ __RPC__in BSTR mailTo) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetActionRunLimitInterval( 
            /* [in] */ FsrmActionType actionType,
            /* [in] */ long delayTimeMinutes) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetActionRunLimitInterval( 
            /* [in] */ FsrmActionType actionType,
            /* [retval][out] */ __RPC__out long *delayTimeMinutes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmSettingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmSetting * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmSetting * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmSetting * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmSetting * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmSetting * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmSetting * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmSetting * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmSetting, get_SmtpServer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SmtpServer )( 
            __RPC__in IFsrmSetting * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *smtpServer);
        
        DECLSPEC_XFGVIRT(IFsrmSetting, put_SmtpServer)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SmtpServer )( 
            __RPC__in IFsrmSetting * This,
            /* [in] */ __RPC__in BSTR smtpServer);
        
        DECLSPEC_XFGVIRT(IFsrmSetting, get_MailFrom)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MailFrom )( 
            __RPC__in IFsrmSetting * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailFrom);
        
        DECLSPEC_XFGVIRT(IFsrmSetting, put_MailFrom)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MailFrom )( 
            __RPC__in IFsrmSetting * This,
            /* [in] */ __RPC__in BSTR mailFrom);
        
        DECLSPEC_XFGVIRT(IFsrmSetting, get_AdminEmail)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AdminEmail )( 
            __RPC__in IFsrmSetting * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *adminEmail);
        
        DECLSPEC_XFGVIRT(IFsrmSetting, put_AdminEmail)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AdminEmail )( 
            __RPC__in IFsrmSetting * This,
            /* [in] */ __RPC__in BSTR adminEmail);
        
        DECLSPEC_XFGVIRT(IFsrmSetting, get_DisableCommandLine)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisableCommandLine )( 
            __RPC__in IFsrmSetting * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *disableCommandLine);
        
        DECLSPEC_XFGVIRT(IFsrmSetting, put_DisableCommandLine)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DisableCommandLine )( 
            __RPC__in IFsrmSetting * This,
            /* [in] */ VARIANT_BOOL disableCommandLine);
        
        DECLSPEC_XFGVIRT(IFsrmSetting, get_EnableScreeningAudit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnableScreeningAudit )( 
            __RPC__in IFsrmSetting * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *enableScreeningAudit);
        
        DECLSPEC_XFGVIRT(IFsrmSetting, put_EnableScreeningAudit)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EnableScreeningAudit )( 
            __RPC__in IFsrmSetting * This,
            /* [in] */ VARIANT_BOOL enableScreeningAudit);
        
        DECLSPEC_XFGVIRT(IFsrmSetting, EmailTest)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EmailTest )( 
            __RPC__in IFsrmSetting * This,
            /* [in] */ __RPC__in BSTR mailTo);
        
        DECLSPEC_XFGVIRT(IFsrmSetting, SetActionRunLimitInterval)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetActionRunLimitInterval )( 
            __RPC__in IFsrmSetting * This,
            /* [in] */ FsrmActionType actionType,
            /* [in] */ long delayTimeMinutes);
        
        DECLSPEC_XFGVIRT(IFsrmSetting, GetActionRunLimitInterval)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetActionRunLimitInterval )( 
            __RPC__in IFsrmSetting * This,
            /* [in] */ FsrmActionType actionType,
            /* [retval][out] */ __RPC__out long *delayTimeMinutes);
        
        END_INTERFACE
    } IFsrmSettingVtbl;

    interface IFsrmSetting
    {
        CONST_VTBL struct IFsrmSettingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmSetting_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmSetting_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmSetting_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmSetting_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmSetting_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmSetting_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmSetting_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmSetting_get_SmtpServer(This,smtpServer)	\
    ( (This)->lpVtbl -> get_SmtpServer(This,smtpServer) ) 

#define IFsrmSetting_put_SmtpServer(This,smtpServer)	\
    ( (This)->lpVtbl -> put_SmtpServer(This,smtpServer) ) 

#define IFsrmSetting_get_MailFrom(This,mailFrom)	\
    ( (This)->lpVtbl -> get_MailFrom(This,mailFrom) ) 

#define IFsrmSetting_put_MailFrom(This,mailFrom)	\
    ( (This)->lpVtbl -> put_MailFrom(This,mailFrom) ) 

#define IFsrmSetting_get_AdminEmail(This,adminEmail)	\
    ( (This)->lpVtbl -> get_AdminEmail(This,adminEmail) ) 

#define IFsrmSetting_put_AdminEmail(This,adminEmail)	\
    ( (This)->lpVtbl -> put_AdminEmail(This,adminEmail) ) 

#define IFsrmSetting_get_DisableCommandLine(This,disableCommandLine)	\
    ( (This)->lpVtbl -> get_DisableCommandLine(This,disableCommandLine) ) 

#define IFsrmSetting_put_DisableCommandLine(This,disableCommandLine)	\
    ( (This)->lpVtbl -> put_DisableCommandLine(This,disableCommandLine) ) 

#define IFsrmSetting_get_EnableScreeningAudit(This,enableScreeningAudit)	\
    ( (This)->lpVtbl -> get_EnableScreeningAudit(This,enableScreeningAudit) ) 

#define IFsrmSetting_put_EnableScreeningAudit(This,enableScreeningAudit)	\
    ( (This)->lpVtbl -> put_EnableScreeningAudit(This,enableScreeningAudit) ) 

#define IFsrmSetting_EmailTest(This,mailTo)	\
    ( (This)->lpVtbl -> EmailTest(This,mailTo) ) 

#define IFsrmSetting_SetActionRunLimitInterval(This,actionType,delayTimeMinutes)	\
    ( (This)->lpVtbl -> SetActionRunLimitInterval(This,actionType,delayTimeMinutes) ) 

#define IFsrmSetting_GetActionRunLimitInterval(This,actionType,delayTimeMinutes)	\
    ( (This)->lpVtbl -> GetActionRunLimitInterval(This,actionType,delayTimeMinutes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmSetting_INTERFACE_DEFINED__ */


#ifndef __IFsrmPathMapper_INTERFACE_DEFINED__
#define __IFsrmPathMapper_INTERFACE_DEFINED__

/* interface IFsrmPathMapper */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmPathMapper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6f4dbfff-6920-4821-a6c3-b7e94c1fd60c")
    IFsrmPathMapper : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSharePathsForLocalPath( 
            /* [in] */ __RPC__in BSTR localPath,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *sharePaths) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmPathMapperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmPathMapper * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmPathMapper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmPathMapper * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmPathMapper * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmPathMapper * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmPathMapper * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmPathMapper * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmPathMapper, GetSharePathsForLocalPath)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSharePathsForLocalPath )( 
            __RPC__in IFsrmPathMapper * This,
            /* [in] */ __RPC__in BSTR localPath,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *sharePaths);
        
        END_INTERFACE
    } IFsrmPathMapperVtbl;

    interface IFsrmPathMapper
    {
        CONST_VTBL struct IFsrmPathMapperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmPathMapper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmPathMapper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmPathMapper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmPathMapper_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmPathMapper_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmPathMapper_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmPathMapper_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmPathMapper_GetSharePathsForLocalPath(This,localPath,sharePaths)	\
    ( (This)->lpVtbl -> GetSharePathsForLocalPath(This,localPath,sharePaths) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmPathMapper_INTERFACE_DEFINED__ */


#ifndef __IFsrmExportImport_INTERFACE_DEFINED__
#define __IFsrmExportImport_INTERFACE_DEFINED__

/* interface IFsrmExportImport */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmExportImport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("efcb0ab1-16c4-4a79-812c-725614c3306b")
    IFsrmExportImport : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExportFileGroups( 
            /* [in] */ __RPC__in BSTR filePath,
            /* [defaultvalue][in] */ __RPC__in VARIANT *fileGroupNamesSafeArray = 0,
            /* [defaultvalue][in] */ __RPC__in BSTR remoteHost = (BSTR)L"") = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ImportFileGroups( 
            /* [in] */ __RPC__in BSTR filePath,
            /* [defaultvalue][in] */ __RPC__in VARIANT *fileGroupNamesSafeArray,
            /* [defaultvalue][in] */ __RPC__in BSTR remoteHost,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **fileGroups) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExportFileScreenTemplates( 
            /* [in] */ __RPC__in BSTR filePath,
            /* [defaultvalue][in] */ __RPC__in VARIANT *templateNamesSafeArray = 0,
            /* [defaultvalue][in] */ __RPC__in BSTR remoteHost = (BSTR)L"") = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ImportFileScreenTemplates( 
            /* [in] */ __RPC__in BSTR filePath,
            /* [defaultvalue][in] */ __RPC__in VARIANT *templateNamesSafeArray,
            /* [defaultvalue][in] */ __RPC__in BSTR remoteHost,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **templates) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExportQuotaTemplates( 
            /* [in] */ __RPC__in BSTR filePath,
            /* [defaultvalue][in] */ __RPC__in VARIANT *templateNamesSafeArray = 0,
            /* [defaultvalue][in] */ __RPC__in BSTR remoteHost = (BSTR)L"") = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ImportQuotaTemplates( 
            /* [in] */ __RPC__in BSTR filePath,
            /* [defaultvalue][in] */ __RPC__in VARIANT *templateNamesSafeArray,
            /* [defaultvalue][in] */ __RPC__in BSTR remoteHost,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **templates) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmExportImportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmExportImport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmExportImport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmExportImport * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmExportImport * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmExportImport * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmExportImport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmExportImport * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmExportImport, ExportFileGroups)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExportFileGroups )( 
            __RPC__in IFsrmExportImport * This,
            /* [in] */ __RPC__in BSTR filePath,
            /* [defaultvalue][in] */ __RPC__in VARIANT *fileGroupNamesSafeArray,
            /* [defaultvalue][in] */ __RPC__in BSTR remoteHost);
        
        DECLSPEC_XFGVIRT(IFsrmExportImport, ImportFileGroups)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ImportFileGroups )( 
            __RPC__in IFsrmExportImport * This,
            /* [in] */ __RPC__in BSTR filePath,
            /* [defaultvalue][in] */ __RPC__in VARIANT *fileGroupNamesSafeArray,
            /* [defaultvalue][in] */ __RPC__in BSTR remoteHost,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **fileGroups);
        
        DECLSPEC_XFGVIRT(IFsrmExportImport, ExportFileScreenTemplates)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExportFileScreenTemplates )( 
            __RPC__in IFsrmExportImport * This,
            /* [in] */ __RPC__in BSTR filePath,
            /* [defaultvalue][in] */ __RPC__in VARIANT *templateNamesSafeArray,
            /* [defaultvalue][in] */ __RPC__in BSTR remoteHost);
        
        DECLSPEC_XFGVIRT(IFsrmExportImport, ImportFileScreenTemplates)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ImportFileScreenTemplates )( 
            __RPC__in IFsrmExportImport * This,
            /* [in] */ __RPC__in BSTR filePath,
            /* [defaultvalue][in] */ __RPC__in VARIANT *templateNamesSafeArray,
            /* [defaultvalue][in] */ __RPC__in BSTR remoteHost,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **templates);
        
        DECLSPEC_XFGVIRT(IFsrmExportImport, ExportQuotaTemplates)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExportQuotaTemplates )( 
            __RPC__in IFsrmExportImport * This,
            /* [in] */ __RPC__in BSTR filePath,
            /* [defaultvalue][in] */ __RPC__in VARIANT *templateNamesSafeArray,
            /* [defaultvalue][in] */ __RPC__in BSTR remoteHost);
        
        DECLSPEC_XFGVIRT(IFsrmExportImport, ImportQuotaTemplates)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ImportQuotaTemplates )( 
            __RPC__in IFsrmExportImport * This,
            /* [in] */ __RPC__in BSTR filePath,
            /* [defaultvalue][in] */ __RPC__in VARIANT *templateNamesSafeArray,
            /* [defaultvalue][in] */ __RPC__in BSTR remoteHost,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **templates);
        
        END_INTERFACE
    } IFsrmExportImportVtbl;

    interface IFsrmExportImport
    {
        CONST_VTBL struct IFsrmExportImportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmExportImport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmExportImport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmExportImport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmExportImport_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmExportImport_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmExportImport_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmExportImport_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmExportImport_ExportFileGroups(This,filePath,fileGroupNamesSafeArray,remoteHost)	\
    ( (This)->lpVtbl -> ExportFileGroups(This,filePath,fileGroupNamesSafeArray,remoteHost) ) 

#define IFsrmExportImport_ImportFileGroups(This,filePath,fileGroupNamesSafeArray,remoteHost,fileGroups)	\
    ( (This)->lpVtbl -> ImportFileGroups(This,filePath,fileGroupNamesSafeArray,remoteHost,fileGroups) ) 

#define IFsrmExportImport_ExportFileScreenTemplates(This,filePath,templateNamesSafeArray,remoteHost)	\
    ( (This)->lpVtbl -> ExportFileScreenTemplates(This,filePath,templateNamesSafeArray,remoteHost) ) 

#define IFsrmExportImport_ImportFileScreenTemplates(This,filePath,templateNamesSafeArray,remoteHost,templates)	\
    ( (This)->lpVtbl -> ImportFileScreenTemplates(This,filePath,templateNamesSafeArray,remoteHost,templates) ) 

#define IFsrmExportImport_ExportQuotaTemplates(This,filePath,templateNamesSafeArray,remoteHost)	\
    ( (This)->lpVtbl -> ExportQuotaTemplates(This,filePath,templateNamesSafeArray,remoteHost) ) 

#define IFsrmExportImport_ImportQuotaTemplates(This,filePath,templateNamesSafeArray,remoteHost,templates)	\
    ( (This)->lpVtbl -> ImportQuotaTemplates(This,filePath,templateNamesSafeArray,remoteHost,templates) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmExportImport_INTERFACE_DEFINED__ */


#ifndef __IFsrmDerivedObjectsResult_INTERFACE_DEFINED__
#define __IFsrmDerivedObjectsResult_INTERFACE_DEFINED__

/* interface IFsrmDerivedObjectsResult */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmDerivedObjectsResult;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("39322a2d-38ee-4d0d-8095-421a80849a82")
    IFsrmDerivedObjectsResult : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DerivedObjects( 
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **derivedObjects) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Results( 
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **results) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmDerivedObjectsResultVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmDerivedObjectsResult * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmDerivedObjectsResult * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmDerivedObjectsResult * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmDerivedObjectsResult * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmDerivedObjectsResult * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmDerivedObjectsResult * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmDerivedObjectsResult * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmDerivedObjectsResult, get_DerivedObjects)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DerivedObjects )( 
            __RPC__in IFsrmDerivedObjectsResult * This,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **derivedObjects);
        
        DECLSPEC_XFGVIRT(IFsrmDerivedObjectsResult, get_Results)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Results )( 
            __RPC__in IFsrmDerivedObjectsResult * This,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **results);
        
        END_INTERFACE
    } IFsrmDerivedObjectsResultVtbl;

    interface IFsrmDerivedObjectsResult
    {
        CONST_VTBL struct IFsrmDerivedObjectsResultVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmDerivedObjectsResult_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmDerivedObjectsResult_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmDerivedObjectsResult_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmDerivedObjectsResult_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmDerivedObjectsResult_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmDerivedObjectsResult_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmDerivedObjectsResult_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmDerivedObjectsResult_get_DerivedObjects(This,derivedObjects)	\
    ( (This)->lpVtbl -> get_DerivedObjects(This,derivedObjects) ) 

#define IFsrmDerivedObjectsResult_get_Results(This,results)	\
    ( (This)->lpVtbl -> get_Results(This,results) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmDerivedObjectsResult_INTERFACE_DEFINED__ */


#ifndef __IFsrmAccessDeniedRemediationClient_INTERFACE_DEFINED__
#define __IFsrmAccessDeniedRemediationClient_INTERFACE_DEFINED__

/* interface IFsrmAccessDeniedRemediationClient */
/* [unique][helpstring][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmAccessDeniedRemediationClient;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("40002314-590B-45A5-8E1B-8C05DA527E52")
    IFsrmAccessDeniedRemediationClient : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Show( 
            /* [in] */ ULONG_PTR parentWnd,
            /* [in] */ __RPC__in BSTR accessPath,
            /* [in] */ AdrClientErrorType errorType,
            /* [in] */ long flags,
            /* [defaultvalue][in] */ __RPC__in BSTR windowTitle,
            /* [defaultvalue][in] */ __RPC__in BSTR windowMessage,
            /* [retval][out] */ __RPC__out long *result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmAccessDeniedRemediationClientVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmAccessDeniedRemediationClient * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmAccessDeniedRemediationClient * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmAccessDeniedRemediationClient * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmAccessDeniedRemediationClient * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmAccessDeniedRemediationClient * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmAccessDeniedRemediationClient * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmAccessDeniedRemediationClient * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmAccessDeniedRemediationClient, Show)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Show )( 
            __RPC__in IFsrmAccessDeniedRemediationClient * This,
            /* [in] */ ULONG_PTR parentWnd,
            /* [in] */ __RPC__in BSTR accessPath,
            /* [in] */ AdrClientErrorType errorType,
            /* [in] */ long flags,
            /* [defaultvalue][in] */ __RPC__in BSTR windowTitle,
            /* [defaultvalue][in] */ __RPC__in BSTR windowMessage,
            /* [retval][out] */ __RPC__out long *result);
        
        END_INTERFACE
    } IFsrmAccessDeniedRemediationClientVtbl;

    interface IFsrmAccessDeniedRemediationClient
    {
        CONST_VTBL struct IFsrmAccessDeniedRemediationClientVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmAccessDeniedRemediationClient_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmAccessDeniedRemediationClient_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmAccessDeniedRemediationClient_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmAccessDeniedRemediationClient_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmAccessDeniedRemediationClient_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmAccessDeniedRemediationClient_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmAccessDeniedRemediationClient_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmAccessDeniedRemediationClient_Show(This,parentWnd,accessPath,errorType,flags,windowTitle,windowMessage,result)	\
    ( (This)->lpVtbl -> Show(This,parentWnd,accessPath,errorType,flags,windowTitle,windowMessage,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmAccessDeniedRemediationClient_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_fsrm_0000_0015 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_fsrm_0000_0015_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_fsrm_0000_0015_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


