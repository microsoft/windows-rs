

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

#ifndef __windows2Egraphics2Edisplay2Einterop_h__
#define __windows2Egraphics2Edisplay2Einterop_h__

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

#ifndef __IDisplayInformationStaticsInterop_FWD_DEFINED__
#define __IDisplayInformationStaticsInterop_FWD_DEFINED__
typedef interface IDisplayInformationStaticsInterop IDisplayInformationStaticsInterop;

#endif 	/* __IDisplayInformationStaticsInterop_FWD_DEFINED__ */


/* header files for imported files */
#include "inspectable.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_windows2Egraphics2Edisplay2Einterop_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if (NTDDI_VERSION >= NTDDI_WIN10_NI)
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_windows2Egraphics2Edisplay2Einterop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Egraphics2Edisplay2Einterop_0000_0000_v0_0_s_ifspec;

#ifndef __IDisplayInformationStaticsInterop_INTERFACE_DEFINED__
#define __IDisplayInformationStaticsInterop_INTERFACE_DEFINED__

/* interface IDisplayInformationStaticsInterop */
/* [object][uuid] */ 


EXTERN_C const IID IID_IDisplayInformationStaticsInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7449121C-382B-4705-8DA7-A795BA482013")
    IDisplayInformationStaticsInterop : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetForWindow( 
            /* [in] */ __RPC__in HWND window,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **displayInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetForMonitor( 
            /* [in] */ __RPC__in HMONITOR monitor,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **displayInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDisplayInformationStaticsInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDisplayInformationStaticsInterop * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDisplayInformationStaticsInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDisplayInformationStaticsInterop * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            __RPC__in IDisplayInformationStaticsInterop * This,
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            __RPC__in IDisplayInformationStaticsInterop * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            __RPC__in IDisplayInformationStaticsInterop * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IDisplayInformationStaticsInterop, GetForWindow)
        HRESULT ( STDMETHODCALLTYPE *GetForWindow )( 
            __RPC__in IDisplayInformationStaticsInterop * This,
            /* [in] */ __RPC__in HWND window,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **displayInfo);
        
        DECLSPEC_XFGVIRT(IDisplayInformationStaticsInterop, GetForMonitor)
        HRESULT ( STDMETHODCALLTYPE *GetForMonitor )( 
            __RPC__in IDisplayInformationStaticsInterop * This,
            /* [in] */ __RPC__in HMONITOR monitor,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **displayInfo);
        
        END_INTERFACE
    } IDisplayInformationStaticsInteropVtbl;

    interface IDisplayInformationStaticsInterop
    {
        CONST_VTBL struct IDisplayInformationStaticsInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDisplayInformationStaticsInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDisplayInformationStaticsInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDisplayInformationStaticsInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDisplayInformationStaticsInterop_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IDisplayInformationStaticsInterop_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IDisplayInformationStaticsInterop_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IDisplayInformationStaticsInterop_GetForWindow(This,window,riid,displayInfo)	\
    ( (This)->lpVtbl -> GetForWindow(This,window,riid,displayInfo) ) 

#define IDisplayInformationStaticsInterop_GetForMonitor(This,monitor,riid,displayInfo)	\
    ( (This)->lpVtbl -> GetForMonitor(This,monitor,riid,displayInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDisplayInformationStaticsInterop_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_windows2Egraphics2Edisplay2Einterop_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#endif //(NTDDI_VERSION >= NTDDI_WIN10_NI)


extern RPC_IF_HANDLE __MIDL_itf_windows2Egraphics2Edisplay2Einterop_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Egraphics2Edisplay2Einterop_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  HMONITOR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HMONITOR * ); 
unsigned char * __RPC_USER  HMONITOR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HMONITOR * ); 
unsigned char * __RPC_USER  HMONITOR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HMONITOR * ); 
void                      __RPC_USER  HMONITOR_UserFree(     __RPC__in unsigned long *, __RPC__in HMONITOR * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  HMONITOR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HMONITOR * ); 
unsigned char * __RPC_USER  HMONITOR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HMONITOR * ); 
unsigned char * __RPC_USER  HMONITOR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HMONITOR * ); 
void                      __RPC_USER  HMONITOR_UserFree64(     __RPC__in unsigned long *, __RPC__in HMONITOR * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


