

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

#ifndef __print3dmanagerinterop_h__
#define __print3dmanagerinterop_h__

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

#ifndef __IPrinting3DManagerInterop_FWD_DEFINED__
#define __IPrinting3DManagerInterop_FWD_DEFINED__
typedef interface IPrinting3DManagerInterop IPrinting3DManagerInterop;

#endif 	/* __IPrinting3DManagerInterop_FWD_DEFINED__ */


/* header files for imported files */
#include "inspectable.h"
#include "asyncinfo.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_print3dmanagerinterop_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_print3dmanagerinterop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_print3dmanagerinterop_0000_0000_v0_0_s_ifspec;

#ifndef __IPrinting3DManagerInterop_INTERFACE_DEFINED__
#define __IPrinting3DManagerInterop_INTERFACE_DEFINED__

/* interface IPrinting3DManagerInterop */
/* [object][uuid] */ 


EXTERN_C const IID IID_IPrinting3DManagerInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9CA31010-1484-4587-B26B-DDDF9F9CAECD")
    IPrinting3DManagerInterop : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetForWindow( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **printManager) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShowPrintUIForWindowAsync( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinting3DManagerInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrinting3DManagerInterop * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrinting3DManagerInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrinting3DManagerInterop * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            __RPC__in IPrinting3DManagerInterop * This,
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            __RPC__in IPrinting3DManagerInterop * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            __RPC__in IPrinting3DManagerInterop * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IPrinting3DManagerInterop, GetForWindow)
        HRESULT ( STDMETHODCALLTYPE *GetForWindow )( 
            __RPC__in IPrinting3DManagerInterop * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **printManager);
        
        DECLSPEC_XFGVIRT(IPrinting3DManagerInterop, ShowPrintUIForWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *ShowPrintUIForWindowAsync )( 
            __RPC__in IPrinting3DManagerInterop * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **asyncOperation);
        
        END_INTERFACE
    } IPrinting3DManagerInteropVtbl;

    interface IPrinting3DManagerInterop
    {
        CONST_VTBL struct IPrinting3DManagerInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinting3DManagerInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinting3DManagerInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinting3DManagerInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinting3DManagerInterop_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IPrinting3DManagerInterop_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IPrinting3DManagerInterop_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IPrinting3DManagerInterop_GetForWindow(This,appWindow,riid,printManager)	\
    ( (This)->lpVtbl -> GetForWindow(This,appWindow,riid,printManager) ) 

#define IPrinting3DManagerInterop_ShowPrintUIForWindowAsync(This,appWindow,riid,asyncOperation)	\
    ( (This)->lpVtbl -> ShowPrintUIForWindowAsync(This,appWindow,riid,asyncOperation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinting3DManagerInterop_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_print3dmanagerinterop_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif //(NTDDI_VERSION >= NTDDI_WINTHRESHOLD)


extern RPC_IF_HANDLE __MIDL_itf_print3dmanagerinterop_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_print3dmanagerinterop_0000_0001_v0_0_s_ifspec;

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


