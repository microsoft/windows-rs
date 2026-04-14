

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

#ifndef __certreqd_h__
#define __certreqd_h__

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

#ifndef __ICertRequestD_FWD_DEFINED__
#define __ICertRequestD_FWD_DEFINED__
typedef interface ICertRequestD ICertRequestD;

#endif 	/* __ICertRequestD_FWD_DEFINED__ */


#ifndef __ICertRequestD2_FWD_DEFINED__
#define __ICertRequestD2_FWD_DEFINED__
typedef interface ICertRequestD2 ICertRequestD2;

#endif 	/* __ICertRequestD2_FWD_DEFINED__ */


/* header files for imported files */
#include "certbase.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_certreqd_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_certreqd_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_certreqd_0000_0000_v0_0_s_ifspec;

#ifndef __ICertRequestD_INTERFACE_DEFINED__
#define __ICertRequestD_INTERFACE_DEFINED__

/* interface ICertRequestD */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ICertRequestD;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d99e6e70-fc88-11d0-b498-00a0c90312f3")
    ICertRequestD : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Request( 
            /* [in] */ DWORD dwFlags,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAuthority,
            /* [ref][out][in] */ __RPC__inout DWORD *pdwRequestId,
            /* [out] */ __RPC__out DWORD *pdwDisposition,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAttributes,
            /* [ref][in] */ __RPC__in const CERTTRANSBLOB *pctbRequest,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbCertChain,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbEncodedCert,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbDispositionMessage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCACert( 
            /* [in] */ DWORD fchain,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAuthority,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Ping( 
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAuthority) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertRequestDVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertRequestD * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertRequestD * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertRequestD * This);
        
        DECLSPEC_XFGVIRT(ICertRequestD, Request)
        HRESULT ( STDMETHODCALLTYPE *Request )( 
            __RPC__in ICertRequestD * This,
            /* [in] */ DWORD dwFlags,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAuthority,
            /* [ref][out][in] */ __RPC__inout DWORD *pdwRequestId,
            /* [out] */ __RPC__out DWORD *pdwDisposition,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAttributes,
            /* [ref][in] */ __RPC__in const CERTTRANSBLOB *pctbRequest,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbCertChain,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbEncodedCert,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbDispositionMessage);
        
        DECLSPEC_XFGVIRT(ICertRequestD, GetCACert)
        HRESULT ( STDMETHODCALLTYPE *GetCACert )( 
            __RPC__in ICertRequestD * This,
            /* [in] */ DWORD fchain,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAuthority,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbOut);
        
        DECLSPEC_XFGVIRT(ICertRequestD, Ping)
        HRESULT ( STDMETHODCALLTYPE *Ping )( 
            __RPC__in ICertRequestD * This,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAuthority);
        
        END_INTERFACE
    } ICertRequestDVtbl;

    interface ICertRequestD
    {
        CONST_VTBL struct ICertRequestDVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertRequestD_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertRequestD_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertRequestD_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertRequestD_Request(This,dwFlags,pwszAuthority,pdwRequestId,pdwDisposition,pwszAttributes,pctbRequest,pctbCertChain,pctbEncodedCert,pctbDispositionMessage)	\
    ( (This)->lpVtbl -> Request(This,dwFlags,pwszAuthority,pdwRequestId,pdwDisposition,pwszAttributes,pctbRequest,pctbCertChain,pctbEncodedCert,pctbDispositionMessage) ) 

#define ICertRequestD_GetCACert(This,fchain,pwszAuthority,pctbOut)	\
    ( (This)->lpVtbl -> GetCACert(This,fchain,pwszAuthority,pctbOut) ) 

#define ICertRequestD_Ping(This,pwszAuthority)	\
    ( (This)->lpVtbl -> Ping(This,pwszAuthority) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertRequestD_INTERFACE_DEFINED__ */


#ifndef __ICertRequestD2_INTERFACE_DEFINED__
#define __ICertRequestD2_INTERFACE_DEFINED__

/* interface ICertRequestD2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ICertRequestD2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5422fd3a-d4b8-4cef-a12e-e87d4ca22e90")
    ICertRequestD2 : public ICertRequestD
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Request2( 
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAuthority,
            /* [in] */ DWORD dwFlags,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszSerialNumber,
            /* [ref][out][in] */ __RPC__inout DWORD *pdwRequestId,
            /* [out] */ __RPC__out DWORD *pdwDisposition,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAttributes,
            /* [ref][in] */ __RPC__in const CERTTRANSBLOB *pctbRequest,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbFullResponse,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbEncodedCert,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbDispositionMessage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCAProperty( 
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAuthority,
            /* [in] */ LONG PropId,
            /* [in] */ LONG PropIndex,
            /* [in] */ LONG PropType,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbPropertyValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCAPropertyInfo( 
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAuthority,
            /* [out] */ __RPC__out LONG *pcProperty,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbPropInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Ping2( 
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAuthority) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertRequestD2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertRequestD2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertRequestD2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertRequestD2 * This);
        
        DECLSPEC_XFGVIRT(ICertRequestD, Request)
        HRESULT ( STDMETHODCALLTYPE *Request )( 
            __RPC__in ICertRequestD2 * This,
            /* [in] */ DWORD dwFlags,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAuthority,
            /* [ref][out][in] */ __RPC__inout DWORD *pdwRequestId,
            /* [out] */ __RPC__out DWORD *pdwDisposition,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAttributes,
            /* [ref][in] */ __RPC__in const CERTTRANSBLOB *pctbRequest,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbCertChain,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbEncodedCert,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbDispositionMessage);
        
        DECLSPEC_XFGVIRT(ICertRequestD, GetCACert)
        HRESULT ( STDMETHODCALLTYPE *GetCACert )( 
            __RPC__in ICertRequestD2 * This,
            /* [in] */ DWORD fchain,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAuthority,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbOut);
        
        DECLSPEC_XFGVIRT(ICertRequestD, Ping)
        HRESULT ( STDMETHODCALLTYPE *Ping )( 
            __RPC__in ICertRequestD2 * This,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAuthority);
        
        DECLSPEC_XFGVIRT(ICertRequestD2, Request2)
        HRESULT ( STDMETHODCALLTYPE *Request2 )( 
            __RPC__in ICertRequestD2 * This,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAuthority,
            /* [in] */ DWORD dwFlags,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszSerialNumber,
            /* [ref][out][in] */ __RPC__inout DWORD *pdwRequestId,
            /* [out] */ __RPC__out DWORD *pdwDisposition,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAttributes,
            /* [ref][in] */ __RPC__in const CERTTRANSBLOB *pctbRequest,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbFullResponse,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbEncodedCert,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbDispositionMessage);
        
        DECLSPEC_XFGVIRT(ICertRequestD2, GetCAProperty)
        HRESULT ( STDMETHODCALLTYPE *GetCAProperty )( 
            __RPC__in ICertRequestD2 * This,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAuthority,
            /* [in] */ LONG PropId,
            /* [in] */ LONG PropIndex,
            /* [in] */ LONG PropType,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbPropertyValue);
        
        DECLSPEC_XFGVIRT(ICertRequestD2, GetCAPropertyInfo)
        HRESULT ( STDMETHODCALLTYPE *GetCAPropertyInfo )( 
            __RPC__in ICertRequestD2 * This,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAuthority,
            /* [out] */ __RPC__out LONG *pcProperty,
            /* [ref][out] */ __RPC__out CERTTRANSBLOB *pctbPropInfo);
        
        DECLSPEC_XFGVIRT(ICertRequestD2, Ping2)
        HRESULT ( STDMETHODCALLTYPE *Ping2 )( 
            __RPC__in ICertRequestD2 * This,
            /* [range][unique][string][in] */ __RPC__in_opt_string const wchar_t *pwszAuthority);
        
        END_INTERFACE
    } ICertRequestD2Vtbl;

    interface ICertRequestD2
    {
        CONST_VTBL struct ICertRequestD2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertRequestD2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertRequestD2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertRequestD2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertRequestD2_Request(This,dwFlags,pwszAuthority,pdwRequestId,pdwDisposition,pwszAttributes,pctbRequest,pctbCertChain,pctbEncodedCert,pctbDispositionMessage)	\
    ( (This)->lpVtbl -> Request(This,dwFlags,pwszAuthority,pdwRequestId,pdwDisposition,pwszAttributes,pctbRequest,pctbCertChain,pctbEncodedCert,pctbDispositionMessage) ) 

#define ICertRequestD2_GetCACert(This,fchain,pwszAuthority,pctbOut)	\
    ( (This)->lpVtbl -> GetCACert(This,fchain,pwszAuthority,pctbOut) ) 

#define ICertRequestD2_Ping(This,pwszAuthority)	\
    ( (This)->lpVtbl -> Ping(This,pwszAuthority) ) 


#define ICertRequestD2_Request2(This,pwszAuthority,dwFlags,pwszSerialNumber,pdwRequestId,pdwDisposition,pwszAttributes,pctbRequest,pctbFullResponse,pctbEncodedCert,pctbDispositionMessage)	\
    ( (This)->lpVtbl -> Request2(This,pwszAuthority,dwFlags,pwszSerialNumber,pdwRequestId,pdwDisposition,pwszAttributes,pctbRequest,pctbFullResponse,pctbEncodedCert,pctbDispositionMessage) ) 

#define ICertRequestD2_GetCAProperty(This,pwszAuthority,PropId,PropIndex,PropType,pctbPropertyValue)	\
    ( (This)->lpVtbl -> GetCAProperty(This,pwszAuthority,PropId,PropIndex,PropType,pctbPropertyValue) ) 

#define ICertRequestD2_GetCAPropertyInfo(This,pwszAuthority,pcProperty,pctbPropInfo)	\
    ( (This)->lpVtbl -> GetCAPropertyInfo(This,pwszAuthority,pcProperty,pctbPropInfo) ) 

#define ICertRequestD2_Ping2(This,pwszAuthority)	\
    ( (This)->lpVtbl -> Ping2(This,pwszAuthority) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertRequestD2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_certreqd_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_certreqd_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_certreqd_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


