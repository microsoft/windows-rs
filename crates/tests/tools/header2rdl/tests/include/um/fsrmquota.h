

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

#ifndef __fsrmquota_h__
#define __fsrmquota_h__

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

#ifndef __IFsrmQuotaBase_FWD_DEFINED__
#define __IFsrmQuotaBase_FWD_DEFINED__
typedef interface IFsrmQuotaBase IFsrmQuotaBase;

#endif 	/* __IFsrmQuotaBase_FWD_DEFINED__ */


#ifndef __IFsrmQuotaObject_FWD_DEFINED__
#define __IFsrmQuotaObject_FWD_DEFINED__
typedef interface IFsrmQuotaObject IFsrmQuotaObject;

#endif 	/* __IFsrmQuotaObject_FWD_DEFINED__ */


#ifndef __IFsrmQuota_FWD_DEFINED__
#define __IFsrmQuota_FWD_DEFINED__
typedef interface IFsrmQuota IFsrmQuota;

#endif 	/* __IFsrmQuota_FWD_DEFINED__ */


#ifndef __IFsrmAutoApplyQuota_FWD_DEFINED__
#define __IFsrmAutoApplyQuota_FWD_DEFINED__
typedef interface IFsrmAutoApplyQuota IFsrmAutoApplyQuota;

#endif 	/* __IFsrmAutoApplyQuota_FWD_DEFINED__ */


#ifndef __IFsrmQuotaManager_FWD_DEFINED__
#define __IFsrmQuotaManager_FWD_DEFINED__
typedef interface IFsrmQuotaManager IFsrmQuotaManager;

#endif 	/* __IFsrmQuotaManager_FWD_DEFINED__ */


#ifndef __IFsrmQuotaManagerEx_FWD_DEFINED__
#define __IFsrmQuotaManagerEx_FWD_DEFINED__
typedef interface IFsrmQuotaManagerEx IFsrmQuotaManagerEx;

#endif 	/* __IFsrmQuotaManagerEx_FWD_DEFINED__ */


#ifndef __IFsrmQuotaTemplate_FWD_DEFINED__
#define __IFsrmQuotaTemplate_FWD_DEFINED__
typedef interface IFsrmQuotaTemplate IFsrmQuotaTemplate;

#endif 	/* __IFsrmQuotaTemplate_FWD_DEFINED__ */


#ifndef __IFsrmQuotaTemplateImported_FWD_DEFINED__
#define __IFsrmQuotaTemplateImported_FWD_DEFINED__
typedef interface IFsrmQuotaTemplateImported IFsrmQuotaTemplateImported;

#endif 	/* __IFsrmQuotaTemplateImported_FWD_DEFINED__ */


#ifndef __IFsrmQuotaTemplateManager_FWD_DEFINED__
#define __IFsrmQuotaTemplateManager_FWD_DEFINED__
typedef interface IFsrmQuotaTemplateManager IFsrmQuotaTemplateManager;

#endif 	/* __IFsrmQuotaTemplateManager_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "fsrmenums.h"
#include "fsrm.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_fsrmquota_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)








#define	FSRM_DISPID_QUOTA_BASE	( ( FSRM_DISPID_FEATURE_QUOTA | 0x100000 )  )

#define	FSRM_DISPID_QUOTA_OBJECT	( ( FSRM_DISPID_QUOTA_BASE | 0x10000 )  )

#define	FSRM_DISPID_QUOTA	( ( FSRM_DISPID_QUOTA_OBJECT | 0x1000 )  )

#define	FSRM_DISPID_AUTOAPPLYQUOTA	( ( FSRM_DISPID_QUOTA_OBJECT | 0x2000 )  )

#define	FSRM_DISPID_QUOTA_TEMPLATE	( ( FSRM_DISPID_QUOTA_BASE | 0x20000 )  )

#define	FSRM_DISPID_QUOTA_TEMPLATE_IMPORTED	( ( FSRM_DISPID_QUOTA_TEMPLATE | 0x1000 )  )

#define	FSRM_DISPID_QUOTA_MANAGER	( ( FSRM_DISPID_FEATURE_QUOTA | 0x200000 )  )

#define	FSRM_DISPID_QUOTA_TEMPLATE_MANAGER	( ( FSRM_DISPID_FEATURE_QUOTA | 0x300000 )  )

#define	FSRM_DISPID_QUOTA_MANAGER_EX	( ( FSRM_DISPID_FEATURE_QUOTA | 0x400000 )  )

typedef long FSRM_QUOTA_THRESHOLD;

#define	FsrmMaxNumberThresholds	( 16 )

#define	FsrmMinThresholdValue	( 1 )

#define	FsrmMaxThresholdValue	( 250 )

#define	FsrmMinQuotaLimit	( 1024 )

#define	FsrmMaxExcludeFolders	( 32 )



extern RPC_IF_HANDLE __MIDL_itf_fsrmquota_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_fsrmquota_0000_0000_v0_0_s_ifspec;

#ifndef __IFsrmQuotaBase_INTERFACE_DEFINED__
#define __IFsrmQuotaBase_INTERFACE_DEFINED__

/* interface IFsrmQuotaBase */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmQuotaBase;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1568a795-3924-4118-b74b-68d8f0fa5daf")
    IFsrmQuotaBase : public IFsrmObject
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_QuotaLimit( 
            /* [retval][out] */ __RPC__out VARIANT *quotaLimit) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_QuotaLimit( 
            /* [in] */ VARIANT quotaLimit) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_QuotaFlags( 
            /* [retval][out] */ __RPC__out long *quotaFlags) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_QuotaFlags( 
            /* [in] */ long quotaFlags) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Thresholds( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *thresholds) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddThreshold( 
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteThreshold( 
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ModifyThreshold( 
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [in] */ FSRM_QUOTA_THRESHOLD newThreshold) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateThresholdAction( 
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [in] */ FsrmActionType actionType,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmAction **action) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumThresholdActions( 
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **actions) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmQuotaBaseVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmQuotaBase * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmQuotaBase * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmQuotaBase * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmQuotaBase * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmQuotaBase * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmQuotaBase * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmQuotaBase * This,
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
            __RPC__in IFsrmQuotaBase * This,
            /* [retval][out] */ __RPC__out FSRM_OBJECT_ID *id);
        
        DECLSPEC_XFGVIRT(IFsrmObject, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IFsrmQuotaBase * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IFsrmQuotaBase * This,
            /* [in] */ __RPC__in BSTR description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmQuotaBase * This);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Commit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IFsrmQuotaBase * This);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_QuotaLimit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuotaLimit )( 
            __RPC__in IFsrmQuotaBase * This,
            /* [retval][out] */ __RPC__out VARIANT *quotaLimit);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, put_QuotaLimit)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_QuotaLimit )( 
            __RPC__in IFsrmQuotaBase * This,
            /* [in] */ VARIANT quotaLimit);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_QuotaFlags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuotaFlags )( 
            __RPC__in IFsrmQuotaBase * This,
            /* [retval][out] */ __RPC__out long *quotaFlags);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, put_QuotaFlags)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_QuotaFlags )( 
            __RPC__in IFsrmQuotaBase * This,
            /* [in] */ long quotaFlags);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_Thresholds)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Thresholds )( 
            __RPC__in IFsrmQuotaBase * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *thresholds);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, AddThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddThreshold )( 
            __RPC__in IFsrmQuotaBase * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, DeleteThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteThreshold )( 
            __RPC__in IFsrmQuotaBase * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, ModifyThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ModifyThreshold )( 
            __RPC__in IFsrmQuotaBase * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [in] */ FSRM_QUOTA_THRESHOLD newThreshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, CreateThresholdAction)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateThresholdAction )( 
            __RPC__in IFsrmQuotaBase * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [in] */ FsrmActionType actionType,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmAction **action);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, EnumThresholdActions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumThresholdActions )( 
            __RPC__in IFsrmQuotaBase * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **actions);
        
        END_INTERFACE
    } IFsrmQuotaBaseVtbl;

    interface IFsrmQuotaBase
    {
        CONST_VTBL struct IFsrmQuotaBaseVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmQuotaBase_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmQuotaBase_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmQuotaBase_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmQuotaBase_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmQuotaBase_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmQuotaBase_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmQuotaBase_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmQuotaBase_get_Id(This,id)	\
    ( (This)->lpVtbl -> get_Id(This,id) ) 

#define IFsrmQuotaBase_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#define IFsrmQuotaBase_put_Description(This,description)	\
    ( (This)->lpVtbl -> put_Description(This,description) ) 

#define IFsrmQuotaBase_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IFsrmQuotaBase_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 


#define IFsrmQuotaBase_get_QuotaLimit(This,quotaLimit)	\
    ( (This)->lpVtbl -> get_QuotaLimit(This,quotaLimit) ) 

#define IFsrmQuotaBase_put_QuotaLimit(This,quotaLimit)	\
    ( (This)->lpVtbl -> put_QuotaLimit(This,quotaLimit) ) 

#define IFsrmQuotaBase_get_QuotaFlags(This,quotaFlags)	\
    ( (This)->lpVtbl -> get_QuotaFlags(This,quotaFlags) ) 

#define IFsrmQuotaBase_put_QuotaFlags(This,quotaFlags)	\
    ( (This)->lpVtbl -> put_QuotaFlags(This,quotaFlags) ) 

#define IFsrmQuotaBase_get_Thresholds(This,thresholds)	\
    ( (This)->lpVtbl -> get_Thresholds(This,thresholds) ) 

#define IFsrmQuotaBase_AddThreshold(This,threshold)	\
    ( (This)->lpVtbl -> AddThreshold(This,threshold) ) 

#define IFsrmQuotaBase_DeleteThreshold(This,threshold)	\
    ( (This)->lpVtbl -> DeleteThreshold(This,threshold) ) 

#define IFsrmQuotaBase_ModifyThreshold(This,threshold,newThreshold)	\
    ( (This)->lpVtbl -> ModifyThreshold(This,threshold,newThreshold) ) 

#define IFsrmQuotaBase_CreateThresholdAction(This,threshold,actionType,action)	\
    ( (This)->lpVtbl -> CreateThresholdAction(This,threshold,actionType,action) ) 

#define IFsrmQuotaBase_EnumThresholdActions(This,threshold,actions)	\
    ( (This)->lpVtbl -> EnumThresholdActions(This,threshold,actions) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmQuotaBase_INTERFACE_DEFINED__ */


#ifndef __IFsrmQuotaObject_INTERFACE_DEFINED__
#define __IFsrmQuotaObject_INTERFACE_DEFINED__

/* interface IFsrmQuotaObject */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmQuotaObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("42dc3511-61d5-48ae-b6dc-59fc00c0a8d6")
    IFsrmQuotaObject : public IFsrmQuotaBase
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UserSid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *userSid) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UserAccount( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *userAccount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SourceTemplateName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *quotaTemplateName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MatchesSourceTemplate( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *matches) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ApplyTemplate( 
            /* [in] */ __RPC__in BSTR quotaTemplateName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmQuotaObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmQuotaObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmQuotaObject * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmQuotaObject * This,
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
            __RPC__in IFsrmQuotaObject * This,
            /* [retval][out] */ __RPC__out FSRM_OBJECT_ID *id);
        
        DECLSPEC_XFGVIRT(IFsrmObject, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [in] */ __RPC__in BSTR description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmQuotaObject * This);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Commit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IFsrmQuotaObject * This);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_QuotaLimit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuotaLimit )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [retval][out] */ __RPC__out VARIANT *quotaLimit);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, put_QuotaLimit)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_QuotaLimit )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [in] */ VARIANT quotaLimit);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_QuotaFlags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuotaFlags )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [retval][out] */ __RPC__out long *quotaFlags);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, put_QuotaFlags)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_QuotaFlags )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [in] */ long quotaFlags);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_Thresholds)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Thresholds )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *thresholds);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, AddThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddThreshold )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, DeleteThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteThreshold )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, ModifyThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ModifyThreshold )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [in] */ FSRM_QUOTA_THRESHOLD newThreshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, CreateThresholdAction)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateThresholdAction )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [in] */ FsrmActionType actionType,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmAction **action);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, EnumThresholdActions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumThresholdActions )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **actions);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, get_Path)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, get_UserSid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserSid )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *userSid);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, get_UserAccount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserAccount )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *userAccount);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, get_SourceTemplateName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SourceTemplateName )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *quotaTemplateName);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, get_MatchesSourceTemplate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MatchesSourceTemplate )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *matches);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, ApplyTemplate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ApplyTemplate )( 
            __RPC__in IFsrmQuotaObject * This,
            /* [in] */ __RPC__in BSTR quotaTemplateName);
        
        END_INTERFACE
    } IFsrmQuotaObjectVtbl;

    interface IFsrmQuotaObject
    {
        CONST_VTBL struct IFsrmQuotaObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmQuotaObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmQuotaObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmQuotaObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmQuotaObject_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmQuotaObject_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmQuotaObject_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmQuotaObject_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmQuotaObject_get_Id(This,id)	\
    ( (This)->lpVtbl -> get_Id(This,id) ) 

#define IFsrmQuotaObject_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#define IFsrmQuotaObject_put_Description(This,description)	\
    ( (This)->lpVtbl -> put_Description(This,description) ) 

#define IFsrmQuotaObject_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IFsrmQuotaObject_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 


#define IFsrmQuotaObject_get_QuotaLimit(This,quotaLimit)	\
    ( (This)->lpVtbl -> get_QuotaLimit(This,quotaLimit) ) 

#define IFsrmQuotaObject_put_QuotaLimit(This,quotaLimit)	\
    ( (This)->lpVtbl -> put_QuotaLimit(This,quotaLimit) ) 

#define IFsrmQuotaObject_get_QuotaFlags(This,quotaFlags)	\
    ( (This)->lpVtbl -> get_QuotaFlags(This,quotaFlags) ) 

#define IFsrmQuotaObject_put_QuotaFlags(This,quotaFlags)	\
    ( (This)->lpVtbl -> put_QuotaFlags(This,quotaFlags) ) 

#define IFsrmQuotaObject_get_Thresholds(This,thresholds)	\
    ( (This)->lpVtbl -> get_Thresholds(This,thresholds) ) 

#define IFsrmQuotaObject_AddThreshold(This,threshold)	\
    ( (This)->lpVtbl -> AddThreshold(This,threshold) ) 

#define IFsrmQuotaObject_DeleteThreshold(This,threshold)	\
    ( (This)->lpVtbl -> DeleteThreshold(This,threshold) ) 

#define IFsrmQuotaObject_ModifyThreshold(This,threshold,newThreshold)	\
    ( (This)->lpVtbl -> ModifyThreshold(This,threshold,newThreshold) ) 

#define IFsrmQuotaObject_CreateThresholdAction(This,threshold,actionType,action)	\
    ( (This)->lpVtbl -> CreateThresholdAction(This,threshold,actionType,action) ) 

#define IFsrmQuotaObject_EnumThresholdActions(This,threshold,actions)	\
    ( (This)->lpVtbl -> EnumThresholdActions(This,threshold,actions) ) 


#define IFsrmQuotaObject_get_Path(This,path)	\
    ( (This)->lpVtbl -> get_Path(This,path) ) 

#define IFsrmQuotaObject_get_UserSid(This,userSid)	\
    ( (This)->lpVtbl -> get_UserSid(This,userSid) ) 

#define IFsrmQuotaObject_get_UserAccount(This,userAccount)	\
    ( (This)->lpVtbl -> get_UserAccount(This,userAccount) ) 

#define IFsrmQuotaObject_get_SourceTemplateName(This,quotaTemplateName)	\
    ( (This)->lpVtbl -> get_SourceTemplateName(This,quotaTemplateName) ) 

#define IFsrmQuotaObject_get_MatchesSourceTemplate(This,matches)	\
    ( (This)->lpVtbl -> get_MatchesSourceTemplate(This,matches) ) 

#define IFsrmQuotaObject_ApplyTemplate(This,quotaTemplateName)	\
    ( (This)->lpVtbl -> ApplyTemplate(This,quotaTemplateName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmQuotaObject_INTERFACE_DEFINED__ */


#ifndef __IFsrmQuota_INTERFACE_DEFINED__
#define __IFsrmQuota_INTERFACE_DEFINED__

/* interface IFsrmQuota */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmQuota;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("377f739d-9647-4b8e-97d2-5ffce6d759cd")
    IFsrmQuota : public IFsrmQuotaObject
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_QuotaUsed( 
            /* [retval][out] */ __RPC__out VARIANT *used) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_QuotaPeakUsage( 
            /* [retval][out] */ __RPC__out VARIANT *peakUsage) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_QuotaPeakUsageTime( 
            /* [retval][out] */ __RPC__out DATE *peakUsageDateTime) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ResetPeakUsage( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RefreshUsageProperties( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmQuotaVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmQuota * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmQuota * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmQuota * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmQuota * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmQuota * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmQuota * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmQuota * This,
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
            __RPC__in IFsrmQuota * This,
            /* [retval][out] */ __RPC__out FSRM_OBJECT_ID *id);
        
        DECLSPEC_XFGVIRT(IFsrmObject, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IFsrmQuota * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IFsrmQuota * This,
            /* [in] */ __RPC__in BSTR description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmQuota * This);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Commit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IFsrmQuota * This);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_QuotaLimit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuotaLimit )( 
            __RPC__in IFsrmQuota * This,
            /* [retval][out] */ __RPC__out VARIANT *quotaLimit);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, put_QuotaLimit)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_QuotaLimit )( 
            __RPC__in IFsrmQuota * This,
            /* [in] */ VARIANT quotaLimit);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_QuotaFlags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuotaFlags )( 
            __RPC__in IFsrmQuota * This,
            /* [retval][out] */ __RPC__out long *quotaFlags);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, put_QuotaFlags)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_QuotaFlags )( 
            __RPC__in IFsrmQuota * This,
            /* [in] */ long quotaFlags);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_Thresholds)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Thresholds )( 
            __RPC__in IFsrmQuota * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *thresholds);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, AddThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddThreshold )( 
            __RPC__in IFsrmQuota * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, DeleteThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteThreshold )( 
            __RPC__in IFsrmQuota * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, ModifyThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ModifyThreshold )( 
            __RPC__in IFsrmQuota * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [in] */ FSRM_QUOTA_THRESHOLD newThreshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, CreateThresholdAction)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateThresholdAction )( 
            __RPC__in IFsrmQuota * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [in] */ FsrmActionType actionType,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmAction **action);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, EnumThresholdActions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumThresholdActions )( 
            __RPC__in IFsrmQuota * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **actions);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, get_Path)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IFsrmQuota * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, get_UserSid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserSid )( 
            __RPC__in IFsrmQuota * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *userSid);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, get_UserAccount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserAccount )( 
            __RPC__in IFsrmQuota * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *userAccount);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, get_SourceTemplateName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SourceTemplateName )( 
            __RPC__in IFsrmQuota * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *quotaTemplateName);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, get_MatchesSourceTemplate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MatchesSourceTemplate )( 
            __RPC__in IFsrmQuota * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *matches);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, ApplyTemplate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ApplyTemplate )( 
            __RPC__in IFsrmQuota * This,
            /* [in] */ __RPC__in BSTR quotaTemplateName);
        
        DECLSPEC_XFGVIRT(IFsrmQuota, get_QuotaUsed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuotaUsed )( 
            __RPC__in IFsrmQuota * This,
            /* [retval][out] */ __RPC__out VARIANT *used);
        
        DECLSPEC_XFGVIRT(IFsrmQuota, get_QuotaPeakUsage)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuotaPeakUsage )( 
            __RPC__in IFsrmQuota * This,
            /* [retval][out] */ __RPC__out VARIANT *peakUsage);
        
        DECLSPEC_XFGVIRT(IFsrmQuota, get_QuotaPeakUsageTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuotaPeakUsageTime )( 
            __RPC__in IFsrmQuota * This,
            /* [retval][out] */ __RPC__out DATE *peakUsageDateTime);
        
        DECLSPEC_XFGVIRT(IFsrmQuota, ResetPeakUsage)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ResetPeakUsage )( 
            __RPC__in IFsrmQuota * This);
        
        DECLSPEC_XFGVIRT(IFsrmQuota, RefreshUsageProperties)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RefreshUsageProperties )( 
            __RPC__in IFsrmQuota * This);
        
        END_INTERFACE
    } IFsrmQuotaVtbl;

    interface IFsrmQuota
    {
        CONST_VTBL struct IFsrmQuotaVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmQuota_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmQuota_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmQuota_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmQuota_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmQuota_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmQuota_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmQuota_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmQuota_get_Id(This,id)	\
    ( (This)->lpVtbl -> get_Id(This,id) ) 

#define IFsrmQuota_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#define IFsrmQuota_put_Description(This,description)	\
    ( (This)->lpVtbl -> put_Description(This,description) ) 

#define IFsrmQuota_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IFsrmQuota_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 


#define IFsrmQuota_get_QuotaLimit(This,quotaLimit)	\
    ( (This)->lpVtbl -> get_QuotaLimit(This,quotaLimit) ) 

#define IFsrmQuota_put_QuotaLimit(This,quotaLimit)	\
    ( (This)->lpVtbl -> put_QuotaLimit(This,quotaLimit) ) 

#define IFsrmQuota_get_QuotaFlags(This,quotaFlags)	\
    ( (This)->lpVtbl -> get_QuotaFlags(This,quotaFlags) ) 

#define IFsrmQuota_put_QuotaFlags(This,quotaFlags)	\
    ( (This)->lpVtbl -> put_QuotaFlags(This,quotaFlags) ) 

#define IFsrmQuota_get_Thresholds(This,thresholds)	\
    ( (This)->lpVtbl -> get_Thresholds(This,thresholds) ) 

#define IFsrmQuota_AddThreshold(This,threshold)	\
    ( (This)->lpVtbl -> AddThreshold(This,threshold) ) 

#define IFsrmQuota_DeleteThreshold(This,threshold)	\
    ( (This)->lpVtbl -> DeleteThreshold(This,threshold) ) 

#define IFsrmQuota_ModifyThreshold(This,threshold,newThreshold)	\
    ( (This)->lpVtbl -> ModifyThreshold(This,threshold,newThreshold) ) 

#define IFsrmQuota_CreateThresholdAction(This,threshold,actionType,action)	\
    ( (This)->lpVtbl -> CreateThresholdAction(This,threshold,actionType,action) ) 

#define IFsrmQuota_EnumThresholdActions(This,threshold,actions)	\
    ( (This)->lpVtbl -> EnumThresholdActions(This,threshold,actions) ) 


#define IFsrmQuota_get_Path(This,path)	\
    ( (This)->lpVtbl -> get_Path(This,path) ) 

#define IFsrmQuota_get_UserSid(This,userSid)	\
    ( (This)->lpVtbl -> get_UserSid(This,userSid) ) 

#define IFsrmQuota_get_UserAccount(This,userAccount)	\
    ( (This)->lpVtbl -> get_UserAccount(This,userAccount) ) 

#define IFsrmQuota_get_SourceTemplateName(This,quotaTemplateName)	\
    ( (This)->lpVtbl -> get_SourceTemplateName(This,quotaTemplateName) ) 

#define IFsrmQuota_get_MatchesSourceTemplate(This,matches)	\
    ( (This)->lpVtbl -> get_MatchesSourceTemplate(This,matches) ) 

#define IFsrmQuota_ApplyTemplate(This,quotaTemplateName)	\
    ( (This)->lpVtbl -> ApplyTemplate(This,quotaTemplateName) ) 


#define IFsrmQuota_get_QuotaUsed(This,used)	\
    ( (This)->lpVtbl -> get_QuotaUsed(This,used) ) 

#define IFsrmQuota_get_QuotaPeakUsage(This,peakUsage)	\
    ( (This)->lpVtbl -> get_QuotaPeakUsage(This,peakUsage) ) 

#define IFsrmQuota_get_QuotaPeakUsageTime(This,peakUsageDateTime)	\
    ( (This)->lpVtbl -> get_QuotaPeakUsageTime(This,peakUsageDateTime) ) 

#define IFsrmQuota_ResetPeakUsage(This)	\
    ( (This)->lpVtbl -> ResetPeakUsage(This) ) 

#define IFsrmQuota_RefreshUsageProperties(This)	\
    ( (This)->lpVtbl -> RefreshUsageProperties(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmQuota_INTERFACE_DEFINED__ */


#ifndef __IFsrmAutoApplyQuota_INTERFACE_DEFINED__
#define __IFsrmAutoApplyQuota_INTERFACE_DEFINED__

/* interface IFsrmAutoApplyQuota */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmAutoApplyQuota;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f82e5729-6aba-4740-bfc7-c7f58f75fb7b")
    IFsrmAutoApplyQuota : public IFsrmQuotaObject
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExcludeFolders( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *folders) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ExcludeFolders( 
            /* [in] */ __RPC__in SAFEARRAY * folders) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CommitAndUpdateDerived( 
            /* [in] */ FsrmCommitOptions commitOptions,
            /* [in] */ FsrmTemplateApplyOptions applyOptions,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmDerivedObjectsResult **derivedObjectsResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmAutoApplyQuotaVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmAutoApplyQuota * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmAutoApplyQuota * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmAutoApplyQuota * This,
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
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [retval][out] */ __RPC__out FSRM_OBJECT_ID *id);
        
        DECLSPEC_XFGVIRT(IFsrmObject, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [in] */ __RPC__in BSTR description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmAutoApplyQuota * This);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Commit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IFsrmAutoApplyQuota * This);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_QuotaLimit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuotaLimit )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [retval][out] */ __RPC__out VARIANT *quotaLimit);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, put_QuotaLimit)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_QuotaLimit )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [in] */ VARIANT quotaLimit);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_QuotaFlags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuotaFlags )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [retval][out] */ __RPC__out long *quotaFlags);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, put_QuotaFlags)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_QuotaFlags )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [in] */ long quotaFlags);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_Thresholds)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Thresholds )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *thresholds);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, AddThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddThreshold )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, DeleteThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteThreshold )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, ModifyThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ModifyThreshold )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [in] */ FSRM_QUOTA_THRESHOLD newThreshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, CreateThresholdAction)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateThresholdAction )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [in] */ FsrmActionType actionType,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmAction **action);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, EnumThresholdActions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumThresholdActions )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **actions);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, get_Path)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, get_UserSid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserSid )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *userSid);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, get_UserAccount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserAccount )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *userAccount);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, get_SourceTemplateName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SourceTemplateName )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *quotaTemplateName);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, get_MatchesSourceTemplate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MatchesSourceTemplate )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *matches);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaObject, ApplyTemplate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ApplyTemplate )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [in] */ __RPC__in BSTR quotaTemplateName);
        
        DECLSPEC_XFGVIRT(IFsrmAutoApplyQuota, get_ExcludeFolders)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExcludeFolders )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *folders);
        
        DECLSPEC_XFGVIRT(IFsrmAutoApplyQuota, put_ExcludeFolders)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ExcludeFolders )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [in] */ __RPC__in SAFEARRAY * folders);
        
        DECLSPEC_XFGVIRT(IFsrmAutoApplyQuota, CommitAndUpdateDerived)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CommitAndUpdateDerived )( 
            __RPC__in IFsrmAutoApplyQuota * This,
            /* [in] */ FsrmCommitOptions commitOptions,
            /* [in] */ FsrmTemplateApplyOptions applyOptions,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmDerivedObjectsResult **derivedObjectsResult);
        
        END_INTERFACE
    } IFsrmAutoApplyQuotaVtbl;

    interface IFsrmAutoApplyQuota
    {
        CONST_VTBL struct IFsrmAutoApplyQuotaVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmAutoApplyQuota_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmAutoApplyQuota_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmAutoApplyQuota_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmAutoApplyQuota_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmAutoApplyQuota_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmAutoApplyQuota_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmAutoApplyQuota_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmAutoApplyQuota_get_Id(This,id)	\
    ( (This)->lpVtbl -> get_Id(This,id) ) 

#define IFsrmAutoApplyQuota_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#define IFsrmAutoApplyQuota_put_Description(This,description)	\
    ( (This)->lpVtbl -> put_Description(This,description) ) 

#define IFsrmAutoApplyQuota_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IFsrmAutoApplyQuota_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 


#define IFsrmAutoApplyQuota_get_QuotaLimit(This,quotaLimit)	\
    ( (This)->lpVtbl -> get_QuotaLimit(This,quotaLimit) ) 

#define IFsrmAutoApplyQuota_put_QuotaLimit(This,quotaLimit)	\
    ( (This)->lpVtbl -> put_QuotaLimit(This,quotaLimit) ) 

#define IFsrmAutoApplyQuota_get_QuotaFlags(This,quotaFlags)	\
    ( (This)->lpVtbl -> get_QuotaFlags(This,quotaFlags) ) 

#define IFsrmAutoApplyQuota_put_QuotaFlags(This,quotaFlags)	\
    ( (This)->lpVtbl -> put_QuotaFlags(This,quotaFlags) ) 

#define IFsrmAutoApplyQuota_get_Thresholds(This,thresholds)	\
    ( (This)->lpVtbl -> get_Thresholds(This,thresholds) ) 

#define IFsrmAutoApplyQuota_AddThreshold(This,threshold)	\
    ( (This)->lpVtbl -> AddThreshold(This,threshold) ) 

#define IFsrmAutoApplyQuota_DeleteThreshold(This,threshold)	\
    ( (This)->lpVtbl -> DeleteThreshold(This,threshold) ) 

#define IFsrmAutoApplyQuota_ModifyThreshold(This,threshold,newThreshold)	\
    ( (This)->lpVtbl -> ModifyThreshold(This,threshold,newThreshold) ) 

#define IFsrmAutoApplyQuota_CreateThresholdAction(This,threshold,actionType,action)	\
    ( (This)->lpVtbl -> CreateThresholdAction(This,threshold,actionType,action) ) 

#define IFsrmAutoApplyQuota_EnumThresholdActions(This,threshold,actions)	\
    ( (This)->lpVtbl -> EnumThresholdActions(This,threshold,actions) ) 


#define IFsrmAutoApplyQuota_get_Path(This,path)	\
    ( (This)->lpVtbl -> get_Path(This,path) ) 

#define IFsrmAutoApplyQuota_get_UserSid(This,userSid)	\
    ( (This)->lpVtbl -> get_UserSid(This,userSid) ) 

#define IFsrmAutoApplyQuota_get_UserAccount(This,userAccount)	\
    ( (This)->lpVtbl -> get_UserAccount(This,userAccount) ) 

#define IFsrmAutoApplyQuota_get_SourceTemplateName(This,quotaTemplateName)	\
    ( (This)->lpVtbl -> get_SourceTemplateName(This,quotaTemplateName) ) 

#define IFsrmAutoApplyQuota_get_MatchesSourceTemplate(This,matches)	\
    ( (This)->lpVtbl -> get_MatchesSourceTemplate(This,matches) ) 

#define IFsrmAutoApplyQuota_ApplyTemplate(This,quotaTemplateName)	\
    ( (This)->lpVtbl -> ApplyTemplate(This,quotaTemplateName) ) 


#define IFsrmAutoApplyQuota_get_ExcludeFolders(This,folders)	\
    ( (This)->lpVtbl -> get_ExcludeFolders(This,folders) ) 

#define IFsrmAutoApplyQuota_put_ExcludeFolders(This,folders)	\
    ( (This)->lpVtbl -> put_ExcludeFolders(This,folders) ) 

#define IFsrmAutoApplyQuota_CommitAndUpdateDerived(This,commitOptions,applyOptions,derivedObjectsResult)	\
    ( (This)->lpVtbl -> CommitAndUpdateDerived(This,commitOptions,applyOptions,derivedObjectsResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmAutoApplyQuota_INTERFACE_DEFINED__ */


#ifndef __IFsrmQuotaManager_INTERFACE_DEFINED__
#define __IFsrmQuotaManager_INTERFACE_DEFINED__

/* interface IFsrmQuotaManager */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmQuotaManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8bb68c7d-19d8-4ffb-809e-be4fc1734014")
    IFsrmQuotaManager : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ActionVariables( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *variables) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ActionVariableDescriptions( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *descriptions) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateQuota( 
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmQuota **quota) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateAutoApplyQuota( 
            /* [in] */ __RPC__in BSTR quotaTemplateName,
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmAutoApplyQuota **quota) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetQuota( 
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmQuota **quota) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetAutoApplyQuota( 
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmAutoApplyQuota **quota) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetRestrictiveQuota( 
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmQuota **quota) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumQuotas( 
            /* [defaultvalue][in] */ __RPC__in BSTR path,
            /* [defaultvalue][in] */ FsrmEnumOptions options,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **quotas) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumAutoApplyQuotas( 
            /* [defaultvalue][in] */ __RPC__in BSTR path,
            /* [defaultvalue][in] */ FsrmEnumOptions options,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **quotas) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumEffectiveQuotas( 
            /* [in] */ __RPC__in BSTR path,
            /* [defaultvalue][in] */ FsrmEnumOptions options,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **quotas) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Scan( 
            /* [in] */ __RPC__in BSTR strPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateQuotaCollection( 
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **collection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmQuotaManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmQuotaManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmQuotaManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmQuotaManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmQuotaManager * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmQuotaManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmQuotaManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmQuotaManager * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, get_ActionVariables)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActionVariables )( 
            __RPC__in IFsrmQuotaManager * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *variables);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, get_ActionVariableDescriptions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActionVariableDescriptions )( 
            __RPC__in IFsrmQuotaManager * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *descriptions);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, CreateQuota)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateQuota )( 
            __RPC__in IFsrmQuotaManager * This,
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmQuota **quota);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, CreateAutoApplyQuota)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateAutoApplyQuota )( 
            __RPC__in IFsrmQuotaManager * This,
            /* [in] */ __RPC__in BSTR quotaTemplateName,
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmAutoApplyQuota **quota);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, GetQuota)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetQuota )( 
            __RPC__in IFsrmQuotaManager * This,
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmQuota **quota);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, GetAutoApplyQuota)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetAutoApplyQuota )( 
            __RPC__in IFsrmQuotaManager * This,
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmAutoApplyQuota **quota);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, GetRestrictiveQuota)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRestrictiveQuota )( 
            __RPC__in IFsrmQuotaManager * This,
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmQuota **quota);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, EnumQuotas)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumQuotas )( 
            __RPC__in IFsrmQuotaManager * This,
            /* [defaultvalue][in] */ __RPC__in BSTR path,
            /* [defaultvalue][in] */ FsrmEnumOptions options,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **quotas);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, EnumAutoApplyQuotas)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumAutoApplyQuotas )( 
            __RPC__in IFsrmQuotaManager * This,
            /* [defaultvalue][in] */ __RPC__in BSTR path,
            /* [defaultvalue][in] */ FsrmEnumOptions options,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **quotas);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, EnumEffectiveQuotas)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumEffectiveQuotas )( 
            __RPC__in IFsrmQuotaManager * This,
            /* [in] */ __RPC__in BSTR path,
            /* [defaultvalue][in] */ FsrmEnumOptions options,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **quotas);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, Scan)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Scan )( 
            __RPC__in IFsrmQuotaManager * This,
            /* [in] */ __RPC__in BSTR strPath);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, CreateQuotaCollection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateQuotaCollection )( 
            __RPC__in IFsrmQuotaManager * This,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **collection);
        
        END_INTERFACE
    } IFsrmQuotaManagerVtbl;

    interface IFsrmQuotaManager
    {
        CONST_VTBL struct IFsrmQuotaManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmQuotaManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmQuotaManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmQuotaManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmQuotaManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmQuotaManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmQuotaManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmQuotaManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmQuotaManager_get_ActionVariables(This,variables)	\
    ( (This)->lpVtbl -> get_ActionVariables(This,variables) ) 

#define IFsrmQuotaManager_get_ActionVariableDescriptions(This,descriptions)	\
    ( (This)->lpVtbl -> get_ActionVariableDescriptions(This,descriptions) ) 

#define IFsrmQuotaManager_CreateQuota(This,path,quota)	\
    ( (This)->lpVtbl -> CreateQuota(This,path,quota) ) 

#define IFsrmQuotaManager_CreateAutoApplyQuota(This,quotaTemplateName,path,quota)	\
    ( (This)->lpVtbl -> CreateAutoApplyQuota(This,quotaTemplateName,path,quota) ) 

#define IFsrmQuotaManager_GetQuota(This,path,quota)	\
    ( (This)->lpVtbl -> GetQuota(This,path,quota) ) 

#define IFsrmQuotaManager_GetAutoApplyQuota(This,path,quota)	\
    ( (This)->lpVtbl -> GetAutoApplyQuota(This,path,quota) ) 

#define IFsrmQuotaManager_GetRestrictiveQuota(This,path,quota)	\
    ( (This)->lpVtbl -> GetRestrictiveQuota(This,path,quota) ) 

#define IFsrmQuotaManager_EnumQuotas(This,path,options,quotas)	\
    ( (This)->lpVtbl -> EnumQuotas(This,path,options,quotas) ) 

#define IFsrmQuotaManager_EnumAutoApplyQuotas(This,path,options,quotas)	\
    ( (This)->lpVtbl -> EnumAutoApplyQuotas(This,path,options,quotas) ) 

#define IFsrmQuotaManager_EnumEffectiveQuotas(This,path,options,quotas)	\
    ( (This)->lpVtbl -> EnumEffectiveQuotas(This,path,options,quotas) ) 

#define IFsrmQuotaManager_Scan(This,strPath)	\
    ( (This)->lpVtbl -> Scan(This,strPath) ) 

#define IFsrmQuotaManager_CreateQuotaCollection(This,collection)	\
    ( (This)->lpVtbl -> CreateQuotaCollection(This,collection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmQuotaManager_INTERFACE_DEFINED__ */


#ifndef __IFsrmQuotaManagerEx_INTERFACE_DEFINED__
#define __IFsrmQuotaManagerEx_INTERFACE_DEFINED__

/* interface IFsrmQuotaManagerEx */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmQuotaManagerEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4846cb01-d430-494f-abb4-b1054999fb09")
    IFsrmQuotaManagerEx : public IFsrmQuotaManager
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsAffectedByQuota( 
            /* [in] */ __RPC__in BSTR path,
            /* [defaultvalue][in] */ FsrmEnumOptions options,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *affected) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmQuotaManagerExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmQuotaManagerEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmQuotaManagerEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmQuotaManagerEx * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmQuotaManagerEx * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmQuotaManagerEx * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmQuotaManagerEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmQuotaManagerEx * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, get_ActionVariables)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActionVariables )( 
            __RPC__in IFsrmQuotaManagerEx * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *variables);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, get_ActionVariableDescriptions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActionVariableDescriptions )( 
            __RPC__in IFsrmQuotaManagerEx * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *descriptions);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, CreateQuota)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateQuota )( 
            __RPC__in IFsrmQuotaManagerEx * This,
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmQuota **quota);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, CreateAutoApplyQuota)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateAutoApplyQuota )( 
            __RPC__in IFsrmQuotaManagerEx * This,
            /* [in] */ __RPC__in BSTR quotaTemplateName,
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmAutoApplyQuota **quota);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, GetQuota)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetQuota )( 
            __RPC__in IFsrmQuotaManagerEx * This,
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmQuota **quota);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, GetAutoApplyQuota)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetAutoApplyQuota )( 
            __RPC__in IFsrmQuotaManagerEx * This,
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmAutoApplyQuota **quota);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, GetRestrictiveQuota)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRestrictiveQuota )( 
            __RPC__in IFsrmQuotaManagerEx * This,
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmQuota **quota);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, EnumQuotas)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumQuotas )( 
            __RPC__in IFsrmQuotaManagerEx * This,
            /* [defaultvalue][in] */ __RPC__in BSTR path,
            /* [defaultvalue][in] */ FsrmEnumOptions options,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **quotas);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, EnumAutoApplyQuotas)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumAutoApplyQuotas )( 
            __RPC__in IFsrmQuotaManagerEx * This,
            /* [defaultvalue][in] */ __RPC__in BSTR path,
            /* [defaultvalue][in] */ FsrmEnumOptions options,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **quotas);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, EnumEffectiveQuotas)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumEffectiveQuotas )( 
            __RPC__in IFsrmQuotaManagerEx * This,
            /* [in] */ __RPC__in BSTR path,
            /* [defaultvalue][in] */ FsrmEnumOptions options,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **quotas);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, Scan)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Scan )( 
            __RPC__in IFsrmQuotaManagerEx * This,
            /* [in] */ __RPC__in BSTR strPath);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManager, CreateQuotaCollection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateQuotaCollection )( 
            __RPC__in IFsrmQuotaManagerEx * This,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **collection);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaManagerEx, IsAffectedByQuota)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsAffectedByQuota )( 
            __RPC__in IFsrmQuotaManagerEx * This,
            /* [in] */ __RPC__in BSTR path,
            /* [defaultvalue][in] */ FsrmEnumOptions options,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *affected);
        
        END_INTERFACE
    } IFsrmQuotaManagerExVtbl;

    interface IFsrmQuotaManagerEx
    {
        CONST_VTBL struct IFsrmQuotaManagerExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmQuotaManagerEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmQuotaManagerEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmQuotaManagerEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmQuotaManagerEx_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmQuotaManagerEx_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmQuotaManagerEx_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmQuotaManagerEx_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmQuotaManagerEx_get_ActionVariables(This,variables)	\
    ( (This)->lpVtbl -> get_ActionVariables(This,variables) ) 

#define IFsrmQuotaManagerEx_get_ActionVariableDescriptions(This,descriptions)	\
    ( (This)->lpVtbl -> get_ActionVariableDescriptions(This,descriptions) ) 

#define IFsrmQuotaManagerEx_CreateQuota(This,path,quota)	\
    ( (This)->lpVtbl -> CreateQuota(This,path,quota) ) 

#define IFsrmQuotaManagerEx_CreateAutoApplyQuota(This,quotaTemplateName,path,quota)	\
    ( (This)->lpVtbl -> CreateAutoApplyQuota(This,quotaTemplateName,path,quota) ) 

#define IFsrmQuotaManagerEx_GetQuota(This,path,quota)	\
    ( (This)->lpVtbl -> GetQuota(This,path,quota) ) 

#define IFsrmQuotaManagerEx_GetAutoApplyQuota(This,path,quota)	\
    ( (This)->lpVtbl -> GetAutoApplyQuota(This,path,quota) ) 

#define IFsrmQuotaManagerEx_GetRestrictiveQuota(This,path,quota)	\
    ( (This)->lpVtbl -> GetRestrictiveQuota(This,path,quota) ) 

#define IFsrmQuotaManagerEx_EnumQuotas(This,path,options,quotas)	\
    ( (This)->lpVtbl -> EnumQuotas(This,path,options,quotas) ) 

#define IFsrmQuotaManagerEx_EnumAutoApplyQuotas(This,path,options,quotas)	\
    ( (This)->lpVtbl -> EnumAutoApplyQuotas(This,path,options,quotas) ) 

#define IFsrmQuotaManagerEx_EnumEffectiveQuotas(This,path,options,quotas)	\
    ( (This)->lpVtbl -> EnumEffectiveQuotas(This,path,options,quotas) ) 

#define IFsrmQuotaManagerEx_Scan(This,strPath)	\
    ( (This)->lpVtbl -> Scan(This,strPath) ) 

#define IFsrmQuotaManagerEx_CreateQuotaCollection(This,collection)	\
    ( (This)->lpVtbl -> CreateQuotaCollection(This,collection) ) 


#define IFsrmQuotaManagerEx_IsAffectedByQuota(This,path,options,affected)	\
    ( (This)->lpVtbl -> IsAffectedByQuota(This,path,options,affected) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmQuotaManagerEx_INTERFACE_DEFINED__ */


#ifndef __IFsrmQuotaTemplate_INTERFACE_DEFINED__
#define __IFsrmQuotaTemplate_INTERFACE_DEFINED__

/* interface IFsrmQuotaTemplate */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmQuotaTemplate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a2efab31-295e-46bb-b976-e86d58b52e8b")
    IFsrmQuotaTemplate : public IFsrmQuotaBase
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR name) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CopyTemplate( 
            /* [in] */ __RPC__in BSTR quotaTemplateName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CommitAndUpdateDerived( 
            /* [in] */ FsrmCommitOptions commitOptions,
            /* [in] */ FsrmTemplateApplyOptions applyOptions,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmDerivedObjectsResult **derivedObjectsResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmQuotaTemplateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmQuotaTemplate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmQuotaTemplate * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmQuotaTemplate * This,
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
            __RPC__in IFsrmQuotaTemplate * This,
            /* [retval][out] */ __RPC__out FSRM_OBJECT_ID *id);
        
        DECLSPEC_XFGVIRT(IFsrmObject, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [in] */ __RPC__in BSTR description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmQuotaTemplate * This);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Commit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IFsrmQuotaTemplate * This);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_QuotaLimit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuotaLimit )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [retval][out] */ __RPC__out VARIANT *quotaLimit);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, put_QuotaLimit)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_QuotaLimit )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [in] */ VARIANT quotaLimit);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_QuotaFlags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuotaFlags )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [retval][out] */ __RPC__out long *quotaFlags);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, put_QuotaFlags)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_QuotaFlags )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [in] */ long quotaFlags);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_Thresholds)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Thresholds )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *thresholds);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, AddThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddThreshold )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, DeleteThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteThreshold )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, ModifyThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ModifyThreshold )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [in] */ FSRM_QUOTA_THRESHOLD newThreshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, CreateThresholdAction)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateThresholdAction )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [in] */ FsrmActionType actionType,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmAction **action);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, EnumThresholdActions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumThresholdActions )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **actions);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaTemplate, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaTemplate, put_Name)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaTemplate, CopyTemplate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CopyTemplate )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [in] */ __RPC__in BSTR quotaTemplateName);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaTemplate, CommitAndUpdateDerived)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CommitAndUpdateDerived )( 
            __RPC__in IFsrmQuotaTemplate * This,
            /* [in] */ FsrmCommitOptions commitOptions,
            /* [in] */ FsrmTemplateApplyOptions applyOptions,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmDerivedObjectsResult **derivedObjectsResult);
        
        END_INTERFACE
    } IFsrmQuotaTemplateVtbl;

    interface IFsrmQuotaTemplate
    {
        CONST_VTBL struct IFsrmQuotaTemplateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmQuotaTemplate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmQuotaTemplate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmQuotaTemplate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmQuotaTemplate_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmQuotaTemplate_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmQuotaTemplate_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmQuotaTemplate_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmQuotaTemplate_get_Id(This,id)	\
    ( (This)->lpVtbl -> get_Id(This,id) ) 

#define IFsrmQuotaTemplate_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#define IFsrmQuotaTemplate_put_Description(This,description)	\
    ( (This)->lpVtbl -> put_Description(This,description) ) 

#define IFsrmQuotaTemplate_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IFsrmQuotaTemplate_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 


#define IFsrmQuotaTemplate_get_QuotaLimit(This,quotaLimit)	\
    ( (This)->lpVtbl -> get_QuotaLimit(This,quotaLimit) ) 

#define IFsrmQuotaTemplate_put_QuotaLimit(This,quotaLimit)	\
    ( (This)->lpVtbl -> put_QuotaLimit(This,quotaLimit) ) 

#define IFsrmQuotaTemplate_get_QuotaFlags(This,quotaFlags)	\
    ( (This)->lpVtbl -> get_QuotaFlags(This,quotaFlags) ) 

#define IFsrmQuotaTemplate_put_QuotaFlags(This,quotaFlags)	\
    ( (This)->lpVtbl -> put_QuotaFlags(This,quotaFlags) ) 

#define IFsrmQuotaTemplate_get_Thresholds(This,thresholds)	\
    ( (This)->lpVtbl -> get_Thresholds(This,thresholds) ) 

#define IFsrmQuotaTemplate_AddThreshold(This,threshold)	\
    ( (This)->lpVtbl -> AddThreshold(This,threshold) ) 

#define IFsrmQuotaTemplate_DeleteThreshold(This,threshold)	\
    ( (This)->lpVtbl -> DeleteThreshold(This,threshold) ) 

#define IFsrmQuotaTemplate_ModifyThreshold(This,threshold,newThreshold)	\
    ( (This)->lpVtbl -> ModifyThreshold(This,threshold,newThreshold) ) 

#define IFsrmQuotaTemplate_CreateThresholdAction(This,threshold,actionType,action)	\
    ( (This)->lpVtbl -> CreateThresholdAction(This,threshold,actionType,action) ) 

#define IFsrmQuotaTemplate_EnumThresholdActions(This,threshold,actions)	\
    ( (This)->lpVtbl -> EnumThresholdActions(This,threshold,actions) ) 


#define IFsrmQuotaTemplate_get_Name(This,name)	\
    ( (This)->lpVtbl -> get_Name(This,name) ) 

#define IFsrmQuotaTemplate_put_Name(This,name)	\
    ( (This)->lpVtbl -> put_Name(This,name) ) 

#define IFsrmQuotaTemplate_CopyTemplate(This,quotaTemplateName)	\
    ( (This)->lpVtbl -> CopyTemplate(This,quotaTemplateName) ) 

#define IFsrmQuotaTemplate_CommitAndUpdateDerived(This,commitOptions,applyOptions,derivedObjectsResult)	\
    ( (This)->lpVtbl -> CommitAndUpdateDerived(This,commitOptions,applyOptions,derivedObjectsResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmQuotaTemplate_INTERFACE_DEFINED__ */


#ifndef __IFsrmQuotaTemplateImported_INTERFACE_DEFINED__
#define __IFsrmQuotaTemplateImported_INTERFACE_DEFINED__

/* interface IFsrmQuotaTemplateImported */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmQuotaTemplateImported;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9a2bf113-a329-44cc-809a-5c00fce8da40")
    IFsrmQuotaTemplateImported : public IFsrmQuotaTemplate
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OverwriteOnCommit( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *overwrite) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_OverwriteOnCommit( 
            /* [in] */ VARIANT_BOOL overwrite) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmQuotaTemplateImportedVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmQuotaTemplateImported * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmQuotaTemplateImported * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmQuotaTemplateImported * This,
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
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [retval][out] */ __RPC__out FSRM_OBJECT_ID *id);
        
        DECLSPEC_XFGVIRT(IFsrmObject, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [in] */ __RPC__in BSTR description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmQuotaTemplateImported * This);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Commit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IFsrmQuotaTemplateImported * This);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_QuotaLimit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuotaLimit )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [retval][out] */ __RPC__out VARIANT *quotaLimit);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, put_QuotaLimit)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_QuotaLimit )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [in] */ VARIANT quotaLimit);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_QuotaFlags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuotaFlags )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [retval][out] */ __RPC__out long *quotaFlags);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, put_QuotaFlags)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_QuotaFlags )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [in] */ long quotaFlags);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, get_Thresholds)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Thresholds )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *thresholds);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, AddThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddThreshold )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, DeleteThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteThreshold )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, ModifyThreshold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ModifyThreshold )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [in] */ FSRM_QUOTA_THRESHOLD newThreshold);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, CreateThresholdAction)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateThresholdAction )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [in] */ FsrmActionType actionType,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmAction **action);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaBase, EnumThresholdActions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumThresholdActions )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [in] */ FSRM_QUOTA_THRESHOLD threshold,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **actions);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaTemplate, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaTemplate, put_Name)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaTemplate, CopyTemplate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CopyTemplate )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [in] */ __RPC__in BSTR quotaTemplateName);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaTemplate, CommitAndUpdateDerived)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CommitAndUpdateDerived )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [in] */ FsrmCommitOptions commitOptions,
            /* [in] */ FsrmTemplateApplyOptions applyOptions,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmDerivedObjectsResult **derivedObjectsResult);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaTemplateImported, get_OverwriteOnCommit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OverwriteOnCommit )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *overwrite);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaTemplateImported, put_OverwriteOnCommit)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OverwriteOnCommit )( 
            __RPC__in IFsrmQuotaTemplateImported * This,
            /* [in] */ VARIANT_BOOL overwrite);
        
        END_INTERFACE
    } IFsrmQuotaTemplateImportedVtbl;

    interface IFsrmQuotaTemplateImported
    {
        CONST_VTBL struct IFsrmQuotaTemplateImportedVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmQuotaTemplateImported_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmQuotaTemplateImported_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmQuotaTemplateImported_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmQuotaTemplateImported_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmQuotaTemplateImported_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmQuotaTemplateImported_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmQuotaTemplateImported_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmQuotaTemplateImported_get_Id(This,id)	\
    ( (This)->lpVtbl -> get_Id(This,id) ) 

#define IFsrmQuotaTemplateImported_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#define IFsrmQuotaTemplateImported_put_Description(This,description)	\
    ( (This)->lpVtbl -> put_Description(This,description) ) 

#define IFsrmQuotaTemplateImported_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IFsrmQuotaTemplateImported_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 


#define IFsrmQuotaTemplateImported_get_QuotaLimit(This,quotaLimit)	\
    ( (This)->lpVtbl -> get_QuotaLimit(This,quotaLimit) ) 

#define IFsrmQuotaTemplateImported_put_QuotaLimit(This,quotaLimit)	\
    ( (This)->lpVtbl -> put_QuotaLimit(This,quotaLimit) ) 

#define IFsrmQuotaTemplateImported_get_QuotaFlags(This,quotaFlags)	\
    ( (This)->lpVtbl -> get_QuotaFlags(This,quotaFlags) ) 

#define IFsrmQuotaTemplateImported_put_QuotaFlags(This,quotaFlags)	\
    ( (This)->lpVtbl -> put_QuotaFlags(This,quotaFlags) ) 

#define IFsrmQuotaTemplateImported_get_Thresholds(This,thresholds)	\
    ( (This)->lpVtbl -> get_Thresholds(This,thresholds) ) 

#define IFsrmQuotaTemplateImported_AddThreshold(This,threshold)	\
    ( (This)->lpVtbl -> AddThreshold(This,threshold) ) 

#define IFsrmQuotaTemplateImported_DeleteThreshold(This,threshold)	\
    ( (This)->lpVtbl -> DeleteThreshold(This,threshold) ) 

#define IFsrmQuotaTemplateImported_ModifyThreshold(This,threshold,newThreshold)	\
    ( (This)->lpVtbl -> ModifyThreshold(This,threshold,newThreshold) ) 

#define IFsrmQuotaTemplateImported_CreateThresholdAction(This,threshold,actionType,action)	\
    ( (This)->lpVtbl -> CreateThresholdAction(This,threshold,actionType,action) ) 

#define IFsrmQuotaTemplateImported_EnumThresholdActions(This,threshold,actions)	\
    ( (This)->lpVtbl -> EnumThresholdActions(This,threshold,actions) ) 


#define IFsrmQuotaTemplateImported_get_Name(This,name)	\
    ( (This)->lpVtbl -> get_Name(This,name) ) 

#define IFsrmQuotaTemplateImported_put_Name(This,name)	\
    ( (This)->lpVtbl -> put_Name(This,name) ) 

#define IFsrmQuotaTemplateImported_CopyTemplate(This,quotaTemplateName)	\
    ( (This)->lpVtbl -> CopyTemplate(This,quotaTemplateName) ) 

#define IFsrmQuotaTemplateImported_CommitAndUpdateDerived(This,commitOptions,applyOptions,derivedObjectsResult)	\
    ( (This)->lpVtbl -> CommitAndUpdateDerived(This,commitOptions,applyOptions,derivedObjectsResult) ) 


#define IFsrmQuotaTemplateImported_get_OverwriteOnCommit(This,overwrite)	\
    ( (This)->lpVtbl -> get_OverwriteOnCommit(This,overwrite) ) 

#define IFsrmQuotaTemplateImported_put_OverwriteOnCommit(This,overwrite)	\
    ( (This)->lpVtbl -> put_OverwriteOnCommit(This,overwrite) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmQuotaTemplateImported_INTERFACE_DEFINED__ */


#ifndef __IFsrmQuotaTemplateManager_INTERFACE_DEFINED__
#define __IFsrmQuotaTemplateManager_INTERFACE_DEFINED__

/* interface IFsrmQuotaTemplateManager */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFsrmQuotaTemplateManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4173ac41-172d-4d52-963c-fdc7e415f717")
    IFsrmQuotaTemplateManager : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateTemplate( 
            /* [retval][out] */ __RPC__deref_out_opt IFsrmQuotaTemplate **quotaTemplate) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetTemplate( 
            /* [defaultvalue][in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmQuotaTemplate **quotaTemplate) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumTemplates( 
            /* [defaultvalue][in] */ FsrmEnumOptions options,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **quotaTemplates) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExportTemplates( 
            /* [defaultvalue][in] */ __RPC__in VARIANT *quotaTemplateNamesArray,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *serializedQuotaTemplates) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ImportTemplates( 
            /* [in] */ __RPC__in BSTR serializedQuotaTemplates,
            /* [defaultvalue][in] */ __RPC__in VARIANT *quotaTemplateNamesArray,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **quotaTemplates) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmQuotaTemplateManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmQuotaTemplateManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmQuotaTemplateManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmQuotaTemplateManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmQuotaTemplateManager * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmQuotaTemplateManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmQuotaTemplateManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmQuotaTemplateManager * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmQuotaTemplateManager, CreateTemplate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTemplate )( 
            __RPC__in IFsrmQuotaTemplateManager * This,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmQuotaTemplate **quotaTemplate);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaTemplateManager, GetTemplate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetTemplate )( 
            __RPC__in IFsrmQuotaTemplateManager * This,
            /* [defaultvalue][in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmQuotaTemplate **quotaTemplate);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaTemplateManager, EnumTemplates)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumTemplates )( 
            __RPC__in IFsrmQuotaTemplateManager * This,
            /* [defaultvalue][in] */ FsrmEnumOptions options,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **quotaTemplates);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaTemplateManager, ExportTemplates)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExportTemplates )( 
            __RPC__in IFsrmQuotaTemplateManager * This,
            /* [defaultvalue][in] */ __RPC__in VARIANT *quotaTemplateNamesArray,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *serializedQuotaTemplates);
        
        DECLSPEC_XFGVIRT(IFsrmQuotaTemplateManager, ImportTemplates)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ImportTemplates )( 
            __RPC__in IFsrmQuotaTemplateManager * This,
            /* [in] */ __RPC__in BSTR serializedQuotaTemplates,
            /* [defaultvalue][in] */ __RPC__in VARIANT *quotaTemplateNamesArray,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCommittableCollection **quotaTemplates);
        
        END_INTERFACE
    } IFsrmQuotaTemplateManagerVtbl;

    interface IFsrmQuotaTemplateManager
    {
        CONST_VTBL struct IFsrmQuotaTemplateManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmQuotaTemplateManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmQuotaTemplateManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmQuotaTemplateManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmQuotaTemplateManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmQuotaTemplateManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmQuotaTemplateManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmQuotaTemplateManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmQuotaTemplateManager_CreateTemplate(This,quotaTemplate)	\
    ( (This)->lpVtbl -> CreateTemplate(This,quotaTemplate) ) 

#define IFsrmQuotaTemplateManager_GetTemplate(This,name,quotaTemplate)	\
    ( (This)->lpVtbl -> GetTemplate(This,name,quotaTemplate) ) 

#define IFsrmQuotaTemplateManager_EnumTemplates(This,options,quotaTemplates)	\
    ( (This)->lpVtbl -> EnumTemplates(This,options,quotaTemplates) ) 

#define IFsrmQuotaTemplateManager_ExportTemplates(This,quotaTemplateNamesArray,serializedQuotaTemplates)	\
    ( (This)->lpVtbl -> ExportTemplates(This,quotaTemplateNamesArray,serializedQuotaTemplates) ) 

#define IFsrmQuotaTemplateManager_ImportTemplates(This,serializedQuotaTemplates,quotaTemplateNamesArray,quotaTemplates)	\
    ( (This)->lpVtbl -> ImportTemplates(This,serializedQuotaTemplates,quotaTemplateNamesArray,quotaTemplates) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmQuotaTemplateManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_fsrmquota_0000_0009 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_fsrmquota_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_fsrmquota_0000_0009_v0_0_s_ifspec;

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


