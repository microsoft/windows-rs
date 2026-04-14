

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

#ifndef __windows2Egraphics2Edisplay2Edisplayenhancementoverride2Einterop_h__
#define __windows2Egraphics2Edisplay2Edisplayenhancementoverride2Einterop_h__

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

#ifndef __IDisplayEnhancementOverrideInterop_FWD_DEFINED__
#define __IDisplayEnhancementOverrideInterop_FWD_DEFINED__
typedef interface IDisplayEnhancementOverrideInterop IDisplayEnhancementOverrideInterop;

#endif 	/* __IDisplayEnhancementOverrideInterop_FWD_DEFINED__ */


/* header files for imported files */
#include "inspectable.h"
#include "asyncinfo.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_windows2Egraphics2Edisplay2Edisplayenhancementoverride2Einterop_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if (NTDDI_VERSION >= NTDDI_WIN10_19H1)
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_windows2Egraphics2Edisplay2Edisplayenhancementoverride2Einterop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Egraphics2Edisplay2Edisplayenhancementoverride2Einterop_0000_0000_v0_0_s_ifspec;

#ifndef __IDisplayEnhancementOverrideInterop_INTERFACE_DEFINED__
#define __IDisplayEnhancementOverrideInterop_INTERFACE_DEFINED__

/* interface IDisplayEnhancementOverrideInterop */
/* [object][uuid] */ 


EXTERN_C const IID IID_IDisplayEnhancementOverrideInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("05F63C7F-0D73-4F91-97E6-7F2E10BA489A")
    IDisplayEnhancementOverrideInterop : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetForWindow( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **displayEnhancementOverride) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDisplayEnhancementOverrideInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDisplayEnhancementOverrideInterop * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDisplayEnhancementOverrideInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDisplayEnhancementOverrideInterop * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            __RPC__in IDisplayEnhancementOverrideInterop * This,
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            __RPC__in IDisplayEnhancementOverrideInterop * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            __RPC__in IDisplayEnhancementOverrideInterop * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IDisplayEnhancementOverrideInterop, GetForWindow)
        HRESULT ( STDMETHODCALLTYPE *GetForWindow )( 
            __RPC__in IDisplayEnhancementOverrideInterop * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **displayEnhancementOverride);
        
        END_INTERFACE
    } IDisplayEnhancementOverrideInteropVtbl;

    interface IDisplayEnhancementOverrideInterop
    {
        CONST_VTBL struct IDisplayEnhancementOverrideInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDisplayEnhancementOverrideInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDisplayEnhancementOverrideInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDisplayEnhancementOverrideInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDisplayEnhancementOverrideInterop_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IDisplayEnhancementOverrideInterop_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IDisplayEnhancementOverrideInterop_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IDisplayEnhancementOverrideInterop_GetForWindow(This,appWindow,riid,displayEnhancementOverride)	\
    ( (This)->lpVtbl -> GetForWindow(This,appWindow,riid,displayEnhancementOverride) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDisplayEnhancementOverrideInterop_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_windows2Egraphics2Edisplay2Edisplayenhancementoverride2Einterop_0000_0001 */
/* [local] */ 

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma endregion
#endif // (NTDDI_VERSION >= NTDDI_WIN10_19H1)


extern RPC_IF_HANDLE __MIDL_itf_windows2Egraphics2Edisplay2Edisplayenhancementoverride2Einterop_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Egraphics2Edisplay2Edisplayenhancementoverride2Einterop_0000_0001_v0_0_s_ifspec;

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


