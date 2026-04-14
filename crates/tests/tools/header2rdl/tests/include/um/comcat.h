

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

#ifndef __comcat_h__
#define __comcat_h__

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

#ifndef __IEnumGUID_FWD_DEFINED__
#define __IEnumGUID_FWD_DEFINED__
typedef interface IEnumGUID IEnumGUID;

#endif 	/* __IEnumGUID_FWD_DEFINED__ */


#ifndef __IEnumCATEGORYINFO_FWD_DEFINED__
#define __IEnumCATEGORYINFO_FWD_DEFINED__
typedef interface IEnumCATEGORYINFO IEnumCATEGORYINFO;

#endif 	/* __IEnumCATEGORYINFO_FWD_DEFINED__ */


#ifndef __ICatRegister_FWD_DEFINED__
#define __ICatRegister_FWD_DEFINED__
typedef interface ICatRegister ICatRegister;

#endif 	/* __ICatRegister_FWD_DEFINED__ */


#ifndef __ICatInformation_FWD_DEFINED__
#define __ICatInformation_FWD_DEFINED__
typedef interface ICatInformation ICatInformation;

#endif 	/* __ICatInformation_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_comcat_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// ComCat.h
//=--------------------------------------------------------------------------=
// Copyright (c) Microsoft Corporation. All rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#pragma comment(lib,"uuid.lib")

//=--------------------------------------------------------------------------=
// OLE Componet Categories Interfaces.
//=--------------------------------------------------------------------------=
//
#include <winapifamily.h>
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



EXTERN_C const CLSID CLSID_StdComponentCategoriesMgr;

////////////////////////////////////////////////////////////////////////////
//  Types
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
typedef GUID CATID;

typedef REFGUID REFCATID;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define IID_IEnumCLSID              IID_IEnumGUID
#define IEnumCLSID                  IEnumGUID
#define LPENUMCLSID                 LPENUMGUID
#define CATID_NULL                   GUID_NULL
#define IsEqualCATID(rcatid1, rcatid2)       IsEqualGUID(rcatid1, rcatid2)
#define IID_IEnumCATID       IID_IEnumGUID
#define IEnumCATID           IEnumGUID

////////////////////////////////////////////////////////////////////////////
//  Category IDs (link to uuid3.lib)
EXTERN_C const CATID CATID_Insertable;
EXTERN_C const CATID CATID_Control;
EXTERN_C const CATID CATID_Programmable;
EXTERN_C const CATID CATID_IsShortcut;
EXTERN_C const CATID CATID_NeverShowExt;
EXTERN_C const CATID CATID_DocObject;
EXTERN_C const CATID CATID_Printable;
EXTERN_C const CATID CATID_RequiresDataPathHost;
EXTERN_C const CATID CATID_PersistsToMoniker;
EXTERN_C const CATID CATID_PersistsToStorage;
EXTERN_C const CATID CATID_PersistsToStreamInit;
EXTERN_C const CATID CATID_PersistsToStream;
EXTERN_C const CATID CATID_PersistsToMemory;
EXTERN_C const CATID CATID_PersistsToFile;
EXTERN_C const CATID CATID_PersistsToPropertyBag;
EXTERN_C const CATID CATID_InternetAware;
EXTERN_C const CATID CATID_DesignTimeUIActivatableControl;

////////////////////////////////////////////////////////////////////////////
//  Interface Definitions
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#ifndef _LPENUMGUID_DEFINED
#define _LPENUMGUID_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0000_v0_0_s_ifspec;

#ifndef __IEnumGUID_INTERFACE_DEFINED__
#define __IEnumGUID_INTERFACE_DEFINED__

/* interface IEnumGUID */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IEnumGUID *LPENUMGUID;


EXTERN_C const IID IID_IEnumGUID;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0002E000-0000-0000-C000-000000000046")
    IEnumGUID : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [annotation][length_is][size_is][out] */ 
            _Out_writes_to_(celt,*pceltFetched)  GUID *rgelt,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumGUID **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumGUIDVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumGUID * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumGUID * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumGUID * This);
        
        DECLSPEC_XFGVIRT(IEnumGUID, Next)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumGUID * This,
            /* [in] */ ULONG celt,
            /* [annotation][length_is][size_is][out] */ 
            _Out_writes_to_(celt,*pceltFetched)  GUID *rgelt,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumGUID, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumGUID * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumGUID, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumGUID * This);
        
        DECLSPEC_XFGVIRT(IEnumGUID, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumGUID * This,
            /* [out] */ __RPC__deref_out_opt IEnumGUID **ppenum);
        
        END_INTERFACE
    } IEnumGUIDVtbl;

    interface IEnumGUID
    {
        CONST_VTBL struct IEnumGUIDVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumGUID_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumGUID_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumGUID_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumGUID_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumGUID_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumGUID_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumGUID_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IEnumGUID_RemoteNext_Proxy( 
    __RPC__in IEnumGUID * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) GUID *rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);


void __RPC_STUB IEnumGUID_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumGUID_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_comcat_0000_0001 */
/* [local] */ 

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#ifndef _LPENUMCATEGORYINFO_DEFINED
#define _LPENUMCATEGORYINFO_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0001_v0_0_s_ifspec;

#ifndef __IEnumCATEGORYINFO_INTERFACE_DEFINED__
#define __IEnumCATEGORYINFO_INTERFACE_DEFINED__

/* interface IEnumCATEGORYINFO */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IEnumCATEGORYINFO *LPENUMCATEGORYINFO;

typedef struct tagCATEGORYINFO
    {
    CATID catid;
    LCID lcid;
    OLECHAR szDescription[ 128 ];
    } 	CATEGORYINFO;

typedef struct tagCATEGORYINFO *LPCATEGORYINFO;


EXTERN_C const IID IID_IEnumCATEGORYINFO;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0002E011-0000-0000-C000-000000000046")
    IEnumCATEGORYINFO : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) CATEGORYINFO *rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumCATEGORYINFO **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumCATEGORYINFOVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumCATEGORYINFO * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumCATEGORYINFO * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumCATEGORYINFO * This);
        
        DECLSPEC_XFGVIRT(IEnumCATEGORYINFO, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumCATEGORYINFO * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) CATEGORYINFO *rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumCATEGORYINFO, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumCATEGORYINFO * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumCATEGORYINFO, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumCATEGORYINFO * This);
        
        DECLSPEC_XFGVIRT(IEnumCATEGORYINFO, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumCATEGORYINFO * This,
            /* [out] */ __RPC__deref_out_opt IEnumCATEGORYINFO **ppenum);
        
        END_INTERFACE
    } IEnumCATEGORYINFOVtbl;

    interface IEnumCATEGORYINFO
    {
        CONST_VTBL struct IEnumCATEGORYINFOVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumCATEGORYINFO_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumCATEGORYINFO_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumCATEGORYINFO_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumCATEGORYINFO_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumCATEGORYINFO_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumCATEGORYINFO_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumCATEGORYINFO_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumCATEGORYINFO_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_comcat_0000_0002 */
/* [local] */ 

#endif
#ifndef _LPCATREGISTER_DEFINED
#define _LPCATREGISTER_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0002_v0_0_s_ifspec;

#ifndef __ICatRegister_INTERFACE_DEFINED__
#define __ICatRegister_INTERFACE_DEFINED__

/* interface ICatRegister */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer ICatRegister *LPCATREGISTER;


EXTERN_C const IID IID_ICatRegister;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0002E012-0000-0000-C000-000000000046")
    ICatRegister : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterCategories( 
            /* [in] */ ULONG cCategories,
            /* [size_is][in] */ __RPC__in_ecount_full(cCategories) CATEGORYINFO rgCategoryInfo[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnRegisterCategories( 
            /* [in] */ ULONG cCategories,
            /* [size_is][in] */ __RPC__in_ecount_full(cCategories) CATID rgcatid[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterClassImplCategories( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ ULONG cCategories,
            /* [size_is][in] */ __RPC__in_ecount_full(cCategories) CATID rgcatid[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnRegisterClassImplCategories( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ ULONG cCategories,
            /* [size_is][in] */ __RPC__in_ecount_full(cCategories) CATID rgcatid[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterClassReqCategories( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ ULONG cCategories,
            /* [size_is][in] */ __RPC__in_ecount_full(cCategories) CATID rgcatid[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnRegisterClassReqCategories( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ ULONG cCategories,
            /* [size_is][in] */ __RPC__in_ecount_full(cCategories) CATID rgcatid[  ]) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICatRegisterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICatRegister * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICatRegister * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICatRegister * This);
        
        DECLSPEC_XFGVIRT(ICatRegister, RegisterCategories)
        HRESULT ( STDMETHODCALLTYPE *RegisterCategories )( 
            __RPC__in ICatRegister * This,
            /* [in] */ ULONG cCategories,
            /* [size_is][in] */ __RPC__in_ecount_full(cCategories) CATEGORYINFO rgCategoryInfo[  ]);
        
        DECLSPEC_XFGVIRT(ICatRegister, UnRegisterCategories)
        HRESULT ( STDMETHODCALLTYPE *UnRegisterCategories )( 
            __RPC__in ICatRegister * This,
            /* [in] */ ULONG cCategories,
            /* [size_is][in] */ __RPC__in_ecount_full(cCategories) CATID rgcatid[  ]);
        
        DECLSPEC_XFGVIRT(ICatRegister, RegisterClassImplCategories)
        HRESULT ( STDMETHODCALLTYPE *RegisterClassImplCategories )( 
            __RPC__in ICatRegister * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ ULONG cCategories,
            /* [size_is][in] */ __RPC__in_ecount_full(cCategories) CATID rgcatid[  ]);
        
        DECLSPEC_XFGVIRT(ICatRegister, UnRegisterClassImplCategories)
        HRESULT ( STDMETHODCALLTYPE *UnRegisterClassImplCategories )( 
            __RPC__in ICatRegister * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ ULONG cCategories,
            /* [size_is][in] */ __RPC__in_ecount_full(cCategories) CATID rgcatid[  ]);
        
        DECLSPEC_XFGVIRT(ICatRegister, RegisterClassReqCategories)
        HRESULT ( STDMETHODCALLTYPE *RegisterClassReqCategories )( 
            __RPC__in ICatRegister * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ ULONG cCategories,
            /* [size_is][in] */ __RPC__in_ecount_full(cCategories) CATID rgcatid[  ]);
        
        DECLSPEC_XFGVIRT(ICatRegister, UnRegisterClassReqCategories)
        HRESULT ( STDMETHODCALLTYPE *UnRegisterClassReqCategories )( 
            __RPC__in ICatRegister * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ ULONG cCategories,
            /* [size_is][in] */ __RPC__in_ecount_full(cCategories) CATID rgcatid[  ]);
        
        END_INTERFACE
    } ICatRegisterVtbl;

    interface ICatRegister
    {
        CONST_VTBL struct ICatRegisterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICatRegister_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICatRegister_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICatRegister_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICatRegister_RegisterCategories(This,cCategories,rgCategoryInfo)	\
    ( (This)->lpVtbl -> RegisterCategories(This,cCategories,rgCategoryInfo) ) 

#define ICatRegister_UnRegisterCategories(This,cCategories,rgcatid)	\
    ( (This)->lpVtbl -> UnRegisterCategories(This,cCategories,rgcatid) ) 

#define ICatRegister_RegisterClassImplCategories(This,rclsid,cCategories,rgcatid)	\
    ( (This)->lpVtbl -> RegisterClassImplCategories(This,rclsid,cCategories,rgcatid) ) 

#define ICatRegister_UnRegisterClassImplCategories(This,rclsid,cCategories,rgcatid)	\
    ( (This)->lpVtbl -> UnRegisterClassImplCategories(This,rclsid,cCategories,rgcatid) ) 

#define ICatRegister_RegisterClassReqCategories(This,rclsid,cCategories,rgcatid)	\
    ( (This)->lpVtbl -> RegisterClassReqCategories(This,rclsid,cCategories,rgcatid) ) 

#define ICatRegister_UnRegisterClassReqCategories(This,rclsid,cCategories,rgcatid)	\
    ( (This)->lpVtbl -> UnRegisterClassReqCategories(This,rclsid,cCategories,rgcatid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICatRegister_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_comcat_0000_0003 */
/* [local] */ 

#endif
#ifndef _LPCATINFORMATION_DEFINED
#define _LPCATINFORMATION_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0003_v0_0_s_ifspec;

#ifndef __ICatInformation_INTERFACE_DEFINED__
#define __ICatInformation_INTERFACE_DEFINED__

/* interface ICatInformation */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer ICatInformation *LPCATINFORMATION;


EXTERN_C const IID IID_ICatInformation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0002E013-0000-0000-C000-000000000046")
    ICatInformation : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumCategories( 
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt IEnumCATEGORYINFO **ppenumCategoryInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCategoryDesc( 
            /* [in] */ __RPC__in REFCATID rcatid,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszDesc) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE EnumClassesOfCategories( 
            /* [in] */ ULONG cImplemented,
            /* [size_is][in] */ const CATID rgcatidImpl[  ],
            /* [in] */ ULONG cRequired,
            /* [size_is][in] */ const CATID rgcatidReq[  ],
            /* [out] */ IEnumGUID **ppenumClsid) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE IsClassOfCategories( 
            /* [in] */ REFCLSID rclsid,
            /* [in] */ ULONG cImplemented,
            /* [size_is][in] */ const CATID rgcatidImpl[  ],
            /* [in] */ ULONG cRequired,
            /* [size_is][in] */ const CATID rgcatidReq[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumImplCategoriesOfClass( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [out] */ __RPC__deref_out_opt IEnumGUID **ppenumCatid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumReqCategoriesOfClass( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [out] */ __RPC__deref_out_opt IEnumGUID **ppenumCatid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICatInformationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICatInformation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICatInformation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICatInformation * This);
        
        DECLSPEC_XFGVIRT(ICatInformation, EnumCategories)
        HRESULT ( STDMETHODCALLTYPE *EnumCategories )( 
            __RPC__in ICatInformation * This,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt IEnumCATEGORYINFO **ppenumCategoryInfo);
        
        DECLSPEC_XFGVIRT(ICatInformation, GetCategoryDesc)
        HRESULT ( STDMETHODCALLTYPE *GetCategoryDesc )( 
            __RPC__in ICatInformation * This,
            /* [in] */ __RPC__in REFCATID rcatid,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszDesc);
        
        DECLSPEC_XFGVIRT(ICatInformation, EnumClassesOfCategories)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EnumClassesOfCategories )( 
            ICatInformation * This,
            /* [in] */ ULONG cImplemented,
            /* [size_is][in] */ const CATID rgcatidImpl[  ],
            /* [in] */ ULONG cRequired,
            /* [size_is][in] */ const CATID rgcatidReq[  ],
            /* [out] */ IEnumGUID **ppenumClsid);
        
        DECLSPEC_XFGVIRT(ICatInformation, IsClassOfCategories)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *IsClassOfCategories )( 
            ICatInformation * This,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ ULONG cImplemented,
            /* [size_is][in] */ const CATID rgcatidImpl[  ],
            /* [in] */ ULONG cRequired,
            /* [size_is][in] */ const CATID rgcatidReq[  ]);
        
        DECLSPEC_XFGVIRT(ICatInformation, EnumImplCategoriesOfClass)
        HRESULT ( STDMETHODCALLTYPE *EnumImplCategoriesOfClass )( 
            __RPC__in ICatInformation * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [out] */ __RPC__deref_out_opt IEnumGUID **ppenumCatid);
        
        DECLSPEC_XFGVIRT(ICatInformation, EnumReqCategoriesOfClass)
        HRESULT ( STDMETHODCALLTYPE *EnumReqCategoriesOfClass )( 
            __RPC__in ICatInformation * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [out] */ __RPC__deref_out_opt IEnumGUID **ppenumCatid);
        
        END_INTERFACE
    } ICatInformationVtbl;

    interface ICatInformation
    {
        CONST_VTBL struct ICatInformationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICatInformation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICatInformation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICatInformation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICatInformation_EnumCategories(This,lcid,ppenumCategoryInfo)	\
    ( (This)->lpVtbl -> EnumCategories(This,lcid,ppenumCategoryInfo) ) 

#define ICatInformation_GetCategoryDesc(This,rcatid,lcid,pszDesc)	\
    ( (This)->lpVtbl -> GetCategoryDesc(This,rcatid,lcid,pszDesc) ) 

#define ICatInformation_EnumClassesOfCategories(This,cImplemented,rgcatidImpl,cRequired,rgcatidReq,ppenumClsid)	\
    ( (This)->lpVtbl -> EnumClassesOfCategories(This,cImplemented,rgcatidImpl,cRequired,rgcatidReq,ppenumClsid) ) 

#define ICatInformation_IsClassOfCategories(This,rclsid,cImplemented,rgcatidImpl,cRequired,rgcatidReq)	\
    ( (This)->lpVtbl -> IsClassOfCategories(This,rclsid,cImplemented,rgcatidImpl,cRequired,rgcatidReq) ) 

#define ICatInformation_EnumImplCategoriesOfClass(This,rclsid,ppenumCatid)	\
    ( (This)->lpVtbl -> EnumImplCategoriesOfClass(This,rclsid,ppenumCatid) ) 

#define ICatInformation_EnumReqCategoriesOfClass(This,rclsid,ppenumCatid)	\
    ( (This)->lpVtbl -> EnumReqCategoriesOfClass(This,rclsid,ppenumCatid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE ICatInformation_RemoteEnumClassesOfCategories_Proxy( 
    __RPC__in ICatInformation * This,
    /* [in] */ ULONG cImplemented,
    /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cImplemented) const CATID rgcatidImpl[  ],
    /* [in] */ ULONG cRequired,
    /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cRequired) const CATID rgcatidReq[  ],
    /* [out] */ __RPC__deref_out_opt IEnumGUID **ppenumClsid);


void __RPC_STUB ICatInformation_RemoteEnumClassesOfCategories_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ICatInformation_RemoteIsClassOfCategories_Proxy( 
    __RPC__in ICatInformation * This,
    /* [in] */ __RPC__in REFCLSID rclsid,
    /* [in] */ ULONG cImplemented,
    /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cImplemented) const CATID rgcatidImpl[  ],
    /* [in] */ ULONG cRequired,
    /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cRequired) const CATID rgcatidReq[  ]);


void __RPC_STUB ICatInformation_RemoteIsClassOfCategories_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __ICatInformation_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_comcat_0000_0004 */
/* [local] */ 

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_comcat_0000_0004_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* [local] */ HRESULT STDMETHODCALLTYPE IEnumGUID_Next_Proxy( 
    IEnumGUID * This,
    /* [in] */ ULONG celt,
    /* [annotation][length_is][size_is][out] */ 
    _Out_writes_to_(celt,*pceltFetched)  GUID *rgelt,
    /* [annotation][out] */ 
    _Out_opt_  ULONG *pceltFetched);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IEnumGUID_Next_Stub( 
    __RPC__in IEnumGUID * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) GUID *rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);

/* [local] */ HRESULT STDMETHODCALLTYPE ICatInformation_EnumClassesOfCategories_Proxy( 
    ICatInformation * This,
    /* [in] */ ULONG cImplemented,
    /* [size_is][in] */ const CATID rgcatidImpl[  ],
    /* [in] */ ULONG cRequired,
    /* [size_is][in] */ const CATID rgcatidReq[  ],
    /* [out] */ IEnumGUID **ppenumClsid);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ICatInformation_EnumClassesOfCategories_Stub( 
    __RPC__in ICatInformation * This,
    /* [in] */ ULONG cImplemented,
    /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cImplemented) const CATID rgcatidImpl[  ],
    /* [in] */ ULONG cRequired,
    /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cRequired) const CATID rgcatidReq[  ],
    /* [out] */ __RPC__deref_out_opt IEnumGUID **ppenumClsid);

/* [local] */ HRESULT STDMETHODCALLTYPE ICatInformation_IsClassOfCategories_Proxy( 
    ICatInformation * This,
    /* [in] */ REFCLSID rclsid,
    /* [in] */ ULONG cImplemented,
    /* [size_is][in] */ const CATID rgcatidImpl[  ],
    /* [in] */ ULONG cRequired,
    /* [size_is][in] */ const CATID rgcatidReq[  ]);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ICatInformation_IsClassOfCategories_Stub( 
    __RPC__in ICatInformation * This,
    /* [in] */ __RPC__in REFCLSID rclsid,
    /* [in] */ ULONG cImplemented,
    /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cImplemented) const CATID rgcatidImpl[  ],
    /* [in] */ ULONG cRequired,
    /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cRequired) const CATID rgcatidReq[  ]);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


