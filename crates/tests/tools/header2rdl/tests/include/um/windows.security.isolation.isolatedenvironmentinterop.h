

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

#ifndef __windows2Esecurity2Eisolation2Eisolatedenvironmentinterop_h__
#define __windows2Esecurity2Eisolation2Eisolatedenvironmentinterop_h__

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

#ifndef __IIsolatedEnvironmentInterop_FWD_DEFINED__
#define __IIsolatedEnvironmentInterop_FWD_DEFINED__
typedef interface IIsolatedEnvironmentInterop IIsolatedEnvironmentInterop;

#endif 	/* __IIsolatedEnvironmentInterop_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_windows2Esecurity2Eisolation2Eisolatedenvironmentinterop_0000_0000 */
/* [local] */ 

#pragma warning(push)
#pragma warning(disable:4668) 
#pragma warning(disable:4001) 
#pragma once
#pragma warning(pop)
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_windows2Esecurity2Eisolation2Eisolatedenvironmentinterop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Esecurity2Eisolation2Eisolatedenvironmentinterop_0000_0000_v0_0_s_ifspec;

#ifndef __IIsolatedEnvironmentInterop_INTERFACE_DEFINED__
#define __IIsolatedEnvironmentInterop_INTERFACE_DEFINED__

/* interface IIsolatedEnvironmentInterop */
/* [uuid][object] */ 


EXTERN_C const IID IID_IIsolatedEnvironmentInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("85713C2E-8E62-46C5-8DE2-C647E1D54636")
    IIsolatedEnvironmentInterop : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetHostHwndInterop( 
            /* [in] */ __RPC__in HWND containerHwnd,
            /* [retval][out] */ __RPC__deref_out_opt HWND *hostHwnd) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsolatedEnvironmentInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IIsolatedEnvironmentInterop * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IIsolatedEnvironmentInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IIsolatedEnvironmentInterop * This);
        
        DECLSPEC_XFGVIRT(IIsolatedEnvironmentInterop, GetHostHwndInterop)
        HRESULT ( STDMETHODCALLTYPE *GetHostHwndInterop )( 
            __RPC__in IIsolatedEnvironmentInterop * This,
            /* [in] */ __RPC__in HWND containerHwnd,
            /* [retval][out] */ __RPC__deref_out_opt HWND *hostHwnd);
        
        END_INTERFACE
    } IIsolatedEnvironmentInteropVtbl;

    interface IIsolatedEnvironmentInterop
    {
        CONST_VTBL struct IIsolatedEnvironmentInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsolatedEnvironmentInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsolatedEnvironmentInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsolatedEnvironmentInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsolatedEnvironmentInterop_GetHostHwndInterop(This,containerHwnd,hostHwnd)	\
    ( (This)->lpVtbl -> GetHostHwndInterop(This,containerHwnd,hostHwnd) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsolatedEnvironmentInterop_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_windows2Esecurity2Eisolation2Eisolatedenvironmentinterop_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_windows2Esecurity2Eisolation2Eisolatedenvironmentinterop_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Esecurity2Eisolation2Eisolatedenvironmentinterop_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


