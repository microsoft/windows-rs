

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

#ifndef __windowsstoragecom_h__
#define __windowsstoragecom_h__

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

#ifndef __IRandomAccessStreamFileAccessMode_FWD_DEFINED__
#define __IRandomAccessStreamFileAccessMode_FWD_DEFINED__
typedef interface IRandomAccessStreamFileAccessMode IRandomAccessStreamFileAccessMode;

#endif 	/* __IRandomAccessStreamFileAccessMode_FWD_DEFINED__ */


#ifndef __IUnbufferedFileHandleOplockCallback_FWD_DEFINED__
#define __IUnbufferedFileHandleOplockCallback_FWD_DEFINED__
typedef interface IUnbufferedFileHandleOplockCallback IUnbufferedFileHandleOplockCallback;

#endif 	/* __IUnbufferedFileHandleOplockCallback_FWD_DEFINED__ */


#ifndef __IUnbufferedFileHandleProvider_FWD_DEFINED__
#define __IUnbufferedFileHandleProvider_FWD_DEFINED__
typedef interface IUnbufferedFileHandleProvider IUnbufferedFileHandleProvider;

#endif 	/* __IUnbufferedFileHandleProvider_FWD_DEFINED__ */


#ifndef __IOplockBreakingHandler_FWD_DEFINED__
#define __IOplockBreakingHandler_FWD_DEFINED__
typedef interface IOplockBreakingHandler IOplockBreakingHandler;

#endif 	/* __IOplockBreakingHandler_FWD_DEFINED__ */


#ifndef __IStorageItemHandleAccess_FWD_DEFINED__
#define __IStorageItemHandleAccess_FWD_DEFINED__
typedef interface IStorageItemHandleAccess IStorageItemHandleAccess;

#endif 	/* __IStorageItemHandleAccess_FWD_DEFINED__ */


#ifndef __IStorageFolderHandleAccess_FWD_DEFINED__
#define __IStorageFolderHandleAccess_FWD_DEFINED__
typedef interface IStorageFolderHandleAccess IStorageFolderHandleAccess;

#endif 	/* __IStorageFolderHandleAccess_FWD_DEFINED__ */


#ifndef __IDDEInitializer_FWD_DEFINED__
#define __IDDEInitializer_FWD_DEFINED__
typedef interface IDDEInitializer IDDEInitializer;

#endif 	/* __IDDEInitializer_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "shobjidl_core.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_windowsstoragecom_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_windowsstoragecom_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windowsstoragecom_0000_0000_v0_0_s_ifspec;

#ifndef __IRandomAccessStreamFileAccessMode_INTERFACE_DEFINED__
#define __IRandomAccessStreamFileAccessMode_INTERFACE_DEFINED__

/* interface IRandomAccessStreamFileAccessMode */
/* [uuid][object] */ 


EXTERN_C const IID IID_IRandomAccessStreamFileAccessMode;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("332E5848-2E15-458E-85C4-C911C0C3D6F4")
    IRandomAccessStreamFileAccessMode : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMode( 
            /* [out] */ __RPC__out DWORD *fileAccessMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRandomAccessStreamFileAccessModeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRandomAccessStreamFileAccessMode * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRandomAccessStreamFileAccessMode * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRandomAccessStreamFileAccessMode * This);
        
        DECLSPEC_XFGVIRT(IRandomAccessStreamFileAccessMode, GetMode)
        HRESULT ( STDMETHODCALLTYPE *GetMode )( 
            __RPC__in IRandomAccessStreamFileAccessMode * This,
            /* [out] */ __RPC__out DWORD *fileAccessMode);
        
        END_INTERFACE
    } IRandomAccessStreamFileAccessModeVtbl;

    interface IRandomAccessStreamFileAccessMode
    {
        CONST_VTBL struct IRandomAccessStreamFileAccessModeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRandomAccessStreamFileAccessMode_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRandomAccessStreamFileAccessMode_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRandomAccessStreamFileAccessMode_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRandomAccessStreamFileAccessMode_GetMode(This,fileAccessMode)	\
    ( (This)->lpVtbl -> GetMode(This,fileAccessMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRandomAccessStreamFileAccessMode_INTERFACE_DEFINED__ */


#ifndef __IUnbufferedFileHandleOplockCallback_INTERFACE_DEFINED__
#define __IUnbufferedFileHandleOplockCallback_INTERFACE_DEFINED__

/* interface IUnbufferedFileHandleOplockCallback */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_IUnbufferedFileHandleOplockCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D1019A0E-6243-4329-8497-2E75894D7710")
    IUnbufferedFileHandleOplockCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnBrokenCallback( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUnbufferedFileHandleOplockCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUnbufferedFileHandleOplockCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUnbufferedFileHandleOplockCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUnbufferedFileHandleOplockCallback * This);
        
        DECLSPEC_XFGVIRT(IUnbufferedFileHandleOplockCallback, OnBrokenCallback)
        HRESULT ( STDMETHODCALLTYPE *OnBrokenCallback )( 
            IUnbufferedFileHandleOplockCallback * This);
        
        END_INTERFACE
    } IUnbufferedFileHandleOplockCallbackVtbl;

    interface IUnbufferedFileHandleOplockCallback
    {
        CONST_VTBL struct IUnbufferedFileHandleOplockCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUnbufferedFileHandleOplockCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUnbufferedFileHandleOplockCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUnbufferedFileHandleOplockCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUnbufferedFileHandleOplockCallback_OnBrokenCallback(This)	\
    ( (This)->lpVtbl -> OnBrokenCallback(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUnbufferedFileHandleOplockCallback_INTERFACE_DEFINED__ */


#ifndef __IUnbufferedFileHandleProvider_INTERFACE_DEFINED__
#define __IUnbufferedFileHandleProvider_INTERFACE_DEFINED__

/* interface IUnbufferedFileHandleProvider */
/* [uuid][object] */ 


EXTERN_C const IID IID_IUnbufferedFileHandleProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A65C9109-42AB-4B94-A7B1-DD2E4E68515E")
    IUnbufferedFileHandleProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OpenUnbufferedFileHandle( 
            /* [in] */ __RPC__in_opt IUnbufferedFileHandleOplockCallback *oplockBreakCallback,
            /* [retval][out] */ __RPC__out DWORD_PTR *fileHandle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CloseUnbufferedFileHandle( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUnbufferedFileHandleProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUnbufferedFileHandleProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUnbufferedFileHandleProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUnbufferedFileHandleProvider * This);
        
        DECLSPEC_XFGVIRT(IUnbufferedFileHandleProvider, OpenUnbufferedFileHandle)
        HRESULT ( STDMETHODCALLTYPE *OpenUnbufferedFileHandle )( 
            __RPC__in IUnbufferedFileHandleProvider * This,
            /* [in] */ __RPC__in_opt IUnbufferedFileHandleOplockCallback *oplockBreakCallback,
            /* [retval][out] */ __RPC__out DWORD_PTR *fileHandle);
        
        DECLSPEC_XFGVIRT(IUnbufferedFileHandleProvider, CloseUnbufferedFileHandle)
        HRESULT ( STDMETHODCALLTYPE *CloseUnbufferedFileHandle )( 
            __RPC__in IUnbufferedFileHandleProvider * This);
        
        END_INTERFACE
    } IUnbufferedFileHandleProviderVtbl;

    interface IUnbufferedFileHandleProvider
    {
        CONST_VTBL struct IUnbufferedFileHandleProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUnbufferedFileHandleProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUnbufferedFileHandleProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUnbufferedFileHandleProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUnbufferedFileHandleProvider_OpenUnbufferedFileHandle(This,oplockBreakCallback,fileHandle)	\
    ( (This)->lpVtbl -> OpenUnbufferedFileHandle(This,oplockBreakCallback,fileHandle) ) 

#define IUnbufferedFileHandleProvider_CloseUnbufferedFileHandle(This)	\
    ( (This)->lpVtbl -> CloseUnbufferedFileHandle(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUnbufferedFileHandleProvider_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_windowsstoragecom_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef /* [v1_enum] */ 
enum HANDLE_OPTIONS
    {
        HO_NONE	= 0,
        HO_OPEN_REQUIRING_OPLOCK	= 0x40000,
        HO_DELETE_ON_CLOSE	= 0x4000000,
        HO_SEQUENTIAL_SCAN	= 0x8000000,
        HO_RANDOM_ACCESS	= 0x10000000,
        HO_NO_BUFFERING	= 0x20000000,
        HO_OVERLAPPED	= 0x40000000,
        HO_WRITE_THROUGH	= 0x80000000
    } 	HANDLE_OPTIONS;

DEFINE_ENUM_FLAG_OPERATORS(HANDLE_OPTIONS);
typedef /* [v1_enum] */ 
enum HANDLE_ACCESS_OPTIONS
    {
        HAO_NONE	= 0,
        HAO_READ_ATTRIBUTES	= 0x80,
        HAO_READ	= 0x120089,
        HAO_WRITE	= 0x120116,
        HAO_DELETE	= 0x10000
    } 	HANDLE_ACCESS_OPTIONS;

DEFINE_ENUM_FLAG_OPERATORS(HANDLE_ACCESS_OPTIONS);
typedef /* [v1_enum] */ 
enum HANDLE_SHARING_OPTIONS
    {
        HSO_SHARE_NONE	= 0,
        HSO_SHARE_READ	= 0x1,
        HSO_SHARE_WRITE	= 0x2,
        HSO_SHARE_DELETE	= 0x4
    } 	HANDLE_SHARING_OPTIONS;

DEFINE_ENUM_FLAG_OPERATORS(HANDLE_SHARING_OPTIONS);
typedef /* [v1_enum] */ 
enum HANDLE_CREATION_OPTIONS
    {
        HCO_CREATE_NEW	= 0x1,
        HCO_CREATE_ALWAYS	= 0x2,
        HCO_OPEN_EXISTING	= 0x3,
        HCO_OPEN_ALWAYS	= 0x4,
        HCO_TRUNCATE_EXISTING	= 0x5
    } 	HANDLE_CREATION_OPTIONS;



extern RPC_IF_HANDLE __MIDL_itf_windowsstoragecom_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windowsstoragecom_0000_0003_v0_0_s_ifspec;

#ifndef __IOplockBreakingHandler_INTERFACE_DEFINED__
#define __IOplockBreakingHandler_INTERFACE_DEFINED__

/* interface IOplockBreakingHandler */
/* [uuid][object] */ 


EXTERN_C const IID IID_IOplockBreakingHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("826ABE3D-3ACD-47D3-84F2-88AAEDCF6304")
    IOplockBreakingHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OplockBreaking( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOplockBreakingHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOplockBreakingHandler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOplockBreakingHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOplockBreakingHandler * This);
        
        DECLSPEC_XFGVIRT(IOplockBreakingHandler, OplockBreaking)
        HRESULT ( STDMETHODCALLTYPE *OplockBreaking )( 
            __RPC__in IOplockBreakingHandler * This);
        
        END_INTERFACE
    } IOplockBreakingHandlerVtbl;

    interface IOplockBreakingHandler
    {
        CONST_VTBL struct IOplockBreakingHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOplockBreakingHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOplockBreakingHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOplockBreakingHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOplockBreakingHandler_OplockBreaking(This)	\
    ( (This)->lpVtbl -> OplockBreaking(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOplockBreakingHandler_INTERFACE_DEFINED__ */


#ifndef __IStorageItemHandleAccess_INTERFACE_DEFINED__
#define __IStorageItemHandleAccess_INTERFACE_DEFINED__

/* interface IStorageItemHandleAccess */
/* [uuid][object] */ 


EXTERN_C const IID IID_IStorageItemHandleAccess;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5CA296B2-2C25-4D22-B785-B885C8201E6A")
    IStorageItemHandleAccess : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ HANDLE_ACCESS_OPTIONS accessOptions,
            /* [in] */ HANDLE_SHARING_OPTIONS sharingOptions,
            /* [in] */ HANDLE_OPTIONS options,
            /* [optional][in] */ __RPC__in_opt IOplockBreakingHandler *oplockBreakingHandler,
            /* [system_handle][retval][out] */ __RPC__deref_out_opt HANDLE *interopHandle) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStorageItemHandleAccessVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStorageItemHandleAccess * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStorageItemHandleAccess * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStorageItemHandleAccess * This);
        
        DECLSPEC_XFGVIRT(IStorageItemHandleAccess, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IStorageItemHandleAccess * This,
            /* [in] */ HANDLE_ACCESS_OPTIONS accessOptions,
            /* [in] */ HANDLE_SHARING_OPTIONS sharingOptions,
            /* [in] */ HANDLE_OPTIONS options,
            /* [optional][in] */ __RPC__in_opt IOplockBreakingHandler *oplockBreakingHandler,
            /* [system_handle][retval][out] */ __RPC__deref_out_opt HANDLE *interopHandle);
        
        END_INTERFACE
    } IStorageItemHandleAccessVtbl;

    interface IStorageItemHandleAccess
    {
        CONST_VTBL struct IStorageItemHandleAccessVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStorageItemHandleAccess_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStorageItemHandleAccess_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStorageItemHandleAccess_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStorageItemHandleAccess_Create(This,accessOptions,sharingOptions,options,oplockBreakingHandler,interopHandle)	\
    ( (This)->lpVtbl -> Create(This,accessOptions,sharingOptions,options,oplockBreakingHandler,interopHandle) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStorageItemHandleAccess_INTERFACE_DEFINED__ */


#ifndef __IStorageFolderHandleAccess_INTERFACE_DEFINED__
#define __IStorageFolderHandleAccess_INTERFACE_DEFINED__

/* interface IStorageFolderHandleAccess */
/* [uuid][object] */ 


EXTERN_C const IID IID_IStorageFolderHandleAccess;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DF19938F-5462-48A0-BE65-D2A3271A08D6")
    IStorageFolderHandleAccess : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [string][in] */ __RPC__in_string LPCWSTR fileName,
            /* [in] */ HANDLE_CREATION_OPTIONS creationOptions,
            /* [in] */ HANDLE_ACCESS_OPTIONS accessOptions,
            /* [in] */ HANDLE_SHARING_OPTIONS sharingOptions,
            /* [in] */ HANDLE_OPTIONS options,
            /* [optional][in] */ __RPC__in_opt IOplockBreakingHandler *oplockBreakingHandler,
            /* [system_handle][retval][out] */ __RPC__deref_out_opt HANDLE *interopHandle) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStorageFolderHandleAccessVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStorageFolderHandleAccess * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStorageFolderHandleAccess * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStorageFolderHandleAccess * This);
        
        DECLSPEC_XFGVIRT(IStorageFolderHandleAccess, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IStorageFolderHandleAccess * This,
            /* [string][in] */ __RPC__in_string LPCWSTR fileName,
            /* [in] */ HANDLE_CREATION_OPTIONS creationOptions,
            /* [in] */ HANDLE_ACCESS_OPTIONS accessOptions,
            /* [in] */ HANDLE_SHARING_OPTIONS sharingOptions,
            /* [in] */ HANDLE_OPTIONS options,
            /* [optional][in] */ __RPC__in_opt IOplockBreakingHandler *oplockBreakingHandler,
            /* [system_handle][retval][out] */ __RPC__deref_out_opt HANDLE *interopHandle);
        
        END_INTERFACE
    } IStorageFolderHandleAccessVtbl;

    interface IStorageFolderHandleAccess
    {
        CONST_VTBL struct IStorageFolderHandleAccessVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStorageFolderHandleAccess_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStorageFolderHandleAccess_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStorageFolderHandleAccess_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStorageFolderHandleAccess_Create(This,fileName,creationOptions,accessOptions,sharingOptions,options,oplockBreakingHandler,interopHandle)	\
    ( (This)->lpVtbl -> Create(This,fileName,creationOptions,accessOptions,sharingOptions,options,oplockBreakingHandler,interopHandle) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStorageFolderHandleAccess_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_windowsstoragecom_0000_0006 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [v1_enum] */ 
enum CreateProcessMethod
    {
        CpCreateProcess	= 0,
        CpCreateProcessAsUser	= 1,
        CpAicLaunchAdminProcess	= 2
    } 	CreateProcessMethod;



extern RPC_IF_HANDLE __MIDL_itf_windowsstoragecom_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windowsstoragecom_0000_0006_v0_0_s_ifspec;

#ifndef __IDDEInitializer_INTERFACE_DEFINED__
#define __IDDEInitializer_INTERFACE_DEFINED__

/* interface IDDEInitializer */
/* [uuid][object] */ 


EXTERN_C const IID IID_IDDEInitializer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("30DC931F-33FC-4FFD-A168-942258CF3CA4")
    IDDEInitializer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [string][in] */ __RPC__in_string LPCWSTR fileExtensionOrProtocol,
            /* [in] */ CreateProcessMethod method,
            /* [string][in] */ __RPC__in_string LPCWSTR currentDirectory,
            /* [in] */ __RPC__in_opt IShellItem *execTarget,
            /* [in] */ __RPC__in_opt IUnknown *site,
            /* [string][in] */ __RPC__in_string LPCWSTR application,
            /* [string][in] */ __RPC__in_string LPCWSTR targetFile,
            /* [string][in] */ __RPC__in_string LPCWSTR arguments,
            /* [string][in] */ __RPC__in_string LPCWSTR verb) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDDEInitializerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDDEInitializer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDDEInitializer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDDEInitializer * This);
        
        DECLSPEC_XFGVIRT(IDDEInitializer, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IDDEInitializer * This,
            /* [string][in] */ __RPC__in_string LPCWSTR fileExtensionOrProtocol,
            /* [in] */ CreateProcessMethod method,
            /* [string][in] */ __RPC__in_string LPCWSTR currentDirectory,
            /* [in] */ __RPC__in_opt IShellItem *execTarget,
            /* [in] */ __RPC__in_opt IUnknown *site,
            /* [string][in] */ __RPC__in_string LPCWSTR application,
            /* [string][in] */ __RPC__in_string LPCWSTR targetFile,
            /* [string][in] */ __RPC__in_string LPCWSTR arguments,
            /* [string][in] */ __RPC__in_string LPCWSTR verb);
        
        END_INTERFACE
    } IDDEInitializerVtbl;

    interface IDDEInitializer
    {
        CONST_VTBL struct IDDEInitializerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDDEInitializer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDDEInitializer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDDEInitializer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDDEInitializer_Initialize(This,fileExtensionOrProtocol,method,currentDirectory,execTarget,site,application,targetFile,arguments,verb)	\
    ( (This)->lpVtbl -> Initialize(This,fileExtensionOrProtocol,method,currentDirectory,execTarget,site,application,targetFile,arguments,verb) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDDEInitializer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_windowsstoragecom_0000_0007 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_windowsstoragecom_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windowsstoragecom_0000_0007_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


