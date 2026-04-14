

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

#ifndef __bidispl_h__
#define __bidispl_h__

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

#ifndef __IBidiRequest_FWD_DEFINED__
#define __IBidiRequest_FWD_DEFINED__
typedef interface IBidiRequest IBidiRequest;

#endif 	/* __IBidiRequest_FWD_DEFINED__ */


#ifndef __IBidiRequestContainer_FWD_DEFINED__
#define __IBidiRequestContainer_FWD_DEFINED__
typedef interface IBidiRequestContainer IBidiRequestContainer;

#endif 	/* __IBidiRequestContainer_FWD_DEFINED__ */


#ifndef __IBidiSpl_FWD_DEFINED__
#define __IBidiSpl_FWD_DEFINED__
typedef interface IBidiSpl IBidiSpl;

#endif 	/* __IBidiSpl_FWD_DEFINED__ */


#ifndef __IBidiSpl2_FWD_DEFINED__
#define __IBidiSpl2_FWD_DEFINED__
typedef interface IBidiSpl2 IBidiSpl2;

#endif 	/* __IBidiSpl2_FWD_DEFINED__ */


#ifndef __BidiRequest_FWD_DEFINED__
#define __BidiRequest_FWD_DEFINED__

#ifdef __cplusplus
typedef class BidiRequest BidiRequest;
#else
typedef struct BidiRequest BidiRequest;
#endif /* __cplusplus */

#endif 	/* __BidiRequest_FWD_DEFINED__ */


#ifndef __BidiRequestContainer_FWD_DEFINED__
#define __BidiRequestContainer_FWD_DEFINED__

#ifdef __cplusplus
typedef class BidiRequestContainer BidiRequestContainer;
#else
typedef struct BidiRequestContainer BidiRequestContainer;
#endif /* __cplusplus */

#endif 	/* __BidiRequestContainer_FWD_DEFINED__ */


#ifndef __BidiSpl_FWD_DEFINED__
#define __BidiSpl_FWD_DEFINED__

#ifdef __cplusplus
typedef class BidiSpl BidiSpl;
#else
typedef struct BidiSpl BidiSpl;
#endif /* __cplusplus */

#endif 	/* __BidiSpl_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_bidispl_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_bidispl_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bidispl_0000_0000_v0_0_s_ifspec;

#ifndef __IBidiRequest_INTERFACE_DEFINED__
#define __IBidiRequest_INTERFACE_DEFINED__

/* interface IBidiRequest */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBidiRequest;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8F348BD7-4B47-4755-8A9D-0F422DF3DC89")
    IBidiRequest : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetSchema( 
            /* [in] */ __RPC__in const LPCWSTR pszSchema) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetInputData( 
            /* [in] */ const DWORD dwType,
            /* [in] */ __RPC__in const BYTE *pData,
            /* [in] */ const UINT uSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetResult( 
            /* [out] */ __RPC__out HRESULT *phr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputData( 
            /* [in] */ const DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszSchema,
            /* [out] */ __RPC__out DWORD *pdwType,
            /* [out] */ __RPC__deref_out_opt BYTE **ppData,
            /* [out] */ __RPC__out ULONG *uSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnumCount( 
            /* [out] */ __RPC__out DWORD *pdwTotal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBidiRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBidiRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBidiRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBidiRequest * This);
        
        DECLSPEC_XFGVIRT(IBidiRequest, SetSchema)
        HRESULT ( STDMETHODCALLTYPE *SetSchema )( 
            __RPC__in IBidiRequest * This,
            /* [in] */ __RPC__in const LPCWSTR pszSchema);
        
        DECLSPEC_XFGVIRT(IBidiRequest, SetInputData)
        HRESULT ( STDMETHODCALLTYPE *SetInputData )( 
            __RPC__in IBidiRequest * This,
            /* [in] */ const DWORD dwType,
            /* [in] */ __RPC__in const BYTE *pData,
            /* [in] */ const UINT uSize);
        
        DECLSPEC_XFGVIRT(IBidiRequest, GetResult)
        HRESULT ( STDMETHODCALLTYPE *GetResult )( 
            __RPC__in IBidiRequest * This,
            /* [out] */ __RPC__out HRESULT *phr);
        
        DECLSPEC_XFGVIRT(IBidiRequest, GetOutputData)
        HRESULT ( STDMETHODCALLTYPE *GetOutputData )( 
            __RPC__in IBidiRequest * This,
            /* [in] */ const DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszSchema,
            /* [out] */ __RPC__out DWORD *pdwType,
            /* [out] */ __RPC__deref_out_opt BYTE **ppData,
            /* [out] */ __RPC__out ULONG *uSize);
        
        DECLSPEC_XFGVIRT(IBidiRequest, GetEnumCount)
        HRESULT ( STDMETHODCALLTYPE *GetEnumCount )( 
            __RPC__in IBidiRequest * This,
            /* [out] */ __RPC__out DWORD *pdwTotal);
        
        END_INTERFACE
    } IBidiRequestVtbl;

    interface IBidiRequest
    {
        CONST_VTBL struct IBidiRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBidiRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBidiRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBidiRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBidiRequest_SetSchema(This,pszSchema)	\
    ( (This)->lpVtbl -> SetSchema(This,pszSchema) ) 

#define IBidiRequest_SetInputData(This,dwType,pData,uSize)	\
    ( (This)->lpVtbl -> SetInputData(This,dwType,pData,uSize) ) 

#define IBidiRequest_GetResult(This,phr)	\
    ( (This)->lpVtbl -> GetResult(This,phr) ) 

#define IBidiRequest_GetOutputData(This,dwIndex,ppszSchema,pdwType,ppData,uSize)	\
    ( (This)->lpVtbl -> GetOutputData(This,dwIndex,ppszSchema,pdwType,ppData,uSize) ) 

#define IBidiRequest_GetEnumCount(This,pdwTotal)	\
    ( (This)->lpVtbl -> GetEnumCount(This,pdwTotal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBidiRequest_INTERFACE_DEFINED__ */


#ifndef __IBidiRequestContainer_INTERFACE_DEFINED__
#define __IBidiRequestContainer_INTERFACE_DEFINED__

/* interface IBidiRequestContainer */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBidiRequestContainer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D752F6C0-94A8-4275-A77D-8F1D1A1121AE")
    IBidiRequestContainer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddRequest( 
            /* [in] */ __RPC__in_opt IBidiRequest *pRequest) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnumObject( 
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **ppenum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRequestCount( 
            /* [out] */ __RPC__out ULONG *puCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBidiRequestContainerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBidiRequestContainer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBidiRequestContainer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBidiRequestContainer * This);
        
        DECLSPEC_XFGVIRT(IBidiRequestContainer, AddRequest)
        HRESULT ( STDMETHODCALLTYPE *AddRequest )( 
            __RPC__in IBidiRequestContainer * This,
            /* [in] */ __RPC__in_opt IBidiRequest *pRequest);
        
        DECLSPEC_XFGVIRT(IBidiRequestContainer, GetEnumObject)
        HRESULT ( STDMETHODCALLTYPE *GetEnumObject )( 
            __RPC__in IBidiRequestContainer * This,
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **ppenum);
        
        DECLSPEC_XFGVIRT(IBidiRequestContainer, GetRequestCount)
        HRESULT ( STDMETHODCALLTYPE *GetRequestCount )( 
            __RPC__in IBidiRequestContainer * This,
            /* [out] */ __RPC__out ULONG *puCount);
        
        END_INTERFACE
    } IBidiRequestContainerVtbl;

    interface IBidiRequestContainer
    {
        CONST_VTBL struct IBidiRequestContainerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBidiRequestContainer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBidiRequestContainer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBidiRequestContainer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBidiRequestContainer_AddRequest(This,pRequest)	\
    ( (This)->lpVtbl -> AddRequest(This,pRequest) ) 

#define IBidiRequestContainer_GetEnumObject(This,ppenum)	\
    ( (This)->lpVtbl -> GetEnumObject(This,ppenum) ) 

#define IBidiRequestContainer_GetRequestCount(This,puCount)	\
    ( (This)->lpVtbl -> GetRequestCount(This,puCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBidiRequestContainer_INTERFACE_DEFINED__ */


#ifndef __IBidiSpl_INTERFACE_DEFINED__
#define __IBidiSpl_INTERFACE_DEFINED__

/* interface IBidiSpl */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBidiSpl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D580DC0E-DE39-4649-BAA8-BF0B85A03A97")
    IBidiSpl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BindDevice( 
            /* [in] */ __RPC__in const LPCWSTR pszDeviceName,
            /* [in] */ const DWORD dwAccess) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnbindDevice( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendRecv( 
            /* [in] */ __RPC__in const LPCWSTR pszAction,
            /* [in] */ __RPC__in_opt IBidiRequest *pRequest) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MultiSendRecv( 
            /* [in] */ __RPC__in const LPCWSTR pszAction,
            /* [in] */ __RPC__in_opt IBidiRequestContainer *pRequestContainer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBidiSplVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBidiSpl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBidiSpl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBidiSpl * This);
        
        DECLSPEC_XFGVIRT(IBidiSpl, BindDevice)
        HRESULT ( STDMETHODCALLTYPE *BindDevice )( 
            __RPC__in IBidiSpl * This,
            /* [in] */ __RPC__in const LPCWSTR pszDeviceName,
            /* [in] */ const DWORD dwAccess);
        
        DECLSPEC_XFGVIRT(IBidiSpl, UnbindDevice)
        HRESULT ( STDMETHODCALLTYPE *UnbindDevice )( 
            __RPC__in IBidiSpl * This);
        
        DECLSPEC_XFGVIRT(IBidiSpl, SendRecv)
        HRESULT ( STDMETHODCALLTYPE *SendRecv )( 
            __RPC__in IBidiSpl * This,
            /* [in] */ __RPC__in const LPCWSTR pszAction,
            /* [in] */ __RPC__in_opt IBidiRequest *pRequest);
        
        DECLSPEC_XFGVIRT(IBidiSpl, MultiSendRecv)
        HRESULT ( STDMETHODCALLTYPE *MultiSendRecv )( 
            __RPC__in IBidiSpl * This,
            /* [in] */ __RPC__in const LPCWSTR pszAction,
            /* [in] */ __RPC__in_opt IBidiRequestContainer *pRequestContainer);
        
        END_INTERFACE
    } IBidiSplVtbl;

    interface IBidiSpl
    {
        CONST_VTBL struct IBidiSplVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBidiSpl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBidiSpl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBidiSpl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBidiSpl_BindDevice(This,pszDeviceName,dwAccess)	\
    ( (This)->lpVtbl -> BindDevice(This,pszDeviceName,dwAccess) ) 

#define IBidiSpl_UnbindDevice(This)	\
    ( (This)->lpVtbl -> UnbindDevice(This) ) 

#define IBidiSpl_SendRecv(This,pszAction,pRequest)	\
    ( (This)->lpVtbl -> SendRecv(This,pszAction,pRequest) ) 

#define IBidiSpl_MultiSendRecv(This,pszAction,pRequestContainer)	\
    ( (This)->lpVtbl -> MultiSendRecv(This,pszAction,pRequestContainer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBidiSpl_INTERFACE_DEFINED__ */


#ifndef __IBidiSpl2_INTERFACE_DEFINED__
#define __IBidiSpl2_INTERFACE_DEFINED__

/* interface IBidiSpl2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBidiSpl2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0E8F51B8-8273-4906-8E7B-BE453FFD2E2B")
    IBidiSpl2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BindDevice( 
            /* [in] */ __RPC__in const LPCWSTR pszDeviceName,
            /* [in] */ const DWORD dwAccess) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnbindDevice( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendRecvXMLString( 
            /* [in] */ __RPC__in BSTR bstrRequest,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrResponse) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendRecvXMLStream( 
            /* [in] */ __RPC__in_opt IStream *pSRequest,
            /* [out] */ __RPC__deref_out_opt IStream **ppSResponse) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBidiSpl2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBidiSpl2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBidiSpl2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBidiSpl2 * This);
        
        DECLSPEC_XFGVIRT(IBidiSpl2, BindDevice)
        HRESULT ( STDMETHODCALLTYPE *BindDevice )( 
            __RPC__in IBidiSpl2 * This,
            /* [in] */ __RPC__in const LPCWSTR pszDeviceName,
            /* [in] */ const DWORD dwAccess);
        
        DECLSPEC_XFGVIRT(IBidiSpl2, UnbindDevice)
        HRESULT ( STDMETHODCALLTYPE *UnbindDevice )( 
            __RPC__in IBidiSpl2 * This);
        
        DECLSPEC_XFGVIRT(IBidiSpl2, SendRecvXMLString)
        HRESULT ( STDMETHODCALLTYPE *SendRecvXMLString )( 
            __RPC__in IBidiSpl2 * This,
            /* [in] */ __RPC__in BSTR bstrRequest,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrResponse);
        
        DECLSPEC_XFGVIRT(IBidiSpl2, SendRecvXMLStream)
        HRESULT ( STDMETHODCALLTYPE *SendRecvXMLStream )( 
            __RPC__in IBidiSpl2 * This,
            /* [in] */ __RPC__in_opt IStream *pSRequest,
            /* [out] */ __RPC__deref_out_opt IStream **ppSResponse);
        
        END_INTERFACE
    } IBidiSpl2Vtbl;

    interface IBidiSpl2
    {
        CONST_VTBL struct IBidiSpl2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBidiSpl2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBidiSpl2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBidiSpl2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBidiSpl2_BindDevice(This,pszDeviceName,dwAccess)	\
    ( (This)->lpVtbl -> BindDevice(This,pszDeviceName,dwAccess) ) 

#define IBidiSpl2_UnbindDevice(This)	\
    ( (This)->lpVtbl -> UnbindDevice(This) ) 

#define IBidiSpl2_SendRecvXMLString(This,bstrRequest,pbstrResponse)	\
    ( (This)->lpVtbl -> SendRecvXMLString(This,bstrRequest,pbstrResponse) ) 

#define IBidiSpl2_SendRecvXMLStream(This,pSRequest,ppSResponse)	\
    ( (This)->lpVtbl -> SendRecvXMLStream(This,pSRequest,ppSResponse) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBidiSpl2_INTERFACE_DEFINED__ */



#ifndef __IBidiSplLib_LIBRARY_DEFINED__
#define __IBidiSplLib_LIBRARY_DEFINED__

/* library IBidiSplLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_IBidiSplLib;

EXTERN_C const CLSID CLSID_BidiRequest;

#ifdef __cplusplus

class DECLSPEC_UUID("B9162A23-45F9-47cc-80F5-FE0FE9B9E1A2")
BidiRequest;
#endif

EXTERN_C const CLSID CLSID_BidiRequestContainer;

#ifdef __cplusplus

class DECLSPEC_UUID("FC5B8A24-DB05-4a01-8388-22EDF6C2BBBA")
BidiRequestContainer;
#endif

EXTERN_C const CLSID CLSID_BidiSpl;

#ifdef __cplusplus

class DECLSPEC_UUID("2A614240-A4C5-4c33-BD87-1BC709331639")
BidiSpl;
#endif
#endif /* __IBidiSplLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_bidispl_0000_0005 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_bidispl_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bidispl_0000_0005_v0_0_s_ifspec;

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


