

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

#ifndef __iisrsta_h__
#define __iisrsta_h__

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

#ifndef __IIisServiceControl_FWD_DEFINED__
#define __IIisServiceControl_FWD_DEFINED__
typedef interface IIisServiceControl IIisServiceControl;

#endif 	/* __IIisServiceControl_FWD_DEFINED__ */


#ifndef __IisServiceControl_FWD_DEFINED__
#define __IisServiceControl_FWD_DEFINED__

#ifdef __cplusplus
typedef class IisServiceControl IisServiceControl;
#else
typedef struct IisServiceControl IisServiceControl;
#endif /* __cplusplus */

#endif 	/* __IisServiceControl_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_iisrsta_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
DEFINE_GUID(IID_IIisServiceControl, 0xE8FB8620, 0x588F, 0x11d2, 0x9d, 0x61, 0x0,0xc0, 0x4f, 0x79, 0xc5, 0xfe);
DEFINE_GUID(CLSID_IisServiceControl, 0xE8FB8621, 0x588F, 0x11d2, 0x9d, 0x61, 0x0,0xc0, 0x4f, 0x79, 0xc5, 0xfe);
DEFINE_GUID(LIBID_IISRSTALib, 0xE8FB8614, 0x588F, 0x11d2, 0x9d, 0x61, 0x0,0xc0, 0x4f, 0x79, 0xc5, 0xfe);


extern RPC_IF_HANDLE __MIDL_itf_iisrsta_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_iisrsta_0000_0000_v0_0_s_ifspec;

#ifndef __IIisServiceControl_INTERFACE_DEFINED__
#define __IIisServiceControl_INTERFACE_DEFINED__

/* interface IIisServiceControl */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IIisServiceControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E8FB8620-588F-11D2-9D61-00C04F79C5FE")
    IIisServiceControl : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Stop( 
            DWORD dwTimeoutMsecs,
            DWORD dwForce) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Start( 
            DWORD dwTimeoutMsecs) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Reboot( 
            DWORD dwTimeouMsecs,
            DWORD dwForceAppsClosed) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Status( 
            /* [in] */ DWORD dwBufferSize,
            /* [size_is][out] */ __RPC__out_ecount_full(dwBufferSize) unsigned char *pbBuffer,
            /* [out] */ __RPC__out DWORD *pdwMDRequiredBufferSize,
            /* [out] */ __RPC__out DWORD *pdwNumServices) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Kill( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIisServiceControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IIisServiceControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IIisServiceControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IIisServiceControl * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IIisServiceControl * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IIisServiceControl * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IIisServiceControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IIisServiceControl * This,
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
        
        DECLSPEC_XFGVIRT(IIisServiceControl, Stop)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IIisServiceControl * This,
            DWORD dwTimeoutMsecs,
            DWORD dwForce);
        
        DECLSPEC_XFGVIRT(IIisServiceControl, Start)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IIisServiceControl * This,
            DWORD dwTimeoutMsecs);
        
        DECLSPEC_XFGVIRT(IIisServiceControl, Reboot)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Reboot )( 
            __RPC__in IIisServiceControl * This,
            DWORD dwTimeouMsecs,
            DWORD dwForceAppsClosed);
        
        DECLSPEC_XFGVIRT(IIisServiceControl, Status)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Status )( 
            __RPC__in IIisServiceControl * This,
            /* [in] */ DWORD dwBufferSize,
            /* [size_is][out] */ __RPC__out_ecount_full(dwBufferSize) unsigned char *pbBuffer,
            /* [out] */ __RPC__out DWORD *pdwMDRequiredBufferSize,
            /* [out] */ __RPC__out DWORD *pdwNumServices);
        
        DECLSPEC_XFGVIRT(IIisServiceControl, Kill)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Kill )( 
            __RPC__in IIisServiceControl * This);
        
        END_INTERFACE
    } IIisServiceControlVtbl;

    interface IIisServiceControl
    {
        CONST_VTBL struct IIisServiceControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIisServiceControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIisServiceControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIisServiceControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIisServiceControl_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IIisServiceControl_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IIisServiceControl_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IIisServiceControl_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IIisServiceControl_Stop(This,dwTimeoutMsecs,dwForce)	\
    ( (This)->lpVtbl -> Stop(This,dwTimeoutMsecs,dwForce) ) 

#define IIisServiceControl_Start(This,dwTimeoutMsecs)	\
    ( (This)->lpVtbl -> Start(This,dwTimeoutMsecs) ) 

#define IIisServiceControl_Reboot(This,dwTimeouMsecs,dwForceAppsClosed)	\
    ( (This)->lpVtbl -> Reboot(This,dwTimeouMsecs,dwForceAppsClosed) ) 

#define IIisServiceControl_Status(This,dwBufferSize,pbBuffer,pdwMDRequiredBufferSize,pdwNumServices)	\
    ( (This)->lpVtbl -> Status(This,dwBufferSize,pbBuffer,pdwMDRequiredBufferSize,pdwNumServices) ) 

#define IIisServiceControl_Kill(This)	\
    ( (This)->lpVtbl -> Kill(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIisServiceControl_INTERFACE_DEFINED__ */



#ifndef __IISRSTALib_LIBRARY_DEFINED__
#define __IISRSTALib_LIBRARY_DEFINED__

/* library IISRSTALib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_IISRSTALib;

EXTERN_C const CLSID CLSID_IisServiceControl;

#ifdef __cplusplus

class DECLSPEC_UUID("E8FB8621-588F-11D2-9D61-00C04F79C5FE")
IisServiceControl;
#endif
#endif /* __IISRSTALib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_iisrsta_0000_0002 */
/* [local] */ 

typedef struct {
DWORD iServiceName;
DWORD iDisplayName;
SERVICE_STATUS ServiceStatus;
} SERIALIZED_ENUM_SERVICE_STATUS;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_iisrsta_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_iisrsta_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


