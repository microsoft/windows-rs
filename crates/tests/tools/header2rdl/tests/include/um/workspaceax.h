

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

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __workspaceax_h__
#define __workspaceax_h__

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

#ifndef __IWorkspaceResTypeRegistry_FWD_DEFINED__
#define __IWorkspaceResTypeRegistry_FWD_DEFINED__
typedef interface IWorkspaceResTypeRegistry IWorkspaceResTypeRegistry;

#endif 	/* __IWorkspaceResTypeRegistry_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_workspaceax_0000_0000 */
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


extern RPC_IF_HANDLE __MIDL_itf_workspaceax_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_workspaceax_0000_0000_v0_0_s_ifspec;

#ifndef __IWorkspaceResTypeRegistry_INTERFACE_DEFINED__
#define __IWorkspaceResTypeRegistry_INTERFACE_DEFINED__

/* interface IWorkspaceResTypeRegistry */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IWorkspaceResTypeRegistry;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1D428C79-6E2E-4351-A361-C0401A03A0BA")
    IWorkspaceResTypeRegistry : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddResourceType( 
            /* [in] */ VARIANT_BOOL fMachineWide,
            /* [in] */ __RPC__in BSTR bstrFileExtension,
            /* [in] */ __RPC__in BSTR bstrLauncher) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteResourceType( 
            /* [in] */ VARIANT_BOOL fMachineWide,
            /* [in] */ __RPC__in BSTR bstrFileExtension) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetRegisteredFileExtensions( 
            /* [in] */ VARIANT_BOOL fMachineWide,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *psaFileExtensions) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetResourceTypeInfo( 
            /* [in] */ VARIANT_BOOL fMachineWide,
            /* [in] */ __RPC__in BSTR bstrFileExtension,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLauncher) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ModifyResourceType( 
            /* [in] */ VARIANT_BOOL fMachineWide,
            /* [in] */ __RPC__in BSTR bstrFileExtension,
            /* [in] */ __RPC__in BSTR bstrLauncher) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWorkspaceResTypeRegistryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWorkspaceResTypeRegistry * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWorkspaceResTypeRegistry * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWorkspaceResTypeRegistry * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWorkspaceResTypeRegistry * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWorkspaceResTypeRegistry * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWorkspaceResTypeRegistry * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWorkspaceResTypeRegistry * This,
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
        
        DECLSPEC_XFGVIRT(IWorkspaceResTypeRegistry, AddResourceType)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddResourceType )( 
            __RPC__in IWorkspaceResTypeRegistry * This,
            /* [in] */ VARIANT_BOOL fMachineWide,
            /* [in] */ __RPC__in BSTR bstrFileExtension,
            /* [in] */ __RPC__in BSTR bstrLauncher);
        
        DECLSPEC_XFGVIRT(IWorkspaceResTypeRegistry, DeleteResourceType)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteResourceType )( 
            __RPC__in IWorkspaceResTypeRegistry * This,
            /* [in] */ VARIANT_BOOL fMachineWide,
            /* [in] */ __RPC__in BSTR bstrFileExtension);
        
        DECLSPEC_XFGVIRT(IWorkspaceResTypeRegistry, GetRegisteredFileExtensions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRegisteredFileExtensions )( 
            __RPC__in IWorkspaceResTypeRegistry * This,
            /* [in] */ VARIANT_BOOL fMachineWide,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *psaFileExtensions);
        
        DECLSPEC_XFGVIRT(IWorkspaceResTypeRegistry, GetResourceTypeInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetResourceTypeInfo )( 
            __RPC__in IWorkspaceResTypeRegistry * This,
            /* [in] */ VARIANT_BOOL fMachineWide,
            /* [in] */ __RPC__in BSTR bstrFileExtension,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLauncher);
        
        DECLSPEC_XFGVIRT(IWorkspaceResTypeRegistry, ModifyResourceType)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ModifyResourceType )( 
            __RPC__in IWorkspaceResTypeRegistry * This,
            /* [in] */ VARIANT_BOOL fMachineWide,
            /* [in] */ __RPC__in BSTR bstrFileExtension,
            /* [in] */ __RPC__in BSTR bstrLauncher);
        
        END_INTERFACE
    } IWorkspaceResTypeRegistryVtbl;

    interface IWorkspaceResTypeRegistry
    {
        CONST_VTBL struct IWorkspaceResTypeRegistryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWorkspaceResTypeRegistry_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWorkspaceResTypeRegistry_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWorkspaceResTypeRegistry_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWorkspaceResTypeRegistry_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWorkspaceResTypeRegistry_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWorkspaceResTypeRegistry_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWorkspaceResTypeRegistry_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWorkspaceResTypeRegistry_AddResourceType(This,fMachineWide,bstrFileExtension,bstrLauncher)	\
    ( (This)->lpVtbl -> AddResourceType(This,fMachineWide,bstrFileExtension,bstrLauncher) ) 

#define IWorkspaceResTypeRegistry_DeleteResourceType(This,fMachineWide,bstrFileExtension)	\
    ( (This)->lpVtbl -> DeleteResourceType(This,fMachineWide,bstrFileExtension) ) 

#define IWorkspaceResTypeRegistry_GetRegisteredFileExtensions(This,fMachineWide,psaFileExtensions)	\
    ( (This)->lpVtbl -> GetRegisteredFileExtensions(This,fMachineWide,psaFileExtensions) ) 

#define IWorkspaceResTypeRegistry_GetResourceTypeInfo(This,fMachineWide,bstrFileExtension,pbstrLauncher)	\
    ( (This)->lpVtbl -> GetResourceTypeInfo(This,fMachineWide,bstrFileExtension,pbstrLauncher) ) 

#define IWorkspaceResTypeRegistry_ModifyResourceType(This,fMachineWide,bstrFileExtension,bstrLauncher)	\
    ( (This)->lpVtbl -> ModifyResourceType(This,fMachineWide,bstrFileExtension,bstrLauncher) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWorkspaceResTypeRegistry_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_workspaceax_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_workspaceax_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_workspaceax_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


