

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

#ifndef __bitscfg_h__
#define __bitscfg_h__

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

#ifndef __IBITSExtensionSetup_FWD_DEFINED__
#define __IBITSExtensionSetup_FWD_DEFINED__
typedef interface IBITSExtensionSetup IBITSExtensionSetup;

#endif 	/* __IBITSExtensionSetup_FWD_DEFINED__ */


#ifndef __IBITSExtensionSetupFactory_FWD_DEFINED__
#define __IBITSExtensionSetupFactory_FWD_DEFINED__
typedef interface IBITSExtensionSetupFactory IBITSExtensionSetupFactory;

#endif 	/* __IBITSExtensionSetupFactory_FWD_DEFINED__ */


#ifndef __BITSExtensionSetupFactory_FWD_DEFINED__
#define __BITSExtensionSetupFactory_FWD_DEFINED__

#ifdef __cplusplus
typedef class BITSExtensionSetupFactory BITSExtensionSetupFactory;
#else
typedef struct BITSExtensionSetupFactory BITSExtensionSetupFactory;
#endif /* __cplusplus */

#endif 	/* __BITSExtensionSetupFactory_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "mstask.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_bitscfg_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_bitscfg_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bitscfg_0000_0000_v0_0_s_ifspec;

#ifndef __IBITSExtensionSetup_INTERFACE_DEFINED__
#define __IBITSExtensionSetup_INTERFACE_DEFINED__

/* interface IBITSExtensionSetup */
/* [object][dual][uuid] */ 


EXTERN_C const IID IID_IBITSExtensionSetup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("29cfbbf7-09e4-4b97-b0bc-f2287e3d8eb3")
    IBITSExtensionSetup : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EnableBITSUploads( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DisableBITSUploads( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetCleanupTaskName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pTaskName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetCleanupTask( 
            /* [in] */ __RPC__in REFIID riid,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBITSExtensionSetupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBITSExtensionSetup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBITSExtensionSetup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBITSExtensionSetup * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IBITSExtensionSetup * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IBITSExtensionSetup * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IBITSExtensionSetup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IBITSExtensionSetup * This,
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
        
        DECLSPEC_XFGVIRT(IBITSExtensionSetup, EnableBITSUploads)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnableBITSUploads )( 
            __RPC__in IBITSExtensionSetup * This);
        
        DECLSPEC_XFGVIRT(IBITSExtensionSetup, DisableBITSUploads)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DisableBITSUploads )( 
            __RPC__in IBITSExtensionSetup * This);
        
        DECLSPEC_XFGVIRT(IBITSExtensionSetup, GetCleanupTaskName)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetCleanupTaskName )( 
            __RPC__in IBITSExtensionSetup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pTaskName);
        
        DECLSPEC_XFGVIRT(IBITSExtensionSetup, GetCleanupTask)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetCleanupTask )( 
            __RPC__in IBITSExtensionSetup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        END_INTERFACE
    } IBITSExtensionSetupVtbl;

    interface IBITSExtensionSetup
    {
        CONST_VTBL struct IBITSExtensionSetupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBITSExtensionSetup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBITSExtensionSetup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBITSExtensionSetup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBITSExtensionSetup_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IBITSExtensionSetup_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IBITSExtensionSetup_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IBITSExtensionSetup_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IBITSExtensionSetup_EnableBITSUploads(This)	\
    ( (This)->lpVtbl -> EnableBITSUploads(This) ) 

#define IBITSExtensionSetup_DisableBITSUploads(This)	\
    ( (This)->lpVtbl -> DisableBITSUploads(This) ) 

#define IBITSExtensionSetup_GetCleanupTaskName(This,pTaskName)	\
    ( (This)->lpVtbl -> GetCleanupTaskName(This,pTaskName) ) 

#define IBITSExtensionSetup_GetCleanupTask(This,riid,ppUnk)	\
    ( (This)->lpVtbl -> GetCleanupTask(This,riid,ppUnk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBITSExtensionSetup_INTERFACE_DEFINED__ */


#ifndef __IBITSExtensionSetupFactory_INTERFACE_DEFINED__
#define __IBITSExtensionSetupFactory_INTERFACE_DEFINED__

/* interface IBITSExtensionSetupFactory */
/* [object][dual][uuid] */ 


EXTERN_C const IID IID_IBITSExtensionSetupFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d5d2d542-5503-4e64-8b48-72ef91a32ee1")
    IBITSExtensionSetupFactory : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetObject( 
            /* [in] */ __RPC__in BSTR Path,
            /* [retval][out] */ __RPC__deref_out_opt IBITSExtensionSetup **ppExtensionSetup) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBITSExtensionSetupFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBITSExtensionSetupFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBITSExtensionSetupFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBITSExtensionSetupFactory * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IBITSExtensionSetupFactory * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IBITSExtensionSetupFactory * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IBITSExtensionSetupFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IBITSExtensionSetupFactory * This,
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
        
        DECLSPEC_XFGVIRT(IBITSExtensionSetupFactory, GetObject)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetObject )( 
            __RPC__in IBITSExtensionSetupFactory * This,
            /* [in] */ __RPC__in BSTR Path,
            /* [retval][out] */ __RPC__deref_out_opt IBITSExtensionSetup **ppExtensionSetup);
        
        END_INTERFACE
    } IBITSExtensionSetupFactoryVtbl;

    interface IBITSExtensionSetupFactory
    {
        CONST_VTBL struct IBITSExtensionSetupFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBITSExtensionSetupFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBITSExtensionSetupFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBITSExtensionSetupFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBITSExtensionSetupFactory_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IBITSExtensionSetupFactory_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IBITSExtensionSetupFactory_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IBITSExtensionSetupFactory_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IBITSExtensionSetupFactory_GetObject(This,Path,ppExtensionSetup)	\
    ( (This)->lpVtbl -> GetObject(This,Path,ppExtensionSetup) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBITSExtensionSetupFactory_INTERFACE_DEFINED__ */



#ifndef __BITSExtensionSetup_LIBRARY_DEFINED__
#define __BITSExtensionSetup_LIBRARY_DEFINED__

/* library BITSExtensionSetup */
/* [version][helpstring][uuid] */ 


EXTERN_C const IID LIBID_BITSExtensionSetup;

EXTERN_C const CLSID CLSID_BITSExtensionSetupFactory;

#ifdef __cplusplus

class DECLSPEC_UUID("efbbab68-7286-4783-94bf-9461d8b7e7e9")
BITSExtensionSetupFactory;
#endif
#endif /* __BITSExtensionSetup_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_bitscfg_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_bitscfg_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bitscfg_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


