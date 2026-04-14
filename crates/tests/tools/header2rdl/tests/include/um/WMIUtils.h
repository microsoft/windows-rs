

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

#ifndef __wmiutils_h__
#define __wmiutils_h__

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

#ifndef __IWbemPathKeyList_FWD_DEFINED__
#define __IWbemPathKeyList_FWD_DEFINED__
typedef interface IWbemPathKeyList IWbemPathKeyList;

#endif 	/* __IWbemPathKeyList_FWD_DEFINED__ */


#ifndef __IWbemPath_FWD_DEFINED__
#define __IWbemPath_FWD_DEFINED__
typedef interface IWbemPath IWbemPath;

#endif 	/* __IWbemPath_FWD_DEFINED__ */


#ifndef __WbemDefPath_FWD_DEFINED__
#define __WbemDefPath_FWD_DEFINED__

#ifdef __cplusplus
typedef class WbemDefPath WbemDefPath;
#else
typedef struct WbemDefPath WbemDefPath;
#endif /* __cplusplus */

#endif 	/* __WbemDefPath_FWD_DEFINED__ */


#ifndef __IWbemQuery_FWD_DEFINED__
#define __IWbemQuery_FWD_DEFINED__
typedef interface IWbemQuery IWbemQuery;

#endif 	/* __IWbemQuery_FWD_DEFINED__ */


#ifndef __WbemQuery_FWD_DEFINED__
#define __WbemQuery_FWD_DEFINED__

#ifdef __cplusplus
typedef class WbemQuery WbemQuery;
#else
typedef struct WbemQuery WbemQuery;
#endif /* __cplusplus */

#endif 	/* __WbemQuery_FWD_DEFINED__ */


#ifndef __IWbemQuery_FWD_DEFINED__
#define __IWbemQuery_FWD_DEFINED__
typedef interface IWbemQuery IWbemQuery;

#endif 	/* __IWbemQuery_FWD_DEFINED__ */


#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wmiutils_0000_0000 */
/* [local] */ 

/*******************************************************************************/
/*                                                                             */
/*    Copyright (c) Microsoft Corporation.  All rights reserved.               */
/*                                                                             */
/*******************************************************************************/
#include <winapifamily.h>
#pragma region Desktop Family or WinMgmt Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT)


extern RPC_IF_HANDLE __MIDL_itf_wmiutils_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmiutils_0000_0000_v0_0_s_ifspec;


#ifndef __WbemUtilities_v1_LIBRARY_DEFINED__
#define __WbemUtilities_v1_LIBRARY_DEFINED__

/* library WbemUtilities_v1 */
/* [uuid] */ 

typedef /* [v1_enum] */ 
enum tag_WBEM_PATH_STATUS_FLAG
    {
        WBEMPATH_INFO_ANON_LOCAL_MACHINE	= 0x1,
        WBEMPATH_INFO_HAS_MACHINE_NAME	= 0x2,
        WBEMPATH_INFO_IS_CLASS_REF	= 0x4,
        WBEMPATH_INFO_IS_INST_REF	= 0x8,
        WBEMPATH_INFO_HAS_SUBSCOPES	= 0x10,
        WBEMPATH_INFO_IS_COMPOUND	= 0x20,
        WBEMPATH_INFO_HAS_V2_REF_PATHS	= 0x40,
        WBEMPATH_INFO_HAS_IMPLIED_KEY	= 0x80,
        WBEMPATH_INFO_CONTAINS_SINGLETON	= 0x100,
        WBEMPATH_INFO_V1_COMPLIANT	= 0x200,
        WBEMPATH_INFO_V2_COMPLIANT	= 0x400,
        WBEMPATH_INFO_CIM_COMPLIANT	= 0x800,
        WBEMPATH_INFO_IS_SINGLETON	= 0x1000,
        WBEMPATH_INFO_IS_PARENT	= 0x2000,
        WBEMPATH_INFO_SERVER_NAMESPACE_ONLY	= 0x4000,
        WBEMPATH_INFO_NATIVE_PATH	= 0x8000,
        WBEMPATH_INFO_WMI_PATH	= 0x10000,
        WBEMPATH_INFO_PATH_HAD_SERVER	= 0x20000
    } 	tag_WBEM_PATH_STATUS_FLAG;

typedef /* [v1_enum] */ 
enum tag_WBEM_PATH_CREATE_FLAG
    {
        WBEMPATH_CREATE_ACCEPT_RELATIVE	= 0x1,
        WBEMPATH_CREATE_ACCEPT_ABSOLUTE	= 0x2,
        WBEMPATH_CREATE_ACCEPT_ALL	= 0x4,
        WBEMPATH_TREAT_SINGLE_IDENT_AS_NS	= 0x8
    } 	tag_WBEM_PATH_CREATE_FLAG;

typedef /* [v1_enum] */ 
enum tag_WBEM_GET_TEXT_FLAGS
    {
        WBEMPATH_COMPRESSED	= 0x1,
        WBEMPATH_GET_RELATIVE_ONLY	= 0x2,
        WBEMPATH_GET_SERVER_TOO	= 0x4,
        WBEMPATH_GET_SERVER_AND_NAMESPACE_ONLY	= 0x8,
        WBEMPATH_GET_NAMESPACE_ONLY	= 0x10,
        WBEMPATH_GET_ORIGINAL	= 0x20
    } 	tag_WBEM_GET_TEXT_FLAGS;

typedef /* [v1_enum] */ 
enum tag_WBEM_GET_KEY_FLAGS
    {
        WBEMPATH_TEXT	= 0x1,
        WBEMPATH_QUOTEDTEXT	= 0x2
    } 	tag_WBEM_GET_KEY_FLAGS;



EXTERN_C const IID LIBID_WbemUtilities_v1;

#ifndef __IWbemPathKeyList_INTERFACE_DEFINED__
#define __IWbemPathKeyList_INTERFACE_DEFINED__

/* interface IWbemPathKeyList */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IWbemPathKeyList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9AE62877-7544-4bb0-AA26-A13824659ED6")
    IWbemPathKeyList : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ ULONG *puKeyCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetKey( 
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ ULONG uFlags,
            /* [in] */ ULONG uCimType,
            /* [in] */ LPVOID pKeyVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetKey2( 
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ ULONG uFlags,
            /* [in] */ ULONG uCimType,
            /* [in] */ VARIANT *pKeyVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetKey( 
            /* [in] */ ULONG uKeyIx,
            /* [in] */ ULONG uFlags,
            /* [out][in] */ ULONG *puNameBufSize,
            /* [annotation][out][in] */ 
            _Out_writes_opt_(*puNameBufSize)  LPWSTR pszKeyName,
            /* [out][in] */ ULONG *puKeyValBufSize,
            /* [out][in] */ LPVOID pKeyVal,
            /* [out] */ ULONG *puApparentCimType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetKey2( 
            /* [in] */ ULONG uKeyIx,
            /* [in] */ ULONG uFlags,
            /* [out][in] */ ULONG *puNameBufSize,
            /* [annotation][out][in] */ 
            _Out_writes_opt_(*puNameBufSize)  LPWSTR pszKeyName,
            /* [out][in] */ VARIANT *pKeyValue,
            /* [out] */ ULONG *puApparentCimType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveKey( 
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ ULONG uFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAllKeys( 
            /* [in] */ ULONG uFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MakeSingleton( 
            /* [in] */ boolean bSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInfo( 
            /* [in] */ ULONG uRequestedInfo,
            /* [out] */ ULONGLONG *puResponse) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetText( 
            /* [in] */ long lFlags,
            /* [out][in] */ ULONG *puBuffLength,
            /* [annotation][string][out][in] */ 
            _Out_writes_(*puBuffLength)  LPWSTR pszText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemPathKeyListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWbemPathKeyList * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWbemPathKeyList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWbemPathKeyList * This);
        
        DECLSPEC_XFGVIRT(IWbemPathKeyList, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IWbemPathKeyList * This,
            /* [out] */ ULONG *puKeyCount);
        
        DECLSPEC_XFGVIRT(IWbemPathKeyList, SetKey)
        HRESULT ( STDMETHODCALLTYPE *SetKey )( 
            IWbemPathKeyList * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ ULONG uFlags,
            /* [in] */ ULONG uCimType,
            /* [in] */ LPVOID pKeyVal);
        
        DECLSPEC_XFGVIRT(IWbemPathKeyList, SetKey2)
        HRESULT ( STDMETHODCALLTYPE *SetKey2 )( 
            IWbemPathKeyList * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ ULONG uFlags,
            /* [in] */ ULONG uCimType,
            /* [in] */ VARIANT *pKeyVal);
        
        DECLSPEC_XFGVIRT(IWbemPathKeyList, GetKey)
        HRESULT ( STDMETHODCALLTYPE *GetKey )( 
            IWbemPathKeyList * This,
            /* [in] */ ULONG uKeyIx,
            /* [in] */ ULONG uFlags,
            /* [out][in] */ ULONG *puNameBufSize,
            /* [annotation][out][in] */ 
            _Out_writes_opt_(*puNameBufSize)  LPWSTR pszKeyName,
            /* [out][in] */ ULONG *puKeyValBufSize,
            /* [out][in] */ LPVOID pKeyVal,
            /* [out] */ ULONG *puApparentCimType);
        
        DECLSPEC_XFGVIRT(IWbemPathKeyList, GetKey2)
        HRESULT ( STDMETHODCALLTYPE *GetKey2 )( 
            IWbemPathKeyList * This,
            /* [in] */ ULONG uKeyIx,
            /* [in] */ ULONG uFlags,
            /* [out][in] */ ULONG *puNameBufSize,
            /* [annotation][out][in] */ 
            _Out_writes_opt_(*puNameBufSize)  LPWSTR pszKeyName,
            /* [out][in] */ VARIANT *pKeyValue,
            /* [out] */ ULONG *puApparentCimType);
        
        DECLSPEC_XFGVIRT(IWbemPathKeyList, RemoveKey)
        HRESULT ( STDMETHODCALLTYPE *RemoveKey )( 
            IWbemPathKeyList * This,
            /* [string][in] */ LPCWSTR wszName,
            /* [in] */ ULONG uFlags);
        
        DECLSPEC_XFGVIRT(IWbemPathKeyList, RemoveAllKeys)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllKeys )( 
            IWbemPathKeyList * This,
            /* [in] */ ULONG uFlags);
        
        DECLSPEC_XFGVIRT(IWbemPathKeyList, MakeSingleton)
        HRESULT ( STDMETHODCALLTYPE *MakeSingleton )( 
            IWbemPathKeyList * This,
            /* [in] */ boolean bSet);
        
        DECLSPEC_XFGVIRT(IWbemPathKeyList, GetInfo)
        HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            IWbemPathKeyList * This,
            /* [in] */ ULONG uRequestedInfo,
            /* [out] */ ULONGLONG *puResponse);
        
        DECLSPEC_XFGVIRT(IWbemPathKeyList, GetText)
        HRESULT ( STDMETHODCALLTYPE *GetText )( 
            IWbemPathKeyList * This,
            /* [in] */ long lFlags,
            /* [out][in] */ ULONG *puBuffLength,
            /* [annotation][string][out][in] */ 
            _Out_writes_(*puBuffLength)  LPWSTR pszText);
        
        END_INTERFACE
    } IWbemPathKeyListVtbl;

    interface IWbemPathKeyList
    {
        CONST_VTBL struct IWbemPathKeyListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemPathKeyList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemPathKeyList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemPathKeyList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemPathKeyList_GetCount(This,puKeyCount)	\
    ( (This)->lpVtbl -> GetCount(This,puKeyCount) ) 

#define IWbemPathKeyList_SetKey(This,wszName,uFlags,uCimType,pKeyVal)	\
    ( (This)->lpVtbl -> SetKey(This,wszName,uFlags,uCimType,pKeyVal) ) 

#define IWbemPathKeyList_SetKey2(This,wszName,uFlags,uCimType,pKeyVal)	\
    ( (This)->lpVtbl -> SetKey2(This,wszName,uFlags,uCimType,pKeyVal) ) 

#define IWbemPathKeyList_GetKey(This,uKeyIx,uFlags,puNameBufSize,pszKeyName,puKeyValBufSize,pKeyVal,puApparentCimType)	\
    ( (This)->lpVtbl -> GetKey(This,uKeyIx,uFlags,puNameBufSize,pszKeyName,puKeyValBufSize,pKeyVal,puApparentCimType) ) 

#define IWbemPathKeyList_GetKey2(This,uKeyIx,uFlags,puNameBufSize,pszKeyName,pKeyValue,puApparentCimType)	\
    ( (This)->lpVtbl -> GetKey2(This,uKeyIx,uFlags,puNameBufSize,pszKeyName,pKeyValue,puApparentCimType) ) 

#define IWbemPathKeyList_RemoveKey(This,wszName,uFlags)	\
    ( (This)->lpVtbl -> RemoveKey(This,wszName,uFlags) ) 

#define IWbemPathKeyList_RemoveAllKeys(This,uFlags)	\
    ( (This)->lpVtbl -> RemoveAllKeys(This,uFlags) ) 

#define IWbemPathKeyList_MakeSingleton(This,bSet)	\
    ( (This)->lpVtbl -> MakeSingleton(This,bSet) ) 

#define IWbemPathKeyList_GetInfo(This,uRequestedInfo,puResponse)	\
    ( (This)->lpVtbl -> GetInfo(This,uRequestedInfo,puResponse) ) 

#define IWbemPathKeyList_GetText(This,lFlags,puBuffLength,pszText)	\
    ( (This)->lpVtbl -> GetText(This,lFlags,puBuffLength,pszText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemPathKeyList_INTERFACE_DEFINED__ */


#ifndef __IWbemPath_INTERFACE_DEFINED__
#define __IWbemPath_INTERFACE_DEFINED__

/* interface IWbemPath */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IWbemPath;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3BC15AF2-736C-477e-9E51-238AF8667DCC")
    IWbemPath : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetText( 
            /* [in] */ ULONG uMode,
            /* [in] */ LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetText( 
            /* [in] */ long lFlags,
            /* [out][in] */ ULONG *puBuffLength,
            /* [annotation][string][out][in] */ 
            _Out_writes_(*puBuffLength)  LPWSTR pszText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInfo( 
            /* [in] */ ULONG uRequestedInfo,
            /* [out] */ ULONGLONG *puResponse) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetServer( 
            /* [string][in] */ LPCWSTR Name) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServer( 
            /* [out][in] */ ULONG *puNameBufLength,
            /* [annotation][string][out][in] */ 
            _Out_writes_(*puNameBufLength)  LPWSTR pName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNamespaceCount( 
            /* [out] */ ULONG *puCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetNamespaceAt( 
            /* [in] */ ULONG uIndex,
            /* [string][in] */ LPCWSTR pszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNamespaceAt( 
            /* [in] */ ULONG uIndex,
            /* [out][in] */ ULONG *puNameBufLength,
            /* [annotation][string][out][in] */ 
            _Out_writes_(*puNameBufLength)  LPWSTR pName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveNamespaceAt( 
            /* [in] */ ULONG uIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAllNamespaces( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScopeCount( 
            /* [out] */ ULONG *puCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetScope( 
            /* [in] */ ULONG uIndex,
            /* [annotation][in] */ 
            _In_  LPWSTR pszClass) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetScopeFromText( 
            /* [in] */ ULONG uIndex,
            /* [annotation][in] */ 
            _In_  LPWSTR pszText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScope( 
            /* [in] */ ULONG uIndex,
            /* [out][in] */ ULONG *puClassNameBufSize,
            /* [annotation][out][in] */ 
            _Out_writes_(*puClassNameBufSize)  LPWSTR pszClass,
            /* [out] */ IWbemPathKeyList **pKeyList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScopeAsText( 
            /* [in] */ ULONG uIndex,
            /* [out][in] */ ULONG *puTextBufSize,
            /* [annotation][out][in] */ 
            _Out_writes_(*puTextBufSize)  LPWSTR pszText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveScope( 
            /* [in] */ ULONG uIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAllScopes( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetClassName( 
            /* [string][in] */ LPCWSTR Name) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetClassName( 
            /* [out][in] */ ULONG *puBuffLength,
            /* [annotation][string][out][in] */ 
            _Out_writes_opt_(*puBuffLength)  LPWSTR pszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetKeyList( 
            /* [out] */ IWbemPathKeyList **pOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateClassPart( 
            /* [in] */ long lFlags,
            /* [string][in] */ LPCWSTR Name) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteClassPart( 
            /* [in] */ long lFlags) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsRelative( 
            /* [annotation][string][in] */ 
            _In_  LPWSTR wszMachine,
            /* [annotation][string][in] */ 
            _In_  LPWSTR wszNamespace) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsRelativeOrChild( 
            /* [annotation][string][in] */ 
            _In_  LPWSTR wszMachine,
            /* [annotation][string][in] */ 
            _In_  LPWSTR wszNamespace,
            /* [in] */ long lFlags) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsLocal( 
            /* [string][in] */ LPCWSTR wszMachine) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsSameClassName( 
            /* [string][in] */ LPCWSTR wszClass) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemPathVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWbemPath * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWbemPath * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWbemPath * This);
        
        DECLSPEC_XFGVIRT(IWbemPath, SetText)
        HRESULT ( STDMETHODCALLTYPE *SetText )( 
            IWbemPath * This,
            /* [in] */ ULONG uMode,
            /* [in] */ LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IWbemPath, GetText)
        HRESULT ( STDMETHODCALLTYPE *GetText )( 
            IWbemPath * This,
            /* [in] */ long lFlags,
            /* [out][in] */ ULONG *puBuffLength,
            /* [annotation][string][out][in] */ 
            _Out_writes_(*puBuffLength)  LPWSTR pszText);
        
        DECLSPEC_XFGVIRT(IWbemPath, GetInfo)
        HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            IWbemPath * This,
            /* [in] */ ULONG uRequestedInfo,
            /* [out] */ ULONGLONG *puResponse);
        
        DECLSPEC_XFGVIRT(IWbemPath, SetServer)
        HRESULT ( STDMETHODCALLTYPE *SetServer )( 
            IWbemPath * This,
            /* [string][in] */ LPCWSTR Name);
        
        DECLSPEC_XFGVIRT(IWbemPath, GetServer)
        HRESULT ( STDMETHODCALLTYPE *GetServer )( 
            IWbemPath * This,
            /* [out][in] */ ULONG *puNameBufLength,
            /* [annotation][string][out][in] */ 
            _Out_writes_(*puNameBufLength)  LPWSTR pName);
        
        DECLSPEC_XFGVIRT(IWbemPath, GetNamespaceCount)
        HRESULT ( STDMETHODCALLTYPE *GetNamespaceCount )( 
            IWbemPath * This,
            /* [out] */ ULONG *puCount);
        
        DECLSPEC_XFGVIRT(IWbemPath, SetNamespaceAt)
        HRESULT ( STDMETHODCALLTYPE *SetNamespaceAt )( 
            IWbemPath * This,
            /* [in] */ ULONG uIndex,
            /* [string][in] */ LPCWSTR pszName);
        
        DECLSPEC_XFGVIRT(IWbemPath, GetNamespaceAt)
        HRESULT ( STDMETHODCALLTYPE *GetNamespaceAt )( 
            IWbemPath * This,
            /* [in] */ ULONG uIndex,
            /* [out][in] */ ULONG *puNameBufLength,
            /* [annotation][string][out][in] */ 
            _Out_writes_(*puNameBufLength)  LPWSTR pName);
        
        DECLSPEC_XFGVIRT(IWbemPath, RemoveNamespaceAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveNamespaceAt )( 
            IWbemPath * This,
            /* [in] */ ULONG uIndex);
        
        DECLSPEC_XFGVIRT(IWbemPath, RemoveAllNamespaces)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllNamespaces )( 
            IWbemPath * This);
        
        DECLSPEC_XFGVIRT(IWbemPath, GetScopeCount)
        HRESULT ( STDMETHODCALLTYPE *GetScopeCount )( 
            IWbemPath * This,
            /* [out] */ ULONG *puCount);
        
        DECLSPEC_XFGVIRT(IWbemPath, SetScope)
        HRESULT ( STDMETHODCALLTYPE *SetScope )( 
            IWbemPath * This,
            /* [in] */ ULONG uIndex,
            /* [annotation][in] */ 
            _In_  LPWSTR pszClass);
        
        DECLSPEC_XFGVIRT(IWbemPath, SetScopeFromText)
        HRESULT ( STDMETHODCALLTYPE *SetScopeFromText )( 
            IWbemPath * This,
            /* [in] */ ULONG uIndex,
            /* [annotation][in] */ 
            _In_  LPWSTR pszText);
        
        DECLSPEC_XFGVIRT(IWbemPath, GetScope)
        HRESULT ( STDMETHODCALLTYPE *GetScope )( 
            IWbemPath * This,
            /* [in] */ ULONG uIndex,
            /* [out][in] */ ULONG *puClassNameBufSize,
            /* [annotation][out][in] */ 
            _Out_writes_(*puClassNameBufSize)  LPWSTR pszClass,
            /* [out] */ IWbemPathKeyList **pKeyList);
        
        DECLSPEC_XFGVIRT(IWbemPath, GetScopeAsText)
        HRESULT ( STDMETHODCALLTYPE *GetScopeAsText )( 
            IWbemPath * This,
            /* [in] */ ULONG uIndex,
            /* [out][in] */ ULONG *puTextBufSize,
            /* [annotation][out][in] */ 
            _Out_writes_(*puTextBufSize)  LPWSTR pszText);
        
        DECLSPEC_XFGVIRT(IWbemPath, RemoveScope)
        HRESULT ( STDMETHODCALLTYPE *RemoveScope )( 
            IWbemPath * This,
            /* [in] */ ULONG uIndex);
        
        DECLSPEC_XFGVIRT(IWbemPath, RemoveAllScopes)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllScopes )( 
            IWbemPath * This);
        
        DECLSPEC_XFGVIRT(IWbemPath, SetClassName)
        HRESULT ( STDMETHODCALLTYPE *SetClassName )( 
            IWbemPath * This,
            /* [string][in] */ LPCWSTR Name);
        
        DECLSPEC_XFGVIRT(IWbemPath, GetClassName)
        HRESULT ( STDMETHODCALLTYPE *GetClassName )( 
            IWbemPath * This,
            /* [out][in] */ ULONG *puBuffLength,
            /* [annotation][string][out][in] */ 
            _Out_writes_opt_(*puBuffLength)  LPWSTR pszName);
        
        DECLSPEC_XFGVIRT(IWbemPath, GetKeyList)
        HRESULT ( STDMETHODCALLTYPE *GetKeyList )( 
            IWbemPath * This,
            /* [out] */ IWbemPathKeyList **pOut);
        
        DECLSPEC_XFGVIRT(IWbemPath, CreateClassPart)
        HRESULT ( STDMETHODCALLTYPE *CreateClassPart )( 
            IWbemPath * This,
            /* [in] */ long lFlags,
            /* [string][in] */ LPCWSTR Name);
        
        DECLSPEC_XFGVIRT(IWbemPath, DeleteClassPart)
        HRESULT ( STDMETHODCALLTYPE *DeleteClassPart )( 
            IWbemPath * This,
            /* [in] */ long lFlags);
        
        DECLSPEC_XFGVIRT(IWbemPath, IsRelative)
        BOOL ( STDMETHODCALLTYPE *IsRelative )( 
            IWbemPath * This,
            /* [annotation][string][in] */ 
            _In_  LPWSTR wszMachine,
            /* [annotation][string][in] */ 
            _In_  LPWSTR wszNamespace);
        
        DECLSPEC_XFGVIRT(IWbemPath, IsRelativeOrChild)
        BOOL ( STDMETHODCALLTYPE *IsRelativeOrChild )( 
            IWbemPath * This,
            /* [annotation][string][in] */ 
            _In_  LPWSTR wszMachine,
            /* [annotation][string][in] */ 
            _In_  LPWSTR wszNamespace,
            /* [in] */ long lFlags);
        
        DECLSPEC_XFGVIRT(IWbemPath, IsLocal)
        BOOL ( STDMETHODCALLTYPE *IsLocal )( 
            IWbemPath * This,
            /* [string][in] */ LPCWSTR wszMachine);
        
        DECLSPEC_XFGVIRT(IWbemPath, IsSameClassName)
        BOOL ( STDMETHODCALLTYPE *IsSameClassName )( 
            IWbemPath * This,
            /* [string][in] */ LPCWSTR wszClass);
        
        END_INTERFACE
    } IWbemPathVtbl;

    interface IWbemPath
    {
        CONST_VTBL struct IWbemPathVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemPath_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemPath_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemPath_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemPath_SetText(This,uMode,pszPath)	\
    ( (This)->lpVtbl -> SetText(This,uMode,pszPath) ) 

#define IWbemPath_GetText(This,lFlags,puBuffLength,pszText)	\
    ( (This)->lpVtbl -> GetText(This,lFlags,puBuffLength,pszText) ) 

#define IWbemPath_GetInfo(This,uRequestedInfo,puResponse)	\
    ( (This)->lpVtbl -> GetInfo(This,uRequestedInfo,puResponse) ) 

#define IWbemPath_SetServer(This,Name)	\
    ( (This)->lpVtbl -> SetServer(This,Name) ) 

#define IWbemPath_GetServer(This,puNameBufLength,pName)	\
    ( (This)->lpVtbl -> GetServer(This,puNameBufLength,pName) ) 

#define IWbemPath_GetNamespaceCount(This,puCount)	\
    ( (This)->lpVtbl -> GetNamespaceCount(This,puCount) ) 

#define IWbemPath_SetNamespaceAt(This,uIndex,pszName)	\
    ( (This)->lpVtbl -> SetNamespaceAt(This,uIndex,pszName) ) 

#define IWbemPath_GetNamespaceAt(This,uIndex,puNameBufLength,pName)	\
    ( (This)->lpVtbl -> GetNamespaceAt(This,uIndex,puNameBufLength,pName) ) 

#define IWbemPath_RemoveNamespaceAt(This,uIndex)	\
    ( (This)->lpVtbl -> RemoveNamespaceAt(This,uIndex) ) 

#define IWbemPath_RemoveAllNamespaces(This)	\
    ( (This)->lpVtbl -> RemoveAllNamespaces(This) ) 

#define IWbemPath_GetScopeCount(This,puCount)	\
    ( (This)->lpVtbl -> GetScopeCount(This,puCount) ) 

#define IWbemPath_SetScope(This,uIndex,pszClass)	\
    ( (This)->lpVtbl -> SetScope(This,uIndex,pszClass) ) 

#define IWbemPath_SetScopeFromText(This,uIndex,pszText)	\
    ( (This)->lpVtbl -> SetScopeFromText(This,uIndex,pszText) ) 

#define IWbemPath_GetScope(This,uIndex,puClassNameBufSize,pszClass,pKeyList)	\
    ( (This)->lpVtbl -> GetScope(This,uIndex,puClassNameBufSize,pszClass,pKeyList) ) 

#define IWbemPath_GetScopeAsText(This,uIndex,puTextBufSize,pszText)	\
    ( (This)->lpVtbl -> GetScopeAsText(This,uIndex,puTextBufSize,pszText) ) 

#define IWbemPath_RemoveScope(This,uIndex)	\
    ( (This)->lpVtbl -> RemoveScope(This,uIndex) ) 

#define IWbemPath_RemoveAllScopes(This)	\
    ( (This)->lpVtbl -> RemoveAllScopes(This) ) 

#define IWbemPath_SetClassName(This,Name)	\
    ( (This)->lpVtbl -> SetClassName(This,Name) ) 

#define IWbemPath_GetClassName(This,puBuffLength,pszName)	\
    ( (This)->lpVtbl -> GetClassName(This,puBuffLength,pszName) ) 

#define IWbemPath_GetKeyList(This,pOut)	\
    ( (This)->lpVtbl -> GetKeyList(This,pOut) ) 

#define IWbemPath_CreateClassPart(This,lFlags,Name)	\
    ( (This)->lpVtbl -> CreateClassPart(This,lFlags,Name) ) 

#define IWbemPath_DeleteClassPart(This,lFlags)	\
    ( (This)->lpVtbl -> DeleteClassPart(This,lFlags) ) 

#define IWbemPath_IsRelative(This,wszMachine,wszNamespace)	\
    ( (This)->lpVtbl -> IsRelative(This,wszMachine,wszNamespace) ) 

#define IWbemPath_IsRelativeOrChild(This,wszMachine,wszNamespace,lFlags)	\
    ( (This)->lpVtbl -> IsRelativeOrChild(This,wszMachine,wszNamespace,lFlags) ) 

#define IWbemPath_IsLocal(This,wszMachine)	\
    ( (This)->lpVtbl -> IsLocal(This,wszMachine) ) 

#define IWbemPath_IsSameClassName(This,wszClass)	\
    ( (This)->lpVtbl -> IsSameClassName(This,wszClass) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemPath_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_WbemDefPath;

#ifdef __cplusplus

class DECLSPEC_UUID("cf4cc405-e2c5-4ddd-b3ce-5e7582d8c9fa")
WbemDefPath;
#endif

#ifndef __IWbemQuery_INTERFACE_DEFINED__
#define __IWbemQuery_INTERFACE_DEFINED__

/* interface IWbemQuery */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IWbemQuery;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("81166f58-dd98-11d3-a120-00105a1f515a")
    IWbemQuery : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Empty( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLanguageFeatures( 
            /* [in] */ ULONG uFlags,
            /* [in] */ ULONG uArraySize,
            /* [in] */ ULONG *puFeatures) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TestLanguageFeatures( 
            /* [in] */ ULONG uFlags,
            /* [out][in] */ ULONG *uArraySize,
            /* [out] */ ULONG *puFeatures) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Parse( 
            /* [in] */ LPCWSTR pszLang,
            /* [in] */ LPCWSTR pszQuery,
            /* [in] */ ULONG uFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAnalysis( 
            /* [in] */ ULONG uAnalysisType,
            /* [in] */ ULONG uFlags,
            /* [out] */ LPVOID *pAnalysis) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FreeMemory( 
            /* [in] */ LPVOID pMem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetQueryInfo( 
            /* [in] */ ULONG uAnalysisType,
            /* [in] */ ULONG uInfoId,
            /* [in] */ ULONG uBufSize,
            /* [out] */ LPVOID pDestBuf) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWbemQueryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWbemQuery * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWbemQuery * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWbemQuery * This);
        
        DECLSPEC_XFGVIRT(IWbemQuery, Empty)
        HRESULT ( STDMETHODCALLTYPE *Empty )( 
            IWbemQuery * This);
        
        DECLSPEC_XFGVIRT(IWbemQuery, SetLanguageFeatures)
        HRESULT ( STDMETHODCALLTYPE *SetLanguageFeatures )( 
            IWbemQuery * This,
            /* [in] */ ULONG uFlags,
            /* [in] */ ULONG uArraySize,
            /* [in] */ ULONG *puFeatures);
        
        DECLSPEC_XFGVIRT(IWbemQuery, TestLanguageFeatures)
        HRESULT ( STDMETHODCALLTYPE *TestLanguageFeatures )( 
            IWbemQuery * This,
            /* [in] */ ULONG uFlags,
            /* [out][in] */ ULONG *uArraySize,
            /* [out] */ ULONG *puFeatures);
        
        DECLSPEC_XFGVIRT(IWbemQuery, Parse)
        HRESULT ( STDMETHODCALLTYPE *Parse )( 
            IWbemQuery * This,
            /* [in] */ LPCWSTR pszLang,
            /* [in] */ LPCWSTR pszQuery,
            /* [in] */ ULONG uFlags);
        
        DECLSPEC_XFGVIRT(IWbemQuery, GetAnalysis)
        HRESULT ( STDMETHODCALLTYPE *GetAnalysis )( 
            IWbemQuery * This,
            /* [in] */ ULONG uAnalysisType,
            /* [in] */ ULONG uFlags,
            /* [out] */ LPVOID *pAnalysis);
        
        DECLSPEC_XFGVIRT(IWbemQuery, FreeMemory)
        HRESULT ( STDMETHODCALLTYPE *FreeMemory )( 
            IWbemQuery * This,
            /* [in] */ LPVOID pMem);
        
        DECLSPEC_XFGVIRT(IWbemQuery, GetQueryInfo)
        HRESULT ( STDMETHODCALLTYPE *GetQueryInfo )( 
            IWbemQuery * This,
            /* [in] */ ULONG uAnalysisType,
            /* [in] */ ULONG uInfoId,
            /* [in] */ ULONG uBufSize,
            /* [out] */ LPVOID pDestBuf);
        
        END_INTERFACE
    } IWbemQueryVtbl;

    interface IWbemQuery
    {
        CONST_VTBL struct IWbemQueryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWbemQuery_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWbemQuery_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWbemQuery_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWbemQuery_Empty(This)	\
    ( (This)->lpVtbl -> Empty(This) ) 

#define IWbemQuery_SetLanguageFeatures(This,uFlags,uArraySize,puFeatures)	\
    ( (This)->lpVtbl -> SetLanguageFeatures(This,uFlags,uArraySize,puFeatures) ) 

#define IWbemQuery_TestLanguageFeatures(This,uFlags,uArraySize,puFeatures)	\
    ( (This)->lpVtbl -> TestLanguageFeatures(This,uFlags,uArraySize,puFeatures) ) 

#define IWbemQuery_Parse(This,pszLang,pszQuery,uFlags)	\
    ( (This)->lpVtbl -> Parse(This,pszLang,pszQuery,uFlags) ) 

#define IWbemQuery_GetAnalysis(This,uAnalysisType,uFlags,pAnalysis)	\
    ( (This)->lpVtbl -> GetAnalysis(This,uAnalysisType,uFlags,pAnalysis) ) 

#define IWbemQuery_FreeMemory(This,pMem)	\
    ( (This)->lpVtbl -> FreeMemory(This,pMem) ) 

#define IWbemQuery_GetQueryInfo(This,uAnalysisType,uInfoId,uBufSize,pDestBuf)	\
    ( (This)->lpVtbl -> GetQueryInfo(This,uAnalysisType,uInfoId,uBufSize,pDestBuf) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWbemQuery_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_WbemQuery;

#ifdef __cplusplus

class DECLSPEC_UUID("EAC8A024-21E2-4523-AD73-A71A0AA2F56A")
WbemQuery;
#endif
#endif /* __WbemUtilities_v1_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_wmiutils_0000_0001 */
/* [local] */ 

typedef /* [public] */ 
enum __MIDL___MIDL_itf_wmiutils_0000_0001_0001
    {
        WMIQ_ANALYSIS_RPN_SEQUENCE	= 0x1,
        WMIQ_ANALYSIS_ASSOC_QUERY	= 0x2,
        WMIQ_ANALYSIS_PROP_ANALYSIS_MATRIX	= 0x3,
        WMIQ_ANALYSIS_QUERY_TEXT	= 0x4,
        WMIQ_ANALYSIS_RESERVED	= 0x8000000
    } 	WMIQ_ANALYSIS_TYPE;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_wmiutils_0000_0001_0002
    {
        WMIQ_RPN_TOKEN_EXPRESSION	= 1,
        WMIQ_RPN_TOKEN_AND	= 2,
        WMIQ_RPN_TOKEN_OR	= 3,
        WMIQ_RPN_TOKEN_NOT	= 4,
        WMIQ_RPN_OP_UNDEFINED	= 0,
        WMIQ_RPN_OP_EQ	= 1,
        WMIQ_RPN_OP_NE	= 2,
        WMIQ_RPN_OP_GE	= 3,
        WMIQ_RPN_OP_LE	= 4,
        WMIQ_RPN_OP_LT	= 5,
        WMIQ_RPN_OP_GT	= 6,
        WMIQ_RPN_OP_LIKE	= 7,
        WMIQ_RPN_OP_ISA	= 8,
        WMIQ_RPN_OP_ISNOTA	= 9,
        WMIQ_RPN_OP_ISNULL	= 10,
        WMIQ_RPN_OP_ISNOTNULL	= 11,
        WMIQ_RPN_LEFT_PROPERTY_NAME	= 0x1,
        WMIQ_RPN_RIGHT_PROPERTY_NAME	= 0x2,
        WMIQ_RPN_CONST2	= 0x4,
        WMIQ_RPN_CONST	= 0x8,
        WMIQ_RPN_RELOP	= 0x10,
        WMIQ_RPN_LEFT_FUNCTION	= 0x20,
        WMIQ_RPN_RIGHT_FUNCTION	= 0x40,
        WMIQ_RPN_GET_TOKEN_TYPE	= 1,
        WMIQ_RPN_GET_EXPR_SHAPE	= 2,
        WMIQ_RPN_GET_LEFT_FUNCTION	= 3,
        WMIQ_RPN_GET_RIGHT_FUNCTION	= 4,
        WMIQ_RPN_GET_RELOP	= 5,
        WMIQ_RPN_NEXT_TOKEN	= 1,
        WMIQ_RPN_FROM_UNARY	= 0x1,
        WMIQ_RPN_FROM_PATH	= 0x2,
        WMIQ_RPN_FROM_CLASS_LIST	= 0x4,
        WMIQ_RPN_FROM_MULTIPLE	= 0x8
    } 	WMIQ_RPN_TOKEN_FLAGS;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_wmiutils_0000_0001_0003
    {
        WMIQ_ASSOCQ_ASSOCIATORS	= 0x1,
        WMIQ_ASSOCQ_REFERENCES	= 0x2,
        WMIQ_ASSOCQ_RESULTCLASS	= 0x4,
        WMIQ_ASSOCQ_ASSOCCLASS	= 0x8,
        WMIQ_ASSOCQ_ROLE	= 0x10,
        WMIQ_ASSOCQ_RESULTROLE	= 0x20,
        WMIQ_ASSOCQ_REQUIREDQUALIFIER	= 0x40,
        WMIQ_ASSOCQ_REQUIREDASSOCQUALIFIER	= 0x80,
        WMIQ_ASSOCQ_CLASSDEFSONLY	= 0x100,
        WMIQ_ASSOCQ_KEYSONLY	= 0x200,
        WMIQ_ASSOCQ_SCHEMAONLY	= 0x400,
        WMIQ_ASSOCQ_CLASSREFSONLY	= 0x800
    } 	WMIQ_ASSOCQ_FLAGS;

typedef struct tag_SWbemQueryQualifiedName
    {
    ULONG m_uVersion;
    ULONG m_uTokenType;
    ULONG m_uNameListSize;
    LPCWSTR *m_ppszNameList;
    BOOL m_bArraysUsed;
    BOOL *m_pbArrayElUsed;
    ULONG *m_puArrayIndex;
    } 	SWbemQueryQualifiedName;

typedef union tag_SWbemRpnConst
    {
    LPCWSTR m_pszStrVal;
    BOOL m_bBoolVal;
    LONG m_lLongVal;
    ULONG m_uLongVal;
    double m_dblVal;
    __int64 m_lVal64;
    __int64 m_uVal64;
    } 	SWbemRpnConst;

typedef struct tag_SWbemRpnQueryToken
    {
    ULONG m_uVersion;
    ULONG m_uTokenType;
    ULONG m_uSubexpressionShape;
    ULONG m_uOperator;
    SWbemQueryQualifiedName *m_pRightIdent;
    SWbemQueryQualifiedName *m_pLeftIdent;
    ULONG m_uConstApparentType;
    SWbemRpnConst m_Const;
    ULONG m_uConst2ApparentType;
    SWbemRpnConst m_Const2;
    LPCWSTR m_pszRightFunc;
    LPCWSTR m_pszLeftFunc;
    } 	SWbemRpnQueryToken;

typedef struct tag_SWbemRpnTokenList
    {
    ULONG m_uVersion;
    ULONG m_uTokenType;
    ULONG m_uNumTokens;
    } 	SWbemRpnTokenList;

typedef 
enum tag_WMIQ_LANGUAGE_FEATURES
    {
        WMIQ_LF1_BASIC_SELECT	= 1,
        WMIQ_LF2_CLASS_NAME_IN_QUERY	= 2,
        WMIQ_LF3_STRING_CASE_FUNCTIONS	= 3,
        WMIQ_LF4_PROP_TO_PROP_TESTS	= 4,
        WMIQ_LF5_COUNT_STAR	= 5,
        WMIQ_LF6_ORDER_BY	= 6,
        WMIQ_LF7_DISTINCT	= 7,
        WMIQ_LF8_ISA	= 8,
        WMIQ_LF9_THIS	= 9,
        WMIQ_LF10_COMPEX_SUBEXPRESSIONS	= 10,
        WMIQ_LF11_ALIASING	= 11,
        WMIQ_LF12_GROUP_BY_HAVING	= 12,
        WMIQ_LF13_WMI_WITHIN	= 13,
        WMIQ_LF14_SQL_WRITE_OPERATIONS	= 14,
        WMIQ_LF15_GO	= 15,
        WMIQ_LF16_SINGLE_LEVEL_TRANSACTIONS	= 16,
        WMIQ_LF17_QUALIFIED_NAMES	= 17,
        WMIQ_LF18_ASSOCIATONS	= 18,
        WMIQ_LF19_SYSTEM_PROPERTIES	= 19,
        WMIQ_LF20_EXTENDED_SYSTEM_PROPERTIES	= 20,
        WMIQ_LF21_SQL89_JOINS	= 21,
        WMIQ_LF22_SQL92_JOINS	= 22,
        WMIQ_LF23_SUBSELECTS	= 23,
        WMIQ_LF24_UMI_EXTENSIONS	= 24,
        WMIQ_LF25_DATEPART	= 25,
        WMIQ_LF26_LIKE	= 26,
        WMIQ_LF27_CIM_TEMPORAL_CONSTRUCTS	= 27,
        WMIQ_LF28_STANDARD_AGGREGATES	= 28,
        WMIQ_LF29_MULTI_LEVEL_ORDER_BY	= 29,
        WMIQ_LF30_WMI_PRAGMAS	= 30,
        WMIQ_LF31_QUALIFIER_TESTS	= 31,
        WMIQ_LF32_SP_EXECUTE	= 32,
        WMIQ_LF33_ARRAY_ACCESS	= 33,
        WMIQ_LF34_UNION	= 34,
        WMIQ_LF35_COMPLEX_SELECT_TARGET	= 35,
        WMIQ_LF36_REFERENCE_TESTS	= 36,
        WMIQ_LF37_SELECT_INTO	= 37,
        WMIQ_LF38_BASIC_DATETIME_TESTS	= 38,
        WMIQ_LF39_COUNT_COLUMN	= 39,
        WMIQ_LF40_BETWEEN	= 40,
        WMIQ_LF_LAST	= 40
    } 	WMIQ_LANGUAGE_FEATURES;

typedef 
enum tag_WMIQ_RPNQ_FEATURE
    {
        WMIQ_RPNF_WHERE_CLAUSE_PRESENT	= 0x1,
        WMIQ_RPNF_QUERY_IS_CONJUNCTIVE	= 0x2,
        WMIQ_RPNF_QUERY_IS_DISJUNCTIVE	= 0x4,
        WMIQ_RPNF_PROJECTION	= 0x8,
        WMIQ_RPNF_FEATURE_SELECT_STAR	= 0x10,
        WMIQ_RPNF_EQUALITY_TESTS_ONLY	= 0x20,
        WMIQ_RPNF_COUNT_STAR	= 0x40,
        WMIQ_RPNF_QUALIFIED_NAMES_USED	= 0x80,
        WMIQ_RPNF_SYSPROP_CLASS_USED	= 0x100,
        WMIQ_RPNF_PROP_TO_PROP_TESTS	= 0x200,
        WMIQ_RPNF_ORDER_BY	= 0x400,
        WMIQ_RPNF_ISA_USED	= 0x800,
        WMIQ_RPNF_GROUP_BY_HAVING	= 0x1000,
        WMIQ_RPNF_ARRAY_ACCESS_USED	= 0x2000
    } 	WMIQ_RPNF_FEATURE;

typedef struct tag_SWbemRpnEncodedQuery
    {
    ULONG m_uVersion;
    ULONG m_uTokenType;
    unsigned __int64 m_uParsedFeatureMask;
    ULONG m_uDetectedArraySize;
    ULONG *m_puDetectedFeatures;
    ULONG m_uSelectListSize;
    SWbemQueryQualifiedName **m_ppSelectList;
    ULONG m_uFromTargetType;
    LPCWSTR m_pszOptionalFromPath;
    ULONG m_uFromListSize;
    LPCWSTR *m_ppszFromList;
    ULONG m_uWhereClauseSize;
    SWbemRpnQueryToken **m_ppRpnWhereClause;
    double m_dblWithinPolling;
    double m_dblWithinWindow;
    ULONG m_uOrderByListSize;
    LPCWSTR *m_ppszOrderByList;
    ULONG *m_uOrderDirectionEl;
    } 	SWbemRpnEncodedQuery;

typedef struct tag_SWbemAnalysisMatrix
    {
    ULONG m_uVersion;
    ULONG m_uMatrixType;
    LPCWSTR m_pszProperty;
    ULONG m_uPropertyType;
    ULONG m_uEntries;
    LPVOID *m_pValues;
    BOOL *m_pbTruthTable;
    } 	SWbemAnalysisMatrix;

typedef struct tag_SWbemAnalysisMatrixList
    {
    ULONG m_uVersion;
    ULONG m_uMatrixType;
    ULONG m_uNumMatrices;
    SWbemAnalysisMatrix *m_pMatrices;
    } 	SWbemAnalysisMatrixList;

typedef struct tag_SWbemAssocQueryInf
    {
    ULONG m_uVersion;
    ULONG m_uAnalysisType;
    ULONG m_uFeatureMask;
    IWbemPath *m_pPath;
    LPWSTR m_pszPath;
    LPWSTR m_pszQueryText;
    LPWSTR m_pszResultClass;
    LPWSTR m_pszAssocClass;
    LPWSTR m_pszRole;
    LPWSTR m_pszResultRole;
    LPWSTR m_pszRequiredQualifier;
    LPWSTR m_pszRequiredAssocQualifier;
    } 	SWbemAssocQueryInf;



extern RPC_IF_HANDLE __MIDL_itf_wmiutils_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmiutils_0000_0001_v0_0_s_ifspec;

/* interface __MIDL_itf_wmiutils_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wmiutils_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmiutils_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


