

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */

#pragma warning( disable: 4049 )  /* more than 64k source lines */


/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 475
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

#ifndef __msaatext_h__
#define __msaatext_h__

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

#ifndef __ITfMSAAControl_FWD_DEFINED__
#define __ITfMSAAControl_FWD_DEFINED__
typedef interface ITfMSAAControl ITfMSAAControl;

#endif 	/* __ITfMSAAControl_FWD_DEFINED__ */


#ifndef __IInternalDocWrap_FWD_DEFINED__
#define __IInternalDocWrap_FWD_DEFINED__
typedef interface IInternalDocWrap IInternalDocWrap;

#endif 	/* __IInternalDocWrap_FWD_DEFINED__ */


#ifndef __ITextStoreACPEx_FWD_DEFINED__
#define __ITextStoreACPEx_FWD_DEFINED__
typedef interface ITextStoreACPEx ITextStoreACPEx;

#endif 	/* __ITextStoreACPEx_FWD_DEFINED__ */


#ifndef __ITextStoreAnchorEx_FWD_DEFINED__
#define __ITextStoreAnchorEx_FWD_DEFINED__
typedef interface ITextStoreAnchorEx ITextStoreAnchorEx;

#endif 	/* __ITextStoreAnchorEx_FWD_DEFINED__ */


#ifndef __ITextStoreACPSinkEx_FWD_DEFINED__
#define __ITextStoreACPSinkEx_FWD_DEFINED__
typedef interface ITextStoreACPSinkEx ITextStoreACPSinkEx;

#endif 	/* __ITextStoreACPSinkEx_FWD_DEFINED__ */


#ifndef __ITextStoreSinkAnchorEx_FWD_DEFINED__
#define __ITextStoreSinkAnchorEx_FWD_DEFINED__
typedef interface ITextStoreSinkAnchorEx ITextStoreSinkAnchorEx;

#endif 	/* __ITextStoreSinkAnchorEx_FWD_DEFINED__ */


#ifndef __IAccDictionary_FWD_DEFINED__
#define __IAccDictionary_FWD_DEFINED__
typedef interface IAccDictionary IAccDictionary;

#endif 	/* __IAccDictionary_FWD_DEFINED__ */


#ifndef __IVersionInfo_FWD_DEFINED__
#define __IVersionInfo_FWD_DEFINED__
typedef interface IVersionInfo IVersionInfo;

#endif 	/* __IVersionInfo_FWD_DEFINED__ */


#ifndef __ICoCreateLocally_FWD_DEFINED__
#define __ICoCreateLocally_FWD_DEFINED__
typedef interface ICoCreateLocally ICoCreateLocally;

#endif 	/* __ICoCreateLocally_FWD_DEFINED__ */


#ifndef __ICoCreatedLocally_FWD_DEFINED__
#define __ICoCreatedLocally_FWD_DEFINED__
typedef interface ICoCreatedLocally ICoCreatedLocally;

#endif 	/* __ICoCreatedLocally_FWD_DEFINED__ */


#ifndef __IAccStore_FWD_DEFINED__
#define __IAccStore_FWD_DEFINED__
typedef interface IAccStore IAccStore;

#endif 	/* __IAccStore_FWD_DEFINED__ */


#ifndef __IAccServerDocMgr_FWD_DEFINED__
#define __IAccServerDocMgr_FWD_DEFINED__
typedef interface IAccServerDocMgr IAccServerDocMgr;

#endif 	/* __IAccServerDocMgr_FWD_DEFINED__ */


#ifndef __IAccClientDocMgr_FWD_DEFINED__
#define __IAccClientDocMgr_FWD_DEFINED__
typedef interface IAccClientDocMgr IAccClientDocMgr;

#endif 	/* __IAccClientDocMgr_FWD_DEFINED__ */


#ifndef __IDocWrap_FWD_DEFINED__
#define __IDocWrap_FWD_DEFINED__
typedef interface IDocWrap IDocWrap;

#endif 	/* __IDocWrap_FWD_DEFINED__ */


#ifndef __IClonableWrapper_FWD_DEFINED__
#define __IClonableWrapper_FWD_DEFINED__
typedef interface IClonableWrapper IClonableWrapper;

#endif 	/* __IClonableWrapper_FWD_DEFINED__ */


#ifndef __MSAAControl_FWD_DEFINED__
#define __MSAAControl_FWD_DEFINED__

#ifdef __cplusplus
typedef class MSAAControl MSAAControl;
#else
typedef struct MSAAControl MSAAControl;
#endif /* __cplusplus */

#endif 	/* __MSAAControl_FWD_DEFINED__ */


#ifndef __AccStore_FWD_DEFINED__
#define __AccStore_FWD_DEFINED__

#ifdef __cplusplus
typedef class AccStore AccStore;
#else
typedef struct AccStore AccStore;
#endif /* __cplusplus */

#endif 	/* __AccStore_FWD_DEFINED__ */


#ifndef __AccDictionary_FWD_DEFINED__
#define __AccDictionary_FWD_DEFINED__

#ifdef __cplusplus
typedef class AccDictionary AccDictionary;
#else
typedef struct AccDictionary AccDictionary;
#endif /* __cplusplus */

#endif 	/* __AccDictionary_FWD_DEFINED__ */


#ifndef __AccServerDocMgr_FWD_DEFINED__
#define __AccServerDocMgr_FWD_DEFINED__

#ifdef __cplusplus
typedef class AccServerDocMgr AccServerDocMgr;
#else
typedef struct AccServerDocMgr AccServerDocMgr;
#endif /* __cplusplus */

#endif 	/* __AccServerDocMgr_FWD_DEFINED__ */


#ifndef __AccClientDocMgr_FWD_DEFINED__
#define __AccClientDocMgr_FWD_DEFINED__

#ifdef __cplusplus
typedef class AccClientDocMgr AccClientDocMgr;
#else
typedef struct AccClientDocMgr AccClientDocMgr;
#endif /* __cplusplus */

#endif 	/* __AccClientDocMgr_FWD_DEFINED__ */


#ifndef __DocWrap_FWD_DEFINED__
#define __DocWrap_FWD_DEFINED__

#ifdef __cplusplus
typedef class DocWrap DocWrap;
#else
typedef struct DocWrap DocWrap;
#endif /* __cplusplus */

#endif 	/* __DocWrap_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "textstor.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_msaatext_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
DEFINE_GUID( IID_ITfMSAAControl, 0xb5f8fb3b,0x393f,0x4f7c,0x84,0xcb,0x50,0x49,0x24,0xc2,0x70,0x5a);
DEFINE_GUID( IID_IInternalDocWrap, 0xE1AA6466,0x9DB4,0x40ba,0xBE,0x03,0x77,0xC3,0x8E,0x8E,0x60,0xB2);
DEFINE_GUID( IID_ITextStoreACPEx, 0xA2DE3BC2,0x3D8E,0x11d3,0x81,0xA9,0xF7,0x53,0xFB,0xE6,0x1A,0x00);
DEFINE_GUID( IID_ITextStoreAnchorEx, 0xA2DE3BC1,0x3D8E,0x11d3,0x81,0xA9,0xF7,0x53,0xFB,0xE6,0x1A,0x00);
DEFINE_GUID( IID_ITextStoreACPSinkEx, 0x2bdf9464,0x41e2,0x43e3,0x95,0x0c,0xa6,0x86,0x5b,0xa2,0x5c,0xd4);
DEFINE_GUID( IID_ITextStoreSinkAnchorEx, 0x25642426,0x028d,0x4474,0x97,0x7b,0x11,0x1b,0xb1,0x14,0xfe,0x3e);
DEFINE_GUID( IID_IAccDictionary, 0x1DC4CB5F,0xD737,0x474d,0xAD,0xE9,0x5C,0xCF,0xC9,0xBC,0x1C,0xC9);
DEFINE_GUID( IID_IVersionInfo, 0x401518EC,0xDB00,0x4611,0x9B,0x29,0x2A,0x0E,0x4B,0x9A,0xFA,0x85);
DEFINE_GUID( IID_ICoCreateLocally, 0x03DE00AA,0xF272,0x41e3,0x99,0xCB,0x03,0xC5,0xE8,0x11,0x4E,0xA0);
DEFINE_GUID( IID_ICoCreatedLocally, 0x0A53EB6C,0x1908,0x4742,0x8C,0xFF,0x2C,0xEE,0x2E,0x93,0xF9,0x4C);
DEFINE_GUID( IID_IAccStore, 0xE2CD4A63,0x2B72,0x4D48,0xB7,0x39,0x95,0xE4,0x76,0x51,0x95,0xBA);
DEFINE_GUID( IID_IAccServerDocMgr, 0xAD7C73CF,0x6DD5,0x4855,0xAB,0xC2,0xB0,0x4B,0xAD,0x5B,0x91,0x53);
DEFINE_GUID( IID_IAccClientDocMgr, 0x4C896039,0x7B6D,0x49e6,0xA8,0xC1,0x45,0x11,0x6A,0x98,0x29,0x2B);
DEFINE_GUID( IID_IDocWrap, 0xDCD285FE,0x0BE0,0x43BD,0x99,0xC9,0xAA,0xAE,0xC5,0x13,0xC5,0x55);
DEFINE_GUID( IID_IClonableWrapper, 0xB33E75FF,0xE84C,0x4dca,0xA2,0x5C,0x33,0xB8,0xDC,0x00,0x33,0x74);
DEFINE_GUID( LIBID_MSAATEXTLib, 0x150E2D7A,0xDAC1,0x4582,0x94,0x7D,0x2A,0x8F,0xD7,0x8B,0x82,0xCD);
DEFINE_GUID( CLSID_MSAAControl, 0x08cd963f,0x7a3e,0x4f5c,0x9b,0xd8,0xd6,0x92,0xbb,0x04,0x3c,0x5b );
DEFINE_GUID( CLSID_AccStore, 0x5440837F,0x4BFF,0x4AE5,0xA1,0xB1,0x77,0x22,0xEC,0xC6,0x33,0x2A );
DEFINE_GUID( CLSID_AccDictionary, 0x6572EE16,0x5FE5,0x4331,0xBB,0x6D,0x76,0xA4,0x9C,0x56,0xE4,0x23 );
DEFINE_GUID( CLSID_AccServerDocMgr, 0x6089A37E,0xEB8A,0x482D,0xBD,0x6F,0xF9,0xF4,0x69,0x04,0xD1,0x6D );
DEFINE_GUID( CLSID_AccClientDocMgr, 0xFC48CC30,0x4F3E,0x4fa1,0x80,0x3B,0xAD,0x0E,0x19,0x6A,0x83,0xB1 );
DEFINE_GUID( CLSID_DocWrap, 0xBF426F7E,0x7A5E,0x44D6,0x83,0x0C,0xA3,0x90,0xEA,0x94,0x62,0xA3 );


extern RPC_IF_HANDLE __MIDL_itf_msaatext_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msaatext_0000_0000_v0_0_s_ifspec;

#ifndef __ITfMSAAControl_INTERFACE_DEFINED__
#define __ITfMSAAControl_INTERFACE_DEFINED__

/* interface ITfMSAAControl */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfMSAAControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b5f8fb3b-393f-4f7c-84cb-504924c2705a")
    ITfMSAAControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SystemEnableMSAA( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SystemDisableMSAA( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfMSAAControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfMSAAControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfMSAAControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfMSAAControl * This);
        
        DECLSPEC_XFGVIRT(ITfMSAAControl, SystemEnableMSAA)
        HRESULT ( STDMETHODCALLTYPE *SystemEnableMSAA )( 
            __RPC__in ITfMSAAControl * This);
        
        DECLSPEC_XFGVIRT(ITfMSAAControl, SystemDisableMSAA)
        HRESULT ( STDMETHODCALLTYPE *SystemDisableMSAA )( 
            __RPC__in ITfMSAAControl * This);
        
        END_INTERFACE
    } ITfMSAAControlVtbl;

    interface ITfMSAAControl
    {
        CONST_VTBL struct ITfMSAAControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfMSAAControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfMSAAControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfMSAAControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfMSAAControl_SystemEnableMSAA(This)	\
    ( (This)->lpVtbl -> SystemEnableMSAA(This) ) 

#define ITfMSAAControl_SystemDisableMSAA(This)	\
    ( (This)->lpVtbl -> SystemDisableMSAA(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfMSAAControl_INTERFACE_DEFINED__ */


#ifndef __IInternalDocWrap_INTERFACE_DEFINED__
#define __IInternalDocWrap_INTERFACE_DEFINED__

/* interface IInternalDocWrap */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IInternalDocWrap;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E1AA6466-9DB4-40ba-BE03-77C38E8E60B2")
    IInternalDocWrap : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE NotifyRevoke( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternalDocWrapVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternalDocWrap * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternalDocWrap * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternalDocWrap * This);
        
        DECLSPEC_XFGVIRT(IInternalDocWrap, NotifyRevoke)
        HRESULT ( STDMETHODCALLTYPE *NotifyRevoke )( 
            IInternalDocWrap * This);
        
        END_INTERFACE
    } IInternalDocWrapVtbl;

    interface IInternalDocWrap
    {
        CONST_VTBL struct IInternalDocWrapVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternalDocWrap_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternalDocWrap_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternalDocWrap_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternalDocWrap_NotifyRevoke(This)	\
    ( (This)->lpVtbl -> NotifyRevoke(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternalDocWrap_INTERFACE_DEFINED__ */


#ifndef __ITextStoreACPEx_INTERFACE_DEFINED__
#define __ITextStoreACPEx_INTERFACE_DEFINED__

/* interface ITextStoreACPEx */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITextStoreACPEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A2DE3BC2-3D8E-11d3-81A9-F753FBE61A00")
    ITextStoreACPEx : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ScrollToRect( 
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [in] */ RECT rc,
            /* [in] */ DWORD dwPosition) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextStoreACPExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextStoreACPEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextStoreACPEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextStoreACPEx * This);
        
        DECLSPEC_XFGVIRT(ITextStoreACPEx, ScrollToRect)
        HRESULT ( STDMETHODCALLTYPE *ScrollToRect )( 
            __RPC__in ITextStoreACPEx * This,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [in] */ RECT rc,
            /* [in] */ DWORD dwPosition);
        
        END_INTERFACE
    } ITextStoreACPExVtbl;

    interface ITextStoreACPEx
    {
        CONST_VTBL struct ITextStoreACPExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextStoreACPEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextStoreACPEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextStoreACPEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextStoreACPEx_ScrollToRect(This,acpStart,acpEnd,rc,dwPosition)	\
    ( (This)->lpVtbl -> ScrollToRect(This,acpStart,acpEnd,rc,dwPosition) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextStoreACPEx_INTERFACE_DEFINED__ */


#ifndef __ITextStoreAnchorEx_INTERFACE_DEFINED__
#define __ITextStoreAnchorEx_INTERFACE_DEFINED__

/* interface ITextStoreAnchorEx */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITextStoreAnchorEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A2DE3BC1-3D8E-11d3-81A9-F753FBE61A00")
    ITextStoreAnchorEx : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ScrollToRect( 
            /* [in] */ __RPC__in_opt IAnchor *pStart,
            /* [in] */ __RPC__in_opt IAnchor *pEnd,
            /* [in] */ RECT rc,
            /* [in] */ DWORD dwPosition) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextStoreAnchorExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextStoreAnchorEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextStoreAnchorEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextStoreAnchorEx * This);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchorEx, ScrollToRect)
        HRESULT ( STDMETHODCALLTYPE *ScrollToRect )( 
            __RPC__in ITextStoreAnchorEx * This,
            /* [in] */ __RPC__in_opt IAnchor *pStart,
            /* [in] */ __RPC__in_opt IAnchor *pEnd,
            /* [in] */ RECT rc,
            /* [in] */ DWORD dwPosition);
        
        END_INTERFACE
    } ITextStoreAnchorExVtbl;

    interface ITextStoreAnchorEx
    {
        CONST_VTBL struct ITextStoreAnchorExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextStoreAnchorEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextStoreAnchorEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextStoreAnchorEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextStoreAnchorEx_ScrollToRect(This,pStart,pEnd,rc,dwPosition)	\
    ( (This)->lpVtbl -> ScrollToRect(This,pStart,pEnd,rc,dwPosition) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextStoreAnchorEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msaatext_0000_0004 */
/* [local] */ 

#define	TS_STRF_START	( 0 )

#define	TS_STRF_MID	( 1 )

#define	TS_STRF_END	( 2 )



extern RPC_IF_HANDLE __MIDL_itf_msaatext_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msaatext_0000_0004_v0_0_s_ifspec;

#ifndef __ITextStoreACPSinkEx_INTERFACE_DEFINED__
#define __ITextStoreACPSinkEx_INTERFACE_DEFINED__

/* interface ITextStoreACPSinkEx */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITextStoreACPSinkEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2bdf9464-41e2-43e3-950c-a6865ba25cd4")
    ITextStoreACPSinkEx : public ITextStoreACPSink
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnDisconnect( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextStoreACPSinkExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextStoreACPSinkEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextStoreACPSinkEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextStoreACPSinkEx * This);
        
        DECLSPEC_XFGVIRT(ITextStoreACPSink, OnTextChange)
        HRESULT ( STDMETHODCALLTYPE *OnTextChange )( 
            __RPC__in ITextStoreACPSinkEx * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in const TS_TEXTCHANGE *pChange);
        
        DECLSPEC_XFGVIRT(ITextStoreACPSink, OnSelectionChange)
        HRESULT ( STDMETHODCALLTYPE *OnSelectionChange )( 
            __RPC__in ITextStoreACPSinkEx * This);
        
        DECLSPEC_XFGVIRT(ITextStoreACPSink, OnLayoutChange)
        HRESULT ( STDMETHODCALLTYPE *OnLayoutChange )( 
            __RPC__in ITextStoreACPSinkEx * This,
            /* [in] */ TsLayoutCode lcode,
            /* [in] */ TsViewCookie vcView);
        
        DECLSPEC_XFGVIRT(ITextStoreACPSink, OnStatusChange)
        HRESULT ( STDMETHODCALLTYPE *OnStatusChange )( 
            __RPC__in ITextStoreACPSinkEx * This,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITextStoreACPSink, OnAttrsChange)
        HRESULT ( STDMETHODCALLTYPE *OnAttrsChange )( 
            __RPC__in ITextStoreACPSinkEx * This,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [in] */ ULONG cAttrs,
            /* [size_is][in] */ __RPC__in_ecount_full(cAttrs) const TS_ATTRID *paAttrs);
        
        DECLSPEC_XFGVIRT(ITextStoreACPSink, OnLockGranted)
        HRESULT ( STDMETHODCALLTYPE *OnLockGranted )( 
            __RPC__in ITextStoreACPSinkEx * This,
            /* [in] */ DWORD dwLockFlags);
        
        DECLSPEC_XFGVIRT(ITextStoreACPSink, OnStartEditTransaction)
        HRESULT ( STDMETHODCALLTYPE *OnStartEditTransaction )( 
            __RPC__in ITextStoreACPSinkEx * This);
        
        DECLSPEC_XFGVIRT(ITextStoreACPSink, OnEndEditTransaction)
        HRESULT ( STDMETHODCALLTYPE *OnEndEditTransaction )( 
            __RPC__in ITextStoreACPSinkEx * This);
        
        DECLSPEC_XFGVIRT(ITextStoreACPSinkEx, OnDisconnect)
        HRESULT ( STDMETHODCALLTYPE *OnDisconnect )( 
            __RPC__in ITextStoreACPSinkEx * This);
        
        END_INTERFACE
    } ITextStoreACPSinkExVtbl;

    interface ITextStoreACPSinkEx
    {
        CONST_VTBL struct ITextStoreACPSinkExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextStoreACPSinkEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextStoreACPSinkEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextStoreACPSinkEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextStoreACPSinkEx_OnTextChange(This,dwFlags,pChange)	\
    ( (This)->lpVtbl -> OnTextChange(This,dwFlags,pChange) ) 

#define ITextStoreACPSinkEx_OnSelectionChange(This)	\
    ( (This)->lpVtbl -> OnSelectionChange(This) ) 

#define ITextStoreACPSinkEx_OnLayoutChange(This,lcode,vcView)	\
    ( (This)->lpVtbl -> OnLayoutChange(This,lcode,vcView) ) 

#define ITextStoreACPSinkEx_OnStatusChange(This,dwFlags)	\
    ( (This)->lpVtbl -> OnStatusChange(This,dwFlags) ) 

#define ITextStoreACPSinkEx_OnAttrsChange(This,acpStart,acpEnd,cAttrs,paAttrs)	\
    ( (This)->lpVtbl -> OnAttrsChange(This,acpStart,acpEnd,cAttrs,paAttrs) ) 

#define ITextStoreACPSinkEx_OnLockGranted(This,dwLockFlags)	\
    ( (This)->lpVtbl -> OnLockGranted(This,dwLockFlags) ) 

#define ITextStoreACPSinkEx_OnStartEditTransaction(This)	\
    ( (This)->lpVtbl -> OnStartEditTransaction(This) ) 

#define ITextStoreACPSinkEx_OnEndEditTransaction(This)	\
    ( (This)->lpVtbl -> OnEndEditTransaction(This) ) 


#define ITextStoreACPSinkEx_OnDisconnect(This)	\
    ( (This)->lpVtbl -> OnDisconnect(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextStoreACPSinkEx_INTERFACE_DEFINED__ */


#ifndef __ITextStoreSinkAnchorEx_INTERFACE_DEFINED__
#define __ITextStoreSinkAnchorEx_INTERFACE_DEFINED__

/* interface ITextStoreSinkAnchorEx */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITextStoreSinkAnchorEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("25642426-028d-4474-977b-111bb114fe3e")
    ITextStoreSinkAnchorEx : public ITextStoreAnchorSink
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnDisconnect( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextStoreSinkAnchorExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextStoreSinkAnchorEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextStoreSinkAnchorEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextStoreSinkAnchorEx * This);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchorSink, OnTextChange)
        HRESULT ( STDMETHODCALLTYPE *OnTextChange )( 
            __RPC__in ITextStoreSinkAnchorEx * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paEnd);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchorSink, OnSelectionChange)
        HRESULT ( STDMETHODCALLTYPE *OnSelectionChange )( 
            __RPC__in ITextStoreSinkAnchorEx * This);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchorSink, OnLayoutChange)
        HRESULT ( STDMETHODCALLTYPE *OnLayoutChange )( 
            __RPC__in ITextStoreSinkAnchorEx * This,
            /* [in] */ TsLayoutCode lcode,
            /* [in] */ TsViewCookie vcView);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchorSink, OnStatusChange)
        HRESULT ( STDMETHODCALLTYPE *OnStatusChange )( 
            __RPC__in ITextStoreSinkAnchorEx * This,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchorSink, OnAttrsChange)
        HRESULT ( STDMETHODCALLTYPE *OnAttrsChange )( 
            __RPC__in ITextStoreSinkAnchorEx * This,
            /* [in] */ __RPC__in_opt IAnchor *paStart,
            /* [in] */ __RPC__in_opt IAnchor *paEnd,
            /* [in] */ ULONG cAttrs,
            /* [size_is][in] */ __RPC__in_ecount_full(cAttrs) const TS_ATTRID *paAttrs);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchorSink, OnLockGranted)
        HRESULT ( STDMETHODCALLTYPE *OnLockGranted )( 
            __RPC__in ITextStoreSinkAnchorEx * This,
            /* [in] */ DWORD dwLockFlags);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchorSink, OnStartEditTransaction)
        HRESULT ( STDMETHODCALLTYPE *OnStartEditTransaction )( 
            __RPC__in ITextStoreSinkAnchorEx * This);
        
        DECLSPEC_XFGVIRT(ITextStoreAnchorSink, OnEndEditTransaction)
        HRESULT ( STDMETHODCALLTYPE *OnEndEditTransaction )( 
            __RPC__in ITextStoreSinkAnchorEx * This);
        
        DECLSPEC_XFGVIRT(ITextStoreSinkAnchorEx, OnDisconnect)
        HRESULT ( STDMETHODCALLTYPE *OnDisconnect )( 
            __RPC__in ITextStoreSinkAnchorEx * This);
        
        END_INTERFACE
    } ITextStoreSinkAnchorExVtbl;

    interface ITextStoreSinkAnchorEx
    {
        CONST_VTBL struct ITextStoreSinkAnchorExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextStoreSinkAnchorEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextStoreSinkAnchorEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextStoreSinkAnchorEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextStoreSinkAnchorEx_OnTextChange(This,dwFlags,paStart,paEnd)	\
    ( (This)->lpVtbl -> OnTextChange(This,dwFlags,paStart,paEnd) ) 

#define ITextStoreSinkAnchorEx_OnSelectionChange(This)	\
    ( (This)->lpVtbl -> OnSelectionChange(This) ) 

#define ITextStoreSinkAnchorEx_OnLayoutChange(This,lcode,vcView)	\
    ( (This)->lpVtbl -> OnLayoutChange(This,lcode,vcView) ) 

#define ITextStoreSinkAnchorEx_OnStatusChange(This,dwFlags)	\
    ( (This)->lpVtbl -> OnStatusChange(This,dwFlags) ) 

#define ITextStoreSinkAnchorEx_OnAttrsChange(This,paStart,paEnd,cAttrs,paAttrs)	\
    ( (This)->lpVtbl -> OnAttrsChange(This,paStart,paEnd,cAttrs,paAttrs) ) 

#define ITextStoreSinkAnchorEx_OnLockGranted(This,dwLockFlags)	\
    ( (This)->lpVtbl -> OnLockGranted(This,dwLockFlags) ) 

#define ITextStoreSinkAnchorEx_OnStartEditTransaction(This)	\
    ( (This)->lpVtbl -> OnStartEditTransaction(This) ) 

#define ITextStoreSinkAnchorEx_OnEndEditTransaction(This)	\
    ( (This)->lpVtbl -> OnEndEditTransaction(This) ) 


#define ITextStoreSinkAnchorEx_OnDisconnect(This)	\
    ( (This)->lpVtbl -> OnDisconnect(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextStoreSinkAnchorEx_INTERFACE_DEFINED__ */


#ifndef __IAccDictionary_INTERFACE_DEFINED__
#define __IAccDictionary_INTERFACE_DEFINED__

/* interface IAccDictionary */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IAccDictionary;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DC4CB5F-D737-474d-ADE9-5CCFC9BC1CC9")
    IAccDictionary : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetLocalizedString( 
            /* [in] */ __RPC__in REFGUID Term,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt BSTR *pResult,
            /* [out] */ __RPC__out LCID *plcid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParentTerm( 
            /* [in] */ __RPC__in REFGUID Term,
            /* [out] */ __RPC__out GUID *pParentTerm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMnemonicString( 
            /* [in] */ __RPC__in REFGUID Term,
            /* [out] */ __RPC__deref_out_opt BSTR *pResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LookupMnemonicTerm( 
            /* [in] */ __RPC__in BSTR bstrMnemonic,
            /* [out] */ __RPC__out GUID *pTerm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertValueToString( 
            /* [in] */ __RPC__in REFGUID Term,
            /* [in] */ LCID lcid,
            /* [in] */ VARIANT varValue,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrResult,
            /* [out] */ __RPC__out LCID *plcid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccDictionaryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAccDictionary * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAccDictionary * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAccDictionary * This);
        
        DECLSPEC_XFGVIRT(IAccDictionary, GetLocalizedString)
        HRESULT ( STDMETHODCALLTYPE *GetLocalizedString )( 
            __RPC__in IAccDictionary * This,
            /* [in] */ __RPC__in REFGUID Term,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt BSTR *pResult,
            /* [out] */ __RPC__out LCID *plcid);
        
        DECLSPEC_XFGVIRT(IAccDictionary, GetParentTerm)
        HRESULT ( STDMETHODCALLTYPE *GetParentTerm )( 
            __RPC__in IAccDictionary * This,
            /* [in] */ __RPC__in REFGUID Term,
            /* [out] */ __RPC__out GUID *pParentTerm);
        
        DECLSPEC_XFGVIRT(IAccDictionary, GetMnemonicString)
        HRESULT ( STDMETHODCALLTYPE *GetMnemonicString )( 
            __RPC__in IAccDictionary * This,
            /* [in] */ __RPC__in REFGUID Term,
            /* [out] */ __RPC__deref_out_opt BSTR *pResult);
        
        DECLSPEC_XFGVIRT(IAccDictionary, LookupMnemonicTerm)
        HRESULT ( STDMETHODCALLTYPE *LookupMnemonicTerm )( 
            __RPC__in IAccDictionary * This,
            /* [in] */ __RPC__in BSTR bstrMnemonic,
            /* [out] */ __RPC__out GUID *pTerm);
        
        DECLSPEC_XFGVIRT(IAccDictionary, ConvertValueToString)
        HRESULT ( STDMETHODCALLTYPE *ConvertValueToString )( 
            __RPC__in IAccDictionary * This,
            /* [in] */ __RPC__in REFGUID Term,
            /* [in] */ LCID lcid,
            /* [in] */ VARIANT varValue,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrResult,
            /* [out] */ __RPC__out LCID *plcid);
        
        END_INTERFACE
    } IAccDictionaryVtbl;

    interface IAccDictionary
    {
        CONST_VTBL struct IAccDictionaryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccDictionary_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccDictionary_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccDictionary_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccDictionary_GetLocalizedString(This,Term,lcid,pResult,plcid)	\
    ( (This)->lpVtbl -> GetLocalizedString(This,Term,lcid,pResult,plcid) ) 

#define IAccDictionary_GetParentTerm(This,Term,pParentTerm)	\
    ( (This)->lpVtbl -> GetParentTerm(This,Term,pParentTerm) ) 

#define IAccDictionary_GetMnemonicString(This,Term,pResult)	\
    ( (This)->lpVtbl -> GetMnemonicString(This,Term,pResult) ) 

#define IAccDictionary_LookupMnemonicTerm(This,bstrMnemonic,pTerm)	\
    ( (This)->lpVtbl -> LookupMnemonicTerm(This,bstrMnemonic,pTerm) ) 

#define IAccDictionary_ConvertValueToString(This,Term,lcid,varValue,pbstrResult,plcid)	\
    ( (This)->lpVtbl -> ConvertValueToString(This,Term,lcid,varValue,pbstrResult,plcid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccDictionary_INTERFACE_DEFINED__ */


#ifndef __IVersionInfo_INTERFACE_DEFINED__
#define __IVersionInfo_INTERFACE_DEFINED__

/* interface IVersionInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVersionInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("401518EC-DB00-4611-9B29-2A0E4B9AFA85")
    IVersionInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSubcomponentCount( 
            /* [in] */ ULONG ulSub,
            /* [out] */ __RPC__out ULONG *ulCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetImplementationID( 
            /* [in] */ ULONG ulSub,
            /* [out] */ __RPC__out GUID *implid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBuildVersion( 
            /* [in] */ ULONG ulSub,
            /* [out] */ __RPC__out DWORD *pdwMajor,
            /* [out] */ __RPC__out DWORD *pdwMinor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetComponentDescription( 
            /* [in] */ ULONG ulSub,
            /* [out] */ __RPC__deref_out_opt BSTR *pImplStr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInstanceDescription( 
            /* [in] */ ULONG ulSub,
            /* [out] */ __RPC__deref_out_opt BSTR *pImplStr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVersionInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVersionInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVersionInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVersionInfo * This);
        
        DECLSPEC_XFGVIRT(IVersionInfo, GetSubcomponentCount)
        HRESULT ( STDMETHODCALLTYPE *GetSubcomponentCount )( 
            __RPC__in IVersionInfo * This,
            /* [in] */ ULONG ulSub,
            /* [out] */ __RPC__out ULONG *ulCount);
        
        DECLSPEC_XFGVIRT(IVersionInfo, GetImplementationID)
        HRESULT ( STDMETHODCALLTYPE *GetImplementationID )( 
            __RPC__in IVersionInfo * This,
            /* [in] */ ULONG ulSub,
            /* [out] */ __RPC__out GUID *implid);
        
        DECLSPEC_XFGVIRT(IVersionInfo, GetBuildVersion)
        HRESULT ( STDMETHODCALLTYPE *GetBuildVersion )( 
            __RPC__in IVersionInfo * This,
            /* [in] */ ULONG ulSub,
            /* [out] */ __RPC__out DWORD *pdwMajor,
            /* [out] */ __RPC__out DWORD *pdwMinor);
        
        DECLSPEC_XFGVIRT(IVersionInfo, GetComponentDescription)
        HRESULT ( STDMETHODCALLTYPE *GetComponentDescription )( 
            __RPC__in IVersionInfo * This,
            /* [in] */ ULONG ulSub,
            /* [out] */ __RPC__deref_out_opt BSTR *pImplStr);
        
        DECLSPEC_XFGVIRT(IVersionInfo, GetInstanceDescription)
        HRESULT ( STDMETHODCALLTYPE *GetInstanceDescription )( 
            __RPC__in IVersionInfo * This,
            /* [in] */ ULONG ulSub,
            /* [out] */ __RPC__deref_out_opt BSTR *pImplStr);
        
        END_INTERFACE
    } IVersionInfoVtbl;

    interface IVersionInfo
    {
        CONST_VTBL struct IVersionInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVersionInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVersionInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVersionInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVersionInfo_GetSubcomponentCount(This,ulSub,ulCount)	\
    ( (This)->lpVtbl -> GetSubcomponentCount(This,ulSub,ulCount) ) 

#define IVersionInfo_GetImplementationID(This,ulSub,implid)	\
    ( (This)->lpVtbl -> GetImplementationID(This,ulSub,implid) ) 

#define IVersionInfo_GetBuildVersion(This,ulSub,pdwMajor,pdwMinor)	\
    ( (This)->lpVtbl -> GetBuildVersion(This,ulSub,pdwMajor,pdwMinor) ) 

#define IVersionInfo_GetComponentDescription(This,ulSub,pImplStr)	\
    ( (This)->lpVtbl -> GetComponentDescription(This,ulSub,pImplStr) ) 

#define IVersionInfo_GetInstanceDescription(This,ulSub,pImplStr)	\
    ( (This)->lpVtbl -> GetInstanceDescription(This,ulSub,pImplStr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVersionInfo_INTERFACE_DEFINED__ */


#ifndef __ICoCreateLocally_INTERFACE_DEFINED__
#define __ICoCreateLocally_INTERFACE_DEFINED__

/* interface ICoCreateLocally */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ICoCreateLocally;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03DE00AA-F272-41e3-99CB-03C5E8114EA0")
    ICoCreateLocally : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CoCreateLocally( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ DWORD dwClsContext,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **punk,
            /* [in] */ __RPC__in REFIID riidParam,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punkParam,
            /* [in] */ VARIANT varParam) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoCreateLocallyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICoCreateLocally * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICoCreateLocally * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICoCreateLocally * This);
        
        DECLSPEC_XFGVIRT(ICoCreateLocally, CoCreateLocally)
        HRESULT ( STDMETHODCALLTYPE *CoCreateLocally )( 
            __RPC__in ICoCreateLocally * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ DWORD dwClsContext,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **punk,
            /* [in] */ __RPC__in REFIID riidParam,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punkParam,
            /* [in] */ VARIANT varParam);
        
        END_INTERFACE
    } ICoCreateLocallyVtbl;

    interface ICoCreateLocally
    {
        CONST_VTBL struct ICoCreateLocallyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoCreateLocally_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoCreateLocally_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoCreateLocally_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoCreateLocally_CoCreateLocally(This,rclsid,dwClsContext,riid,punk,riidParam,punkParam,varParam)	\
    ( (This)->lpVtbl -> CoCreateLocally(This,rclsid,dwClsContext,riid,punk,riidParam,punkParam,varParam) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoCreateLocally_INTERFACE_DEFINED__ */


#ifndef __ICoCreatedLocally_INTERFACE_DEFINED__
#define __ICoCreatedLocally_INTERFACE_DEFINED__

/* interface ICoCreatedLocally */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ICoCreatedLocally;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0A53EB6C-1908-4742-8CFF-2CEE2E93F94C")
    ICoCreatedLocally : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE LocalInit( 
            /* [in] */ __RPC__in_opt IUnknown *punkLocalObject,
            /* [in] */ __RPC__in REFIID riidParam,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punkParam,
            /* [in] */ VARIANT varParam) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoCreatedLocallyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICoCreatedLocally * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICoCreatedLocally * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICoCreatedLocally * This);
        
        DECLSPEC_XFGVIRT(ICoCreatedLocally, LocalInit)
        HRESULT ( STDMETHODCALLTYPE *LocalInit )( 
            __RPC__in ICoCreatedLocally * This,
            /* [in] */ __RPC__in_opt IUnknown *punkLocalObject,
            /* [in] */ __RPC__in REFIID riidParam,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punkParam,
            /* [in] */ VARIANT varParam);
        
        END_INTERFACE
    } ICoCreatedLocallyVtbl;

    interface ICoCreatedLocally
    {
        CONST_VTBL struct ICoCreatedLocallyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoCreatedLocally_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoCreatedLocally_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoCreatedLocally_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoCreatedLocally_LocalInit(This,punkLocalObject,riidParam,punkParam,varParam)	\
    ( (This)->lpVtbl -> LocalInit(This,punkLocalObject,riidParam,punkParam,varParam) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoCreatedLocally_INTERFACE_DEFINED__ */


#ifndef __IAccStore_INTERFACE_DEFINED__
#define __IAccStore_INTERFACE_DEFINED__

/* interface IAccStore */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IAccStore;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E2CD4A63-2B72-4D48-B739-95E4765195BA")
    IAccStore : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Register( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unregister( 
            /* [in] */ __RPC__in_opt IUnknown *punk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDocuments( 
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **enumUnknown) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LookupByHWND( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LookupByPoint( 
            /* [in] */ POINT pt,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDocumentFocus( 
            /* [in] */ __RPC__in_opt IUnknown *punk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFocused( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccStoreVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAccStore * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAccStore * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAccStore * This);
        
        DECLSPEC_XFGVIRT(IAccStore, Register)
        HRESULT ( STDMETHODCALLTYPE *Register )( 
            __RPC__in IAccStore * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk);
        
        DECLSPEC_XFGVIRT(IAccStore, Unregister)
        HRESULT ( STDMETHODCALLTYPE *Unregister )( 
            __RPC__in IAccStore * This,
            /* [in] */ __RPC__in_opt IUnknown *punk);
        
        DECLSPEC_XFGVIRT(IAccStore, GetDocuments)
        HRESULT ( STDMETHODCALLTYPE *GetDocuments )( 
            __RPC__in IAccStore * This,
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **enumUnknown);
        
        DECLSPEC_XFGVIRT(IAccStore, LookupByHWND)
        HRESULT ( STDMETHODCALLTYPE *LookupByHWND )( 
            __RPC__in IAccStore * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(IAccStore, LookupByPoint)
        HRESULT ( STDMETHODCALLTYPE *LookupByPoint )( 
            __RPC__in IAccStore * This,
            /* [in] */ POINT pt,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(IAccStore, OnDocumentFocus)
        HRESULT ( STDMETHODCALLTYPE *OnDocumentFocus )( 
            __RPC__in IAccStore * This,
            /* [in] */ __RPC__in_opt IUnknown *punk);
        
        DECLSPEC_XFGVIRT(IAccStore, GetFocused)
        HRESULT ( STDMETHODCALLTYPE *GetFocused )( 
            __RPC__in IAccStore * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        END_INTERFACE
    } IAccStoreVtbl;

    interface IAccStore
    {
        CONST_VTBL struct IAccStoreVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccStore_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccStore_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccStore_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccStore_Register(This,riid,punk)	\
    ( (This)->lpVtbl -> Register(This,riid,punk) ) 

#define IAccStore_Unregister(This,punk)	\
    ( (This)->lpVtbl -> Unregister(This,punk) ) 

#define IAccStore_GetDocuments(This,enumUnknown)	\
    ( (This)->lpVtbl -> GetDocuments(This,enumUnknown) ) 

#define IAccStore_LookupByHWND(This,hWnd,riid,ppunk)	\
    ( (This)->lpVtbl -> LookupByHWND(This,hWnd,riid,ppunk) ) 

#define IAccStore_LookupByPoint(This,pt,riid,ppunk)	\
    ( (This)->lpVtbl -> LookupByPoint(This,pt,riid,ppunk) ) 

#define IAccStore_OnDocumentFocus(This,punk)	\
    ( (This)->lpVtbl -> OnDocumentFocus(This,punk) ) 

#define IAccStore_GetFocused(This,riid,ppunk)	\
    ( (This)->lpVtbl -> GetFocused(This,riid,ppunk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccStore_INTERFACE_DEFINED__ */


#ifndef __IAccServerDocMgr_INTERFACE_DEFINED__
#define __IAccServerDocMgr_INTERFACE_DEFINED__

/* interface IAccServerDocMgr */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IAccServerDocMgr;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AD7C73CF-6DD5-4855-ABC2-B04BAD5B9153")
    IAccServerDocMgr : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE NewDocument( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RevokeDocument( 
            /* [in] */ __RPC__in_opt IUnknown *punk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDocumentFocus( 
            /* [in] */ __RPC__in_opt IUnknown *punk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccServerDocMgrVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAccServerDocMgr * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAccServerDocMgr * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAccServerDocMgr * This);
        
        DECLSPEC_XFGVIRT(IAccServerDocMgr, NewDocument)
        HRESULT ( STDMETHODCALLTYPE *NewDocument )( 
            __RPC__in IAccServerDocMgr * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk);
        
        DECLSPEC_XFGVIRT(IAccServerDocMgr, RevokeDocument)
        HRESULT ( STDMETHODCALLTYPE *RevokeDocument )( 
            __RPC__in IAccServerDocMgr * This,
            /* [in] */ __RPC__in_opt IUnknown *punk);
        
        DECLSPEC_XFGVIRT(IAccServerDocMgr, OnDocumentFocus)
        HRESULT ( STDMETHODCALLTYPE *OnDocumentFocus )( 
            __RPC__in IAccServerDocMgr * This,
            /* [in] */ __RPC__in_opt IUnknown *punk);
        
        END_INTERFACE
    } IAccServerDocMgrVtbl;

    interface IAccServerDocMgr
    {
        CONST_VTBL struct IAccServerDocMgrVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccServerDocMgr_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccServerDocMgr_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccServerDocMgr_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccServerDocMgr_NewDocument(This,riid,punk)	\
    ( (This)->lpVtbl -> NewDocument(This,riid,punk) ) 

#define IAccServerDocMgr_RevokeDocument(This,punk)	\
    ( (This)->lpVtbl -> RevokeDocument(This,punk) ) 

#define IAccServerDocMgr_OnDocumentFocus(This,punk)	\
    ( (This)->lpVtbl -> OnDocumentFocus(This,punk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccServerDocMgr_INTERFACE_DEFINED__ */


#ifndef __IAccClientDocMgr_INTERFACE_DEFINED__
#define __IAccClientDocMgr_INTERFACE_DEFINED__

/* interface IAccClientDocMgr */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IAccClientDocMgr;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4C896039-7B6D-49e6-A8C1-45116A98292B")
    IAccClientDocMgr : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDocuments( 
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **enumUnknown) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LookupByHWND( 
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LookupByPoint( 
            /* [in] */ POINT pt,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFocused( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccClientDocMgrVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAccClientDocMgr * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAccClientDocMgr * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAccClientDocMgr * This);
        
        DECLSPEC_XFGVIRT(IAccClientDocMgr, GetDocuments)
        HRESULT ( STDMETHODCALLTYPE *GetDocuments )( 
            __RPC__in IAccClientDocMgr * This,
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **enumUnknown);
        
        DECLSPEC_XFGVIRT(IAccClientDocMgr, LookupByHWND)
        HRESULT ( STDMETHODCALLTYPE *LookupByHWND )( 
            __RPC__in IAccClientDocMgr * This,
            /* [in] */ __RPC__in HWND hWnd,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(IAccClientDocMgr, LookupByPoint)
        HRESULT ( STDMETHODCALLTYPE *LookupByPoint )( 
            __RPC__in IAccClientDocMgr * This,
            /* [in] */ POINT pt,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(IAccClientDocMgr, GetFocused)
        HRESULT ( STDMETHODCALLTYPE *GetFocused )( 
            __RPC__in IAccClientDocMgr * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        END_INTERFACE
    } IAccClientDocMgrVtbl;

    interface IAccClientDocMgr
    {
        CONST_VTBL struct IAccClientDocMgrVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccClientDocMgr_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccClientDocMgr_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccClientDocMgr_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccClientDocMgr_GetDocuments(This,enumUnknown)	\
    ( (This)->lpVtbl -> GetDocuments(This,enumUnknown) ) 

#define IAccClientDocMgr_LookupByHWND(This,hWnd,riid,ppunk)	\
    ( (This)->lpVtbl -> LookupByHWND(This,hWnd,riid,ppunk) ) 

#define IAccClientDocMgr_LookupByPoint(This,pt,riid,ppunk)	\
    ( (This)->lpVtbl -> LookupByPoint(This,pt,riid,ppunk) ) 

#define IAccClientDocMgr_GetFocused(This,riid,ppunk)	\
    ( (This)->lpVtbl -> GetFocused(This,riid,ppunk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccClientDocMgr_INTERFACE_DEFINED__ */


#ifndef __IDocWrap_INTERFACE_DEFINED__
#define __IDocWrap_INTERFACE_DEFINED__

/* interface IDocWrap */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDocWrap;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCD285FE-0BE0-43BD-99C9-AAAEC513C555")
    IDocWrap : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDoc( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWrappedDoc( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDocWrapVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDocWrap * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDocWrap * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDocWrap * This);
        
        DECLSPEC_XFGVIRT(IDocWrap, SetDoc)
        HRESULT ( STDMETHODCALLTYPE *SetDoc )( 
            __RPC__in IDocWrap * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk);
        
        DECLSPEC_XFGVIRT(IDocWrap, GetWrappedDoc)
        HRESULT ( STDMETHODCALLTYPE *GetWrappedDoc )( 
            __RPC__in IDocWrap * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        END_INTERFACE
    } IDocWrapVtbl;

    interface IDocWrap
    {
        CONST_VTBL struct IDocWrapVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDocWrap_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDocWrap_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDocWrap_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDocWrap_SetDoc(This,riid,punk)	\
    ( (This)->lpVtbl -> SetDoc(This,riid,punk) ) 

#define IDocWrap_GetWrappedDoc(This,riid,ppunk)	\
    ( (This)->lpVtbl -> GetWrappedDoc(This,riid,ppunk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDocWrap_INTERFACE_DEFINED__ */


#ifndef __IClonableWrapper_INTERFACE_DEFINED__
#define __IClonableWrapper_INTERFACE_DEFINED__

/* interface IClonableWrapper */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IClonableWrapper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B33E75FF-E84C-4dca-A25C-33B8DC003374")
    IClonableWrapper : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CloneNewWrapper( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IClonableWrapperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IClonableWrapper * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IClonableWrapper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IClonableWrapper * This);
        
        DECLSPEC_XFGVIRT(IClonableWrapper, CloneNewWrapper)
        HRESULT ( STDMETHODCALLTYPE *CloneNewWrapper )( 
            __RPC__in IClonableWrapper * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        END_INTERFACE
    } IClonableWrapperVtbl;

    interface IClonableWrapper
    {
        CONST_VTBL struct IClonableWrapperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IClonableWrapper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IClonableWrapper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IClonableWrapper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IClonableWrapper_CloneNewWrapper(This,riid,ppv)	\
    ( (This)->lpVtbl -> CloneNewWrapper(This,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IClonableWrapper_INTERFACE_DEFINED__ */



#ifndef __MSAATEXTLib_LIBRARY_DEFINED__
#define __MSAATEXTLib_LIBRARY_DEFINED__

/* library MSAATEXTLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_MSAATEXTLib;

EXTERN_C const CLSID CLSID_MSAAControl;

#ifdef __cplusplus

class DECLSPEC_UUID("08cd963f-7a3e-4f5c-9bd8-d692bb043c5b")
MSAAControl;
#endif

EXTERN_C const CLSID CLSID_AccStore;

#ifdef __cplusplus

class DECLSPEC_UUID("5440837F-4BFF-4AE5-A1B1-7722ECC6332A")
AccStore;
#endif

EXTERN_C const CLSID CLSID_AccDictionary;

#ifdef __cplusplus

class DECLSPEC_UUID("6572EE16-5FE5-4331-BB6D-76A49C56E423")
AccDictionary;
#endif

EXTERN_C const CLSID CLSID_AccServerDocMgr;

#ifdef __cplusplus

class DECLSPEC_UUID("6089A37E-EB8A-482D-BD6F-F9F46904D16D")
AccServerDocMgr;
#endif

EXTERN_C const CLSID CLSID_AccClientDocMgr;

#ifdef __cplusplus

class DECLSPEC_UUID("FC48CC30-4F3E-4fa1-803B-AD0E196A83B1")
AccClientDocMgr;
#endif

EXTERN_C const CLSID CLSID_DocWrap;

#ifdef __cplusplus

class DECLSPEC_UUID("BF426F7E-7A5E-44D6-830C-A390EA9462A3")
DocWrap;
#endif
#endif /* __MSAATEXTLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_msaatext_0000_0016 */
/* [local] */ 

DEFINE_GUID(IID_ITextStoreACP,               0x28888fe3,0xc2a0,0x483a,0xa3,0xea,0x8c,0xb1,0xce,0x51,0xff,0x3d);
DEFINE_GUID(IID_ITextStoreAnchor,            0x9b2077b0,0x5f18,0x4dec,0xbe,0xe9,0x3c,0xc7,0x22,0xf5,0xdf,0xe0);
DEFINE_GUID(IID_IAnchor,                     0x0feb7e34,0x5a60,0x4356,0x8e,0xf7,0xab,0xde,0xc2,0xff,0x7c,0xf8);
DEFINE_GUID(IID_ITextStoreAnchorSink,        0xaa80e905,0x2021,0x11d2,0x93,0xe0,0x00,0x60,0xb0,0x67,0xb8,0x6e);
DEFINE_GUID(IID_ITextStoreACPSink,           0x22d44c94,0xa419,0x4542,0xa2,0x72,0xae,0x26,0x09,0x3e,0xce,0xcf);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_msaatext_0000_0016_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msaatext_0000_0016_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


