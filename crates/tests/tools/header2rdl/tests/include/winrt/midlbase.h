

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

#ifndef __midlbase_h__
#define __midlbase_h__

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

#ifndef __IUnknown_FWD_DEFINED__
#define __IUnknown_FWD_DEFINED__
typedef interface IUnknown IUnknown;

#endif 	/* __IUnknown_FWD_DEFINED__ */


#ifndef __IInspectable_FWD_DEFINED__
#define __IInspectable_FWD_DEFINED__
typedef interface IInspectable IInspectable;

#endif 	/* __IInspectable_FWD_DEFINED__ */


#ifndef __IAsyncInfo_FWD_DEFINED__
#define __IAsyncInfo_FWD_DEFINED__
typedef interface IAsyncInfo IAsyncInfo;

#endif 	/* __IAsyncInfo_FWD_DEFINED__ */


#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_midlbase_0000_0000 */
/* [local] */ 

#if 0 // midlbase: Suppress duplicate definition of Windows types required for WinRT compilation
typedef unsigned long HRESULT;

typedef /* [public][public][public] */ struct __MIDL___MIDL_itf_midlbase_0000_0000_0001
    {
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    byte Data4[ 8 ];
    } 	GUID;

typedef wchar_t WCHAR;

typedef byte BYTE;

typedef short INT16;

typedef unsigned short UINT16;

typedef unsigned long ULONG;

typedef unsigned int UINT32;

typedef int INT32;

typedef __int64 INT64;

typedef unsigned __int64 UINT64;

typedef float FLOAT;

typedef double DOUBLE;



extern RPC_IF_HANDLE __MIDL_itf_midlbase_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_midlbase_0000_0000_v0_0_s_ifspec;

#ifndef __IUnknown_INTERFACE_DEFINED__
#define __IUnknown_INTERFACE_DEFINED__

/* interface IUnknown */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IUnknown;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000000-0000-0000-C000-000000000046")
    IUnknown
    {
    public:
        BEGIN_INTERFACE
        virtual HRESULT STDMETHODCALLTYPE QueryInterface( 
            /* [in] */ const GUID *riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject) = 0;
        
        virtual ULONG STDMETHODCALLTYPE AddRef( void) = 0;
        
        virtual ULONG STDMETHODCALLTYPE Release( void) = 0;
        
        END_INTERFACE
    };
    
    
#else 	/* C style interface */

    typedef struct IUnknownVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUnknown * This,
            /* [in] */ const GUID *riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUnknown * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUnknown * This);
        
        END_INTERFACE
    } IUnknownVtbl;

    interface IUnknown
    {
        CONST_VTBL struct IUnknownVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUnknown_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUnknown_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUnknown_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



HRESULT STDMETHODCALLTYPE IUnknown_QueryInterface_Proxy( 
    IUnknown * This,
    /* [in] */ const GUID *riid,
    /* [annotation][iid_is][out] */ 
    __RPC__deref_out  void **ppvObject);


void __RPC_STUB IUnknown_QueryInterface_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


ULONG STDMETHODCALLTYPE IUnknown_AddRef_Proxy( 
    IUnknown * This);


void __RPC_STUB IUnknown_AddRef_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


ULONG STDMETHODCALLTYPE IUnknown_Release_Proxy( 
    IUnknown * This);


void __RPC_STUB IUnknown_Release_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IUnknown_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_midlbase_0000_0001 */
/* [local] */ 

typedef struct HSTRING__
    {
    int unused;
    } 	HSTRING__;

typedef struct _FLAGGED_WORD_BLOB
    {
    ULONG fFlags;
    ULONG clSize;
    /* [size_is] */ unsigned short asData[ 1 ];
    } 	FLAGGED_WORD_BLOB;

typedef /* [unique] */  __RPC_unique_pointer FLAGGED_WORD_BLOB *wireBSTR;

typedef /* [unique][wire_marshal] */  __RPC_unique_pointer HSTRING__ *HSTRING;

typedef struct EventRegistrationToken
    {
    __int64 value;
    } 	EventRegistrationToken;

typedef /* [v1_enum] */ 
enum TrustLevel
    {
        BaseTrust	= 0,
        PartialTrust	= ( BaseTrust + 1 ) ,
        FullTrust	= ( PartialTrust + 1 ) 
    } 	TrustLevel;



extern RPC_IF_HANDLE __MIDL_itf_midlbase_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_midlbase_0000_0001_v0_0_s_ifspec;

#ifndef __IInspectable_INTERFACE_DEFINED__
#define __IInspectable_INTERFACE_DEFINED__

/* interface IInspectable */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IInspectable;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AF86E2E0-B12D-4c6a-9C5A-D7AA65101E90")
    IInspectable : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetIids( 
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) GUID **iids) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRuntimeClassName( 
            /* [out] */ __RPC__deref_out_opt HSTRING *className) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTrustLevel( 
            /* [out] */ __RPC__out TrustLevel *trustLevel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInspectableVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInspectable * This,
            /* [in] */ __RPC__in const GUID *riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInspectable * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInspectable * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            __RPC__in IInspectable * This,
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) GUID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            __RPC__in IInspectable * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            __RPC__in IInspectable * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);
        
        END_INTERFACE
    } IInspectableVtbl;

    interface IInspectable
    {
        CONST_VTBL struct IInspectableVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInspectable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInspectable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInspectable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInspectable_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IInspectable_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IInspectable_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInspectable_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_midlbase_0000_0002 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum AsyncStatus
    {
        Started	= 0,
        Completed	= ( Started + 1 ) ,
        Canceled	= ( Completed + 1 ) ,
        Error	= ( Canceled + 1 ) 
    } 	AsyncStatus;



extern RPC_IF_HANDLE __MIDL_itf_midlbase_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_midlbase_0000_0002_v0_0_s_ifspec;

#ifndef __IAsyncInfo_INTERFACE_DEFINED__
#define __IAsyncInfo_INTERFACE_DEFINED__

/* interface IAsyncInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IAsyncInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000036-0000-0000-C000-000000000046")
    IAsyncInfo : public IInspectable
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__out unsigned __int32 *id) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out AsyncStatus *status) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ErrorCode( 
            /* [retval][out] */ __RPC__out HRESULT *errorCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAsyncInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAsyncInfo * This,
            /* [in] */ __RPC__in const GUID *riid,
            /* [annotation][iid_is][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAsyncInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAsyncInfo * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            __RPC__in IAsyncInfo * This,
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) GUID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            __RPC__in IAsyncInfo * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            __RPC__in IAsyncInfo * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IAsyncInfo, get_Id)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IAsyncInfo * This,
            /* [retval][out] */ __RPC__out unsigned __int32 *id);
        
        DECLSPEC_XFGVIRT(IAsyncInfo, get_Status)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IAsyncInfo * This,
            /* [retval][out] */ __RPC__out AsyncStatus *status);
        
        DECLSPEC_XFGVIRT(IAsyncInfo, get_ErrorCode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ErrorCode )( 
            __RPC__in IAsyncInfo * This,
            /* [retval][out] */ __RPC__out HRESULT *errorCode);
        
        DECLSPEC_XFGVIRT(IAsyncInfo, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IAsyncInfo * This);
        
        DECLSPEC_XFGVIRT(IAsyncInfo, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IAsyncInfo * This);
        
        END_INTERFACE
    } IAsyncInfoVtbl;

    interface IAsyncInfo
    {
        CONST_VTBL struct IAsyncInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAsyncInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAsyncInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAsyncInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAsyncInfo_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IAsyncInfo_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IAsyncInfo_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IAsyncInfo_get_Id(This,id)	\
    ( (This)->lpVtbl -> get_Id(This,id) ) 

#define IAsyncInfo_get_Status(This,status)	\
    ( (This)->lpVtbl -> get_Status(This,status) ) 

#define IAsyncInfo_get_ErrorCode(This,errorCode)	\
    ( (This)->lpVtbl -> get_ErrorCode(This,errorCode) ) 

#define IAsyncInfo_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IAsyncInfo_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAsyncInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_midlbase_0000_0003 */
/* [local] */ 

#else
#include <oaidl.h>
#include <hstring.h>
#include <inspectable.h>
#include <asyncinfo.h>
#include <eventtoken.h>
#include <ivectorchangedeventargs.h>
#endif // 0 WinRTBase: Suppress duplicate definition of Windows types required for WinRT compilation


extern RPC_IF_HANDLE __MIDL_itf_midlbase_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_midlbase_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  HSTRING_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HSTRING * ); 
unsigned char * __RPC_USER  HSTRING_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HSTRING * ); 
unsigned char * __RPC_USER  HSTRING_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HSTRING * ); 
void                      __RPC_USER  HSTRING_UserFree(     __RPC__in unsigned long *, __RPC__in HSTRING * ); 

unsigned long             __RPC_USER  HSTRING_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HSTRING * ); 
unsigned char * __RPC_USER  HSTRING_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HSTRING * ); 
unsigned char * __RPC_USER  HSTRING_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HSTRING * ); 
void                      __RPC_USER  HSTRING_UserFree64(     __RPC__in unsigned long *, __RPC__in HSTRING * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


