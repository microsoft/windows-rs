

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

#ifndef __propsys_h__
#define __propsys_h__

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

#ifndef __IInitializeWithFile_FWD_DEFINED__
#define __IInitializeWithFile_FWD_DEFINED__
typedef interface IInitializeWithFile IInitializeWithFile;

#endif 	/* __IInitializeWithFile_FWD_DEFINED__ */


#ifndef __IInitializeWithStream_FWD_DEFINED__
#define __IInitializeWithStream_FWD_DEFINED__
typedef interface IInitializeWithStream IInitializeWithStream;

#endif 	/* __IInitializeWithStream_FWD_DEFINED__ */


#ifndef __IPropertyStore_FWD_DEFINED__
#define __IPropertyStore_FWD_DEFINED__
typedef interface IPropertyStore IPropertyStore;

#endif 	/* __IPropertyStore_FWD_DEFINED__ */


#ifndef __INamedPropertyStore_FWD_DEFINED__
#define __INamedPropertyStore_FWD_DEFINED__
typedef interface INamedPropertyStore INamedPropertyStore;

#endif 	/* __INamedPropertyStore_FWD_DEFINED__ */


#ifndef __IObjectWithPropertyKey_FWD_DEFINED__
#define __IObjectWithPropertyKey_FWD_DEFINED__
typedef interface IObjectWithPropertyKey IObjectWithPropertyKey;

#endif 	/* __IObjectWithPropertyKey_FWD_DEFINED__ */


#ifndef __IPropertyChange_FWD_DEFINED__
#define __IPropertyChange_FWD_DEFINED__
typedef interface IPropertyChange IPropertyChange;

#endif 	/* __IPropertyChange_FWD_DEFINED__ */


#ifndef __IPropertyChangeArray_FWD_DEFINED__
#define __IPropertyChangeArray_FWD_DEFINED__
typedef interface IPropertyChangeArray IPropertyChangeArray;

#endif 	/* __IPropertyChangeArray_FWD_DEFINED__ */


#ifndef __IPropertyStoreCapabilities_FWD_DEFINED__
#define __IPropertyStoreCapabilities_FWD_DEFINED__
typedef interface IPropertyStoreCapabilities IPropertyStoreCapabilities;

#endif 	/* __IPropertyStoreCapabilities_FWD_DEFINED__ */


#ifndef __IPropertyStoreCache_FWD_DEFINED__
#define __IPropertyStoreCache_FWD_DEFINED__
typedef interface IPropertyStoreCache IPropertyStoreCache;

#endif 	/* __IPropertyStoreCache_FWD_DEFINED__ */


#ifndef __IPropertyEnumType_FWD_DEFINED__
#define __IPropertyEnumType_FWD_DEFINED__
typedef interface IPropertyEnumType IPropertyEnumType;

#endif 	/* __IPropertyEnumType_FWD_DEFINED__ */


#ifndef __IPropertyEnumType2_FWD_DEFINED__
#define __IPropertyEnumType2_FWD_DEFINED__
typedef interface IPropertyEnumType2 IPropertyEnumType2;

#endif 	/* __IPropertyEnumType2_FWD_DEFINED__ */


#ifndef __IPropertyEnumTypeList_FWD_DEFINED__
#define __IPropertyEnumTypeList_FWD_DEFINED__
typedef interface IPropertyEnumTypeList IPropertyEnumTypeList;

#endif 	/* __IPropertyEnumTypeList_FWD_DEFINED__ */


#ifndef __IPropertyDescription_FWD_DEFINED__
#define __IPropertyDescription_FWD_DEFINED__
typedef interface IPropertyDescription IPropertyDescription;

#endif 	/* __IPropertyDescription_FWD_DEFINED__ */


#ifndef __IPropertyDescription2_FWD_DEFINED__
#define __IPropertyDescription2_FWD_DEFINED__
typedef interface IPropertyDescription2 IPropertyDescription2;

#endif 	/* __IPropertyDescription2_FWD_DEFINED__ */


#ifndef __IPropertyDescriptionAliasInfo_FWD_DEFINED__
#define __IPropertyDescriptionAliasInfo_FWD_DEFINED__
typedef interface IPropertyDescriptionAliasInfo IPropertyDescriptionAliasInfo;

#endif 	/* __IPropertyDescriptionAliasInfo_FWD_DEFINED__ */


#ifndef __IPropertyDescriptionSearchInfo_FWD_DEFINED__
#define __IPropertyDescriptionSearchInfo_FWD_DEFINED__
typedef interface IPropertyDescriptionSearchInfo IPropertyDescriptionSearchInfo;

#endif 	/* __IPropertyDescriptionSearchInfo_FWD_DEFINED__ */


#ifndef __IPropertyDescriptionRelatedPropertyInfo_FWD_DEFINED__
#define __IPropertyDescriptionRelatedPropertyInfo_FWD_DEFINED__
typedef interface IPropertyDescriptionRelatedPropertyInfo IPropertyDescriptionRelatedPropertyInfo;

#endif 	/* __IPropertyDescriptionRelatedPropertyInfo_FWD_DEFINED__ */


#ifndef __IPropertySystem_FWD_DEFINED__
#define __IPropertySystem_FWD_DEFINED__
typedef interface IPropertySystem IPropertySystem;

#endif 	/* __IPropertySystem_FWD_DEFINED__ */


#ifndef __IPropertyDescriptionList_FWD_DEFINED__
#define __IPropertyDescriptionList_FWD_DEFINED__
typedef interface IPropertyDescriptionList IPropertyDescriptionList;

#endif 	/* __IPropertyDescriptionList_FWD_DEFINED__ */


#ifndef __IPropertyStoreFactory_FWD_DEFINED__
#define __IPropertyStoreFactory_FWD_DEFINED__
typedef interface IPropertyStoreFactory IPropertyStoreFactory;

#endif 	/* __IPropertyStoreFactory_FWD_DEFINED__ */


#ifndef __IDelayedPropertyStoreFactory_FWD_DEFINED__
#define __IDelayedPropertyStoreFactory_FWD_DEFINED__
typedef interface IDelayedPropertyStoreFactory IDelayedPropertyStoreFactory;

#endif 	/* __IDelayedPropertyStoreFactory_FWD_DEFINED__ */


#ifndef __IPersistSerializedPropStorage_FWD_DEFINED__
#define __IPersistSerializedPropStorage_FWD_DEFINED__
typedef interface IPersistSerializedPropStorage IPersistSerializedPropStorage;

#endif 	/* __IPersistSerializedPropStorage_FWD_DEFINED__ */


#ifndef __IPersistSerializedPropStorage2_FWD_DEFINED__
#define __IPersistSerializedPropStorage2_FWD_DEFINED__
typedef interface IPersistSerializedPropStorage2 IPersistSerializedPropStorage2;

#endif 	/* __IPersistSerializedPropStorage2_FWD_DEFINED__ */


#ifndef __IPropertySystemChangeNotify_FWD_DEFINED__
#define __IPropertySystemChangeNotify_FWD_DEFINED__
typedef interface IPropertySystemChangeNotify IPropertySystemChangeNotify;

#endif 	/* __IPropertySystemChangeNotify_FWD_DEFINED__ */


#ifndef __ICreateObject_FWD_DEFINED__
#define __ICreateObject_FWD_DEFINED__
typedef interface ICreateObject ICreateObject;

#endif 	/* __ICreateObject_FWD_DEFINED__ */


#ifndef __InMemoryPropertyStore_FWD_DEFINED__
#define __InMemoryPropertyStore_FWD_DEFINED__

#ifdef __cplusplus
typedef class InMemoryPropertyStore InMemoryPropertyStore;
#else
typedef struct InMemoryPropertyStore InMemoryPropertyStore;
#endif /* __cplusplus */

#endif 	/* __InMemoryPropertyStore_FWD_DEFINED__ */


#ifndef __InMemoryPropertyStoreMarshalByValue_FWD_DEFINED__
#define __InMemoryPropertyStoreMarshalByValue_FWD_DEFINED__

#ifdef __cplusplus
typedef class InMemoryPropertyStoreMarshalByValue InMemoryPropertyStoreMarshalByValue;
#else
typedef struct InMemoryPropertyStoreMarshalByValue InMemoryPropertyStoreMarshalByValue;
#endif /* __cplusplus */

#endif 	/* __InMemoryPropertyStoreMarshalByValue_FWD_DEFINED__ */


#ifndef __PropertySystem_FWD_DEFINED__
#define __PropertySystem_FWD_DEFINED__

#ifdef __cplusplus
typedef class PropertySystem PropertySystem;
#else
typedef struct PropertySystem PropertySystem;
#endif /* __cplusplus */

#endif 	/* __PropertySystem_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "oleidl.h"
#include "ocidl.h"
#include "shtypes.h"
#include "StructuredQueryCondition.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_propsys_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#ifndef PSSTDAPI
#if defined(_PROPSYS_)
#define PSSTDAPI          STDAPI
#define PSSTDAPI_(type)   STDAPI_(type)
#else
#define PSSTDAPI          EXTERN_C DECLSPEC_IMPORT HRESULT STDAPICALLTYPE
#define PSSTDAPI_(type)   EXTERN_C DECLSPEC_IMPORT type STDAPICALLTYPE
#endif
#endif // PSSTDAPI
#if 0
typedef PROPERTYKEY *REFPROPERTYKEY;

#endif // 0
#include <propkeydef.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if _MSC_VER >= 1200
#pragma warning(push)
#ifndef _MSC_EXTENSIONS
#pragma warning(disable:4309) /* truncation of constant value */
#endif
#endif


extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0000_v0_0_s_ifspec;

#ifndef __IInitializeWithFile_INTERFACE_DEFINED__
#define __IInitializeWithFile_INTERFACE_DEFINED__

/* interface IInitializeWithFile */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IInitializeWithFile;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b7d14566-0509-4cce-a71f-0a554233bd9b")
    IInitializeWithFile : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszFilePath,
            /* [in] */ DWORD grfMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInitializeWithFileVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInitializeWithFile * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInitializeWithFile * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInitializeWithFile * This);
        
        DECLSPEC_XFGVIRT(IInitializeWithFile, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IInitializeWithFile * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszFilePath,
            /* [in] */ DWORD grfMode);
        
        END_INTERFACE
    } IInitializeWithFileVtbl;

    interface IInitializeWithFile
    {
        CONST_VTBL struct IInitializeWithFileVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInitializeWithFile_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInitializeWithFile_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInitializeWithFile_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInitializeWithFile_Initialize(This,pszFilePath,grfMode)	\
    ( (This)->lpVtbl -> Initialize(This,pszFilePath,grfMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInitializeWithFile_INTERFACE_DEFINED__ */


#ifndef __IInitializeWithStream_INTERFACE_DEFINED__
#define __IInitializeWithStream_INTERFACE_DEFINED__

/* interface IInitializeWithStream */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IInitializeWithStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b824b49d-22ac-4161-ac8a-9916e8fa3f7f")
    IInitializeWithStream : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Initialize( 
            /* [annotation][in] */ 
            _In_  IStream *pstream,
            /* [annotation][in] */ 
            _In_  DWORD grfMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInitializeWithStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInitializeWithStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInitializeWithStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInitializeWithStream * This);
        
        DECLSPEC_XFGVIRT(IInitializeWithStream, Initialize)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IInitializeWithStream * This,
            /* [annotation][in] */ 
            _In_  IStream *pstream,
            /* [annotation][in] */ 
            _In_  DWORD grfMode);
        
        END_INTERFACE
    } IInitializeWithStreamVtbl;

    interface IInitializeWithStream
    {
        CONST_VTBL struct IInitializeWithStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInitializeWithStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInitializeWithStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInitializeWithStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInitializeWithStream_Initialize(This,pstream,grfMode)	\
    ( (This)->lpVtbl -> Initialize(This,pstream,grfMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IInitializeWithStream_RemoteInitialize_Proxy( 
    __RPC__in IInitializeWithStream * This,
    /* [in] */ __RPC__in_opt IStream *pstream,
    /* [in] */ DWORD grfMode);


void __RPC_STUB IInitializeWithStream_RemoteInitialize_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IInitializeWithStream_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_propsys_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application and Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0002_v0_0_s_ifspec;

#ifndef __IPropertyStore_INTERFACE_DEFINED__
#define __IPropertyStore_INTERFACE_DEFINED__

/* interface IPropertyStore */
/* [unique][object][helpstring][uuid] */ 


EXTERN_C const IID IID_IPropertyStore;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("886d8eeb-8cf2-4446-8d02-cdba1dbdcf99")
    IPropertyStore : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out DWORD *cProps) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ DWORD iProp,
            /* [out] */ __RPC__out PROPERTYKEY *pkey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [out] */ __RPC__out PROPVARIANT *pv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValue( 
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [in] */ __RPC__in REFPROPVARIANT propvar) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Commit( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertyStoreVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertyStore * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertyStore * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertyStore * This);
        
        DECLSPEC_XFGVIRT(IPropertyStore, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IPropertyStore * This,
            /* [out] */ __RPC__out DWORD *cProps);
        
        DECLSPEC_XFGVIRT(IPropertyStore, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IPropertyStore * This,
            /* [in] */ DWORD iProp,
            /* [out] */ __RPC__out PROPERTYKEY *pkey);
        
        DECLSPEC_XFGVIRT(IPropertyStore, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in IPropertyStore * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [out] */ __RPC__out PROPVARIANT *pv);
        
        DECLSPEC_XFGVIRT(IPropertyStore, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in IPropertyStore * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [in] */ __RPC__in REFPROPVARIANT propvar);
        
        DECLSPEC_XFGVIRT(IPropertyStore, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IPropertyStore * This);
        
        END_INTERFACE
    } IPropertyStoreVtbl;

    interface IPropertyStore
    {
        CONST_VTBL struct IPropertyStoreVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertyStore_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertyStore_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertyStore_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertyStore_GetCount(This,cProps)	\
    ( (This)->lpVtbl -> GetCount(This,cProps) ) 

#define IPropertyStore_GetAt(This,iProp,pkey)	\
    ( (This)->lpVtbl -> GetAt(This,iProp,pkey) ) 

#define IPropertyStore_GetValue(This,key,pv)	\
    ( (This)->lpVtbl -> GetValue(This,key,pv) ) 

#define IPropertyStore_SetValue(This,key,propvar)	\
    ( (This)->lpVtbl -> SetValue(This,key,propvar) ) 

#define IPropertyStore_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertyStore_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_propsys_0000_0003 */
/* [local] */ 

typedef /* [unique] */  __RPC_unique_pointer IPropertyStore *LPPROPERTYSTORE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
PSSTDAPI PropVariantToWinRTPropertyValue(_In_ REFPROPVARIANT propvar, _In_ REFIID riid, _COM_Outptr_result_maybenull_ void **ppv);
PSSTDAPI WinRTPropertyValueToPropVariant(_In_opt_ IUnknown *punkPropertyValue, _Out_ PROPVARIANT *ppropvar);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0003_v0_0_s_ifspec;

#ifndef __INamedPropertyStore_INTERFACE_DEFINED__
#define __INamedPropertyStore_INTERFACE_DEFINED__

/* interface INamedPropertyStore */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_INamedPropertyStore;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("71604b0f-97b0-4764-8577-2f13e98a1422")
    INamedPropertyStore : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNamedValue( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszName,
            /* [out] */ __RPC__out PROPVARIANT *ppropvar) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetNamedValue( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszName,
            /* [in] */ __RPC__in REFPROPVARIANT propvar) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNameCount( 
            /* [out] */ __RPC__out DWORD *pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNameAt( 
            /* [in] */ DWORD iProp,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INamedPropertyStoreVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INamedPropertyStore * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INamedPropertyStore * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INamedPropertyStore * This);
        
        DECLSPEC_XFGVIRT(INamedPropertyStore, GetNamedValue)
        HRESULT ( STDMETHODCALLTYPE *GetNamedValue )( 
            __RPC__in INamedPropertyStore * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszName,
            /* [out] */ __RPC__out PROPVARIANT *ppropvar);
        
        DECLSPEC_XFGVIRT(INamedPropertyStore, SetNamedValue)
        HRESULT ( STDMETHODCALLTYPE *SetNamedValue )( 
            __RPC__in INamedPropertyStore * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszName,
            /* [in] */ __RPC__in REFPROPVARIANT propvar);
        
        DECLSPEC_XFGVIRT(INamedPropertyStore, GetNameCount)
        HRESULT ( STDMETHODCALLTYPE *GetNameCount )( 
            __RPC__in INamedPropertyStore * This,
            /* [out] */ __RPC__out DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(INamedPropertyStore, GetNameAt)
        HRESULT ( STDMETHODCALLTYPE *GetNameAt )( 
            __RPC__in INamedPropertyStore * This,
            /* [in] */ DWORD iProp,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        END_INTERFACE
    } INamedPropertyStoreVtbl;

    interface INamedPropertyStore
    {
        CONST_VTBL struct INamedPropertyStoreVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INamedPropertyStore_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INamedPropertyStore_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INamedPropertyStore_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INamedPropertyStore_GetNamedValue(This,pszName,ppropvar)	\
    ( (This)->lpVtbl -> GetNamedValue(This,pszName,ppropvar) ) 

#define INamedPropertyStore_SetNamedValue(This,pszName,propvar)	\
    ( (This)->lpVtbl -> SetNamedValue(This,pszName,propvar) ) 

#define INamedPropertyStore_GetNameCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetNameCount(This,pdwCount) ) 

#define INamedPropertyStore_GetNameAt(This,iProp,pbstrName)	\
    ( (This)->lpVtbl -> GetNameAt(This,iProp,pbstrName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INamedPropertyStore_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_propsys_0000_0004 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum GETPROPERTYSTOREFLAGS
    {
        GPS_DEFAULT	= 0,
        GPS_HANDLERPROPERTIESONLY	= 0x1,
        GPS_READWRITE	= 0x2,
        GPS_TEMPORARY	= 0x4,
        GPS_FASTPROPERTIESONLY	= 0x8,
        GPS_OPENSLOWITEM	= 0x10,
        GPS_DELAYCREATION	= 0x20,
        GPS_BESTEFFORT	= 0x40,
        GPS_NO_OPLOCK	= 0x80,
        GPS_PREFERQUERYPROPERTIES	= 0x100,
        GPS_EXTRINSICPROPERTIES	= 0x200,
        GPS_EXTRINSICPROPERTIESONLY	= 0x400,
        GPS_VOLATILEPROPERTIES	= 0x800,
        GPS_VOLATILEPROPERTIESONLY	= 0x1000,
        GPS_MASK_VALID	= 0x1fff
    } 	GETPROPERTYSTOREFLAGS;

DEFINE_ENUM_FLAG_OPERATORS(GETPROPERTYSTOREFLAGS)


extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0004_v0_0_s_ifspec;

#ifndef __IObjectWithPropertyKey_INTERFACE_DEFINED__
#define __IObjectWithPropertyKey_INTERFACE_DEFINED__

/* interface IObjectWithPropertyKey */
/* [uuid][object] */ 


EXTERN_C const IID IID_IObjectWithPropertyKey;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fc0ca0a7-c316-4fd2-9031-3e628e6d4f23")
    IObjectWithPropertyKey : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetPropertyKey( 
            /* [in] */ __RPC__in REFPROPERTYKEY key) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyKey( 
            /* [out] */ __RPC__out PROPERTYKEY *pkey) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IObjectWithPropertyKeyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IObjectWithPropertyKey * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IObjectWithPropertyKey * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IObjectWithPropertyKey * This);
        
        DECLSPEC_XFGVIRT(IObjectWithPropertyKey, SetPropertyKey)
        HRESULT ( STDMETHODCALLTYPE *SetPropertyKey )( 
            __RPC__in IObjectWithPropertyKey * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key);
        
        DECLSPEC_XFGVIRT(IObjectWithPropertyKey, GetPropertyKey)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyKey )( 
            __RPC__in IObjectWithPropertyKey * This,
            /* [out] */ __RPC__out PROPERTYKEY *pkey);
        
        END_INTERFACE
    } IObjectWithPropertyKeyVtbl;

    interface IObjectWithPropertyKey
    {
        CONST_VTBL struct IObjectWithPropertyKeyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IObjectWithPropertyKey_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IObjectWithPropertyKey_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IObjectWithPropertyKey_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IObjectWithPropertyKey_SetPropertyKey(This,key)	\
    ( (This)->lpVtbl -> SetPropertyKey(This,key) ) 

#define IObjectWithPropertyKey_GetPropertyKey(This,pkey)	\
    ( (This)->lpVtbl -> GetPropertyKey(This,pkey) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IObjectWithPropertyKey_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_propsys_0000_0005 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum PKA_FLAGS
    {
        PKA_SET	= 0,
        PKA_APPEND	= ( PKA_SET + 1 ) ,
        PKA_DELETE	= ( PKA_APPEND + 1 ) 
    } 	PKA_FLAGS;



extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0005_v0_0_s_ifspec;

#ifndef __IPropertyChange_INTERFACE_DEFINED__
#define __IPropertyChange_INTERFACE_DEFINED__

/* interface IPropertyChange */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IPropertyChange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f917bc8a-1bba-4478-a245-1bde03eb9431")
    IPropertyChange : public IObjectWithPropertyKey
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ApplyToPropVariant( 
            /* [in] */ __RPC__in REFPROPVARIANT propvarIn,
            /* [out] */ __RPC__out PROPVARIANT *ppropvarOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertyChangeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertyChange * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertyChange * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertyChange * This);
        
        DECLSPEC_XFGVIRT(IObjectWithPropertyKey, SetPropertyKey)
        HRESULT ( STDMETHODCALLTYPE *SetPropertyKey )( 
            __RPC__in IPropertyChange * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key);
        
        DECLSPEC_XFGVIRT(IObjectWithPropertyKey, GetPropertyKey)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyKey )( 
            __RPC__in IPropertyChange * This,
            /* [out] */ __RPC__out PROPERTYKEY *pkey);
        
        DECLSPEC_XFGVIRT(IPropertyChange, ApplyToPropVariant)
        HRESULT ( STDMETHODCALLTYPE *ApplyToPropVariant )( 
            __RPC__in IPropertyChange * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvarIn,
            /* [out] */ __RPC__out PROPVARIANT *ppropvarOut);
        
        END_INTERFACE
    } IPropertyChangeVtbl;

    interface IPropertyChange
    {
        CONST_VTBL struct IPropertyChangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertyChange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertyChange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertyChange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertyChange_SetPropertyKey(This,key)	\
    ( (This)->lpVtbl -> SetPropertyKey(This,key) ) 

#define IPropertyChange_GetPropertyKey(This,pkey)	\
    ( (This)->lpVtbl -> GetPropertyKey(This,pkey) ) 


#define IPropertyChange_ApplyToPropVariant(This,propvarIn,ppropvarOut)	\
    ( (This)->lpVtbl -> ApplyToPropVariant(This,propvarIn,ppropvarOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertyChange_INTERFACE_DEFINED__ */


#ifndef __IPropertyChangeArray_INTERFACE_DEFINED__
#define __IPropertyChangeArray_INTERFACE_DEFINED__

/* interface IPropertyChangeArray */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IPropertyChangeArray;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("380f5cad-1b5e-42f2-805d-637fd392d31e")
    IPropertyChangeArray : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out UINT *pcOperations) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT iIndex,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertAt( 
            /* [in] */ UINT iIndex,
            /* [in] */ __RPC__in_opt IPropertyChange *ppropChange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in_opt IPropertyChange *ppropChange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AppendOrReplace( 
            /* [in] */ __RPC__in_opt IPropertyChange *ppropChange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            /* [in] */ UINT iIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsKeyInArray( 
            /* [in] */ __RPC__in REFPROPERTYKEY key) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertyChangeArrayVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertyChangeArray * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertyChangeArray * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertyChangeArray * This);
        
        DECLSPEC_XFGVIRT(IPropertyChangeArray, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IPropertyChangeArray * This,
            /* [out] */ __RPC__out UINT *pcOperations);
        
        DECLSPEC_XFGVIRT(IPropertyChangeArray, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IPropertyChangeArray * This,
            /* [in] */ UINT iIndex,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IPropertyChangeArray, InsertAt)
        HRESULT ( STDMETHODCALLTYPE *InsertAt )( 
            __RPC__in IPropertyChangeArray * This,
            /* [in] */ UINT iIndex,
            /* [in] */ __RPC__in_opt IPropertyChange *ppropChange);
        
        DECLSPEC_XFGVIRT(IPropertyChangeArray, Append)
        HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in IPropertyChangeArray * This,
            /* [in] */ __RPC__in_opt IPropertyChange *ppropChange);
        
        DECLSPEC_XFGVIRT(IPropertyChangeArray, AppendOrReplace)
        HRESULT ( STDMETHODCALLTYPE *AppendOrReplace )( 
            __RPC__in IPropertyChangeArray * This,
            /* [in] */ __RPC__in_opt IPropertyChange *ppropChange);
        
        DECLSPEC_XFGVIRT(IPropertyChangeArray, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in IPropertyChangeArray * This,
            /* [in] */ UINT iIndex);
        
        DECLSPEC_XFGVIRT(IPropertyChangeArray, IsKeyInArray)
        HRESULT ( STDMETHODCALLTYPE *IsKeyInArray )( 
            __RPC__in IPropertyChangeArray * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key);
        
        END_INTERFACE
    } IPropertyChangeArrayVtbl;

    interface IPropertyChangeArray
    {
        CONST_VTBL struct IPropertyChangeArrayVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertyChangeArray_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertyChangeArray_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertyChangeArray_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertyChangeArray_GetCount(This,pcOperations)	\
    ( (This)->lpVtbl -> GetCount(This,pcOperations) ) 

#define IPropertyChangeArray_GetAt(This,iIndex,riid,ppv)	\
    ( (This)->lpVtbl -> GetAt(This,iIndex,riid,ppv) ) 

#define IPropertyChangeArray_InsertAt(This,iIndex,ppropChange)	\
    ( (This)->lpVtbl -> InsertAt(This,iIndex,ppropChange) ) 

#define IPropertyChangeArray_Append(This,ppropChange)	\
    ( (This)->lpVtbl -> Append(This,ppropChange) ) 

#define IPropertyChangeArray_AppendOrReplace(This,ppropChange)	\
    ( (This)->lpVtbl -> AppendOrReplace(This,ppropChange) ) 

#define IPropertyChangeArray_RemoveAt(This,iIndex)	\
    ( (This)->lpVtbl -> RemoveAt(This,iIndex) ) 

#define IPropertyChangeArray_IsKeyInArray(This,key)	\
    ( (This)->lpVtbl -> IsKeyInArray(This,key) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertyChangeArray_INTERFACE_DEFINED__ */


#ifndef __IPropertyStoreCapabilities_INTERFACE_DEFINED__
#define __IPropertyStoreCapabilities_INTERFACE_DEFINED__

/* interface IPropertyStoreCapabilities */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IPropertyStoreCapabilities;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c8e2d566-186e-4d49-bf41-6909ead56acc")
    IPropertyStoreCapabilities : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsPropertyWritable( 
            /* [in] */ __RPC__in REFPROPERTYKEY key) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertyStoreCapabilitiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertyStoreCapabilities * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertyStoreCapabilities * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertyStoreCapabilities * This);
        
        DECLSPEC_XFGVIRT(IPropertyStoreCapabilities, IsPropertyWritable)
        HRESULT ( STDMETHODCALLTYPE *IsPropertyWritable )( 
            __RPC__in IPropertyStoreCapabilities * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key);
        
        END_INTERFACE
    } IPropertyStoreCapabilitiesVtbl;

    interface IPropertyStoreCapabilities
    {
        CONST_VTBL struct IPropertyStoreCapabilitiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertyStoreCapabilities_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertyStoreCapabilities_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertyStoreCapabilities_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertyStoreCapabilities_IsPropertyWritable(This,key)	\
    ( (This)->lpVtbl -> IsPropertyWritable(This,key) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertyStoreCapabilities_INTERFACE_DEFINED__ */


#ifndef __IPropertyStoreCache_INTERFACE_DEFINED__
#define __IPropertyStoreCache_INTERFACE_DEFINED__

/* interface IPropertyStoreCache */
/* [unique][object][uuid] */ 

typedef /* [v1_enum] */ 
enum PSC_STATE
    {
        PSC_NORMAL	= 0,
        PSC_NOTINSOURCE	= 1,
        PSC_DIRTY	= 2,
        PSC_READONLY	= 3
    } 	PSC_STATE;


EXTERN_C const IID IID_IPropertyStoreCache;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3017056d-9a91-4e90-937d-746c72abbf4f")
    IPropertyStoreCache : public IPropertyStore
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetState( 
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [out] */ __RPC__out PSC_STATE *pstate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValueAndState( 
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [out] */ __RPC__out PROPVARIANT *ppropvar,
            /* [out] */ __RPC__out PSC_STATE *pstate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetState( 
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [in] */ PSC_STATE state) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValueAndState( 
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *ppropvar,
            /* [in] */ PSC_STATE state) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertyStoreCacheVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertyStoreCache * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertyStoreCache * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertyStoreCache * This);
        
        DECLSPEC_XFGVIRT(IPropertyStore, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IPropertyStoreCache * This,
            /* [out] */ __RPC__out DWORD *cProps);
        
        DECLSPEC_XFGVIRT(IPropertyStore, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IPropertyStoreCache * This,
            /* [in] */ DWORD iProp,
            /* [out] */ __RPC__out PROPERTYKEY *pkey);
        
        DECLSPEC_XFGVIRT(IPropertyStore, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in IPropertyStoreCache * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [out] */ __RPC__out PROPVARIANT *pv);
        
        DECLSPEC_XFGVIRT(IPropertyStore, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in IPropertyStoreCache * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [in] */ __RPC__in REFPROPVARIANT propvar);
        
        DECLSPEC_XFGVIRT(IPropertyStore, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IPropertyStoreCache * This);
        
        DECLSPEC_XFGVIRT(IPropertyStoreCache, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            __RPC__in IPropertyStoreCache * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [out] */ __RPC__out PSC_STATE *pstate);
        
        DECLSPEC_XFGVIRT(IPropertyStoreCache, GetValueAndState)
        HRESULT ( STDMETHODCALLTYPE *GetValueAndState )( 
            __RPC__in IPropertyStoreCache * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [out] */ __RPC__out PROPVARIANT *ppropvar,
            /* [out] */ __RPC__out PSC_STATE *pstate);
        
        DECLSPEC_XFGVIRT(IPropertyStoreCache, SetState)
        HRESULT ( STDMETHODCALLTYPE *SetState )( 
            __RPC__in IPropertyStoreCache * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [in] */ PSC_STATE state);
        
        DECLSPEC_XFGVIRT(IPropertyStoreCache, SetValueAndState)
        HRESULT ( STDMETHODCALLTYPE *SetValueAndState )( 
            __RPC__in IPropertyStoreCache * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *ppropvar,
            /* [in] */ PSC_STATE state);
        
        END_INTERFACE
    } IPropertyStoreCacheVtbl;

    interface IPropertyStoreCache
    {
        CONST_VTBL struct IPropertyStoreCacheVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertyStoreCache_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertyStoreCache_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertyStoreCache_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertyStoreCache_GetCount(This,cProps)	\
    ( (This)->lpVtbl -> GetCount(This,cProps) ) 

#define IPropertyStoreCache_GetAt(This,iProp,pkey)	\
    ( (This)->lpVtbl -> GetAt(This,iProp,pkey) ) 

#define IPropertyStoreCache_GetValue(This,key,pv)	\
    ( (This)->lpVtbl -> GetValue(This,key,pv) ) 

#define IPropertyStoreCache_SetValue(This,key,propvar)	\
    ( (This)->lpVtbl -> SetValue(This,key,propvar) ) 

#define IPropertyStoreCache_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 


#define IPropertyStoreCache_GetState(This,key,pstate)	\
    ( (This)->lpVtbl -> GetState(This,key,pstate) ) 

#define IPropertyStoreCache_GetValueAndState(This,key,ppropvar,pstate)	\
    ( (This)->lpVtbl -> GetValueAndState(This,key,ppropvar,pstate) ) 

#define IPropertyStoreCache_SetState(This,key,state)	\
    ( (This)->lpVtbl -> SetState(This,key,state) ) 

#define IPropertyStoreCache_SetValueAndState(This,key,ppropvar,state)	\
    ( (This)->lpVtbl -> SetValueAndState(This,key,ppropvar,state) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertyStoreCache_INTERFACE_DEFINED__ */


#ifndef __IPropertyEnumType_INTERFACE_DEFINED__
#define __IPropertyEnumType_INTERFACE_DEFINED__

/* interface IPropertyEnumType */
/* [unique][object][uuid] */ 

typedef /* [v1_enum] */ 
enum PROPENUMTYPE
    {
        PET_DISCRETEVALUE	= 0,
        PET_RANGEDVALUE	= 1,
        PET_DEFAULTVALUE	= 2,
        PET_ENDRANGE	= 3
    } 	PROPENUMTYPE;


EXTERN_C const IID IID_IPropertyEnumType;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("11e1fbf9-2d56-4a6b-8db3-7cd193a471f2")
    IPropertyEnumType : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetEnumType( 
            /* [out] */ __RPC__out PROPENUMTYPE *penumtype) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [out] */ __RPC__out PROPVARIANT *ppropvar) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRangeMinValue( 
            /* [out] */ __RPC__out PROPVARIANT *ppropvarMin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRangeSetValue( 
            /* [out] */ __RPC__out PROPVARIANT *ppropvarSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDisplayText( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszDisplay) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertyEnumTypeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertyEnumType * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertyEnumType * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertyEnumType * This);
        
        DECLSPEC_XFGVIRT(IPropertyEnumType, GetEnumType)
        HRESULT ( STDMETHODCALLTYPE *GetEnumType )( 
            __RPC__in IPropertyEnumType * This,
            /* [out] */ __RPC__out PROPENUMTYPE *penumtype);
        
        DECLSPEC_XFGVIRT(IPropertyEnumType, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in IPropertyEnumType * This,
            /* [out] */ __RPC__out PROPVARIANT *ppropvar);
        
        DECLSPEC_XFGVIRT(IPropertyEnumType, GetRangeMinValue)
        HRESULT ( STDMETHODCALLTYPE *GetRangeMinValue )( 
            __RPC__in IPropertyEnumType * This,
            /* [out] */ __RPC__out PROPVARIANT *ppropvarMin);
        
        DECLSPEC_XFGVIRT(IPropertyEnumType, GetRangeSetValue)
        HRESULT ( STDMETHODCALLTYPE *GetRangeSetValue )( 
            __RPC__in IPropertyEnumType * This,
            /* [out] */ __RPC__out PROPVARIANT *ppropvarSet);
        
        DECLSPEC_XFGVIRT(IPropertyEnumType, GetDisplayText)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayText )( 
            __RPC__in IPropertyEnumType * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszDisplay);
        
        END_INTERFACE
    } IPropertyEnumTypeVtbl;

    interface IPropertyEnumType
    {
        CONST_VTBL struct IPropertyEnumTypeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertyEnumType_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertyEnumType_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertyEnumType_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertyEnumType_GetEnumType(This,penumtype)	\
    ( (This)->lpVtbl -> GetEnumType(This,penumtype) ) 

#define IPropertyEnumType_GetValue(This,ppropvar)	\
    ( (This)->lpVtbl -> GetValue(This,ppropvar) ) 

#define IPropertyEnumType_GetRangeMinValue(This,ppropvarMin)	\
    ( (This)->lpVtbl -> GetRangeMinValue(This,ppropvarMin) ) 

#define IPropertyEnumType_GetRangeSetValue(This,ppropvarSet)	\
    ( (This)->lpVtbl -> GetRangeSetValue(This,ppropvarSet) ) 

#define IPropertyEnumType_GetDisplayText(This,ppszDisplay)	\
    ( (This)->lpVtbl -> GetDisplayText(This,ppszDisplay) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertyEnumType_INTERFACE_DEFINED__ */


#ifndef __IPropertyEnumType2_INTERFACE_DEFINED__
#define __IPropertyEnumType2_INTERFACE_DEFINED__

/* interface IPropertyEnumType2 */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IPropertyEnumType2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9b6e051c-5ddd-4321-9070-fe2acb55e794")
    IPropertyEnumType2 : public IPropertyEnumType
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetImageReference( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszImageRes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertyEnumType2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertyEnumType2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertyEnumType2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertyEnumType2 * This);
        
        DECLSPEC_XFGVIRT(IPropertyEnumType, GetEnumType)
        HRESULT ( STDMETHODCALLTYPE *GetEnumType )( 
            __RPC__in IPropertyEnumType2 * This,
            /* [out] */ __RPC__out PROPENUMTYPE *penumtype);
        
        DECLSPEC_XFGVIRT(IPropertyEnumType, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in IPropertyEnumType2 * This,
            /* [out] */ __RPC__out PROPVARIANT *ppropvar);
        
        DECLSPEC_XFGVIRT(IPropertyEnumType, GetRangeMinValue)
        HRESULT ( STDMETHODCALLTYPE *GetRangeMinValue )( 
            __RPC__in IPropertyEnumType2 * This,
            /* [out] */ __RPC__out PROPVARIANT *ppropvarMin);
        
        DECLSPEC_XFGVIRT(IPropertyEnumType, GetRangeSetValue)
        HRESULT ( STDMETHODCALLTYPE *GetRangeSetValue )( 
            __RPC__in IPropertyEnumType2 * This,
            /* [out] */ __RPC__out PROPVARIANT *ppropvarSet);
        
        DECLSPEC_XFGVIRT(IPropertyEnumType, GetDisplayText)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayText )( 
            __RPC__in IPropertyEnumType2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszDisplay);
        
        DECLSPEC_XFGVIRT(IPropertyEnumType2, GetImageReference)
        HRESULT ( STDMETHODCALLTYPE *GetImageReference )( 
            __RPC__in IPropertyEnumType2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszImageRes);
        
        END_INTERFACE
    } IPropertyEnumType2Vtbl;

    interface IPropertyEnumType2
    {
        CONST_VTBL struct IPropertyEnumType2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertyEnumType2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertyEnumType2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertyEnumType2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertyEnumType2_GetEnumType(This,penumtype)	\
    ( (This)->lpVtbl -> GetEnumType(This,penumtype) ) 

#define IPropertyEnumType2_GetValue(This,ppropvar)	\
    ( (This)->lpVtbl -> GetValue(This,ppropvar) ) 

#define IPropertyEnumType2_GetRangeMinValue(This,ppropvarMin)	\
    ( (This)->lpVtbl -> GetRangeMinValue(This,ppropvarMin) ) 

#define IPropertyEnumType2_GetRangeSetValue(This,ppropvarSet)	\
    ( (This)->lpVtbl -> GetRangeSetValue(This,ppropvarSet) ) 

#define IPropertyEnumType2_GetDisplayText(This,ppszDisplay)	\
    ( (This)->lpVtbl -> GetDisplayText(This,ppszDisplay) ) 


#define IPropertyEnumType2_GetImageReference(This,ppszImageRes)	\
    ( (This)->lpVtbl -> GetImageReference(This,ppszImageRes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertyEnumType2_INTERFACE_DEFINED__ */


#ifndef __IPropertyEnumTypeList_INTERFACE_DEFINED__
#define __IPropertyEnumTypeList_INTERFACE_DEFINED__

/* interface IPropertyEnumTypeList */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IPropertyEnumTypeList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a99400f4-3d84-4557-94ba-1242fb2cc9a6")
    IPropertyEnumTypeList : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out UINT *pctypes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT itype,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConditionAt( 
            /* [in] */ UINT nIndex,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindMatchingIndex( 
            /* [in] */ __RPC__in REFPROPVARIANT propvarCmp,
            /* [out] */ __RPC__out UINT *pnIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertyEnumTypeListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertyEnumTypeList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertyEnumTypeList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertyEnumTypeList * This);
        
        DECLSPEC_XFGVIRT(IPropertyEnumTypeList, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IPropertyEnumTypeList * This,
            /* [out] */ __RPC__out UINT *pctypes);
        
        DECLSPEC_XFGVIRT(IPropertyEnumTypeList, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IPropertyEnumTypeList * This,
            /* [in] */ UINT itype,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IPropertyEnumTypeList, GetConditionAt)
        HRESULT ( STDMETHODCALLTYPE *GetConditionAt )( 
            __RPC__in IPropertyEnumTypeList * This,
            /* [in] */ UINT nIndex,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IPropertyEnumTypeList, FindMatchingIndex)
        HRESULT ( STDMETHODCALLTYPE *FindMatchingIndex )( 
            __RPC__in IPropertyEnumTypeList * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvarCmp,
            /* [out] */ __RPC__out UINT *pnIndex);
        
        END_INTERFACE
    } IPropertyEnumTypeListVtbl;

    interface IPropertyEnumTypeList
    {
        CONST_VTBL struct IPropertyEnumTypeListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertyEnumTypeList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertyEnumTypeList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertyEnumTypeList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertyEnumTypeList_GetCount(This,pctypes)	\
    ( (This)->lpVtbl -> GetCount(This,pctypes) ) 

#define IPropertyEnumTypeList_GetAt(This,itype,riid,ppv)	\
    ( (This)->lpVtbl -> GetAt(This,itype,riid,ppv) ) 

#define IPropertyEnumTypeList_GetConditionAt(This,nIndex,riid,ppv)	\
    ( (This)->lpVtbl -> GetConditionAt(This,nIndex,riid,ppv) ) 

#define IPropertyEnumTypeList_FindMatchingIndex(This,propvarCmp,pnIndex)	\
    ( (This)->lpVtbl -> FindMatchingIndex(This,propvarCmp,pnIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertyEnumTypeList_INTERFACE_DEFINED__ */


#ifndef __IPropertyDescription_INTERFACE_DEFINED__
#define __IPropertyDescription_INTERFACE_DEFINED__

/* interface IPropertyDescription */
/* [unique][object][uuid] */ 

typedef /* [v1_enum] */ 
enum PROPDESC_TYPE_FLAGS
    {
        PDTF_DEFAULT	= 0,
        PDTF_MULTIPLEVALUES	= 0x1,
        PDTF_ISINNATE	= 0x2,
        PDTF_ISGROUP	= 0x4,
        PDTF_CANGROUPBY	= 0x8,
        PDTF_CANSTACKBY	= 0x10,
        PDTF_ISTREEPROPERTY	= 0x20,
        PDTF_INCLUDEINFULLTEXTQUERY	= 0x40,
        PDTF_ISVIEWABLE	= 0x80,
        PDTF_ISQUERYABLE	= 0x100,
        PDTF_CANBEPURGED	= 0x200,
        PDTF_SEARCHRAWVALUE	= 0x400,
        PDTF_DONTCOERCEEMPTYSTRINGS	= 0x800,
        PDTF_ALWAYSINSUPPLEMENTALSTORE	= 0x1000,
        PDTF_ISSYSTEMPROPERTY	= 0x80000000,
        PDTF_MASK_ALL	= 0x80001fff
    } 	PROPDESC_TYPE_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS(PROPDESC_TYPE_FLAGS)
typedef /* [v1_enum] */ 
enum PROPDESC_VIEW_FLAGS
    {
        PDVF_DEFAULT	= 0,
        PDVF_CENTERALIGN	= 0x1,
        PDVF_RIGHTALIGN	= 0x2,
        PDVF_BEGINNEWGROUP	= 0x4,
        PDVF_FILLAREA	= 0x8,
        PDVF_SORTDESCENDING	= 0x10,
        PDVF_SHOWONLYIFPRESENT	= 0x20,
        PDVF_SHOWBYDEFAULT	= 0x40,
        PDVF_SHOWINPRIMARYLIST	= 0x80,
        PDVF_SHOWINSECONDARYLIST	= 0x100,
        PDVF_HIDELABEL	= 0x200,
        PDVF_HIDDEN	= 0x800,
        PDVF_CANWRAP	= 0x1000,
        PDVF_MASK_ALL	= 0x1bff
    } 	PROPDESC_VIEW_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS(PROPDESC_VIEW_FLAGS)
typedef /* [v1_enum] */ 
enum PROPDESC_DISPLAYTYPE
    {
        PDDT_STRING	= 0,
        PDDT_NUMBER	= 1,
        PDDT_BOOLEAN	= 2,
        PDDT_DATETIME	= 3,
        PDDT_ENUMERATED	= 4
    } 	PROPDESC_DISPLAYTYPE;

typedef /* [v1_enum] */ 
enum PROPDESC_GROUPING_RANGE
    {
        PDGR_DISCRETE	= 0,
        PDGR_ALPHANUMERIC	= 1,
        PDGR_SIZE	= 2,
        PDGR_DYNAMIC	= 3,
        PDGR_DATE	= 4,
        PDGR_PERCENT	= 5,
        PDGR_ENUMERATED	= 6
    } 	PROPDESC_GROUPING_RANGE;

typedef /* [v1_enum] */ 
enum PROPDESC_FORMAT_FLAGS
    {
        PDFF_DEFAULT	= 0,
        PDFF_PREFIXNAME	= 0x1,
        PDFF_FILENAME	= 0x2,
        PDFF_ALWAYSKB	= 0x4,
        PDFF_RESERVED_RIGHTTOLEFT	= 0x8,
        PDFF_SHORTTIME	= 0x10,
        PDFF_LONGTIME	= 0x20,
        PDFF_HIDETIME	= 0x40,
        PDFF_SHORTDATE	= 0x80,
        PDFF_LONGDATE	= 0x100,
        PDFF_HIDEDATE	= 0x200,
        PDFF_RELATIVEDATE	= 0x400,
        PDFF_USEEDITINVITATION	= 0x800,
        PDFF_READONLY	= 0x1000,
        PDFF_NOAUTOREADINGORDER	= 0x2000
    } 	PROPDESC_FORMAT_FLAGS;

#define PDFF_PREFERFRIENDLY    static_cast<PROPDESC_FORMAT_FLAGS>(0x10000000) // Use even friendlier date/time descriptions ;internal
DEFINE_ENUM_FLAG_OPERATORS(PROPDESC_FORMAT_FLAGS)
typedef /* [v1_enum] */ 
enum PROPDESC_SORTDESCRIPTION
    {
        PDSD_GENERAL	= 0,
        PDSD_A_Z	= 1,
        PDSD_LOWEST_HIGHEST	= 2,
        PDSD_SMALLEST_BIGGEST	= 3,
        PDSD_OLDEST_NEWEST	= 4
    } 	PROPDESC_SORTDESCRIPTION;

typedef /* [v1_enum] */ 
enum PROPDESC_RELATIVEDESCRIPTION_TYPE
    {
        PDRDT_GENERAL	= 0,
        PDRDT_DATE	= 1,
        PDRDT_SIZE	= 2,
        PDRDT_COUNT	= 3,
        PDRDT_REVISION	= 4,
        PDRDT_LENGTH	= 5,
        PDRDT_DURATION	= 6,
        PDRDT_SPEED	= 7,
        PDRDT_RATE	= 8,
        PDRDT_RATING	= 9,
        PDRDT_PRIORITY	= 10
    } 	PROPDESC_RELATIVEDESCRIPTION_TYPE;

typedef /* [v1_enum] */ 
enum PROPDESC_AGGREGATION_TYPE
    {
        PDAT_DEFAULT	= 0,
        PDAT_FIRST	= 1,
        PDAT_SUM	= 2,
        PDAT_AVERAGE	= 3,
        PDAT_DATERANGE	= 4,
        PDAT_UNION	= 5,
        PDAT_MAX	= 6,
        PDAT_MIN	= 7
    } 	PROPDESC_AGGREGATION_TYPE;

typedef /* [v1_enum] */ 
enum PROPDESC_CONDITION_TYPE
    {
        PDCOT_NONE	= 0,
        PDCOT_STRING	= 1,
        PDCOT_SIZE	= 2,
        PDCOT_DATETIME	= 3,
        PDCOT_BOOLEAN	= 4,
        PDCOT_NUMBER	= 5
    } 	PROPDESC_CONDITION_TYPE;


EXTERN_C const IID IID_IPropertyDescription;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6f79d558-3e96-4549-a1d1-7d75d2288814")
    IPropertyDescription : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPropertyKey( 
            /* [out] */ __RPC__out PROPERTYKEY *pkey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCanonicalName( 
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyType( 
            /* [out] */ __RPC__out VARTYPE *pvartype) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDisplayName( 
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEditInvitation( 
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszInvite) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTypeFlags( 
            /* [in] */ PROPDESC_TYPE_FLAGS mask,
            /* [out] */ __RPC__out PROPDESC_TYPE_FLAGS *ppdtFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetViewFlags( 
            /* [out] */ __RPC__out PROPDESC_VIEW_FLAGS *ppdvFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultColumnWidth( 
            /* [out] */ __RPC__out UINT *pcxChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDisplayType( 
            /* [out] */ __RPC__out PROPDESC_DISPLAYTYPE *pdisplaytype) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColumnState( 
            /* [out] */ __RPC__out SHCOLSTATEF *pcsFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGroupingRange( 
            /* [out] */ __RPC__out PROPDESC_GROUPING_RANGE *pgr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRelativeDescriptionType( 
            /* [out] */ __RPC__out PROPDESC_RELATIVEDESCRIPTION_TYPE *prdt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRelativeDescription( 
            /* [in] */ __RPC__in REFPROPVARIANT propvar1,
            /* [in] */ __RPC__in REFPROPVARIANT propvar2,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDesc1,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDesc2) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSortDescription( 
            /* [out] */ __RPC__out PROPDESC_SORTDESCRIPTION *psd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSortDescriptionLabel( 
            /* [in] */ BOOL fDescending,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAggregationType( 
            /* [out] */ __RPC__out PROPDESC_AGGREGATION_TYPE *paggtype) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConditionType( 
            /* [out] */ __RPC__out PROPDESC_CONDITION_TYPE *pcontype,
            /* [out] */ __RPC__out CONDITION_OPERATION *popDefault) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnumTypeList( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE CoerceToCanonicalValue( 
            /* [annotation][out][in] */ 
            _Inout_  PROPVARIANT *ppropvar) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FormatForDisplay( 
            /* [in] */ __RPC__in REFPROPVARIANT propvar,
            /* [in] */ PROPDESC_FORMAT_FLAGS pdfFlags,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDisplay) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsValueCanonical( 
            /* [in] */ __RPC__in REFPROPVARIANT propvar) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertyDescriptionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertyDescription * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertyDescription * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertyDescription * This);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetPropertyKey)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyKey )( 
            __RPC__in IPropertyDescription * This,
            /* [out] */ __RPC__out PROPERTYKEY *pkey);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetCanonicalName)
        HRESULT ( STDMETHODCALLTYPE *GetCanonicalName )( 
            __RPC__in IPropertyDescription * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszName);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetPropertyType)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyType )( 
            __RPC__in IPropertyDescription * This,
            /* [out] */ __RPC__out VARTYPE *pvartype);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            __RPC__in IPropertyDescription * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszName);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetEditInvitation)
        HRESULT ( STDMETHODCALLTYPE *GetEditInvitation )( 
            __RPC__in IPropertyDescription * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszInvite);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetTypeFlags)
        HRESULT ( STDMETHODCALLTYPE *GetTypeFlags )( 
            __RPC__in IPropertyDescription * This,
            /* [in] */ PROPDESC_TYPE_FLAGS mask,
            /* [out] */ __RPC__out PROPDESC_TYPE_FLAGS *ppdtFlags);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetViewFlags)
        HRESULT ( STDMETHODCALLTYPE *GetViewFlags )( 
            __RPC__in IPropertyDescription * This,
            /* [out] */ __RPC__out PROPDESC_VIEW_FLAGS *ppdvFlags);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetDefaultColumnWidth)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultColumnWidth )( 
            __RPC__in IPropertyDescription * This,
            /* [out] */ __RPC__out UINT *pcxChars);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetDisplayType)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayType )( 
            __RPC__in IPropertyDescription * This,
            /* [out] */ __RPC__out PROPDESC_DISPLAYTYPE *pdisplaytype);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetColumnState)
        HRESULT ( STDMETHODCALLTYPE *GetColumnState )( 
            __RPC__in IPropertyDescription * This,
            /* [out] */ __RPC__out SHCOLSTATEF *pcsFlags);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetGroupingRange)
        HRESULT ( STDMETHODCALLTYPE *GetGroupingRange )( 
            __RPC__in IPropertyDescription * This,
            /* [out] */ __RPC__out PROPDESC_GROUPING_RANGE *pgr);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetRelativeDescriptionType)
        HRESULT ( STDMETHODCALLTYPE *GetRelativeDescriptionType )( 
            __RPC__in IPropertyDescription * This,
            /* [out] */ __RPC__out PROPDESC_RELATIVEDESCRIPTION_TYPE *prdt);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetRelativeDescription)
        HRESULT ( STDMETHODCALLTYPE *GetRelativeDescription )( 
            __RPC__in IPropertyDescription * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvar1,
            /* [in] */ __RPC__in REFPROPVARIANT propvar2,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDesc1,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDesc2);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetSortDescription)
        HRESULT ( STDMETHODCALLTYPE *GetSortDescription )( 
            __RPC__in IPropertyDescription * This,
            /* [out] */ __RPC__out PROPDESC_SORTDESCRIPTION *psd);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetSortDescriptionLabel)
        HRESULT ( STDMETHODCALLTYPE *GetSortDescriptionLabel )( 
            __RPC__in IPropertyDescription * This,
            /* [in] */ BOOL fDescending,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDescription);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetAggregationType)
        HRESULT ( STDMETHODCALLTYPE *GetAggregationType )( 
            __RPC__in IPropertyDescription * This,
            /* [out] */ __RPC__out PROPDESC_AGGREGATION_TYPE *paggtype);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetConditionType)
        HRESULT ( STDMETHODCALLTYPE *GetConditionType )( 
            __RPC__in IPropertyDescription * This,
            /* [out] */ __RPC__out PROPDESC_CONDITION_TYPE *pcontype,
            /* [out] */ __RPC__out CONDITION_OPERATION *popDefault);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetEnumTypeList)
        HRESULT ( STDMETHODCALLTYPE *GetEnumTypeList )( 
            __RPC__in IPropertyDescription * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, CoerceToCanonicalValue)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CoerceToCanonicalValue )( 
            IPropertyDescription * This,
            /* [annotation][out][in] */ 
            _Inout_  PROPVARIANT *ppropvar);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, FormatForDisplay)
        HRESULT ( STDMETHODCALLTYPE *FormatForDisplay )( 
            __RPC__in IPropertyDescription * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvar,
            /* [in] */ PROPDESC_FORMAT_FLAGS pdfFlags,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDisplay);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, IsValueCanonical)
        HRESULT ( STDMETHODCALLTYPE *IsValueCanonical )( 
            __RPC__in IPropertyDescription * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvar);
        
        END_INTERFACE
    } IPropertyDescriptionVtbl;

    interface IPropertyDescription
    {
        CONST_VTBL struct IPropertyDescriptionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertyDescription_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertyDescription_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertyDescription_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertyDescription_GetPropertyKey(This,pkey)	\
    ( (This)->lpVtbl -> GetPropertyKey(This,pkey) ) 

#define IPropertyDescription_GetCanonicalName(This,ppszName)	\
    ( (This)->lpVtbl -> GetCanonicalName(This,ppszName) ) 

#define IPropertyDescription_GetPropertyType(This,pvartype)	\
    ( (This)->lpVtbl -> GetPropertyType(This,pvartype) ) 

#define IPropertyDescription_GetDisplayName(This,ppszName)	\
    ( (This)->lpVtbl -> GetDisplayName(This,ppszName) ) 

#define IPropertyDescription_GetEditInvitation(This,ppszInvite)	\
    ( (This)->lpVtbl -> GetEditInvitation(This,ppszInvite) ) 

#define IPropertyDescription_GetTypeFlags(This,mask,ppdtFlags)	\
    ( (This)->lpVtbl -> GetTypeFlags(This,mask,ppdtFlags) ) 

#define IPropertyDescription_GetViewFlags(This,ppdvFlags)	\
    ( (This)->lpVtbl -> GetViewFlags(This,ppdvFlags) ) 

#define IPropertyDescription_GetDefaultColumnWidth(This,pcxChars)	\
    ( (This)->lpVtbl -> GetDefaultColumnWidth(This,pcxChars) ) 

#define IPropertyDescription_GetDisplayType(This,pdisplaytype)	\
    ( (This)->lpVtbl -> GetDisplayType(This,pdisplaytype) ) 

#define IPropertyDescription_GetColumnState(This,pcsFlags)	\
    ( (This)->lpVtbl -> GetColumnState(This,pcsFlags) ) 

#define IPropertyDescription_GetGroupingRange(This,pgr)	\
    ( (This)->lpVtbl -> GetGroupingRange(This,pgr) ) 

#define IPropertyDescription_GetRelativeDescriptionType(This,prdt)	\
    ( (This)->lpVtbl -> GetRelativeDescriptionType(This,prdt) ) 

#define IPropertyDescription_GetRelativeDescription(This,propvar1,propvar2,ppszDesc1,ppszDesc2)	\
    ( (This)->lpVtbl -> GetRelativeDescription(This,propvar1,propvar2,ppszDesc1,ppszDesc2) ) 

#define IPropertyDescription_GetSortDescription(This,psd)	\
    ( (This)->lpVtbl -> GetSortDescription(This,psd) ) 

#define IPropertyDescription_GetSortDescriptionLabel(This,fDescending,ppszDescription)	\
    ( (This)->lpVtbl -> GetSortDescriptionLabel(This,fDescending,ppszDescription) ) 

#define IPropertyDescription_GetAggregationType(This,paggtype)	\
    ( (This)->lpVtbl -> GetAggregationType(This,paggtype) ) 

#define IPropertyDescription_GetConditionType(This,pcontype,popDefault)	\
    ( (This)->lpVtbl -> GetConditionType(This,pcontype,popDefault) ) 

#define IPropertyDescription_GetEnumTypeList(This,riid,ppv)	\
    ( (This)->lpVtbl -> GetEnumTypeList(This,riid,ppv) ) 

#define IPropertyDescription_CoerceToCanonicalValue(This,ppropvar)	\
    ( (This)->lpVtbl -> CoerceToCanonicalValue(This,ppropvar) ) 

#define IPropertyDescription_FormatForDisplay(This,propvar,pdfFlags,ppszDisplay)	\
    ( (This)->lpVtbl -> FormatForDisplay(This,propvar,pdfFlags,ppszDisplay) ) 

#define IPropertyDescription_IsValueCanonical(This,propvar)	\
    ( (This)->lpVtbl -> IsValueCanonical(This,propvar) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IPropertyDescription_RemoteCoerceToCanonicalValue_Proxy( 
    __RPC__in IPropertyDescription * This,
    /* [in] */ __RPC__in REFPROPVARIANT propvar,
    /* [out] */ __RPC__out PROPVARIANT *ppropvar);


void __RPC_STUB IPropertyDescription_RemoteCoerceToCanonicalValue_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IPropertyDescription_INTERFACE_DEFINED__ */


#ifndef __IPropertyDescription2_INTERFACE_DEFINED__
#define __IPropertyDescription2_INTERFACE_DEFINED__

/* interface IPropertyDescription2 */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IPropertyDescription2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("57d2eded-5062-400e-b107-5dae79fe57a6")
    IPropertyDescription2 : public IPropertyDescription
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetImageReferenceForValue( 
            /* [in] */ __RPC__in REFPROPVARIANT propvar,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszImageRes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertyDescription2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertyDescription2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertyDescription2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertyDescription2 * This);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetPropertyKey)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyKey )( 
            __RPC__in IPropertyDescription2 * This,
            /* [out] */ __RPC__out PROPERTYKEY *pkey);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetCanonicalName)
        HRESULT ( STDMETHODCALLTYPE *GetCanonicalName )( 
            __RPC__in IPropertyDescription2 * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszName);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetPropertyType)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyType )( 
            __RPC__in IPropertyDescription2 * This,
            /* [out] */ __RPC__out VARTYPE *pvartype);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            __RPC__in IPropertyDescription2 * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszName);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetEditInvitation)
        HRESULT ( STDMETHODCALLTYPE *GetEditInvitation )( 
            __RPC__in IPropertyDescription2 * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszInvite);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetTypeFlags)
        HRESULT ( STDMETHODCALLTYPE *GetTypeFlags )( 
            __RPC__in IPropertyDescription2 * This,
            /* [in] */ PROPDESC_TYPE_FLAGS mask,
            /* [out] */ __RPC__out PROPDESC_TYPE_FLAGS *ppdtFlags);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetViewFlags)
        HRESULT ( STDMETHODCALLTYPE *GetViewFlags )( 
            __RPC__in IPropertyDescription2 * This,
            /* [out] */ __RPC__out PROPDESC_VIEW_FLAGS *ppdvFlags);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetDefaultColumnWidth)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultColumnWidth )( 
            __RPC__in IPropertyDescription2 * This,
            /* [out] */ __RPC__out UINT *pcxChars);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetDisplayType)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayType )( 
            __RPC__in IPropertyDescription2 * This,
            /* [out] */ __RPC__out PROPDESC_DISPLAYTYPE *pdisplaytype);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetColumnState)
        HRESULT ( STDMETHODCALLTYPE *GetColumnState )( 
            __RPC__in IPropertyDescription2 * This,
            /* [out] */ __RPC__out SHCOLSTATEF *pcsFlags);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetGroupingRange)
        HRESULT ( STDMETHODCALLTYPE *GetGroupingRange )( 
            __RPC__in IPropertyDescription2 * This,
            /* [out] */ __RPC__out PROPDESC_GROUPING_RANGE *pgr);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetRelativeDescriptionType)
        HRESULT ( STDMETHODCALLTYPE *GetRelativeDescriptionType )( 
            __RPC__in IPropertyDescription2 * This,
            /* [out] */ __RPC__out PROPDESC_RELATIVEDESCRIPTION_TYPE *prdt);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetRelativeDescription)
        HRESULT ( STDMETHODCALLTYPE *GetRelativeDescription )( 
            __RPC__in IPropertyDescription2 * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvar1,
            /* [in] */ __RPC__in REFPROPVARIANT propvar2,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDesc1,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDesc2);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetSortDescription)
        HRESULT ( STDMETHODCALLTYPE *GetSortDescription )( 
            __RPC__in IPropertyDescription2 * This,
            /* [out] */ __RPC__out PROPDESC_SORTDESCRIPTION *psd);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetSortDescriptionLabel)
        HRESULT ( STDMETHODCALLTYPE *GetSortDescriptionLabel )( 
            __RPC__in IPropertyDescription2 * This,
            /* [in] */ BOOL fDescending,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDescription);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetAggregationType)
        HRESULT ( STDMETHODCALLTYPE *GetAggregationType )( 
            __RPC__in IPropertyDescription2 * This,
            /* [out] */ __RPC__out PROPDESC_AGGREGATION_TYPE *paggtype);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetConditionType)
        HRESULT ( STDMETHODCALLTYPE *GetConditionType )( 
            __RPC__in IPropertyDescription2 * This,
            /* [out] */ __RPC__out PROPDESC_CONDITION_TYPE *pcontype,
            /* [out] */ __RPC__out CONDITION_OPERATION *popDefault);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetEnumTypeList)
        HRESULT ( STDMETHODCALLTYPE *GetEnumTypeList )( 
            __RPC__in IPropertyDescription2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, CoerceToCanonicalValue)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CoerceToCanonicalValue )( 
            IPropertyDescription2 * This,
            /* [annotation][out][in] */ 
            _Inout_  PROPVARIANT *ppropvar);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, FormatForDisplay)
        HRESULT ( STDMETHODCALLTYPE *FormatForDisplay )( 
            __RPC__in IPropertyDescription2 * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvar,
            /* [in] */ PROPDESC_FORMAT_FLAGS pdfFlags,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDisplay);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, IsValueCanonical)
        HRESULT ( STDMETHODCALLTYPE *IsValueCanonical )( 
            __RPC__in IPropertyDescription2 * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvar);
        
        DECLSPEC_XFGVIRT(IPropertyDescription2, GetImageReferenceForValue)
        HRESULT ( STDMETHODCALLTYPE *GetImageReferenceForValue )( 
            __RPC__in IPropertyDescription2 * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvar,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszImageRes);
        
        END_INTERFACE
    } IPropertyDescription2Vtbl;

    interface IPropertyDescription2
    {
        CONST_VTBL struct IPropertyDescription2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertyDescription2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertyDescription2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertyDescription2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertyDescription2_GetPropertyKey(This,pkey)	\
    ( (This)->lpVtbl -> GetPropertyKey(This,pkey) ) 

#define IPropertyDescription2_GetCanonicalName(This,ppszName)	\
    ( (This)->lpVtbl -> GetCanonicalName(This,ppszName) ) 

#define IPropertyDescription2_GetPropertyType(This,pvartype)	\
    ( (This)->lpVtbl -> GetPropertyType(This,pvartype) ) 

#define IPropertyDescription2_GetDisplayName(This,ppszName)	\
    ( (This)->lpVtbl -> GetDisplayName(This,ppszName) ) 

#define IPropertyDescription2_GetEditInvitation(This,ppszInvite)	\
    ( (This)->lpVtbl -> GetEditInvitation(This,ppszInvite) ) 

#define IPropertyDescription2_GetTypeFlags(This,mask,ppdtFlags)	\
    ( (This)->lpVtbl -> GetTypeFlags(This,mask,ppdtFlags) ) 

#define IPropertyDescription2_GetViewFlags(This,ppdvFlags)	\
    ( (This)->lpVtbl -> GetViewFlags(This,ppdvFlags) ) 

#define IPropertyDescription2_GetDefaultColumnWidth(This,pcxChars)	\
    ( (This)->lpVtbl -> GetDefaultColumnWidth(This,pcxChars) ) 

#define IPropertyDescription2_GetDisplayType(This,pdisplaytype)	\
    ( (This)->lpVtbl -> GetDisplayType(This,pdisplaytype) ) 

#define IPropertyDescription2_GetColumnState(This,pcsFlags)	\
    ( (This)->lpVtbl -> GetColumnState(This,pcsFlags) ) 

#define IPropertyDescription2_GetGroupingRange(This,pgr)	\
    ( (This)->lpVtbl -> GetGroupingRange(This,pgr) ) 

#define IPropertyDescription2_GetRelativeDescriptionType(This,prdt)	\
    ( (This)->lpVtbl -> GetRelativeDescriptionType(This,prdt) ) 

#define IPropertyDescription2_GetRelativeDescription(This,propvar1,propvar2,ppszDesc1,ppszDesc2)	\
    ( (This)->lpVtbl -> GetRelativeDescription(This,propvar1,propvar2,ppszDesc1,ppszDesc2) ) 

#define IPropertyDescription2_GetSortDescription(This,psd)	\
    ( (This)->lpVtbl -> GetSortDescription(This,psd) ) 

#define IPropertyDescription2_GetSortDescriptionLabel(This,fDescending,ppszDescription)	\
    ( (This)->lpVtbl -> GetSortDescriptionLabel(This,fDescending,ppszDescription) ) 

#define IPropertyDescription2_GetAggregationType(This,paggtype)	\
    ( (This)->lpVtbl -> GetAggregationType(This,paggtype) ) 

#define IPropertyDescription2_GetConditionType(This,pcontype,popDefault)	\
    ( (This)->lpVtbl -> GetConditionType(This,pcontype,popDefault) ) 

#define IPropertyDescription2_GetEnumTypeList(This,riid,ppv)	\
    ( (This)->lpVtbl -> GetEnumTypeList(This,riid,ppv) ) 

#define IPropertyDescription2_CoerceToCanonicalValue(This,ppropvar)	\
    ( (This)->lpVtbl -> CoerceToCanonicalValue(This,ppropvar) ) 

#define IPropertyDescription2_FormatForDisplay(This,propvar,pdfFlags,ppszDisplay)	\
    ( (This)->lpVtbl -> FormatForDisplay(This,propvar,pdfFlags,ppszDisplay) ) 

#define IPropertyDescription2_IsValueCanonical(This,propvar)	\
    ( (This)->lpVtbl -> IsValueCanonical(This,propvar) ) 


#define IPropertyDescription2_GetImageReferenceForValue(This,propvar,ppszImageRes)	\
    ( (This)->lpVtbl -> GetImageReferenceForValue(This,propvar,ppszImageRes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertyDescription2_INTERFACE_DEFINED__ */


#ifndef __IPropertyDescriptionAliasInfo_INTERFACE_DEFINED__
#define __IPropertyDescriptionAliasInfo_INTERFACE_DEFINED__

/* interface IPropertyDescriptionAliasInfo */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IPropertyDescriptionAliasInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f67104fc-2af9-46fd-b32d-243c1404f3d1")
    IPropertyDescriptionAliasInfo : public IPropertyDescription
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSortByAlias( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAdditionalSortByAliases( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertyDescriptionAliasInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertyDescriptionAliasInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertyDescriptionAliasInfo * This);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetPropertyKey)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyKey )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [out] */ __RPC__out PROPERTYKEY *pkey);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetCanonicalName)
        HRESULT ( STDMETHODCALLTYPE *GetCanonicalName )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszName);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetPropertyType)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyType )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [out] */ __RPC__out VARTYPE *pvartype);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszName);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetEditInvitation)
        HRESULT ( STDMETHODCALLTYPE *GetEditInvitation )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszInvite);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetTypeFlags)
        HRESULT ( STDMETHODCALLTYPE *GetTypeFlags )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [in] */ PROPDESC_TYPE_FLAGS mask,
            /* [out] */ __RPC__out PROPDESC_TYPE_FLAGS *ppdtFlags);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetViewFlags)
        HRESULT ( STDMETHODCALLTYPE *GetViewFlags )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [out] */ __RPC__out PROPDESC_VIEW_FLAGS *ppdvFlags);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetDefaultColumnWidth)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultColumnWidth )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [out] */ __RPC__out UINT *pcxChars);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetDisplayType)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayType )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [out] */ __RPC__out PROPDESC_DISPLAYTYPE *pdisplaytype);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetColumnState)
        HRESULT ( STDMETHODCALLTYPE *GetColumnState )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [out] */ __RPC__out SHCOLSTATEF *pcsFlags);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetGroupingRange)
        HRESULT ( STDMETHODCALLTYPE *GetGroupingRange )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [out] */ __RPC__out PROPDESC_GROUPING_RANGE *pgr);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetRelativeDescriptionType)
        HRESULT ( STDMETHODCALLTYPE *GetRelativeDescriptionType )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [out] */ __RPC__out PROPDESC_RELATIVEDESCRIPTION_TYPE *prdt);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetRelativeDescription)
        HRESULT ( STDMETHODCALLTYPE *GetRelativeDescription )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvar1,
            /* [in] */ __RPC__in REFPROPVARIANT propvar2,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDesc1,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDesc2);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetSortDescription)
        HRESULT ( STDMETHODCALLTYPE *GetSortDescription )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [out] */ __RPC__out PROPDESC_SORTDESCRIPTION *psd);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetSortDescriptionLabel)
        HRESULT ( STDMETHODCALLTYPE *GetSortDescriptionLabel )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [in] */ BOOL fDescending,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDescription);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetAggregationType)
        HRESULT ( STDMETHODCALLTYPE *GetAggregationType )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [out] */ __RPC__out PROPDESC_AGGREGATION_TYPE *paggtype);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetConditionType)
        HRESULT ( STDMETHODCALLTYPE *GetConditionType )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [out] */ __RPC__out PROPDESC_CONDITION_TYPE *pcontype,
            /* [out] */ __RPC__out CONDITION_OPERATION *popDefault);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetEnumTypeList)
        HRESULT ( STDMETHODCALLTYPE *GetEnumTypeList )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, CoerceToCanonicalValue)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CoerceToCanonicalValue )( 
            IPropertyDescriptionAliasInfo * This,
            /* [annotation][out][in] */ 
            _Inout_  PROPVARIANT *ppropvar);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, FormatForDisplay)
        HRESULT ( STDMETHODCALLTYPE *FormatForDisplay )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvar,
            /* [in] */ PROPDESC_FORMAT_FLAGS pdfFlags,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDisplay);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, IsValueCanonical)
        HRESULT ( STDMETHODCALLTYPE *IsValueCanonical )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvar);
        
        DECLSPEC_XFGVIRT(IPropertyDescriptionAliasInfo, GetSortByAlias)
        HRESULT ( STDMETHODCALLTYPE *GetSortByAlias )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IPropertyDescriptionAliasInfo, GetAdditionalSortByAliases)
        HRESULT ( STDMETHODCALLTYPE *GetAdditionalSortByAliases )( 
            __RPC__in IPropertyDescriptionAliasInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        END_INTERFACE
    } IPropertyDescriptionAliasInfoVtbl;

    interface IPropertyDescriptionAliasInfo
    {
        CONST_VTBL struct IPropertyDescriptionAliasInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertyDescriptionAliasInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertyDescriptionAliasInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertyDescriptionAliasInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertyDescriptionAliasInfo_GetPropertyKey(This,pkey)	\
    ( (This)->lpVtbl -> GetPropertyKey(This,pkey) ) 

#define IPropertyDescriptionAliasInfo_GetCanonicalName(This,ppszName)	\
    ( (This)->lpVtbl -> GetCanonicalName(This,ppszName) ) 

#define IPropertyDescriptionAliasInfo_GetPropertyType(This,pvartype)	\
    ( (This)->lpVtbl -> GetPropertyType(This,pvartype) ) 

#define IPropertyDescriptionAliasInfo_GetDisplayName(This,ppszName)	\
    ( (This)->lpVtbl -> GetDisplayName(This,ppszName) ) 

#define IPropertyDescriptionAliasInfo_GetEditInvitation(This,ppszInvite)	\
    ( (This)->lpVtbl -> GetEditInvitation(This,ppszInvite) ) 

#define IPropertyDescriptionAliasInfo_GetTypeFlags(This,mask,ppdtFlags)	\
    ( (This)->lpVtbl -> GetTypeFlags(This,mask,ppdtFlags) ) 

#define IPropertyDescriptionAliasInfo_GetViewFlags(This,ppdvFlags)	\
    ( (This)->lpVtbl -> GetViewFlags(This,ppdvFlags) ) 

#define IPropertyDescriptionAliasInfo_GetDefaultColumnWidth(This,pcxChars)	\
    ( (This)->lpVtbl -> GetDefaultColumnWidth(This,pcxChars) ) 

#define IPropertyDescriptionAliasInfo_GetDisplayType(This,pdisplaytype)	\
    ( (This)->lpVtbl -> GetDisplayType(This,pdisplaytype) ) 

#define IPropertyDescriptionAliasInfo_GetColumnState(This,pcsFlags)	\
    ( (This)->lpVtbl -> GetColumnState(This,pcsFlags) ) 

#define IPropertyDescriptionAliasInfo_GetGroupingRange(This,pgr)	\
    ( (This)->lpVtbl -> GetGroupingRange(This,pgr) ) 

#define IPropertyDescriptionAliasInfo_GetRelativeDescriptionType(This,prdt)	\
    ( (This)->lpVtbl -> GetRelativeDescriptionType(This,prdt) ) 

#define IPropertyDescriptionAliasInfo_GetRelativeDescription(This,propvar1,propvar2,ppszDesc1,ppszDesc2)	\
    ( (This)->lpVtbl -> GetRelativeDescription(This,propvar1,propvar2,ppszDesc1,ppszDesc2) ) 

#define IPropertyDescriptionAliasInfo_GetSortDescription(This,psd)	\
    ( (This)->lpVtbl -> GetSortDescription(This,psd) ) 

#define IPropertyDescriptionAliasInfo_GetSortDescriptionLabel(This,fDescending,ppszDescription)	\
    ( (This)->lpVtbl -> GetSortDescriptionLabel(This,fDescending,ppszDescription) ) 

#define IPropertyDescriptionAliasInfo_GetAggregationType(This,paggtype)	\
    ( (This)->lpVtbl -> GetAggregationType(This,paggtype) ) 

#define IPropertyDescriptionAliasInfo_GetConditionType(This,pcontype,popDefault)	\
    ( (This)->lpVtbl -> GetConditionType(This,pcontype,popDefault) ) 

#define IPropertyDescriptionAliasInfo_GetEnumTypeList(This,riid,ppv)	\
    ( (This)->lpVtbl -> GetEnumTypeList(This,riid,ppv) ) 

#define IPropertyDescriptionAliasInfo_CoerceToCanonicalValue(This,ppropvar)	\
    ( (This)->lpVtbl -> CoerceToCanonicalValue(This,ppropvar) ) 

#define IPropertyDescriptionAliasInfo_FormatForDisplay(This,propvar,pdfFlags,ppszDisplay)	\
    ( (This)->lpVtbl -> FormatForDisplay(This,propvar,pdfFlags,ppszDisplay) ) 

#define IPropertyDescriptionAliasInfo_IsValueCanonical(This,propvar)	\
    ( (This)->lpVtbl -> IsValueCanonical(This,propvar) ) 


#define IPropertyDescriptionAliasInfo_GetSortByAlias(This,riid,ppv)	\
    ( (This)->lpVtbl -> GetSortByAlias(This,riid,ppv) ) 

#define IPropertyDescriptionAliasInfo_GetAdditionalSortByAliases(This,riid,ppv)	\
    ( (This)->lpVtbl -> GetAdditionalSortByAliases(This,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertyDescriptionAliasInfo_INTERFACE_DEFINED__ */


#ifndef __IPropertyDescriptionSearchInfo_INTERFACE_DEFINED__
#define __IPropertyDescriptionSearchInfo_INTERFACE_DEFINED__

/* interface IPropertyDescriptionSearchInfo */
/* [unique][object][uuid] */ 

typedef /* [v1_enum] */ 
enum PROPDESC_SEARCHINFO_FLAGS
    {
        PDSIF_DEFAULT	= 0,
        PDSIF_ININVERTEDINDEX	= 0x1,
        PDSIF_ISCOLUMN	= 0x2,
        PDSIF_ISCOLUMNSPARSE	= 0x4,
        PDSIF_ALWAYSINCLUDE	= 0x8,
        PDSIF_USEFORTYPEAHEAD	= 0x10
    } 	PROPDESC_SEARCHINFO_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS(PROPDESC_SEARCHINFO_FLAGS)
typedef /* [v1_enum] */ 
enum PROPDESC_COLUMNINDEX_TYPE
    {
        PDCIT_NONE	= 0,
        PDCIT_ONDISK	= 1,
        PDCIT_INMEMORY	= 2,
        PDCIT_ONDEMAND	= 3,
        PDCIT_ONDISKALL	= 4,
        PDCIT_ONDISKVECTOR	= 5
    } 	PROPDESC_COLUMNINDEX_TYPE;


EXTERN_C const IID IID_IPropertyDescriptionSearchInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("078f91bd-29a2-440f-924e-46a291524520")
    IPropertyDescriptionSearchInfo : public IPropertyDescription
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSearchInfoFlags( 
            /* [out] */ __RPC__out PROPDESC_SEARCHINFO_FLAGS *ppdsiFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColumnIndexType( 
            /* [out] */ __RPC__out PROPDESC_COLUMNINDEX_TYPE *ppdciType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProjectionString( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszProjection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxSize( 
            /* [out] */ __RPC__out UINT *pcbMaxSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertyDescriptionSearchInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertyDescriptionSearchInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertyDescriptionSearchInfo * This);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetPropertyKey)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyKey )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [out] */ __RPC__out PROPERTYKEY *pkey);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetCanonicalName)
        HRESULT ( STDMETHODCALLTYPE *GetCanonicalName )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszName);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetPropertyType)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyType )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [out] */ __RPC__out VARTYPE *pvartype);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszName);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetEditInvitation)
        HRESULT ( STDMETHODCALLTYPE *GetEditInvitation )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszInvite);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetTypeFlags)
        HRESULT ( STDMETHODCALLTYPE *GetTypeFlags )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [in] */ PROPDESC_TYPE_FLAGS mask,
            /* [out] */ __RPC__out PROPDESC_TYPE_FLAGS *ppdtFlags);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetViewFlags)
        HRESULT ( STDMETHODCALLTYPE *GetViewFlags )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [out] */ __RPC__out PROPDESC_VIEW_FLAGS *ppdvFlags);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetDefaultColumnWidth)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultColumnWidth )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [out] */ __RPC__out UINT *pcxChars);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetDisplayType)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayType )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [out] */ __RPC__out PROPDESC_DISPLAYTYPE *pdisplaytype);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetColumnState)
        HRESULT ( STDMETHODCALLTYPE *GetColumnState )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [out] */ __RPC__out SHCOLSTATEF *pcsFlags);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetGroupingRange)
        HRESULT ( STDMETHODCALLTYPE *GetGroupingRange )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [out] */ __RPC__out PROPDESC_GROUPING_RANGE *pgr);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetRelativeDescriptionType)
        HRESULT ( STDMETHODCALLTYPE *GetRelativeDescriptionType )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [out] */ __RPC__out PROPDESC_RELATIVEDESCRIPTION_TYPE *prdt);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetRelativeDescription)
        HRESULT ( STDMETHODCALLTYPE *GetRelativeDescription )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvar1,
            /* [in] */ __RPC__in REFPROPVARIANT propvar2,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDesc1,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDesc2);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetSortDescription)
        HRESULT ( STDMETHODCALLTYPE *GetSortDescription )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [out] */ __RPC__out PROPDESC_SORTDESCRIPTION *psd);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetSortDescriptionLabel)
        HRESULT ( STDMETHODCALLTYPE *GetSortDescriptionLabel )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [in] */ BOOL fDescending,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDescription);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetAggregationType)
        HRESULT ( STDMETHODCALLTYPE *GetAggregationType )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [out] */ __RPC__out PROPDESC_AGGREGATION_TYPE *paggtype);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetConditionType)
        HRESULT ( STDMETHODCALLTYPE *GetConditionType )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [out] */ __RPC__out PROPDESC_CONDITION_TYPE *pcontype,
            /* [out] */ __RPC__out CONDITION_OPERATION *popDefault);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetEnumTypeList)
        HRESULT ( STDMETHODCALLTYPE *GetEnumTypeList )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, CoerceToCanonicalValue)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CoerceToCanonicalValue )( 
            IPropertyDescriptionSearchInfo * This,
            /* [annotation][out][in] */ 
            _Inout_  PROPVARIANT *ppropvar);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, FormatForDisplay)
        HRESULT ( STDMETHODCALLTYPE *FormatForDisplay )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvar,
            /* [in] */ PROPDESC_FORMAT_FLAGS pdfFlags,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDisplay);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, IsValueCanonical)
        HRESULT ( STDMETHODCALLTYPE *IsValueCanonical )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvar);
        
        DECLSPEC_XFGVIRT(IPropertyDescriptionSearchInfo, GetSearchInfoFlags)
        HRESULT ( STDMETHODCALLTYPE *GetSearchInfoFlags )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [out] */ __RPC__out PROPDESC_SEARCHINFO_FLAGS *ppdsiFlags);
        
        DECLSPEC_XFGVIRT(IPropertyDescriptionSearchInfo, GetColumnIndexType)
        HRESULT ( STDMETHODCALLTYPE *GetColumnIndexType )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [out] */ __RPC__out PROPDESC_COLUMNINDEX_TYPE *ppdciType);
        
        DECLSPEC_XFGVIRT(IPropertyDescriptionSearchInfo, GetProjectionString)
        HRESULT ( STDMETHODCALLTYPE *GetProjectionString )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszProjection);
        
        DECLSPEC_XFGVIRT(IPropertyDescriptionSearchInfo, GetMaxSize)
        HRESULT ( STDMETHODCALLTYPE *GetMaxSize )( 
            __RPC__in IPropertyDescriptionSearchInfo * This,
            /* [out] */ __RPC__out UINT *pcbMaxSize);
        
        END_INTERFACE
    } IPropertyDescriptionSearchInfoVtbl;

    interface IPropertyDescriptionSearchInfo
    {
        CONST_VTBL struct IPropertyDescriptionSearchInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertyDescriptionSearchInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertyDescriptionSearchInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertyDescriptionSearchInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertyDescriptionSearchInfo_GetPropertyKey(This,pkey)	\
    ( (This)->lpVtbl -> GetPropertyKey(This,pkey) ) 

#define IPropertyDescriptionSearchInfo_GetCanonicalName(This,ppszName)	\
    ( (This)->lpVtbl -> GetCanonicalName(This,ppszName) ) 

#define IPropertyDescriptionSearchInfo_GetPropertyType(This,pvartype)	\
    ( (This)->lpVtbl -> GetPropertyType(This,pvartype) ) 

#define IPropertyDescriptionSearchInfo_GetDisplayName(This,ppszName)	\
    ( (This)->lpVtbl -> GetDisplayName(This,ppszName) ) 

#define IPropertyDescriptionSearchInfo_GetEditInvitation(This,ppszInvite)	\
    ( (This)->lpVtbl -> GetEditInvitation(This,ppszInvite) ) 

#define IPropertyDescriptionSearchInfo_GetTypeFlags(This,mask,ppdtFlags)	\
    ( (This)->lpVtbl -> GetTypeFlags(This,mask,ppdtFlags) ) 

#define IPropertyDescriptionSearchInfo_GetViewFlags(This,ppdvFlags)	\
    ( (This)->lpVtbl -> GetViewFlags(This,ppdvFlags) ) 

#define IPropertyDescriptionSearchInfo_GetDefaultColumnWidth(This,pcxChars)	\
    ( (This)->lpVtbl -> GetDefaultColumnWidth(This,pcxChars) ) 

#define IPropertyDescriptionSearchInfo_GetDisplayType(This,pdisplaytype)	\
    ( (This)->lpVtbl -> GetDisplayType(This,pdisplaytype) ) 

#define IPropertyDescriptionSearchInfo_GetColumnState(This,pcsFlags)	\
    ( (This)->lpVtbl -> GetColumnState(This,pcsFlags) ) 

#define IPropertyDescriptionSearchInfo_GetGroupingRange(This,pgr)	\
    ( (This)->lpVtbl -> GetGroupingRange(This,pgr) ) 

#define IPropertyDescriptionSearchInfo_GetRelativeDescriptionType(This,prdt)	\
    ( (This)->lpVtbl -> GetRelativeDescriptionType(This,prdt) ) 

#define IPropertyDescriptionSearchInfo_GetRelativeDescription(This,propvar1,propvar2,ppszDesc1,ppszDesc2)	\
    ( (This)->lpVtbl -> GetRelativeDescription(This,propvar1,propvar2,ppszDesc1,ppszDesc2) ) 

#define IPropertyDescriptionSearchInfo_GetSortDescription(This,psd)	\
    ( (This)->lpVtbl -> GetSortDescription(This,psd) ) 

#define IPropertyDescriptionSearchInfo_GetSortDescriptionLabel(This,fDescending,ppszDescription)	\
    ( (This)->lpVtbl -> GetSortDescriptionLabel(This,fDescending,ppszDescription) ) 

#define IPropertyDescriptionSearchInfo_GetAggregationType(This,paggtype)	\
    ( (This)->lpVtbl -> GetAggregationType(This,paggtype) ) 

#define IPropertyDescriptionSearchInfo_GetConditionType(This,pcontype,popDefault)	\
    ( (This)->lpVtbl -> GetConditionType(This,pcontype,popDefault) ) 

#define IPropertyDescriptionSearchInfo_GetEnumTypeList(This,riid,ppv)	\
    ( (This)->lpVtbl -> GetEnumTypeList(This,riid,ppv) ) 

#define IPropertyDescriptionSearchInfo_CoerceToCanonicalValue(This,ppropvar)	\
    ( (This)->lpVtbl -> CoerceToCanonicalValue(This,ppropvar) ) 

#define IPropertyDescriptionSearchInfo_FormatForDisplay(This,propvar,pdfFlags,ppszDisplay)	\
    ( (This)->lpVtbl -> FormatForDisplay(This,propvar,pdfFlags,ppszDisplay) ) 

#define IPropertyDescriptionSearchInfo_IsValueCanonical(This,propvar)	\
    ( (This)->lpVtbl -> IsValueCanonical(This,propvar) ) 


#define IPropertyDescriptionSearchInfo_GetSearchInfoFlags(This,ppdsiFlags)	\
    ( (This)->lpVtbl -> GetSearchInfoFlags(This,ppdsiFlags) ) 

#define IPropertyDescriptionSearchInfo_GetColumnIndexType(This,ppdciType)	\
    ( (This)->lpVtbl -> GetColumnIndexType(This,ppdciType) ) 

#define IPropertyDescriptionSearchInfo_GetProjectionString(This,ppszProjection)	\
    ( (This)->lpVtbl -> GetProjectionString(This,ppszProjection) ) 

#define IPropertyDescriptionSearchInfo_GetMaxSize(This,pcbMaxSize)	\
    ( (This)->lpVtbl -> GetMaxSize(This,pcbMaxSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertyDescriptionSearchInfo_INTERFACE_DEFINED__ */


#ifndef __IPropertyDescriptionRelatedPropertyInfo_INTERFACE_DEFINED__
#define __IPropertyDescriptionRelatedPropertyInfo_INTERFACE_DEFINED__

/* interface IPropertyDescriptionRelatedPropertyInfo */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IPropertyDescriptionRelatedPropertyInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("507393f4-2a3d-4a60-b59e-d9c75716c2dd")
    IPropertyDescriptionRelatedPropertyInfo : public IPropertyDescription
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRelatedProperty( 
            /* [in] */ __RPC__in LPCWSTR pszRelationshipName,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertyDescriptionRelatedPropertyInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetPropertyKey)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyKey )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [out] */ __RPC__out PROPERTYKEY *pkey);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetCanonicalName)
        HRESULT ( STDMETHODCALLTYPE *GetCanonicalName )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszName);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetPropertyType)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyType )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [out] */ __RPC__out VARTYPE *pvartype);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszName);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetEditInvitation)
        HRESULT ( STDMETHODCALLTYPE *GetEditInvitation )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszInvite);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetTypeFlags)
        HRESULT ( STDMETHODCALLTYPE *GetTypeFlags )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [in] */ PROPDESC_TYPE_FLAGS mask,
            /* [out] */ __RPC__out PROPDESC_TYPE_FLAGS *ppdtFlags);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetViewFlags)
        HRESULT ( STDMETHODCALLTYPE *GetViewFlags )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [out] */ __RPC__out PROPDESC_VIEW_FLAGS *ppdvFlags);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetDefaultColumnWidth)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultColumnWidth )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [out] */ __RPC__out UINT *pcxChars);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetDisplayType)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayType )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [out] */ __RPC__out PROPDESC_DISPLAYTYPE *pdisplaytype);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetColumnState)
        HRESULT ( STDMETHODCALLTYPE *GetColumnState )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [out] */ __RPC__out SHCOLSTATEF *pcsFlags);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetGroupingRange)
        HRESULT ( STDMETHODCALLTYPE *GetGroupingRange )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [out] */ __RPC__out PROPDESC_GROUPING_RANGE *pgr);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetRelativeDescriptionType)
        HRESULT ( STDMETHODCALLTYPE *GetRelativeDescriptionType )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [out] */ __RPC__out PROPDESC_RELATIVEDESCRIPTION_TYPE *prdt);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetRelativeDescription)
        HRESULT ( STDMETHODCALLTYPE *GetRelativeDescription )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvar1,
            /* [in] */ __RPC__in REFPROPVARIANT propvar2,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDesc1,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDesc2);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetSortDescription)
        HRESULT ( STDMETHODCALLTYPE *GetSortDescription )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [out] */ __RPC__out PROPDESC_SORTDESCRIPTION *psd);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetSortDescriptionLabel)
        HRESULT ( STDMETHODCALLTYPE *GetSortDescriptionLabel )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [in] */ BOOL fDescending,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDescription);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetAggregationType)
        HRESULT ( STDMETHODCALLTYPE *GetAggregationType )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [out] */ __RPC__out PROPDESC_AGGREGATION_TYPE *paggtype);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetConditionType)
        HRESULT ( STDMETHODCALLTYPE *GetConditionType )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [out] */ __RPC__out PROPDESC_CONDITION_TYPE *pcontype,
            /* [out] */ __RPC__out CONDITION_OPERATION *popDefault);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, GetEnumTypeList)
        HRESULT ( STDMETHODCALLTYPE *GetEnumTypeList )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, CoerceToCanonicalValue)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CoerceToCanonicalValue )( 
            IPropertyDescriptionRelatedPropertyInfo * This,
            /* [annotation][out][in] */ 
            _Inout_  PROPVARIANT *ppropvar);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, FormatForDisplay)
        HRESULT ( STDMETHODCALLTYPE *FormatForDisplay )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvar,
            /* [in] */ PROPDESC_FORMAT_FLAGS pdfFlags,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDisplay);
        
        DECLSPEC_XFGVIRT(IPropertyDescription, IsValueCanonical)
        HRESULT ( STDMETHODCALLTYPE *IsValueCanonical )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [in] */ __RPC__in REFPROPVARIANT propvar);
        
        DECLSPEC_XFGVIRT(IPropertyDescriptionRelatedPropertyInfo, GetRelatedProperty)
        HRESULT ( STDMETHODCALLTYPE *GetRelatedProperty )( 
            __RPC__in IPropertyDescriptionRelatedPropertyInfo * This,
            /* [in] */ __RPC__in LPCWSTR pszRelationshipName,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        END_INTERFACE
    } IPropertyDescriptionRelatedPropertyInfoVtbl;

    interface IPropertyDescriptionRelatedPropertyInfo
    {
        CONST_VTBL struct IPropertyDescriptionRelatedPropertyInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertyDescriptionRelatedPropertyInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertyDescriptionRelatedPropertyInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertyDescriptionRelatedPropertyInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertyDescriptionRelatedPropertyInfo_GetPropertyKey(This,pkey)	\
    ( (This)->lpVtbl -> GetPropertyKey(This,pkey) ) 

#define IPropertyDescriptionRelatedPropertyInfo_GetCanonicalName(This,ppszName)	\
    ( (This)->lpVtbl -> GetCanonicalName(This,ppszName) ) 

#define IPropertyDescriptionRelatedPropertyInfo_GetPropertyType(This,pvartype)	\
    ( (This)->lpVtbl -> GetPropertyType(This,pvartype) ) 

#define IPropertyDescriptionRelatedPropertyInfo_GetDisplayName(This,ppszName)	\
    ( (This)->lpVtbl -> GetDisplayName(This,ppszName) ) 

#define IPropertyDescriptionRelatedPropertyInfo_GetEditInvitation(This,ppszInvite)	\
    ( (This)->lpVtbl -> GetEditInvitation(This,ppszInvite) ) 

#define IPropertyDescriptionRelatedPropertyInfo_GetTypeFlags(This,mask,ppdtFlags)	\
    ( (This)->lpVtbl -> GetTypeFlags(This,mask,ppdtFlags) ) 

#define IPropertyDescriptionRelatedPropertyInfo_GetViewFlags(This,ppdvFlags)	\
    ( (This)->lpVtbl -> GetViewFlags(This,ppdvFlags) ) 

#define IPropertyDescriptionRelatedPropertyInfo_GetDefaultColumnWidth(This,pcxChars)	\
    ( (This)->lpVtbl -> GetDefaultColumnWidth(This,pcxChars) ) 

#define IPropertyDescriptionRelatedPropertyInfo_GetDisplayType(This,pdisplaytype)	\
    ( (This)->lpVtbl -> GetDisplayType(This,pdisplaytype) ) 

#define IPropertyDescriptionRelatedPropertyInfo_GetColumnState(This,pcsFlags)	\
    ( (This)->lpVtbl -> GetColumnState(This,pcsFlags) ) 

#define IPropertyDescriptionRelatedPropertyInfo_GetGroupingRange(This,pgr)	\
    ( (This)->lpVtbl -> GetGroupingRange(This,pgr) ) 

#define IPropertyDescriptionRelatedPropertyInfo_GetRelativeDescriptionType(This,prdt)	\
    ( (This)->lpVtbl -> GetRelativeDescriptionType(This,prdt) ) 

#define IPropertyDescriptionRelatedPropertyInfo_GetRelativeDescription(This,propvar1,propvar2,ppszDesc1,ppszDesc2)	\
    ( (This)->lpVtbl -> GetRelativeDescription(This,propvar1,propvar2,ppszDesc1,ppszDesc2) ) 

#define IPropertyDescriptionRelatedPropertyInfo_GetSortDescription(This,psd)	\
    ( (This)->lpVtbl -> GetSortDescription(This,psd) ) 

#define IPropertyDescriptionRelatedPropertyInfo_GetSortDescriptionLabel(This,fDescending,ppszDescription)	\
    ( (This)->lpVtbl -> GetSortDescriptionLabel(This,fDescending,ppszDescription) ) 

#define IPropertyDescriptionRelatedPropertyInfo_GetAggregationType(This,paggtype)	\
    ( (This)->lpVtbl -> GetAggregationType(This,paggtype) ) 

#define IPropertyDescriptionRelatedPropertyInfo_GetConditionType(This,pcontype,popDefault)	\
    ( (This)->lpVtbl -> GetConditionType(This,pcontype,popDefault) ) 

#define IPropertyDescriptionRelatedPropertyInfo_GetEnumTypeList(This,riid,ppv)	\
    ( (This)->lpVtbl -> GetEnumTypeList(This,riid,ppv) ) 

#define IPropertyDescriptionRelatedPropertyInfo_CoerceToCanonicalValue(This,ppropvar)	\
    ( (This)->lpVtbl -> CoerceToCanonicalValue(This,ppropvar) ) 

#define IPropertyDescriptionRelatedPropertyInfo_FormatForDisplay(This,propvar,pdfFlags,ppszDisplay)	\
    ( (This)->lpVtbl -> FormatForDisplay(This,propvar,pdfFlags,ppszDisplay) ) 

#define IPropertyDescriptionRelatedPropertyInfo_IsValueCanonical(This,propvar)	\
    ( (This)->lpVtbl -> IsValueCanonical(This,propvar) ) 


#define IPropertyDescriptionRelatedPropertyInfo_GetRelatedProperty(This,pszRelationshipName,riid,ppv)	\
    ( (This)->lpVtbl -> GetRelatedProperty(This,pszRelationshipName,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertyDescriptionRelatedPropertyInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_propsys_0000_0017 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum PROPDESC_ENUMFILTER
    {
        PDEF_ALL	= 0,
        PDEF_SYSTEM	= 1,
        PDEF_NONSYSTEM	= 2,
        PDEF_VIEWABLE	= 3,
        PDEF_QUERYABLE	= 4,
        PDEF_INFULLTEXTQUERY	= 5,
        PDEF_COLUMN	= 6
    } 	PROPDESC_ENUMFILTER;



extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0017_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0017_v0_0_s_ifspec;

#ifndef __IPropertySystem_INTERFACE_DEFINED__
#define __IPropertySystem_INTERFACE_DEFINED__

/* interface IPropertySystem */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IPropertySystem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ca724e8a-c3e6-442b-88a4-6fb0db8035a3")
    IPropertySystem : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPropertyDescription( 
            /* [in] */ __RPC__in REFPROPERTYKEY propkey,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyDescriptionByName( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszCanonicalName,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyDescriptionListFromString( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszPropList,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumeratePropertyDescriptions( 
            /* [in] */ PROPDESC_ENUMFILTER filterOn,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FormatForDisplay( 
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [in] */ __RPC__in REFPROPVARIANT propvar,
            /* [in] */ PROPDESC_FORMAT_FLAGS pdff,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(cchText) LPWSTR pszText,
            /* [range][in] */ __RPC__in_range(0,0x8000) DWORD cchText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FormatForDisplayAlloc( 
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [in] */ __RPC__in REFPROPVARIANT propvar,
            /* [in] */ PROPDESC_FORMAT_FLAGS pdff,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDisplay) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterPropertySchema( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterPropertySchema( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RefreshPropertySchema( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertySystemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertySystem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertySystem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertySystem * This);
        
        DECLSPEC_XFGVIRT(IPropertySystem, GetPropertyDescription)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyDescription )( 
            __RPC__in IPropertySystem * This,
            /* [in] */ __RPC__in REFPROPERTYKEY propkey,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IPropertySystem, GetPropertyDescriptionByName)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyDescriptionByName )( 
            __RPC__in IPropertySystem * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszCanonicalName,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IPropertySystem, GetPropertyDescriptionListFromString)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyDescriptionListFromString )( 
            __RPC__in IPropertySystem * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszPropList,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IPropertySystem, EnumeratePropertyDescriptions)
        HRESULT ( STDMETHODCALLTYPE *EnumeratePropertyDescriptions )( 
            __RPC__in IPropertySystem * This,
            /* [in] */ PROPDESC_ENUMFILTER filterOn,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IPropertySystem, FormatForDisplay)
        HRESULT ( STDMETHODCALLTYPE *FormatForDisplay )( 
            __RPC__in IPropertySystem * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [in] */ __RPC__in REFPROPVARIANT propvar,
            /* [in] */ PROPDESC_FORMAT_FLAGS pdff,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(cchText) LPWSTR pszText,
            /* [range][in] */ __RPC__in_range(0,0x8000) DWORD cchText);
        
        DECLSPEC_XFGVIRT(IPropertySystem, FormatForDisplayAlloc)
        HRESULT ( STDMETHODCALLTYPE *FormatForDisplayAlloc )( 
            __RPC__in IPropertySystem * This,
            /* [in] */ __RPC__in REFPROPERTYKEY key,
            /* [in] */ __RPC__in REFPROPVARIANT propvar,
            /* [in] */ PROPDESC_FORMAT_FLAGS pdff,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *ppszDisplay);
        
        DECLSPEC_XFGVIRT(IPropertySystem, RegisterPropertySchema)
        HRESULT ( STDMETHODCALLTYPE *RegisterPropertySchema )( 
            __RPC__in IPropertySystem * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IPropertySystem, UnregisterPropertySchema)
        HRESULT ( STDMETHODCALLTYPE *UnregisterPropertySchema )( 
            __RPC__in IPropertySystem * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IPropertySystem, RefreshPropertySchema)
        HRESULT ( STDMETHODCALLTYPE *RefreshPropertySchema )( 
            __RPC__in IPropertySystem * This);
        
        END_INTERFACE
    } IPropertySystemVtbl;

    interface IPropertySystem
    {
        CONST_VTBL struct IPropertySystemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertySystem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertySystem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertySystem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertySystem_GetPropertyDescription(This,propkey,riid,ppv)	\
    ( (This)->lpVtbl -> GetPropertyDescription(This,propkey,riid,ppv) ) 

#define IPropertySystem_GetPropertyDescriptionByName(This,pszCanonicalName,riid,ppv)	\
    ( (This)->lpVtbl -> GetPropertyDescriptionByName(This,pszCanonicalName,riid,ppv) ) 

#define IPropertySystem_GetPropertyDescriptionListFromString(This,pszPropList,riid,ppv)	\
    ( (This)->lpVtbl -> GetPropertyDescriptionListFromString(This,pszPropList,riid,ppv) ) 

#define IPropertySystem_EnumeratePropertyDescriptions(This,filterOn,riid,ppv)	\
    ( (This)->lpVtbl -> EnumeratePropertyDescriptions(This,filterOn,riid,ppv) ) 

#define IPropertySystem_FormatForDisplay(This,key,propvar,pdff,pszText,cchText)	\
    ( (This)->lpVtbl -> FormatForDisplay(This,key,propvar,pdff,pszText,cchText) ) 

#define IPropertySystem_FormatForDisplayAlloc(This,key,propvar,pdff,ppszDisplay)	\
    ( (This)->lpVtbl -> FormatForDisplayAlloc(This,key,propvar,pdff,ppszDisplay) ) 

#define IPropertySystem_RegisterPropertySchema(This,pszPath)	\
    ( (This)->lpVtbl -> RegisterPropertySchema(This,pszPath) ) 

#define IPropertySystem_UnregisterPropertySchema(This,pszPath)	\
    ( (This)->lpVtbl -> UnregisterPropertySchema(This,pszPath) ) 

#define IPropertySystem_RefreshPropertySchema(This)	\
    ( (This)->lpVtbl -> RefreshPropertySchema(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertySystem_INTERFACE_DEFINED__ */


#ifndef __IPropertyDescriptionList_INTERFACE_DEFINED__
#define __IPropertyDescriptionList_INTERFACE_DEFINED__

/* interface IPropertyDescriptionList */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IPropertyDescriptionList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1f9fc1d0-c39b-4b26-817f-011967d3440e")
    IPropertyDescriptionList : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out UINT *pcElem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT iElem,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertyDescriptionListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertyDescriptionList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertyDescriptionList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertyDescriptionList * This);
        
        DECLSPEC_XFGVIRT(IPropertyDescriptionList, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IPropertyDescriptionList * This,
            /* [out] */ __RPC__out UINT *pcElem);
        
        DECLSPEC_XFGVIRT(IPropertyDescriptionList, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IPropertyDescriptionList * This,
            /* [in] */ UINT iElem,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        END_INTERFACE
    } IPropertyDescriptionListVtbl;

    interface IPropertyDescriptionList
    {
        CONST_VTBL struct IPropertyDescriptionListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertyDescriptionList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertyDescriptionList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertyDescriptionList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertyDescriptionList_GetCount(This,pcElem)	\
    ( (This)->lpVtbl -> GetCount(This,pcElem) ) 

#define IPropertyDescriptionList_GetAt(This,iElem,riid,ppv)	\
    ( (This)->lpVtbl -> GetAt(This,iElem,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertyDescriptionList_INTERFACE_DEFINED__ */


#ifndef __IPropertyStoreFactory_INTERFACE_DEFINED__
#define __IPropertyStoreFactory_INTERFACE_DEFINED__

/* interface IPropertyStoreFactory */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IPropertyStoreFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bc110b6d-57e8-4148-a9c6-91015ab2f3a5")
    IPropertyStoreFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPropertyStore( 
            /* [in] */ GETPROPERTYSTOREFLAGS flags,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkFactory,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyStoreForKeys( 
            /* [unique][in] */ __RPC__in_opt const PROPERTYKEY *rgKeys,
            /* [in] */ UINT cKeys,
            /* [in] */ GETPROPERTYSTOREFLAGS flags,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertyStoreFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertyStoreFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertyStoreFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertyStoreFactory * This);
        
        DECLSPEC_XFGVIRT(IPropertyStoreFactory, GetPropertyStore)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyStore )( 
            __RPC__in IPropertyStoreFactory * This,
            /* [in] */ GETPROPERTYSTOREFLAGS flags,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkFactory,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IPropertyStoreFactory, GetPropertyStoreForKeys)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyStoreForKeys )( 
            __RPC__in IPropertyStoreFactory * This,
            /* [unique][in] */ __RPC__in_opt const PROPERTYKEY *rgKeys,
            /* [in] */ UINT cKeys,
            /* [in] */ GETPROPERTYSTOREFLAGS flags,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        END_INTERFACE
    } IPropertyStoreFactoryVtbl;

    interface IPropertyStoreFactory
    {
        CONST_VTBL struct IPropertyStoreFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertyStoreFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertyStoreFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertyStoreFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertyStoreFactory_GetPropertyStore(This,flags,pUnkFactory,riid,ppv)	\
    ( (This)->lpVtbl -> GetPropertyStore(This,flags,pUnkFactory,riid,ppv) ) 

#define IPropertyStoreFactory_GetPropertyStoreForKeys(This,rgKeys,cKeys,flags,riid,ppv)	\
    ( (This)->lpVtbl -> GetPropertyStoreForKeys(This,rgKeys,cKeys,flags,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertyStoreFactory_INTERFACE_DEFINED__ */


#ifndef __IDelayedPropertyStoreFactory_INTERFACE_DEFINED__
#define __IDelayedPropertyStoreFactory_INTERFACE_DEFINED__

/* interface IDelayedPropertyStoreFactory */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IDelayedPropertyStoreFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("40d4577f-e237-4bdb-bd69-58f089431b6a")
    IDelayedPropertyStoreFactory : public IPropertyStoreFactory
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDelayedPropertyStore( 
            /* [in] */ GETPROPERTYSTOREFLAGS flags,
            /* [in] */ DWORD dwStoreId,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDelayedPropertyStoreFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDelayedPropertyStoreFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDelayedPropertyStoreFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDelayedPropertyStoreFactory * This);
        
        DECLSPEC_XFGVIRT(IPropertyStoreFactory, GetPropertyStore)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyStore )( 
            __RPC__in IDelayedPropertyStoreFactory * This,
            /* [in] */ GETPROPERTYSTOREFLAGS flags,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkFactory,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IPropertyStoreFactory, GetPropertyStoreForKeys)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyStoreForKeys )( 
            __RPC__in IDelayedPropertyStoreFactory * This,
            /* [unique][in] */ __RPC__in_opt const PROPERTYKEY *rgKeys,
            /* [in] */ UINT cKeys,
            /* [in] */ GETPROPERTYSTOREFLAGS flags,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        DECLSPEC_XFGVIRT(IDelayedPropertyStoreFactory, GetDelayedPropertyStore)
        HRESULT ( STDMETHODCALLTYPE *GetDelayedPropertyStore )( 
            __RPC__in IDelayedPropertyStoreFactory * This,
            /* [in] */ GETPROPERTYSTOREFLAGS flags,
            /* [in] */ DWORD dwStoreId,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        END_INTERFACE
    } IDelayedPropertyStoreFactoryVtbl;

    interface IDelayedPropertyStoreFactory
    {
        CONST_VTBL struct IDelayedPropertyStoreFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDelayedPropertyStoreFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDelayedPropertyStoreFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDelayedPropertyStoreFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDelayedPropertyStoreFactory_GetPropertyStore(This,flags,pUnkFactory,riid,ppv)	\
    ( (This)->lpVtbl -> GetPropertyStore(This,flags,pUnkFactory,riid,ppv) ) 

#define IDelayedPropertyStoreFactory_GetPropertyStoreForKeys(This,rgKeys,cKeys,flags,riid,ppv)	\
    ( (This)->lpVtbl -> GetPropertyStoreForKeys(This,rgKeys,cKeys,flags,riid,ppv) ) 


#define IDelayedPropertyStoreFactory_GetDelayedPropertyStore(This,flags,dwStoreId,riid,ppv)	\
    ( (This)->lpVtbl -> GetDelayedPropertyStore(This,flags,dwStoreId,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDelayedPropertyStoreFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_propsys_0000_0021 */
/* [local] */ 

/* [v1_enum] */ 
enum _PERSIST_SPROPSTORE_FLAGS
    {
        FPSPS_DEFAULT	= 0,
        FPSPS_READONLY	= 0x1,
        FPSPS_TREAT_NEW_VALUES_AS_DIRTY	= 0x2
    } ;
typedef int PERSIST_SPROPSTORE_FLAGS;

typedef struct tagSERIALIZEDPROPSTORAGE SERIALIZEDPROPSTORAGE;

typedef SERIALIZEDPROPSTORAGE __unaligned *PUSERIALIZEDPROPSTORAGE;

typedef const SERIALIZEDPROPSTORAGE __unaligned *PCUSERIALIZEDPROPSTORAGE;



extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0021_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0021_v0_0_s_ifspec;

#ifndef __IPersistSerializedPropStorage_INTERFACE_DEFINED__
#define __IPersistSerializedPropStorage_INTERFACE_DEFINED__

/* interface IPersistSerializedPropStorage */
/* [object][local][unique][uuid] */ 


EXTERN_C const IID IID_IPersistSerializedPropStorage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e318ad57-0aa0-450f-aca5-6fab7103d917")
    IPersistSerializedPropStorage : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetFlags( 
            /* [in] */ PERSIST_SPROPSTORE_FLAGS flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPropertyStorage( 
            /* [annotation][in] */ 
            _In_reads_bytes_(cb)  PCUSERIALIZEDPROPSTORAGE psps,
            /* [annotation][in] */ 
            _In_  DWORD cb) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyStorage( 
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*pcb)  SERIALIZEDPROPSTORAGE **ppsps,
            /* [annotation][out] */ 
            _Out_  DWORD *pcb) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPersistSerializedPropStorageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPersistSerializedPropStorage * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPersistSerializedPropStorage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPersistSerializedPropStorage * This);
        
        DECLSPEC_XFGVIRT(IPersistSerializedPropStorage, SetFlags)
        HRESULT ( STDMETHODCALLTYPE *SetFlags )( 
            IPersistSerializedPropStorage * This,
            /* [in] */ PERSIST_SPROPSTORE_FLAGS flags);
        
        DECLSPEC_XFGVIRT(IPersistSerializedPropStorage, SetPropertyStorage)
        HRESULT ( STDMETHODCALLTYPE *SetPropertyStorage )( 
            IPersistSerializedPropStorage * This,
            /* [annotation][in] */ 
            _In_reads_bytes_(cb)  PCUSERIALIZEDPROPSTORAGE psps,
            /* [annotation][in] */ 
            _In_  DWORD cb);
        
        DECLSPEC_XFGVIRT(IPersistSerializedPropStorage, GetPropertyStorage)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyStorage )( 
            IPersistSerializedPropStorage * This,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*pcb)  SERIALIZEDPROPSTORAGE **ppsps,
            /* [annotation][out] */ 
            _Out_  DWORD *pcb);
        
        END_INTERFACE
    } IPersistSerializedPropStorageVtbl;

    interface IPersistSerializedPropStorage
    {
        CONST_VTBL struct IPersistSerializedPropStorageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPersistSerializedPropStorage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPersistSerializedPropStorage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPersistSerializedPropStorage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPersistSerializedPropStorage_SetFlags(This,flags)	\
    ( (This)->lpVtbl -> SetFlags(This,flags) ) 

#define IPersistSerializedPropStorage_SetPropertyStorage(This,psps,cb)	\
    ( (This)->lpVtbl -> SetPropertyStorage(This,psps,cb) ) 

#define IPersistSerializedPropStorage_GetPropertyStorage(This,ppsps,pcb)	\
    ( (This)->lpVtbl -> GetPropertyStorage(This,ppsps,pcb) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPersistSerializedPropStorage_INTERFACE_DEFINED__ */


#ifndef __IPersistSerializedPropStorage2_INTERFACE_DEFINED__
#define __IPersistSerializedPropStorage2_INTERFACE_DEFINED__

/* interface IPersistSerializedPropStorage2 */
/* [object][local][unique][uuid] */ 


EXTERN_C const IID IID_IPersistSerializedPropStorage2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("77effa68-4f98-4366-ba72-573b3d880571")
    IPersistSerializedPropStorage2 : public IPersistSerializedPropStorage
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPropertyStorageSize( 
            /* [annotation][out] */ 
            _Out_  DWORD *pcb) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyStorageBuffer( 
            /* [annotation][out] */ 
            _Out_writes_bytes_to_(cb, *pcbWritten)  SERIALIZEDPROPSTORAGE *psps,
            /* [in] */ DWORD cb,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbWritten) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPersistSerializedPropStorage2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPersistSerializedPropStorage2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPersistSerializedPropStorage2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPersistSerializedPropStorage2 * This);
        
        DECLSPEC_XFGVIRT(IPersistSerializedPropStorage, SetFlags)
        HRESULT ( STDMETHODCALLTYPE *SetFlags )( 
            IPersistSerializedPropStorage2 * This,
            /* [in] */ PERSIST_SPROPSTORE_FLAGS flags);
        
        DECLSPEC_XFGVIRT(IPersistSerializedPropStorage, SetPropertyStorage)
        HRESULT ( STDMETHODCALLTYPE *SetPropertyStorage )( 
            IPersistSerializedPropStorage2 * This,
            /* [annotation][in] */ 
            _In_reads_bytes_(cb)  PCUSERIALIZEDPROPSTORAGE psps,
            /* [annotation][in] */ 
            _In_  DWORD cb);
        
        DECLSPEC_XFGVIRT(IPersistSerializedPropStorage, GetPropertyStorage)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyStorage )( 
            IPersistSerializedPropStorage2 * This,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*pcb)  SERIALIZEDPROPSTORAGE **ppsps,
            /* [annotation][out] */ 
            _Out_  DWORD *pcb);
        
        DECLSPEC_XFGVIRT(IPersistSerializedPropStorage2, GetPropertyStorageSize)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyStorageSize )( 
            IPersistSerializedPropStorage2 * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcb);
        
        DECLSPEC_XFGVIRT(IPersistSerializedPropStorage2, GetPropertyStorageBuffer)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyStorageBuffer )( 
            IPersistSerializedPropStorage2 * This,
            /* [annotation][out] */ 
            _Out_writes_bytes_to_(cb, *pcbWritten)  SERIALIZEDPROPSTORAGE *psps,
            /* [in] */ DWORD cb,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbWritten);
        
        END_INTERFACE
    } IPersistSerializedPropStorage2Vtbl;

    interface IPersistSerializedPropStorage2
    {
        CONST_VTBL struct IPersistSerializedPropStorage2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPersistSerializedPropStorage2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPersistSerializedPropStorage2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPersistSerializedPropStorage2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPersistSerializedPropStorage2_SetFlags(This,flags)	\
    ( (This)->lpVtbl -> SetFlags(This,flags) ) 

#define IPersistSerializedPropStorage2_SetPropertyStorage(This,psps,cb)	\
    ( (This)->lpVtbl -> SetPropertyStorage(This,psps,cb) ) 

#define IPersistSerializedPropStorage2_GetPropertyStorage(This,ppsps,pcb)	\
    ( (This)->lpVtbl -> GetPropertyStorage(This,ppsps,pcb) ) 


#define IPersistSerializedPropStorage2_GetPropertyStorageSize(This,pcb)	\
    ( (This)->lpVtbl -> GetPropertyStorageSize(This,pcb) ) 

#define IPersistSerializedPropStorage2_GetPropertyStorageBuffer(This,psps,cb,pcbWritten)	\
    ( (This)->lpVtbl -> GetPropertyStorageBuffer(This,psps,cb,pcbWritten) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPersistSerializedPropStorage2_INTERFACE_DEFINED__ */


#ifndef __IPropertySystemChangeNotify_INTERFACE_DEFINED__
#define __IPropertySystemChangeNotify_INTERFACE_DEFINED__

/* interface IPropertySystemChangeNotify */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IPropertySystemChangeNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fa955fd9-38be-4879-a6ce-824cf52d609f")
    IPropertySystemChangeNotify : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SchemaRefreshed( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertySystemChangeNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertySystemChangeNotify * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertySystemChangeNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertySystemChangeNotify * This);
        
        DECLSPEC_XFGVIRT(IPropertySystemChangeNotify, SchemaRefreshed)
        HRESULT ( STDMETHODCALLTYPE *SchemaRefreshed )( 
            __RPC__in IPropertySystemChangeNotify * This);
        
        END_INTERFACE
    } IPropertySystemChangeNotifyVtbl;

    interface IPropertySystemChangeNotify
    {
        CONST_VTBL struct IPropertySystemChangeNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertySystemChangeNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertySystemChangeNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertySystemChangeNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertySystemChangeNotify_SchemaRefreshed(This)	\
    ( (This)->lpVtbl -> SchemaRefreshed(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertySystemChangeNotify_INTERFACE_DEFINED__ */


#ifndef __ICreateObject_INTERFACE_DEFINED__
#define __ICreateObject_INTERFACE_DEFINED__

/* interface ICreateObject */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_ICreateObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("75121952-e0d0-43e5-9380-1d80483acf72")
    ICreateObject : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateObject( 
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICreateObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICreateObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICreateObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICreateObject * This);
        
        DECLSPEC_XFGVIRT(ICreateObject, CreateObject)
        HRESULT ( STDMETHODCALLTYPE *CreateObject )( 
            __RPC__in ICreateObject * This,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        END_INTERFACE
    } ICreateObjectVtbl;

    interface ICreateObject
    {
        CONST_VTBL struct ICreateObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICreateObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICreateObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICreateObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICreateObject_CreateObject(This,clsid,pUnkOuter,riid,ppv)	\
    ( (This)->lpVtbl -> CreateObject(This,clsid,pUnkOuter,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICreateObject_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_propsys_0000_0025 */
/* [local] */ 

// Format a property value for display purposes
PSSTDAPI PSFormatForDisplay(
    _In_ REFPROPERTYKEY propkey,
    _In_ REFPROPVARIANT propvar,
    _In_ PROPDESC_FORMAT_FLAGS pdfFlags,
    _Out_writes_(cchText) LPWSTR pwszText,
    _In_ DWORD cchText);

PSSTDAPI PSFormatForDisplayAlloc(
    _In_ REFPROPERTYKEY key,
    _In_ REFPROPVARIANT propvar,
    _In_ PROPDESC_FORMAT_FLAGS pdff,
    _Outptr_ PWSTR *ppszDisplay);

PSSTDAPI PSFormatPropertyValue(
    _In_ IPropertyStore *pps,
    _In_ IPropertyDescription *ppd,
    _In_ PROPDESC_FORMAT_FLAGS pdff,
    _Outptr_ LPWSTR *ppszDisplay);

// Retrieve the image reference associated with a property value (if specified)
PSSTDAPI PSGetImageReferenceForValue(
    _In_ REFPROPERTYKEY propkey,
    _In_ REFPROPVARIANT propvar,
    _Outptr_ PWSTR *ppszImageRes);


#define PKEY_PIDSTR_MAX     10   // will take care of any long integer value
#define GUIDSTRING_MAX      (1 + 8 + 1 + 4 + 1 + 4 + 1 + 4 + 1 + 12 + 1 + 1)  // "{12345678-1234-1234-1234-123456789012}"
#define PKEYSTR_MAX         (GUIDSTRING_MAX + 1 + PKEY_PIDSTR_MAX)

// Convert a PROPERTYKEY to and from a PWSTR
PSSTDAPI PSStringFromPropertyKey(
    _In_ REFPROPERTYKEY pkey,
    _Out_writes_(cch) LPWSTR psz,
    _In_ UINT cch);

PSSTDAPI PSPropertyKeyFromString(
    _In_ LPCWSTR pszString,
    _Out_ PROPERTYKEY *pkey);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


// Creates an in-memory property store
// Returns an IPropertyStore, IPersistSerializedPropStorage, and related interfaces interface
PSSTDAPI PSCreateMemoryPropertyStore(
    _In_ REFIID riid,
    _Outptr_ void **ppv);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


// Create a read-only, delay-bind multiplexing property store
// Returns an IPropertyStore interface or related interfaces
PSSTDAPI PSCreateDelayedMultiplexPropertyStore(
    _In_ GETPROPERTYSTOREFLAGS flags,
    _In_ IDelayedPropertyStoreFactory *pdpsf,
    _In_reads_(cStores) const DWORD *rgStoreIds,
    _In_ DWORD cStores,
    _In_ REFIID riid,
    _Outptr_ void **ppv);


// Create a read-only property store from one or more sources (which each must support either IPropertyStore or IPropertySetStorage)
// Returns an IPropertyStore interface or related interfaces
PSSTDAPI PSCreateMultiplexPropertyStore(
    _In_reads_(cStores) IUnknown **prgpunkStores,
    _In_ DWORD cStores,
    _In_ REFIID riid,
    _Outptr_ void **ppv);


// Create a container for IPropertyChanges
// Returns an IPropertyChangeArray interface
PSSTDAPI PSCreatePropertyChangeArray(
    _In_reads_opt_(cChanges) const PROPERTYKEY *rgpropkey,
    _In_reads_opt_(cChanges) const PKA_FLAGS *rgflags,
    _In_reads_opt_(cChanges) const PROPVARIANT *rgpropvar,
    _In_ UINT cChanges,
    _In_ REFIID riid,
    _Outptr_ void **ppv);


// Create a simple property change
// Returns an IPropertyChange interface
PSSTDAPI PSCreateSimplePropertyChange(
    _In_ PKA_FLAGS flags,
    _In_ REFPROPERTYKEY key,
    _In_ REFPROPVARIANT propvar,
    _In_ REFIID riid,
    _Outptr_ void **ppv);


// Get a property description
// Returns an IPropertyDescription interface
PSSTDAPI PSGetPropertyDescription(
    _In_ REFPROPERTYKEY propkey,
    _In_ REFIID riid,
    _Outptr_ void **ppv);

PSSTDAPI PSGetPropertyDescriptionByName(
    _In_ LPCWSTR pszCanonicalName,
    _In_ REFIID riid,
    _Outptr_ void **ppv);


// Lookup a per-machine registered file property handler
PSSTDAPI PSLookupPropertyHandlerCLSID(
    _In_ PCWSTR pszFilePath,
    _Out_ CLSID *pclsid);
// Get a property handler, on Vista or downlevel to XP
// punkItem is a shell item created with an SHCreateItemXXX API
// Returns an IPropertyStore
PSSTDAPI PSGetItemPropertyHandler(
    _In_ IUnknown *punkItem,
    _In_ BOOL fReadWrite,
    _In_ REFIID riid,
    _Outptr_ void **ppv);


// Get a property handler, on Vista or downlevel to XP
// punkItem is a shell item created with an SHCreateItemXXX API
// punkCreateObject supports ICreateObject
// Returns an IPropertyStore
PSSTDAPI PSGetItemPropertyHandlerWithCreateObject(
    _In_ IUnknown *punkItem,
    _In_ BOOL fReadWrite,
    _In_ IUnknown *punkCreateObject,
    _In_ REFIID riid,
    _Outptr_ void **ppv);


// Get or set a property value from a store
PSSTDAPI PSGetPropertyValue(
    _In_ IPropertyStore *pps,
    _In_ IPropertyDescription *ppd,
    _Out_ PROPVARIANT *ppropvar);

PSSTDAPI PSSetPropertyValue(
    _In_ IPropertyStore *pps,
    _In_ IPropertyDescription *ppd,
    _In_ REFPROPVARIANT propvar);


// Interact with the set of property descriptions
PSSTDAPI PSRegisterPropertySchema(
    _In_ PCWSTR pszPath);

PSSTDAPI PSUnregisterPropertySchema(
    _In_ PCWSTR pszPath);

PSSTDAPI PSRefreshPropertySchema(void);

// Returns either: IPropertyDescriptionList or IEnumUnknown interfaces
PSSTDAPI PSEnumeratePropertyDescriptions(
    _In_ PROPDESC_ENUMFILTER filterOn,
    _In_ REFIID riid,
    _Outptr_ void **ppv);


// Convert between a PROPERTYKEY and its canonical name
PSSTDAPI PSGetPropertyKeyFromName(
    _In_ PCWSTR pszName,
    _Out_ PROPERTYKEY *ppropkey);

PSSTDAPI PSGetNameFromPropertyKey(
    _In_ REFPROPERTYKEY propkey,
    _Outptr_ PWSTR *ppszCanonicalName);


// Coerce and canonicalize a property value
PSSTDAPI PSCoerceToCanonicalValue(
    _In_ REFPROPERTYKEY key,
    _Inout_ PROPVARIANT *ppropvar);


// Convert a 'prop:' string into a list of property descriptions
// Returns an IPropertyDescriptionList interface
PSSTDAPI PSGetPropertyDescriptionListFromString(
    _In_ LPCWSTR pszPropList,
    _In_ REFIID riid,
    _Outptr_ void **ppv);


// Wrap an IPropertySetStorage interface in an IPropertyStore interface
// Returns an IPropertyStore or related interface
PSSTDAPI PSCreatePropertyStoreFromPropertySetStorage(
    _In_ IPropertySetStorage *ppss,
    _In_ DWORD grfMode,
    _In_ REFIID riid,
    _Outptr_ void **ppv);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


// punkSource must support IPropertyStore or IPropertySetStorage
// On success, the returned ppv is guaranteed to support IPropertyStore.
// If punkSource already supports IPropertyStore, no wrapper is created.
PSSTDAPI PSCreatePropertyStoreFromObject(
    _In_ IUnknown *punk,
    _In_ DWORD grfMode,
    _In_ REFIID riid,
    _Outptr_ void **ppv);


// punkSource must support IPropertyStore
// riid may be IPropertyStore, IPropertySetStorage, IPropertyStoreCapabilities, or IObjectProvider
PSSTDAPI PSCreateAdapterFromPropertyStore(
    _In_ IPropertyStore *pps,
    _In_ REFIID riid,
    _Outptr_ void **ppv);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


// Talk to the property system using an interface
// Returns an IPropertySystem interface
PSSTDAPI PSGetPropertySystem(
    _In_ REFIID riid,
    _Outptr_ void **ppv);


// Obtain a value from serialized property storage
PSSTDAPI PSGetPropertyFromPropertyStorage(
    _In_reads_bytes_(cb) PCUSERIALIZEDPROPSTORAGE psps, 
    _In_ DWORD cb, 
    _In_ REFPROPERTYKEY rpkey, 
    _Out_ PROPVARIANT *ppropvar);


// Obtain a named value from serialized property storage
PSSTDAPI PSGetNamedPropertyFromPropertyStorage(
    _In_reads_bytes_(cb) PCUSERIALIZEDPROPSTORAGE psps, 
    _In_ DWORD cb, 
    _In_ LPCWSTR pszName, 
    _Out_ PROPVARIANT *ppropvar);


// Helper functions for reading and writing values from IPropertyBag's.
PSSTDAPI PSPropertyBag_ReadType(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _Out_ VARIANT *var,
    VARTYPE type);
PSSTDAPI PSPropertyBag_ReadStr(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _Out_writes_(characterCount) LPWSTR value,
    int characterCount);
PSSTDAPI PSPropertyBag_ReadStrAlloc(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _Outptr_ PWSTR *value);
PSSTDAPI PSPropertyBag_ReadBSTR(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _Outptr_ BSTR *value);
PSSTDAPI PSPropertyBag_WriteStr(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _In_ LPCWSTR value);
PSSTDAPI PSPropertyBag_WriteBSTR(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _In_ BSTR value);
PSSTDAPI PSPropertyBag_ReadInt(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _Out_ INT *value);
PSSTDAPI PSPropertyBag_WriteInt(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    INT value);
PSSTDAPI PSPropertyBag_ReadSHORT(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _Out_ SHORT *value);
PSSTDAPI PSPropertyBag_WriteSHORT(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    SHORT value);
PSSTDAPI PSPropertyBag_ReadLONG(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _Out_ LONG *value);
PSSTDAPI PSPropertyBag_WriteLONG(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    LONG value);
PSSTDAPI PSPropertyBag_ReadDWORD(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _Out_ DWORD *value);
PSSTDAPI PSPropertyBag_WriteDWORD(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    DWORD value);
PSSTDAPI PSPropertyBag_ReadBOOL(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _Out_ BOOL *value);
PSSTDAPI PSPropertyBag_WriteBOOL(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    BOOL value);
PSSTDAPI PSPropertyBag_ReadPOINTL(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _Out_ POINTL *value);
PSSTDAPI PSPropertyBag_WritePOINTL(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _In_ const POINTL *value);
PSSTDAPI PSPropertyBag_ReadPOINTS(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _Out_ POINTS *value);
PSSTDAPI PSPropertyBag_WritePOINTS(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _In_ const POINTS *value);
PSSTDAPI PSPropertyBag_ReadRECTL(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _Out_ RECTL *value);
PSSTDAPI PSPropertyBag_WriteRECTL(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _In_ const RECTL *value);
PSSTDAPI PSPropertyBag_ReadStream(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _Outptr_ IStream **value);
PSSTDAPI PSPropertyBag_WriteStream(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _In_ IStream *value);
PSSTDAPI PSPropertyBag_Delete(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName);
PSSTDAPI PSPropertyBag_ReadULONGLONG(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _Out_ ULONGLONG *value);
PSSTDAPI PSPropertyBag_WriteULONGLONG(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    ULONGLONG value);
PSSTDAPI PSPropertyBag_ReadUnknown(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _In_ REFIID riid,
    _Outptr_ void **ppv);
PSSTDAPI PSPropertyBag_WriteUnknown(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _In_ IUnknown *punk);
PSSTDAPI PSPropertyBag_ReadGUID(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _Out_ GUID *value);
PSSTDAPI PSPropertyBag_WriteGUID(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _In_ const GUID *value);
PSSTDAPI PSPropertyBag_ReadPropertyKey(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _Out_ PROPERTYKEY *value);
PSSTDAPI PSPropertyBag_WritePropertyKey(
    _In_ IPropertyBag *propBag,
    _In_ LPCWSTR propName,
    _In_ REFPROPERTYKEY value);


extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0025_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0025_v0_0_s_ifspec;


#ifndef __PropSysObjects_LIBRARY_DEFINED__
#define __PropSysObjects_LIBRARY_DEFINED__

/* library PropSysObjects */
/* [version][lcid][uuid] */ 


EXTERN_C const IID LIBID_PropSysObjects;

EXTERN_C const CLSID CLSID_InMemoryPropertyStore;

#ifdef __cplusplus

class DECLSPEC_UUID("9a02e012-6303-4e1e-b9a1-630f802592c5")
InMemoryPropertyStore;
#endif

EXTERN_C const CLSID CLSID_InMemoryPropertyStoreMarshalByValue;

#ifdef __cplusplus

class DECLSPEC_UUID("D4CA0E2D-6DA7-4b75-A97C-5F306F0EAEDC")
InMemoryPropertyStoreMarshalByValue;
#endif

EXTERN_C const CLSID CLSID_PropertySystem;

#ifdef __cplusplus

class DECLSPEC_UUID("b8967f85-58ae-4f46-9fb2-5d7904798f4b")
PropertySystem;
#endif
#endif /* __PropSysObjects_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_propsys_0000_0026 */
/* [local] */ 

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0026_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_propsys_0000_0026_v0_0_s_ifspec;

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

/* [local] */ HRESULT STDMETHODCALLTYPE IInitializeWithStream_Initialize_Proxy( 
    IInitializeWithStream * This,
    /* [annotation][in] */ 
    _In_  IStream *pstream,
    /* [annotation][in] */ 
    _In_  DWORD grfMode);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IInitializeWithStream_Initialize_Stub( 
    __RPC__in IInitializeWithStream * This,
    /* [in] */ __RPC__in_opt IStream *pstream,
    /* [in] */ DWORD grfMode);

/* [local] */ HRESULT STDMETHODCALLTYPE IPropertyDescription_CoerceToCanonicalValue_Proxy( 
    IPropertyDescription * This,
    /* [annotation][out][in] */ 
    _Inout_  PROPVARIANT *ppropvar);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IPropertyDescription_CoerceToCanonicalValue_Stub( 
    __RPC__in IPropertyDescription * This,
    /* [in] */ __RPC__in REFPROPVARIANT propvar,
    /* [out] */ __RPC__out PROPVARIANT *ppropvar);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


