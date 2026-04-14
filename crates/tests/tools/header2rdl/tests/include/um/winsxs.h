

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

#ifndef __winsxs_h__
#define __winsxs_h__

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

#ifndef __IAssemblyName_FWD_DEFINED__
#define __IAssemblyName_FWD_DEFINED__
typedef interface IAssemblyName IAssemblyName;

#endif 	/* __IAssemblyName_FWD_DEFINED__ */


#ifndef __IAssemblyCacheItem_FWD_DEFINED__
#define __IAssemblyCacheItem_FWD_DEFINED__
typedef interface IAssemblyCacheItem IAssemblyCacheItem;

#endif 	/* __IAssemblyCacheItem_FWD_DEFINED__ */


#ifndef __IAssemblyCache_FWD_DEFINED__
#define __IAssemblyCache_FWD_DEFINED__
typedef interface IAssemblyCache IAssemblyCache;

#endif 	/* __IAssemblyCache_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "oleidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_winsxs_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// winsxs.h
//=--------------------------------------------------------------------------=
// (C) Copyright 1995-1998 Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma comment(lib,"uuid.lib")

//---------------------------------------------------------------------------=
// Fusion Interfaces for sxs assemblies.

#define STREAM_FORMAT_COMPLIB_MODULE    0
#define STREAM_FORMAT_COMPLIB_MANIFEST  1
#define STREAM_FORMAT_WIN32_MODULE      2
#define STREAM_FORMAT_WIN32_MANIFEST    4



EXTERN_C const IID IID_IAssemblyName;       
EXTERN_C const IID IID_IAssemblyCacheItem;  
EXTERN_C const IID IID_IAssemblyScavenger;      
EXTERN_C const IID IID_IAssemblyCache;      
#ifndef _LPFUSION_DEFINED
#define _LPFUSION_DEFINED
#define IASSEMBLYCACHEITEM_COMMIT_FLAG_REFRESH (0x00000001)
#define QUERYASMINFO_FLAG_VALIDATE             (0x1)
#define ASSEMBLYINFO_FLAG_INSTALLED (0x00000001)
#define ASSEMBLYINFO_FLAG_PAYLOADRESIDENT (0x00000002)
typedef struct _ASSEMBLY_INFO
    {
    ULONG cbAssemblyInfo;
    DWORD dwAssemblyFlags;
    ULARGE_INTEGER uliAssemblySizeInKB;
    LPWSTR pszCurrentAssemblyPathBuf;
    ULONG cchBuf;
    } 	ASSEMBLY_INFO;

#define IASSEMBLYCACHE_UNINSTALL_DISPOSITION_UNINSTALLED (1)
#define IASSEMBLYCACHE_UNINSTALL_DISPOSITION_STILL_IN_USE (2)
#define IASSEMBLYCACHE_UNINSTALL_DISPOSITION_ALREADY_UNINSTALLED (3)
#define IASSEMBLYCACHE_UNINSTALL_DISPOSITION_DELETE_PENDING (4)
#define IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_INSTALLED (1)
#define IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_REFRESHED (2)
#define IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_ALREADY_INSTALLED (3)
// {8cedc215-ac4b-488b-93c0-a50a49cb2fb8}
DEFINE_GUID(FUSION_REFCOUNT_UNINSTALL_SUBKEY_GUID, 0x8cedc215, 0xac4b, 0x488b, 0x93, 0xc0, 0xa5, 0x0a, 0x49, 0xcb, 0x2f, 0xb8);

// {b02f9d65-fb77-4f7a-afa5-b391309f11c9}
DEFINE_GUID(FUSION_REFCOUNT_FILEPATH_GUID, 0xb02f9d65, 0xfb77, 0x4f7a, 0xaf, 0xa5, 0xb3, 0x91, 0x30, 0x9f, 0x11, 0xc9);

// {2ec93463-b0c3-45e1-8364-327e96aea856}
DEFINE_GUID(FUSION_REFCOUNT_OPAQUE_STRING_GUID, 0x2ec93463, 0xb0c3, 0x45e1, 0x83, 0x64, 0x32, 0x7e, 0x96, 0xae, 0xa8, 0x56);
typedef struct _FUSION_INSTALL_REFERENCE_
    {
    DWORD cbSize;
    DWORD dwFlags;
    GUID guidScheme;
    LPCWSTR szIdentifier;
    LPCWSTR szNonCannonicalData;
    } 	FUSION_INSTALL_REFERENCE;

typedef struct _FUSION_INSTALL_REFERENCE_ *LPFUSION_INSTALL_REFERENCE;

typedef const struct _FUSION_INSTALL_REFERENCE_ *LPCFUSION_INSTALL_REFERENCE;



extern RPC_IF_HANDLE __MIDL_itf_winsxs_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_winsxs_0000_0000_v0_0_s_ifspec;

#ifndef __IAssemblyName_INTERFACE_DEFINED__
#define __IAssemblyName_INTERFACE_DEFINED__

/* interface IAssemblyName */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IAssemblyName *LPASSEMBLYNAME;

typedef /* [public] */ 
enum __MIDL_IAssemblyName_0001
    {
        ASM_NAME_PUBLIC_KEY	= 0,
        ASM_NAME_PUBLIC_KEY_TOKEN	= ( ASM_NAME_PUBLIC_KEY + 1 ) ,
        ASM_NAME_HASH_VALUE	= ( ASM_NAME_PUBLIC_KEY_TOKEN + 1 ) ,
        ASM_NAME_NAME	= ( ASM_NAME_HASH_VALUE + 1 ) ,
        ASM_NAME_MAJOR_VERSION	= ( ASM_NAME_NAME + 1 ) ,
        ASM_NAME_MINOR_VERSION	= ( ASM_NAME_MAJOR_VERSION + 1 ) ,
        ASM_NAME_BUILD_NUMBER	= ( ASM_NAME_MINOR_VERSION + 1 ) ,
        ASM_NAME_REVISION_NUMBER	= ( ASM_NAME_BUILD_NUMBER + 1 ) ,
        ASM_NAME_CULTURE	= ( ASM_NAME_REVISION_NUMBER + 1 ) ,
        ASM_NAME_PROCESSOR_ID_ARRAY	= ( ASM_NAME_CULTURE + 1 ) ,
        ASM_NAME_OSINFO_ARRAY	= ( ASM_NAME_PROCESSOR_ID_ARRAY + 1 ) ,
        ASM_NAME_HASH_ALGID	= ( ASM_NAME_OSINFO_ARRAY + 1 ) ,
        ASM_NAME_ALIAS	= ( ASM_NAME_HASH_ALGID + 1 ) ,
        ASM_NAME_CODEBASE_URL	= ( ASM_NAME_ALIAS + 1 ) ,
        ASM_NAME_CODEBASE_LASTMOD	= ( ASM_NAME_CODEBASE_URL + 1 ) ,
        ASM_NAME_NULL_PUBLIC_KEY	= ( ASM_NAME_CODEBASE_LASTMOD + 1 ) ,
        ASM_NAME_NULL_PUBLIC_KEY_TOKEN	= ( ASM_NAME_NULL_PUBLIC_KEY + 1 ) ,
        ASM_NAME_CUSTOM	= ( ASM_NAME_NULL_PUBLIC_KEY_TOKEN + 1 ) ,
        ASM_NAME_NULL_CUSTOM	= ( ASM_NAME_CUSTOM + 1 ) ,
        ASM_NAME_MVID	= ( ASM_NAME_NULL_CUSTOM + 1 ) ,
        ASM_NAME_MAX_PARAMS	= ( ASM_NAME_MVID + 1 ) 
    } 	ASM_NAME;

typedef /* [public] */ 
enum __MIDL_IAssemblyName_0002
    {
        ASM_BINDF_FORCE_CACHE_INSTALL	= 0x1,
        ASM_BINDF_RFS_INTEGRITY_CHECK	= 0x2,
        ASM_BINDF_RFS_MODULE_CHECK	= 0x4,
        ASM_BINDF_BINPATH_PROBE_ONLY	= 0x8,
        ASM_BINDF_SHARED_BINPATH_HINT	= 0x10,
        ASM_BINDF_PARENT_ASM_HINT	= 0x20
    } 	ASM_BIND_FLAGS;

typedef /* [public] */ 
enum __MIDL_IAssemblyName_0003
    {
        ASM_DISPLAYF_VERSION	= 0x1,
        ASM_DISPLAYF_CULTURE	= 0x2,
        ASM_DISPLAYF_PUBLIC_KEY_TOKEN	= 0x4,
        ASM_DISPLAYF_PUBLIC_KEY	= 0x8,
        ASM_DISPLAYF_CUSTOM	= 0x10,
        ASM_DISPLAYF_PROCESSORARCHITECTURE	= 0x20,
        ASM_DISPLAYF_LANGUAGEID	= 0x40
    } 	ASM_DISPLAY_FLAGS;

typedef /* [public] */ 
enum __MIDL_IAssemblyName_0004
    {
        ASM_CMPF_NAME	= 0x1,
        ASM_CMPF_MAJOR_VERSION	= 0x2,
        ASM_CMPF_MINOR_VERSION	= 0x4,
        ASM_CMPF_BUILD_NUMBER	= 0x8,
        ASM_CMPF_REVISION_NUMBER	= 0x10,
        ASM_CMPF_PUBLIC_KEY_TOKEN	= 0x20,
        ASM_CMPF_CULTURE	= 0x40,
        ASM_CMPF_CUSTOM	= 0x80,
        ASM_CMPF_ALL	= ( ( ( ( ( ( ( ASM_CMPF_NAME | ASM_CMPF_MAJOR_VERSION )  | ASM_CMPF_MINOR_VERSION )  | ASM_CMPF_REVISION_NUMBER )  | ASM_CMPF_BUILD_NUMBER )  | ASM_CMPF_PUBLIC_KEY_TOKEN )  | ASM_CMPF_CULTURE )  | ASM_CMPF_CUSTOM ) ,
        ASM_CMPF_DEFAULT	= 0x100
    } 	ASM_CMP_FLAGS;


EXTERN_C const IID IID_IAssemblyName;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD193BC0-B4BC-11d2-9833-00C04FC31D2E")
    IAssemblyName : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ DWORD PropertyId,
            /* [in] */ LPVOID pvProperty,
            /* [in] */ DWORD cbProperty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ DWORD PropertyId,
            /* [out] */ LPVOID pvProperty,
            /* [out][in] */ LPDWORD pcbProperty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finalize( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDisplayName( 
            /* [annotation][out] */ 
            _Out_writes_opt_(*pccDisplayName)  LPWSTR szDisplayName,
            /* [annotation][out][in] */ 
            _Inout_  LPDWORD pccDisplayName,
            /* [in] */ DWORD dwDisplayFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reserved( 
            /* [in] */ REFIID refIID,
            /* [in] */ IUnknown *pUnkReserved1,
            /* [in] */ IUnknown *pUnkReserved2,
            /* [in] */ LPCOLESTR szReserved,
            /* [in] */ LONGLONG llReserved,
            /* [in] */ LPVOID pvReserved,
            /* [in] */ DWORD cbReserved,
            /* [out] */ LPVOID *ppReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [annotation][out][in] */ 
            _Inout_  LPDWORD lpcwBuffer,
            /* [annotation][out] */ 
            _Out_writes_opt_(*lpcwBuffer)  LPWSTR pwzName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersion( 
            /* [out] */ LPDWORD pdwVersionHi,
            /* [out] */ LPDWORD pdwVersionLow) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsEqual( 
            /* [in] */ IAssemblyName *pName,
            /* [in] */ DWORD dwCmpFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ IAssemblyName **pName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAssemblyNameVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAssemblyName * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAssemblyName * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAssemblyName * This);
        
        DECLSPEC_XFGVIRT(IAssemblyName, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            IAssemblyName * This,
            /* [in] */ DWORD PropertyId,
            /* [in] */ LPVOID pvProperty,
            /* [in] */ DWORD cbProperty);
        
        DECLSPEC_XFGVIRT(IAssemblyName, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            IAssemblyName * This,
            /* [in] */ DWORD PropertyId,
            /* [out] */ LPVOID pvProperty,
            /* [out][in] */ LPDWORD pcbProperty);
        
        DECLSPEC_XFGVIRT(IAssemblyName, Finalize)
        HRESULT ( STDMETHODCALLTYPE *Finalize )( 
            IAssemblyName * This);
        
        DECLSPEC_XFGVIRT(IAssemblyName, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            IAssemblyName * This,
            /* [annotation][out] */ 
            _Out_writes_opt_(*pccDisplayName)  LPWSTR szDisplayName,
            /* [annotation][out][in] */ 
            _Inout_  LPDWORD pccDisplayName,
            /* [in] */ DWORD dwDisplayFlags);
        
        DECLSPEC_XFGVIRT(IAssemblyName, Reserved)
        HRESULT ( STDMETHODCALLTYPE *Reserved )( 
            IAssemblyName * This,
            /* [in] */ REFIID refIID,
            /* [in] */ IUnknown *pUnkReserved1,
            /* [in] */ IUnknown *pUnkReserved2,
            /* [in] */ LPCOLESTR szReserved,
            /* [in] */ LONGLONG llReserved,
            /* [in] */ LPVOID pvReserved,
            /* [in] */ DWORD cbReserved,
            /* [out] */ LPVOID *ppReserved);
        
        DECLSPEC_XFGVIRT(IAssemblyName, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            IAssemblyName * This,
            /* [annotation][out][in] */ 
            _Inout_  LPDWORD lpcwBuffer,
            /* [annotation][out] */ 
            _Out_writes_opt_(*lpcwBuffer)  LPWSTR pwzName);
        
        DECLSPEC_XFGVIRT(IAssemblyName, GetVersion)
        HRESULT ( STDMETHODCALLTYPE *GetVersion )( 
            IAssemblyName * This,
            /* [out] */ LPDWORD pdwVersionHi,
            /* [out] */ LPDWORD pdwVersionLow);
        
        DECLSPEC_XFGVIRT(IAssemblyName, IsEqual)
        HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            IAssemblyName * This,
            /* [in] */ IAssemblyName *pName,
            /* [in] */ DWORD dwCmpFlags);
        
        DECLSPEC_XFGVIRT(IAssemblyName, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IAssemblyName * This,
            /* [out] */ IAssemblyName **pName);
        
        END_INTERFACE
    } IAssemblyNameVtbl;

    interface IAssemblyName
    {
        CONST_VTBL struct IAssemblyNameVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAssemblyName_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAssemblyName_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAssemblyName_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAssemblyName_SetProperty(This,PropertyId,pvProperty,cbProperty)	\
    ( (This)->lpVtbl -> SetProperty(This,PropertyId,pvProperty,cbProperty) ) 

#define IAssemblyName_GetProperty(This,PropertyId,pvProperty,pcbProperty)	\
    ( (This)->lpVtbl -> GetProperty(This,PropertyId,pvProperty,pcbProperty) ) 

#define IAssemblyName_Finalize(This)	\
    ( (This)->lpVtbl -> Finalize(This) ) 

#define IAssemblyName_GetDisplayName(This,szDisplayName,pccDisplayName,dwDisplayFlags)	\
    ( (This)->lpVtbl -> GetDisplayName(This,szDisplayName,pccDisplayName,dwDisplayFlags) ) 

#define IAssemblyName_Reserved(This,refIID,pUnkReserved1,pUnkReserved2,szReserved,llReserved,pvReserved,cbReserved,ppReserved)	\
    ( (This)->lpVtbl -> Reserved(This,refIID,pUnkReserved1,pUnkReserved2,szReserved,llReserved,pvReserved,cbReserved,ppReserved) ) 

#define IAssemblyName_GetName(This,lpcwBuffer,pwzName)	\
    ( (This)->lpVtbl -> GetName(This,lpcwBuffer,pwzName) ) 

#define IAssemblyName_GetVersion(This,pdwVersionHi,pdwVersionLow)	\
    ( (This)->lpVtbl -> GetVersion(This,pdwVersionHi,pdwVersionLow) ) 

#define IAssemblyName_IsEqual(This,pName,dwCmpFlags)	\
    ( (This)->lpVtbl -> IsEqual(This,pName,dwCmpFlags) ) 

#define IAssemblyName_Clone(This,pName)	\
    ( (This)->lpVtbl -> Clone(This,pName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAssemblyName_INTERFACE_DEFINED__ */


#ifndef __IAssemblyCacheItem_INTERFACE_DEFINED__
#define __IAssemblyCacheItem_INTERFACE_DEFINED__

/* interface IAssemblyCacheItem */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAssemblyCacheItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9e3aaeb4-d1cd-11d2-bab9-00c04f8eceae")
    IAssemblyCacheItem : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateStream( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ LPCWSTR pszStreamName,
            /* [in] */ DWORD dwFormat,
            /* [in] */ DWORD dwFormatFlags,
            /* [out] */ IStream **ppIStream,
            /* [optional][in] */ ULARGE_INTEGER *puliMaxSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Commit( 
            /* [in] */ DWORD dwFlags,
            /* [optional][out] */ ULONG *pulDisposition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AbortItem( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAssemblyCacheItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAssemblyCacheItem * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAssemblyCacheItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAssemblyCacheItem * This);
        
        DECLSPEC_XFGVIRT(IAssemblyCacheItem, CreateStream)
        HRESULT ( STDMETHODCALLTYPE *CreateStream )( 
            IAssemblyCacheItem * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ LPCWSTR pszStreamName,
            /* [in] */ DWORD dwFormat,
            /* [in] */ DWORD dwFormatFlags,
            /* [out] */ IStream **ppIStream,
            /* [optional][in] */ ULARGE_INTEGER *puliMaxSize);
        
        DECLSPEC_XFGVIRT(IAssemblyCacheItem, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            IAssemblyCacheItem * This,
            /* [in] */ DWORD dwFlags,
            /* [optional][out] */ ULONG *pulDisposition);
        
        DECLSPEC_XFGVIRT(IAssemblyCacheItem, AbortItem)
        HRESULT ( STDMETHODCALLTYPE *AbortItem )( 
            IAssemblyCacheItem * This);
        
        END_INTERFACE
    } IAssemblyCacheItemVtbl;

    interface IAssemblyCacheItem
    {
        CONST_VTBL struct IAssemblyCacheItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAssemblyCacheItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAssemblyCacheItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAssemblyCacheItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAssemblyCacheItem_CreateStream(This,dwFlags,pszStreamName,dwFormat,dwFormatFlags,ppIStream,puliMaxSize)	\
    ( (This)->lpVtbl -> CreateStream(This,dwFlags,pszStreamName,dwFormat,dwFormatFlags,ppIStream,puliMaxSize) ) 

#define IAssemblyCacheItem_Commit(This,dwFlags,pulDisposition)	\
    ( (This)->lpVtbl -> Commit(This,dwFlags,pulDisposition) ) 

#define IAssemblyCacheItem_AbortItem(This)	\
    ( (This)->lpVtbl -> AbortItem(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAssemblyCacheItem_INTERFACE_DEFINED__ */


#ifndef __IAssemblyCache_INTERFACE_DEFINED__
#define __IAssemblyCache_INTERFACE_DEFINED__

/* interface IAssemblyCache */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAssemblyCache;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e707dcde-d1cd-11d2-bab9-00c04f8eceae")
    IAssemblyCache : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE UninstallAssembly( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ LPCWSTR pszAssemblyName,
            /* [in] */ LPCFUSION_INSTALL_REFERENCE pRefData,
            /* [optional][out] */ ULONG *pulDisposition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryAssemblyInfo( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ LPCWSTR pszAssemblyName,
            /* [out][in] */ ASSEMBLY_INFO *pAsmInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateAssemblyCacheItem( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ PVOID pvReserved,
            /* [out] */ IAssemblyCacheItem **ppAsmItem,
            /* [optional][in] */ LPCWSTR pszAssemblyName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reserved( 
            /* [out] */ IUnknown **ppUnk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InstallAssembly( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ LPCWSTR pszManifestFilePath,
            /* [in] */ LPCFUSION_INSTALL_REFERENCE pRefData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAssemblyCacheVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAssemblyCache * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAssemblyCache * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAssemblyCache * This);
        
        DECLSPEC_XFGVIRT(IAssemblyCache, UninstallAssembly)
        HRESULT ( STDMETHODCALLTYPE *UninstallAssembly )( 
            IAssemblyCache * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ LPCWSTR pszAssemblyName,
            /* [in] */ LPCFUSION_INSTALL_REFERENCE pRefData,
            /* [optional][out] */ ULONG *pulDisposition);
        
        DECLSPEC_XFGVIRT(IAssemblyCache, QueryAssemblyInfo)
        HRESULT ( STDMETHODCALLTYPE *QueryAssemblyInfo )( 
            IAssemblyCache * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ LPCWSTR pszAssemblyName,
            /* [out][in] */ ASSEMBLY_INFO *pAsmInfo);
        
        DECLSPEC_XFGVIRT(IAssemblyCache, CreateAssemblyCacheItem)
        HRESULT ( STDMETHODCALLTYPE *CreateAssemblyCacheItem )( 
            IAssemblyCache * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ PVOID pvReserved,
            /* [out] */ IAssemblyCacheItem **ppAsmItem,
            /* [optional][in] */ LPCWSTR pszAssemblyName);
        
        DECLSPEC_XFGVIRT(IAssemblyCache, Reserved)
        HRESULT ( STDMETHODCALLTYPE *Reserved )( 
            IAssemblyCache * This,
            /* [out] */ IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IAssemblyCache, InstallAssembly)
        HRESULT ( STDMETHODCALLTYPE *InstallAssembly )( 
            IAssemblyCache * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ LPCWSTR pszManifestFilePath,
            /* [in] */ LPCFUSION_INSTALL_REFERENCE pRefData);
        
        END_INTERFACE
    } IAssemblyCacheVtbl;

    interface IAssemblyCache
    {
        CONST_VTBL struct IAssemblyCacheVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAssemblyCache_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAssemblyCache_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAssemblyCache_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAssemblyCache_UninstallAssembly(This,dwFlags,pszAssemblyName,pRefData,pulDisposition)	\
    ( (This)->lpVtbl -> UninstallAssembly(This,dwFlags,pszAssemblyName,pRefData,pulDisposition) ) 

#define IAssemblyCache_QueryAssemblyInfo(This,dwFlags,pszAssemblyName,pAsmInfo)	\
    ( (This)->lpVtbl -> QueryAssemblyInfo(This,dwFlags,pszAssemblyName,pAsmInfo) ) 

#define IAssemblyCache_CreateAssemblyCacheItem(This,dwFlags,pvReserved,ppAsmItem,pszAssemblyName)	\
    ( (This)->lpVtbl -> CreateAssemblyCacheItem(This,dwFlags,pvReserved,ppAsmItem,pszAssemblyName) ) 

#define IAssemblyCache_Reserved(This,ppUnk)	\
    ( (This)->lpVtbl -> Reserved(This,ppUnk) ) 

#define IAssemblyCache_InstallAssembly(This,dwFlags,pszManifestFilePath,pRefData)	\
    ( (This)->lpVtbl -> InstallAssembly(This,dwFlags,pszManifestFilePath,pRefData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAssemblyCache_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_winsxs_0000_0003 */
/* [local] */ 

typedef /* [public] */ 
enum __MIDL___MIDL_itf_winsxs_0000_0003_0001
    {
        CANOF_PARSE_DISPLAY_NAME	= 0x1,
        CANOF_SET_DEFAULT_VALUES	= 0x2
    } 	CREATE_ASM_NAME_OBJ_FLAGS;

STDAPI CreateAssemblyNameObject(LPASSEMBLYNAME *ppAssemblyNameObj, LPCWSTR szAssemblyName, DWORD dwFlags, LPVOID pvReserved);             
STDAPI CreateAssemblyCache(IAssemblyCache **ppAsmCache, DWORD dwReserved); 
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_winsxs_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_winsxs_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


