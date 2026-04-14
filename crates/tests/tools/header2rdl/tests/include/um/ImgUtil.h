

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

#ifndef __imgutil_h__
#define __imgutil_h__

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

#ifndef __ISniffStream_FWD_DEFINED__
#define __ISniffStream_FWD_DEFINED__
typedef interface ISniffStream ISniffStream;

#endif 	/* __ISniffStream_FWD_DEFINED__ */


#ifndef __IDithererImpl_FWD_DEFINED__
#define __IDithererImpl_FWD_DEFINED__
typedef interface IDithererImpl IDithererImpl;

#endif 	/* __IDithererImpl_FWD_DEFINED__ */


#ifndef __CoDitherToRGB8_FWD_DEFINED__
#define __CoDitherToRGB8_FWD_DEFINED__

#ifdef __cplusplus
typedef class CoDitherToRGB8 CoDitherToRGB8;
#else
typedef struct CoDitherToRGB8 CoDitherToRGB8;
#endif /* __cplusplus */

#endif 	/* __CoDitherToRGB8_FWD_DEFINED__ */


#ifndef __CoSniffStream_FWD_DEFINED__
#define __CoSniffStream_FWD_DEFINED__

#ifdef __cplusplus
typedef class CoSniffStream CoSniffStream;
#else
typedef struct CoSniffStream CoSniffStream;
#endif /* __cplusplus */

#endif 	/* __CoSniffStream_FWD_DEFINED__ */


#ifndef __CoMapMIMEToCLSID_FWD_DEFINED__
#define __CoMapMIMEToCLSID_FWD_DEFINED__

#ifdef __cplusplus
typedef class CoMapMIMEToCLSID CoMapMIMEToCLSID;
#else
typedef struct CoMapMIMEToCLSID CoMapMIMEToCLSID;
#endif /* __cplusplus */

#endif 	/* __CoMapMIMEToCLSID_FWD_DEFINED__ */


/* header files for imported files */
#include "ocmm.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_imgutil_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// imgutil.h
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
#include <ddraw.h>

STDAPI CreateMIMEMap( IMapMIMEToCLSID** ppMap );
STDAPI DecodeImage( IStream* pStream, IMapMIMEToCLSID* pMap,
   IUnknown* pEventSink );
STDAPI SniffStream( IStream* pInStream, UINT* pnFormat,
   IStream** ppOutStream );
STDAPI GetMaxMIMEIDBytes( ULONG* pnMaxBytes );
STDAPI IdentifyMIMEType( const BYTE* pbBytes, ULONG nBytes,
   UINT* pnFormat );
STDAPI ComputeInvCMAP(const RGBQUAD *pRGBColors, ULONG nColors, BYTE *pInvTable,
   ULONG cbTable);
STDAPI DitherTo8( BYTE * pDestBits, LONG nDestPitch, BYTE * pSrcBits, LONG nSrcPitch,
   REFGUID bfidSrc, RGBQUAD * prgbDestColors, RGBQUAD * prgbSrcColors, BYTE * pbDestInvMap,
   LONG x, LONG y, LONG cx, LONG cy, LONG lDestTrans, LONG lSrcTrans);
STDAPI CreateDDrawSurfaceOnDIB(HBITMAP hbmDib, IDirectDrawSurface **ppSurface);
STDAPI DecodeImageEx( IStream* pStream, IMapMIMEToCLSID* pMap,
   IUnknown* pEventSink, LPCWSTR pszMIMETypeParam );


extern RPC_IF_HANDLE __MIDL_itf_imgutil_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imgutil_0000_0000_v0_0_s_ifspec;

#ifndef __ISniffStream_INTERFACE_DEFINED__
#define __ISniffStream_INTERFACE_DEFINED__

/* interface ISniffStream */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_ISniffStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4EF17940-30E0-11d0-B724-00AA006C1A01")
    ISniffStream : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Init( 
            IStream *pStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Peek( 
            void *pBuffer,
            ULONG nBytes,
            ULONG *pnBytesRead) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISniffStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISniffStream * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISniffStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISniffStream * This);
        
        DECLSPEC_XFGVIRT(ISniffStream, Init)
        HRESULT ( STDMETHODCALLTYPE *Init )( 
            ISniffStream * This,
            IStream *pStream);
        
        DECLSPEC_XFGVIRT(ISniffStream, Peek)
        HRESULT ( STDMETHODCALLTYPE *Peek )( 
            ISniffStream * This,
            void *pBuffer,
            ULONG nBytes,
            ULONG *pnBytesRead);
        
        END_INTERFACE
    } ISniffStreamVtbl;

    interface ISniffStream
    {
        CONST_VTBL struct ISniffStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISniffStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISniffStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISniffStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISniffStream_Init(This,pStream)	\
    ( (This)->lpVtbl -> Init(This,pStream) ) 

#define ISniffStream_Peek(This,pBuffer,nBytes,pnBytesRead)	\
    ( (This)->lpVtbl -> Peek(This,pBuffer,nBytes,pnBytesRead) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISniffStream_INTERFACE_DEFINED__ */


#ifndef __IDithererImpl_INTERFACE_DEFINED__
#define __IDithererImpl_INTERFACE_DEFINED__

/* interface IDithererImpl */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IDithererImpl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7C48E840-3910-11d0-86FC-00A0C913F750")
    IDithererImpl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDestColorTable( 
            ULONG nColors,
            const RGBQUAD *prgbColors) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEventSink( 
            IImageDecodeEventSink *pEventSink) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDithererImplVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDithererImpl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDithererImpl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDithererImpl * This);
        
        DECLSPEC_XFGVIRT(IDithererImpl, SetDestColorTable)
        HRESULT ( STDMETHODCALLTYPE *SetDestColorTable )( 
            IDithererImpl * This,
            ULONG nColors,
            const RGBQUAD *prgbColors);
        
        DECLSPEC_XFGVIRT(IDithererImpl, SetEventSink)
        HRESULT ( STDMETHODCALLTYPE *SetEventSink )( 
            IDithererImpl * This,
            IImageDecodeEventSink *pEventSink);
        
        END_INTERFACE
    } IDithererImplVtbl;

    interface IDithererImpl
    {
        CONST_VTBL struct IDithererImplVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDithererImpl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDithererImpl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDithererImpl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDithererImpl_SetDestColorTable(This,nColors,prgbColors)	\
    ( (This)->lpVtbl -> SetDestColorTable(This,nColors,prgbColors) ) 

#define IDithererImpl_SetEventSink(This,pEventSink)	\
    ( (This)->lpVtbl -> SetEventSink(This,pEventSink) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDithererImpl_INTERFACE_DEFINED__ */



#ifndef __ImgUtilLib_LIBRARY_DEFINED__
#define __ImgUtilLib_LIBRARY_DEFINED__

/* library ImgUtilLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_ImgUtilLib;

EXTERN_C const CLSID CLSID_CoDitherToRGB8;

#ifdef __cplusplus

class DECLSPEC_UUID("A860CE50-3910-11d0-86FC-00A0C913F750")
CoDitherToRGB8;
#endif

EXTERN_C const CLSID CLSID_CoSniffStream;

#ifdef __cplusplus

class DECLSPEC_UUID("6A01FDA0-30DF-11d0-B724-00AA006C1A01")
CoSniffStream;
#endif

EXTERN_C const CLSID CLSID_CoMapMIMEToCLSID;

#ifdef __cplusplus

class DECLSPEC_UUID("30C3B080-30FB-11d0-B724-00AA006C1A01")
CoMapMIMEToCLSID;
#endif
#endif /* __ImgUtilLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_imgutil_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_imgutil_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imgutil_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


