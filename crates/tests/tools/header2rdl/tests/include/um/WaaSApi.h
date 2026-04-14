

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

#ifndef __waasapi_h__
#define __waasapi_h__

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

#ifndef __IWaaSAssessor_FWD_DEFINED__
#define __IWaaSAssessor_FWD_DEFINED__
typedef interface IWaaSAssessor IWaaSAssessor;

#endif 	/* __IWaaSAssessor_FWD_DEFINED__ */


#ifndef __WaaSAssessor_FWD_DEFINED__
#define __WaaSAssessor_FWD_DEFINED__

#ifdef __cplusplus
typedef class WaaSAssessor WaaSAssessor;
#else
typedef struct WaaSAssessor WaaSAssessor;
#endif /* __cplusplus */

#endif 	/* __WaaSAssessor_FWD_DEFINED__ */


/* header files for imported files */
#include "WaaSAPITypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_waasapi_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if ( _MSC_VER >= NTDDI_WINTHRESHOLD )
#pragma once
#endif

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_waasapi_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_waasapi_0000_0000_v0_0_s_ifspec;

#ifndef __IWaaSAssessor_INTERFACE_DEFINED__
#define __IWaaSAssessor_INTERFACE_DEFINED__

/* interface IWaaSAssessor */
/* [uuid][object] */ 


EXTERN_C const IID IID_IWaaSAssessor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2347bbef-1a3b-45a4-902d-3e09c269b45e")
    IWaaSAssessor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOSUpdateAssessment( 
            /* [retval][out] */ __RPC__out OSUpdateAssessment *result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWaaSAssessorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWaaSAssessor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWaaSAssessor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWaaSAssessor * This);
        
        DECLSPEC_XFGVIRT(IWaaSAssessor, GetOSUpdateAssessment)
        HRESULT ( STDMETHODCALLTYPE *GetOSUpdateAssessment )( 
            __RPC__in IWaaSAssessor * This,
            /* [retval][out] */ __RPC__out OSUpdateAssessment *result);
        
        END_INTERFACE
    } IWaaSAssessorVtbl;

    interface IWaaSAssessor
    {
        CONST_VTBL struct IWaaSAssessorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWaaSAssessor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWaaSAssessor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWaaSAssessor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWaaSAssessor_GetOSUpdateAssessment(This,result)	\
    ( (This)->lpVtbl -> GetOSUpdateAssessment(This,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWaaSAssessor_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_waasapi_0000_0001 */
/* [local] */ 

#ifndef WAASASSESSOR_INTERNAL


extern RPC_IF_HANDLE __MIDL_itf_waasapi_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_waasapi_0000_0001_v0_0_s_ifspec;


#ifndef __WaaSAssessorLib_LIBRARY_DEFINED__
#define __WaaSAssessorLib_LIBRARY_DEFINED__

/* library WaaSAssessorLib */
/* [version][uuid] */ 


EXTERN_C const IID LIBID_WaaSAssessorLib;

EXTERN_C const CLSID CLSID_WaaSAssessor;

#ifdef __cplusplus

class DECLSPEC_UUID("098ef871-fa9f-46af-8958-c083515d7c9c")
WaaSAssessor;
#endif
#endif /* __WaaSAssessorLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_waasapi_0000_0002 */
/* [local] */ 

#endif /* WAASASSESSOR_INTERNAL */
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_waasapi_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_waasapi_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


