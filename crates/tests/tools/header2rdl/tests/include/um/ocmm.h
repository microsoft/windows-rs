

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

#ifndef __ocmm_h__
#define __ocmm_h__

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

#ifndef __ITimerService_FWD_DEFINED__
#define __ITimerService_FWD_DEFINED__
typedef interface ITimerService ITimerService;

#endif 	/* __ITimerService_FWD_DEFINED__ */


#ifndef __ITimer_FWD_DEFINED__
#define __ITimer_FWD_DEFINED__
typedef interface ITimer ITimer;

#endif 	/* __ITimer_FWD_DEFINED__ */


#ifndef __ITimerEx_FWD_DEFINED__
#define __ITimerEx_FWD_DEFINED__
typedef interface ITimerEx ITimerEx;

#endif 	/* __ITimerEx_FWD_DEFINED__ */


#ifndef __ITimerSink_FWD_DEFINED__
#define __ITimerSink_FWD_DEFINED__
typedef interface ITimerSink ITimerSink;

#endif 	/* __ITimerSink_FWD_DEFINED__ */


#ifndef __IMapMIMEToCLSID_FWD_DEFINED__
#define __IMapMIMEToCLSID_FWD_DEFINED__
typedef interface IMapMIMEToCLSID IMapMIMEToCLSID;

#endif 	/* __IMapMIMEToCLSID_FWD_DEFINED__ */


#ifndef __IImageDecodeFilter_FWD_DEFINED__
#define __IImageDecodeFilter_FWD_DEFINED__
typedef interface IImageDecodeFilter IImageDecodeFilter;

#endif 	/* __IImageDecodeFilter_FWD_DEFINED__ */


#ifndef __IImageDecodeEventSink_FWD_DEFINED__
#define __IImageDecodeEventSink_FWD_DEFINED__
typedef interface IImageDecodeEventSink IImageDecodeEventSink;

#endif 	/* __IImageDecodeEventSink_FWD_DEFINED__ */


#ifndef __IImageDecodeEventSink2_FWD_DEFINED__
#define __IImageDecodeEventSink2_FWD_DEFINED__
typedef interface IImageDecodeEventSink2 IImageDecodeEventSink2;

#endif 	/* __IImageDecodeEventSink2_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "oleidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_ocmm_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// ocmm.h
//=--------------------------------------------------------------------------=
// (C) Copyright Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define SURFACE_LOCK_EXCLUSIVE        0x01
#define SURFACE_LOCK_ALLOW_DISCARD    0x02
#define SURFACE_LOCK_WAIT             0x04

#define E_SURFACE_NOSURFACE             0x8000C000L
#define E_SURFACE_UNKNOWN_FORMAT        0x8000C001L
#define E_SURFACE_NOTMYPOINTER          0x8000C002L
#define E_SURFACE_DISCARDED             0x8000C003L
#define E_SURFACE_NODC                  0x8000C004L
#define E_SURFACE_NOTMYDC               0x8000C005L
#define S_SURFACE_DISCARDED             0x0000C003L

typedef GUID BFID;

#ifndef RGBQUAD_DEFINED
#define RGBQUAD_DEFINED
typedef struct tagRGBQUAD RGBQUAD;

#endif
EXTERN_C const GUID BFID_MONOCHROME;
EXTERN_C const GUID BFID_RGB_4;
EXTERN_C const GUID BFID_RGB_8;
EXTERN_C const GUID BFID_RGB_555;
EXTERN_C const GUID BFID_RGB_565;
EXTERN_C const GUID BFID_RGB_24;
EXTERN_C const GUID BFID_RGB_32;
EXTERN_C const GUID BFID_RGBA_32;
EXTERN_C const GUID BFID_GRAY_8;
EXTERN_C const GUID BFID_GRAY_16;

#define SID_SDirectDraw3 IID_IDirectDraw3

#define COLOR_NO_TRANSPARENT 0xFFFFFFFF

#define IMGDECODE_EVENT_PROGRESS 0x01
#define IMGDECODE_EVENT_PALETTE 0x02
#define IMGDECODE_EVENT_BEGINBITS 0x04
#define IMGDECODE_EVENT_BITSCOMPLETE 0x08
#define IMGDECODE_EVENT_USEDDRAW 0x10

#define IMGDECODE_HINT_TOPDOWN 0x01
#define IMGDECODE_HINT_BOTTOMUP 0x02
#define IMGDECODE_HINT_FULLWIDTH 0x04

#define MAPMIME_DEFAULT 0
#define MAPMIME_CLSID 1
#define MAPMIME_DISABLE 2
#define MAPMIME_DEFAULT_ALWAYS 3

#define BFID_INDEXED_RGB_8 BFID_RGB_8
#define BFID_INDEXED_RGB_4 BFID_RGB_4
#define BFID_INDEXED_RGB_1 BFID_MONOCHROME

EXTERN_C const GUID CLSID_IImageDecodeFilter;

EXTERN_C const GUID NAMEDTIMER_DRAW;

#define TIMERMODE_NORMAL            0
#define TIMERMODE_VISIBILITYAWARE   1







extern RPC_IF_HANDLE __MIDL_itf_ocmm_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ocmm_0000_0000_v0_0_s_ifspec;

#ifndef __ITimerService_INTERFACE_DEFINED__
#define __ITimerService_INTERFACE_DEFINED__

/* interface ITimerService */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITimerService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3050f35f-98b5-11cf-bb82-00aa00bdce0b")
    ITimerService : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateTimer( 
            /* [in] */ __RPC__in_opt ITimer *pReferenceTimer,
            /* [out] */ __RPC__deref_out_opt ITimer **ppNewTimer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNamedTimer( 
            /* [in] */ __RPC__in REFGUID rguidName,
            /* [out] */ __RPC__deref_out_opt ITimer **ppTimer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetNamedTimerReference( 
            /* [in] */ __RPC__in REFGUID rguidName,
            /* [in] */ __RPC__in_opt ITimer *pReferenceTimer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITimerServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITimerService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITimerService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITimerService * This);
        
        DECLSPEC_XFGVIRT(ITimerService, CreateTimer)
        HRESULT ( STDMETHODCALLTYPE *CreateTimer )( 
            __RPC__in ITimerService * This,
            /* [in] */ __RPC__in_opt ITimer *pReferenceTimer,
            /* [out] */ __RPC__deref_out_opt ITimer **ppNewTimer);
        
        DECLSPEC_XFGVIRT(ITimerService, GetNamedTimer)
        HRESULT ( STDMETHODCALLTYPE *GetNamedTimer )( 
            __RPC__in ITimerService * This,
            /* [in] */ __RPC__in REFGUID rguidName,
            /* [out] */ __RPC__deref_out_opt ITimer **ppTimer);
        
        DECLSPEC_XFGVIRT(ITimerService, SetNamedTimerReference)
        HRESULT ( STDMETHODCALLTYPE *SetNamedTimerReference )( 
            __RPC__in ITimerService * This,
            /* [in] */ __RPC__in REFGUID rguidName,
            /* [in] */ __RPC__in_opt ITimer *pReferenceTimer);
        
        END_INTERFACE
    } ITimerServiceVtbl;

    interface ITimerService
    {
        CONST_VTBL struct ITimerServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITimerService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITimerService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITimerService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITimerService_CreateTimer(This,pReferenceTimer,ppNewTimer)	\
    ( (This)->lpVtbl -> CreateTimer(This,pReferenceTimer,ppNewTimer) ) 

#define ITimerService_GetNamedTimer(This,rguidName,ppTimer)	\
    ( (This)->lpVtbl -> GetNamedTimer(This,rguidName,ppTimer) ) 

#define ITimerService_SetNamedTimerReference(This,rguidName,pReferenceTimer)	\
    ( (This)->lpVtbl -> SetNamedTimerReference(This,rguidName,pReferenceTimer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITimerService_INTERFACE_DEFINED__ */


#ifndef __ITimer_INTERFACE_DEFINED__
#define __ITimer_INTERFACE_DEFINED__

/* interface ITimer */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITimer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3050f360-98b5-11cf-bb82-00aa00bdce0b")
    ITimer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Advise( 
            /* [in] */ VARIANT vtimeMin,
            /* [in] */ VARIANT vtimeMax,
            /* [in] */ VARIANT vtimeInterval,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt ITimerSink *pTimerSink,
            /* [out] */ __RPC__out DWORD *pdwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unadvise( 
            /* [in] */ DWORD dwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Freeze( 
            /* [in] */ BOOL fFreeze) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTime( 
            /* [out] */ __RPC__out VARIANT *pvtime) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITimerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITimer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITimer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITimer * This);
        
        DECLSPEC_XFGVIRT(ITimer, Advise)
        HRESULT ( STDMETHODCALLTYPE *Advise )( 
            __RPC__in ITimer * This,
            /* [in] */ VARIANT vtimeMin,
            /* [in] */ VARIANT vtimeMax,
            /* [in] */ VARIANT vtimeInterval,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt ITimerSink *pTimerSink,
            /* [out] */ __RPC__out DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(ITimer, Unadvise)
        HRESULT ( STDMETHODCALLTYPE *Unadvise )( 
            __RPC__in ITimer * This,
            /* [in] */ DWORD dwCookie);
        
        DECLSPEC_XFGVIRT(ITimer, Freeze)
        HRESULT ( STDMETHODCALLTYPE *Freeze )( 
            __RPC__in ITimer * This,
            /* [in] */ BOOL fFreeze);
        
        DECLSPEC_XFGVIRT(ITimer, GetTime)
        HRESULT ( STDMETHODCALLTYPE *GetTime )( 
            __RPC__in ITimer * This,
            /* [out] */ __RPC__out VARIANT *pvtime);
        
        END_INTERFACE
    } ITimerVtbl;

    interface ITimer
    {
        CONST_VTBL struct ITimerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITimer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITimer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITimer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITimer_Advise(This,vtimeMin,vtimeMax,vtimeInterval,dwFlags,pTimerSink,pdwCookie)	\
    ( (This)->lpVtbl -> Advise(This,vtimeMin,vtimeMax,vtimeInterval,dwFlags,pTimerSink,pdwCookie) ) 

#define ITimer_Unadvise(This,dwCookie)	\
    ( (This)->lpVtbl -> Unadvise(This,dwCookie) ) 

#define ITimer_Freeze(This,fFreeze)	\
    ( (This)->lpVtbl -> Freeze(This,fFreeze) ) 

#define ITimer_GetTime(This,pvtime)	\
    ( (This)->lpVtbl -> GetTime(This,pvtime) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITimer_INTERFACE_DEFINED__ */


#ifndef __ITimerEx_INTERFACE_DEFINED__
#define __ITimerEx_INTERFACE_DEFINED__

/* interface ITimerEx */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITimerEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("30510414-98b5-11cf-bb82-00aa00bdce0b")
    ITimerEx : public ITimer
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetMode( 
            /* [in] */ DWORD dwMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITimerExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITimerEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITimerEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITimerEx * This);
        
        DECLSPEC_XFGVIRT(ITimer, Advise)
        HRESULT ( STDMETHODCALLTYPE *Advise )( 
            __RPC__in ITimerEx * This,
            /* [in] */ VARIANT vtimeMin,
            /* [in] */ VARIANT vtimeMax,
            /* [in] */ VARIANT vtimeInterval,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt ITimerSink *pTimerSink,
            /* [out] */ __RPC__out DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(ITimer, Unadvise)
        HRESULT ( STDMETHODCALLTYPE *Unadvise )( 
            __RPC__in ITimerEx * This,
            /* [in] */ DWORD dwCookie);
        
        DECLSPEC_XFGVIRT(ITimer, Freeze)
        HRESULT ( STDMETHODCALLTYPE *Freeze )( 
            __RPC__in ITimerEx * This,
            /* [in] */ BOOL fFreeze);
        
        DECLSPEC_XFGVIRT(ITimer, GetTime)
        HRESULT ( STDMETHODCALLTYPE *GetTime )( 
            __RPC__in ITimerEx * This,
            /* [out] */ __RPC__out VARIANT *pvtime);
        
        DECLSPEC_XFGVIRT(ITimerEx, SetMode)
        HRESULT ( STDMETHODCALLTYPE *SetMode )( 
            __RPC__in ITimerEx * This,
            /* [in] */ DWORD dwMode);
        
        END_INTERFACE
    } ITimerExVtbl;

    interface ITimerEx
    {
        CONST_VTBL struct ITimerExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITimerEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITimerEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITimerEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITimerEx_Advise(This,vtimeMin,vtimeMax,vtimeInterval,dwFlags,pTimerSink,pdwCookie)	\
    ( (This)->lpVtbl -> Advise(This,vtimeMin,vtimeMax,vtimeInterval,dwFlags,pTimerSink,pdwCookie) ) 

#define ITimerEx_Unadvise(This,dwCookie)	\
    ( (This)->lpVtbl -> Unadvise(This,dwCookie) ) 

#define ITimerEx_Freeze(This,fFreeze)	\
    ( (This)->lpVtbl -> Freeze(This,fFreeze) ) 

#define ITimerEx_GetTime(This,pvtime)	\
    ( (This)->lpVtbl -> GetTime(This,pvtime) ) 


#define ITimerEx_SetMode(This,dwMode)	\
    ( (This)->lpVtbl -> SetMode(This,dwMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITimerEx_INTERFACE_DEFINED__ */


#ifndef __ITimerSink_INTERFACE_DEFINED__
#define __ITimerSink_INTERFACE_DEFINED__

/* interface ITimerSink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITimerSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3050f361-98b5-11cf-bb82-00aa00bdce0b")
    ITimerSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnTimer( 
            /* [in] */ VARIANT vtimeAdvise) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITimerSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITimerSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITimerSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITimerSink * This);
        
        DECLSPEC_XFGVIRT(ITimerSink, OnTimer)
        HRESULT ( STDMETHODCALLTYPE *OnTimer )( 
            __RPC__in ITimerSink * This,
            /* [in] */ VARIANT vtimeAdvise);
        
        END_INTERFACE
    } ITimerSinkVtbl;

    interface ITimerSink
    {
        CONST_VTBL struct ITimerSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITimerSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITimerSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITimerSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITimerSink_OnTimer(This,vtimeAdvise)	\
    ( (This)->lpVtbl -> OnTimer(This,vtimeAdvise) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITimerSink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_ocmm_0000_0004 */
/* [local] */ 

#define SID_STimerService IID_ITimerService






extern RPC_IF_HANDLE __MIDL_itf_ocmm_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ocmm_0000_0004_v0_0_s_ifspec;

#ifndef __IMapMIMEToCLSID_INTERFACE_DEFINED__
#define __IMapMIMEToCLSID_INTERFACE_DEFINED__

/* interface IMapMIMEToCLSID */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMapMIMEToCLSID;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D9E89500-30FA-11d0-B724-00AA006C1A01")
    IMapMIMEToCLSID : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnableDefaultMappings( 
            BOOL bEnable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MapMIMEToCLSID( 
            __RPC__in LPCOLESTR pszMIMEType,
            __RPC__in CLSID *pCLSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMapping( 
            __RPC__in LPCOLESTR pszMIMEType,
            DWORD dwMapMode,
            __RPC__in REFCLSID clsid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMapMIMEToCLSIDVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMapMIMEToCLSID * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMapMIMEToCLSID * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMapMIMEToCLSID * This);
        
        DECLSPEC_XFGVIRT(IMapMIMEToCLSID, EnableDefaultMappings)
        HRESULT ( STDMETHODCALLTYPE *EnableDefaultMappings )( 
            __RPC__in IMapMIMEToCLSID * This,
            BOOL bEnable);
        
        DECLSPEC_XFGVIRT(IMapMIMEToCLSID, MapMIMEToCLSID)
        HRESULT ( STDMETHODCALLTYPE *MapMIMEToCLSID )( 
            __RPC__in IMapMIMEToCLSID * This,
            __RPC__in LPCOLESTR pszMIMEType,
            __RPC__in CLSID *pCLSID);
        
        DECLSPEC_XFGVIRT(IMapMIMEToCLSID, SetMapping)
        HRESULT ( STDMETHODCALLTYPE *SetMapping )( 
            __RPC__in IMapMIMEToCLSID * This,
            __RPC__in LPCOLESTR pszMIMEType,
            DWORD dwMapMode,
            __RPC__in REFCLSID clsid);
        
        END_INTERFACE
    } IMapMIMEToCLSIDVtbl;

    interface IMapMIMEToCLSID
    {
        CONST_VTBL struct IMapMIMEToCLSIDVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMapMIMEToCLSID_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMapMIMEToCLSID_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMapMIMEToCLSID_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMapMIMEToCLSID_EnableDefaultMappings(This,bEnable)	\
    ( (This)->lpVtbl -> EnableDefaultMappings(This,bEnable) ) 

#define IMapMIMEToCLSID_MapMIMEToCLSID(This,pszMIMEType,pCLSID)	\
    ( (This)->lpVtbl -> MapMIMEToCLSID(This,pszMIMEType,pCLSID) ) 

#define IMapMIMEToCLSID_SetMapping(This,pszMIMEType,dwMapMode,clsid)	\
    ( (This)->lpVtbl -> SetMapping(This,pszMIMEType,dwMapMode,clsid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMapMIMEToCLSID_INTERFACE_DEFINED__ */


#ifndef __IImageDecodeFilter_INTERFACE_DEFINED__
#define __IImageDecodeFilter_INTERFACE_DEFINED__

/* interface IImageDecodeFilter */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IImageDecodeFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A3CCEDF3-2DE2-11D0-86F4-00A0C913F750")
    IImageDecodeFilter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            __RPC__in_opt IImageDecodeEventSink *pEventSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Process( 
            __RPC__in_opt IStream *pStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Terminate( 
            HRESULT hrStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IImageDecodeFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IImageDecodeFilter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IImageDecodeFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IImageDecodeFilter * This);
        
        DECLSPEC_XFGVIRT(IImageDecodeFilter, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IImageDecodeFilter * This,
            __RPC__in_opt IImageDecodeEventSink *pEventSink);
        
        DECLSPEC_XFGVIRT(IImageDecodeFilter, Process)
        HRESULT ( STDMETHODCALLTYPE *Process )( 
            __RPC__in IImageDecodeFilter * This,
            __RPC__in_opt IStream *pStream);
        
        DECLSPEC_XFGVIRT(IImageDecodeFilter, Terminate)
        HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            __RPC__in IImageDecodeFilter * This,
            HRESULT hrStatus);
        
        END_INTERFACE
    } IImageDecodeFilterVtbl;

    interface IImageDecodeFilter
    {
        CONST_VTBL struct IImageDecodeFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IImageDecodeFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IImageDecodeFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IImageDecodeFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IImageDecodeFilter_Initialize(This,pEventSink)	\
    ( (This)->lpVtbl -> Initialize(This,pEventSink) ) 

#define IImageDecodeFilter_Process(This,pStream)	\
    ( (This)->lpVtbl -> Process(This,pStream) ) 

#define IImageDecodeFilter_Terminate(This,hrStatus)	\
    ( (This)->lpVtbl -> Terminate(This,hrStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IImageDecodeFilter_INTERFACE_DEFINED__ */


#ifndef __IImageDecodeEventSink_INTERFACE_DEFINED__
#define __IImageDecodeEventSink_INTERFACE_DEFINED__

/* interface IImageDecodeEventSink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IImageDecodeEventSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BAA342A0-2DED-11d0-86F4-00A0C913F750")
    IImageDecodeEventSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSurface( 
            /* [in] */ LONG nWidth,
            /* [in] */ LONG nHeight,
            /* [in] */ __RPC__in REFGUID bfid,
            /* [in] */ ULONG nPasses,
            /* [in] */ DWORD dwHints,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppSurface) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnBeginDecode( 
            /* [out] */ __RPC__out DWORD *pdwEvents,
            /* [out] */ __RPC__out ULONG *pnFormats,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnFormats) BFID **ppFormats) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnBitsComplete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDecodeComplete( 
            /* [in] */ HRESULT hrStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnPalette( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnProgress( 
            /* [in] */ __RPC__in RECT *pBounds,
            /* [in] */ BOOL bComplete) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IImageDecodeEventSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IImageDecodeEventSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IImageDecodeEventSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IImageDecodeEventSink * This);
        
        DECLSPEC_XFGVIRT(IImageDecodeEventSink, GetSurface)
        HRESULT ( STDMETHODCALLTYPE *GetSurface )( 
            __RPC__in IImageDecodeEventSink * This,
            /* [in] */ LONG nWidth,
            /* [in] */ LONG nHeight,
            /* [in] */ __RPC__in REFGUID bfid,
            /* [in] */ ULONG nPasses,
            /* [in] */ DWORD dwHints,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppSurface);
        
        DECLSPEC_XFGVIRT(IImageDecodeEventSink, OnBeginDecode)
        HRESULT ( STDMETHODCALLTYPE *OnBeginDecode )( 
            __RPC__in IImageDecodeEventSink * This,
            /* [out] */ __RPC__out DWORD *pdwEvents,
            /* [out] */ __RPC__out ULONG *pnFormats,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pnFormats) BFID **ppFormats);
        
        DECLSPEC_XFGVIRT(IImageDecodeEventSink, OnBitsComplete)
        HRESULT ( STDMETHODCALLTYPE *OnBitsComplete )( 
            __RPC__in IImageDecodeEventSink * This);
        
        DECLSPEC_XFGVIRT(IImageDecodeEventSink, OnDecodeComplete)
        HRESULT ( STDMETHODCALLTYPE *OnDecodeComplete )( 
            __RPC__in IImageDecodeEventSink * This,
            /* [in] */ HRESULT hrStatus);
        
        DECLSPEC_XFGVIRT(IImageDecodeEventSink, OnPalette)
        HRESULT ( STDMETHODCALLTYPE *OnPalette )( 
            __RPC__in IImageDecodeEventSink * This);
        
        DECLSPEC_XFGVIRT(IImageDecodeEventSink, OnProgress)
        HRESULT ( STDMETHODCALLTYPE *OnProgress )( 
            __RPC__in IImageDecodeEventSink * This,
            /* [in] */ __RPC__in RECT *pBounds,
            /* [in] */ BOOL bComplete);
        
        END_INTERFACE
    } IImageDecodeEventSinkVtbl;

    interface IImageDecodeEventSink
    {
        CONST_VTBL struct IImageDecodeEventSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IImageDecodeEventSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IImageDecodeEventSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IImageDecodeEventSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IImageDecodeEventSink_GetSurface(This,nWidth,nHeight,bfid,nPasses,dwHints,ppSurface)	\
    ( (This)->lpVtbl -> GetSurface(This,nWidth,nHeight,bfid,nPasses,dwHints,ppSurface) ) 

#define IImageDecodeEventSink_OnBeginDecode(This,pdwEvents,pnFormats,ppFormats)	\
    ( (This)->lpVtbl -> OnBeginDecode(This,pdwEvents,pnFormats,ppFormats) ) 

#define IImageDecodeEventSink_OnBitsComplete(This)	\
    ( (This)->lpVtbl -> OnBitsComplete(This) ) 

#define IImageDecodeEventSink_OnDecodeComplete(This,hrStatus)	\
    ( (This)->lpVtbl -> OnDecodeComplete(This,hrStatus) ) 

#define IImageDecodeEventSink_OnPalette(This)	\
    ( (This)->lpVtbl -> OnPalette(This) ) 

#define IImageDecodeEventSink_OnProgress(This,pBounds,bComplete)	\
    ( (This)->lpVtbl -> OnProgress(This,pBounds,bComplete) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IImageDecodeEventSink_INTERFACE_DEFINED__ */


#ifndef __IImageDecodeEventSink2_INTERFACE_DEFINED__
#define __IImageDecodeEventSink2_INTERFACE_DEFINED__

/* interface IImageDecodeEventSink2 */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IImageDecodeEventSink2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8EBD8A57-8A96-48c9-84A6-962E2DB9C931")
    IImageDecodeEventSink2 : public IImageDecodeEventSink
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsAlphaPremultRequired( 
            /* [out] */ BOOL *pfPremultAlpha) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IImageDecodeEventSink2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IImageDecodeEventSink2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IImageDecodeEventSink2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IImageDecodeEventSink2 * This);
        
        DECLSPEC_XFGVIRT(IImageDecodeEventSink, GetSurface)
        HRESULT ( STDMETHODCALLTYPE *GetSurface )( 
            IImageDecodeEventSink2 * This,
            /* [in] */ LONG nWidth,
            /* [in] */ LONG nHeight,
            /* [in] */ REFGUID bfid,
            /* [in] */ ULONG nPasses,
            /* [in] */ DWORD dwHints,
            /* [out] */ IUnknown **ppSurface);
        
        DECLSPEC_XFGVIRT(IImageDecodeEventSink, OnBeginDecode)
        HRESULT ( STDMETHODCALLTYPE *OnBeginDecode )( 
            IImageDecodeEventSink2 * This,
            /* [out] */ DWORD *pdwEvents,
            /* [out] */ ULONG *pnFormats,
            /* [size_is][size_is][out] */ BFID **ppFormats);
        
        DECLSPEC_XFGVIRT(IImageDecodeEventSink, OnBitsComplete)
        HRESULT ( STDMETHODCALLTYPE *OnBitsComplete )( 
            IImageDecodeEventSink2 * This);
        
        DECLSPEC_XFGVIRT(IImageDecodeEventSink, OnDecodeComplete)
        HRESULT ( STDMETHODCALLTYPE *OnDecodeComplete )( 
            IImageDecodeEventSink2 * This,
            /* [in] */ HRESULT hrStatus);
        
        DECLSPEC_XFGVIRT(IImageDecodeEventSink, OnPalette)
        HRESULT ( STDMETHODCALLTYPE *OnPalette )( 
            IImageDecodeEventSink2 * This);
        
        DECLSPEC_XFGVIRT(IImageDecodeEventSink, OnProgress)
        HRESULT ( STDMETHODCALLTYPE *OnProgress )( 
            IImageDecodeEventSink2 * This,
            /* [in] */ RECT *pBounds,
            /* [in] */ BOOL bComplete);
        
        DECLSPEC_XFGVIRT(IImageDecodeEventSink2, IsAlphaPremultRequired)
        HRESULT ( STDMETHODCALLTYPE *IsAlphaPremultRequired )( 
            IImageDecodeEventSink2 * This,
            /* [out] */ BOOL *pfPremultAlpha);
        
        END_INTERFACE
    } IImageDecodeEventSink2Vtbl;

    interface IImageDecodeEventSink2
    {
        CONST_VTBL struct IImageDecodeEventSink2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IImageDecodeEventSink2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IImageDecodeEventSink2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IImageDecodeEventSink2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IImageDecodeEventSink2_GetSurface(This,nWidth,nHeight,bfid,nPasses,dwHints,ppSurface)	\
    ( (This)->lpVtbl -> GetSurface(This,nWidth,nHeight,bfid,nPasses,dwHints,ppSurface) ) 

#define IImageDecodeEventSink2_OnBeginDecode(This,pdwEvents,pnFormats,ppFormats)	\
    ( (This)->lpVtbl -> OnBeginDecode(This,pdwEvents,pnFormats,ppFormats) ) 

#define IImageDecodeEventSink2_OnBitsComplete(This)	\
    ( (This)->lpVtbl -> OnBitsComplete(This) ) 

#define IImageDecodeEventSink2_OnDecodeComplete(This,hrStatus)	\
    ( (This)->lpVtbl -> OnDecodeComplete(This,hrStatus) ) 

#define IImageDecodeEventSink2_OnPalette(This)	\
    ( (This)->lpVtbl -> OnPalette(This) ) 

#define IImageDecodeEventSink2_OnProgress(This,pBounds,bComplete)	\
    ( (This)->lpVtbl -> OnProgress(This,pBounds,bComplete) ) 


#define IImageDecodeEventSink2_IsAlphaPremultRequired(This,pfPremultAlpha)	\
    ( (This)->lpVtbl -> IsAlphaPremultRequired(This,pfPremultAlpha) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IImageDecodeEventSink2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_ocmm_0000_0008 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_ocmm_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ocmm_0000_0008_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


