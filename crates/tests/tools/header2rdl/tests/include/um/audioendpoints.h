

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

#ifndef __audioendpoints_h__
#define __audioendpoints_h__

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

#ifndef __IAudioEndpointFormatControl_FWD_DEFINED__
#define __IAudioEndpointFormatControl_FWD_DEFINED__
typedef interface IAudioEndpointFormatControl IAudioEndpointFormatControl;

#endif 	/* __IAudioEndpointFormatControl_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_audioendpoints_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define   ENDPOINT_FORMAT_RESET_MIX_ONLY     0x00000001


extern RPC_IF_HANDLE __MIDL_itf_audioendpoints_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_audioendpoints_0000_0000_v0_0_s_ifspec;

#ifndef __IAudioEndpointFormatControl_INTERFACE_DEFINED__
#define __IAudioEndpointFormatControl_INTERFACE_DEFINED__

/* interface IAudioEndpointFormatControl */
/* [unique][helpstring][nonextensible][uuid][local][object] */ 


EXTERN_C const IID IID_IAudioEndpointFormatControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("784CFD40-9F89-456E-A1A6-873B006A664E")
    IAudioEndpointFormatControl : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ResetToDefault( 
            /* [annotation][in] */ 
            _In_  DWORD ResetFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioEndpointFormatControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioEndpointFormatControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioEndpointFormatControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioEndpointFormatControl * This);
        
        DECLSPEC_XFGVIRT(IAudioEndpointFormatControl, ResetToDefault)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ResetToDefault )( 
            IAudioEndpointFormatControl * This,
            /* [annotation][in] */ 
            _In_  DWORD ResetFlags);
        
        END_INTERFACE
    } IAudioEndpointFormatControlVtbl;

    interface IAudioEndpointFormatControl
    {
        CONST_VTBL struct IAudioEndpointFormatControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioEndpointFormatControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioEndpointFormatControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioEndpointFormatControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioEndpointFormatControl_ResetToDefault(This,ResetFlags)	\
    ( (This)->lpVtbl -> ResetToDefault(This,ResetFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioEndpointFormatControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_audioendpoints_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_audioendpoints_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_audioendpoints_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


