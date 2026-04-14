

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

#ifndef __sharewindowcommandsourceinterop_h__
#define __sharewindowcommandsourceinterop_h__

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

#if defined(__cplusplus)
#if defined(__MIDL_USE_C_ENUM)
#define MIDL_ENUM enum
#else
#define MIDL_ENUM enum class
#endif
#endif


/* Forward Declarations */ 

#ifndef __IShareWindowCommandEventArgsInterop_FWD_DEFINED__
#define __IShareWindowCommandEventArgsInterop_FWD_DEFINED__
typedef interface IShareWindowCommandEventArgsInterop IShareWindowCommandEventArgsInterop;

#endif 	/* __IShareWindowCommandEventArgsInterop_FWD_DEFINED__ */


#ifndef __IShareWindowCommandSourceInterop_FWD_DEFINED__
#define __IShareWindowCommandSourceInterop_FWD_DEFINED__
typedef interface IShareWindowCommandSourceInterop IShareWindowCommandSourceInterop;

#endif 	/* __IShareWindowCommandSourceInterop_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_sharewindowcommandsourceinterop_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if (NTDDI_VERSION >= NTDDI_WIN10_CO)
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_sharewindowcommandsourceinterop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_sharewindowcommandsourceinterop_0000_0000_v0_0_s_ifspec;

#ifndef __IShareWindowCommandEventArgsInterop_INTERFACE_DEFINED__
#define __IShareWindowCommandEventArgsInterop_INTERFACE_DEFINED__

/* interface IShareWindowCommandEventArgsInterop */
/* [object][uuid] */ 


EXTERN_C const IID IID_IShareWindowCommandEventArgsInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6571a721-643d-43d4-aca4-6b6f5f30f1ad")
    IShareWindowCommandEventArgsInterop : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetWindow( 
            /* [retval][out] */ __RPC__deref_out_opt HWND *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShareWindowCommandEventArgsInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShareWindowCommandEventArgsInterop * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShareWindowCommandEventArgsInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShareWindowCommandEventArgsInterop * This);
        
        DECLSPEC_XFGVIRT(IShareWindowCommandEventArgsInterop, GetWindow)
        HRESULT ( STDMETHODCALLTYPE *GetWindow )( 
            __RPC__in IShareWindowCommandEventArgsInterop * This,
            /* [retval][out] */ __RPC__deref_out_opt HWND *value);
        
        END_INTERFACE
    } IShareWindowCommandEventArgsInteropVtbl;

    interface IShareWindowCommandEventArgsInterop
    {
        CONST_VTBL struct IShareWindowCommandEventArgsInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShareWindowCommandEventArgsInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShareWindowCommandEventArgsInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShareWindowCommandEventArgsInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShareWindowCommandEventArgsInterop_GetWindow(This,value)	\
    ( (This)->lpVtbl -> GetWindow(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShareWindowCommandEventArgsInterop_INTERFACE_DEFINED__ */


#ifndef __IShareWindowCommandSourceInterop_INTERFACE_DEFINED__
#define __IShareWindowCommandSourceInterop_INTERFACE_DEFINED__

/* interface IShareWindowCommandSourceInterop */
/* [object][uuid] */ 


EXTERN_C const IID IID_IShareWindowCommandSourceInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("461a191f-8424-43a6-a0fa-3451a22f56ab")
    IShareWindowCommandSourceInterop : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetForWindow( 
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **shareWindowCommandSource) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShareWindowCommandSourceInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShareWindowCommandSourceInterop * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShareWindowCommandSourceInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShareWindowCommandSourceInterop * This);
        
        DECLSPEC_XFGVIRT(IShareWindowCommandSourceInterop, GetForWindow)
        HRESULT ( STDMETHODCALLTYPE *GetForWindow )( 
            __RPC__in IShareWindowCommandSourceInterop * This,
            /* [in] */ __RPC__in HWND appWindow,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt void **shareWindowCommandSource);
        
        END_INTERFACE
    } IShareWindowCommandSourceInteropVtbl;

    interface IShareWindowCommandSourceInterop
    {
        CONST_VTBL struct IShareWindowCommandSourceInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShareWindowCommandSourceInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShareWindowCommandSourceInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShareWindowCommandSourceInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShareWindowCommandSourceInterop_GetForWindow(This,appWindow,riid,shareWindowCommandSource)	\
    ( (This)->lpVtbl -> GetForWindow(This,appWindow,riid,shareWindowCommandSource) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShareWindowCommandSourceInterop_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_sharewindowcommandsourceinterop_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif //(NTDDI_VERSION >= NTDDI_WIN10_CO)


extern RPC_IF_HANDLE __MIDL_itf_sharewindowcommandsourceinterop_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_sharewindowcommandsourceinterop_0000_0002_v0_0_s_ifspec;

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


