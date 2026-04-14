

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

#ifndef __filter_h__
#define __filter_h__

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

#ifndef __IFilter_FWD_DEFINED__
#define __IFilter_FWD_DEFINED__
typedef interface IFilter IFilter;

#endif 	/* __IFilter_FWD_DEFINED__ */


#ifndef __IPixelFilter_FWD_DEFINED__
#define __IPixelFilter_FWD_DEFINED__
typedef interface IPixelFilter IPixelFilter;

#endif 	/* __IPixelFilter_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "objidl.h"
#include "propidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_filter_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 1993-1998.
//
//--------------------------------------------------------------------------
#if !defined(_TAGFULLPROPSPEC_DEFINED_)
#define _TAGFULLPROPSPEC_DEFINED_
typedef struct tagFULLPROPSPEC
    {
    GUID guidPropSet;
    PROPSPEC psProperty;
    } 	FULLPROPSPEC;

#endif // #if !defined(_TAGFULLPROPSPEC_DEFINED_)


extern RPC_IF_HANDLE __MIDL_itf_filter_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_filter_0000_0000_v0_0_s_ifspec;

#ifndef __IFilter_INTERFACE_DEFINED__
#define __IFilter_INTERFACE_DEFINED__

/* interface IFilter */
/* [unique][uuid][object][local] */ 

#ifndef _tagIFILTER_INIT_DEFINED
typedef 
enum tagIFILTER_INIT
    {
        IFILTER_INIT_CANON_PARAGRAPHS	= 1,
        IFILTER_INIT_HARD_LINE_BREAKS	= 2,
        IFILTER_INIT_CANON_HYPHENS	= 4,
        IFILTER_INIT_CANON_SPACES	= 8,
        IFILTER_INIT_APPLY_INDEX_ATTRIBUTES	= 16,
        IFILTER_INIT_APPLY_OTHER_ATTRIBUTES	= 32,
        IFILTER_INIT_APPLY_CRAWL_ATTRIBUTES	= 256,
        IFILTER_INIT_INDEXING_ONLY	= 64,
        IFILTER_INIT_SEARCH_LINKS	= 128,
        IFILTER_INIT_FILTER_OWNED_VALUE_OK	= 512,
        IFILTER_INIT_FILTER_AGGRESSIVE_BREAK	= 1024,
        IFILTER_INIT_DISABLE_EMBEDDED	= 2048,
        IFILTER_INIT_EMIT_FORMATTING	= 4096
    } 	IFILTER_INIT;

#define _tagIFILTER_INIT_DEFINED
#define _IFILTER_INIT_DEFINED
#endif
#ifndef _tagIFILTER_FLAGS_DEFINED
typedef 
enum tagIFILTER_FLAGS
    {
        IFILTER_FLAGS_OLE_PROPERTIES	= 1
    } 	IFILTER_FLAGS;

#define _tagIFILTER_FLAGS_DEFINED
#define _IFILTER_FLAGS_DEFINED
#endif
#ifndef _tagCHUNKSTATE_DEFINED
typedef 
enum tagCHUNKSTATE
    {
        CHUNK_TEXT	= 0x1,
        CHUNK_VALUE	= 0x2,
        CHUNK_FILTER_OWNED_VALUE	= 0x4,
        CHUNK_IMAGE	= 0x8
    } 	CHUNKSTATE;

#define _tagCHUNKSTATE_DEFINED
#define _CHUNKSTATE_DEFINED
#endif
#ifndef _tagCHUNK_BREAKTYPE_DEFINED
typedef 
enum tagCHUNK_BREAKTYPE
    {
        CHUNK_NO_BREAK	= 0,
        CHUNK_EOW	= 1,
        CHUNK_EOS	= 2,
        CHUNK_EOP	= 3,
        CHUNK_EOC	= 4
    } 	CHUNK_BREAKTYPE;

#define _tagCHUNK_BREAKTYPE_DEFINED
#define _CHUNK_BREAKTYPE_DEFINED
#endif
#ifndef _tagFILTERREGION_DEFINED
typedef struct tagFILTERREGION
    {
    ULONG idChunk;
    ULONG cwcStart;
    ULONG cwcExtent;
    } 	FILTERREGION;

#define _tagFILTERREGION_DEFINED
#define _FILTERREGION_DEFINED
#endif
#ifndef _tagSTAT_CHUNK_DEFINED
typedef struct tagSTAT_CHUNK
    {
    ULONG idChunk;
    CHUNK_BREAKTYPE breakType;
    CHUNKSTATE flags;
    LCID locale;
    FULLPROPSPEC attribute;
    ULONG idChunkSource;
    ULONG cwcStartSource;
    ULONG cwcLenSource;
    } 	STAT_CHUNK;

#define _tagSTAT_CHUNK_DEFINED
#define _STAT_CHUNK_DEFINED
#endif

EXTERN_C const IID IID_IFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("89BCB740-6119-101A-BCB7-00DD010655AF")
    IFilter : public IUnknown
    {
    public:
        virtual SCODE STDMETHODCALLTYPE Init( 
            /* [in] */ ULONG grfFlags,
            /* [in] */ ULONG cAttributes,
            /* [size_is][in] */ const FULLPROPSPEC *aAttributes,
            /* [out] */ ULONG *pFlags) = 0;
        
        virtual SCODE STDMETHODCALLTYPE GetChunk( 
            /* [out] */ STAT_CHUNK *pStat) = 0;
        
        virtual SCODE STDMETHODCALLTYPE GetText( 
            /* [out][in] */ ULONG *pcwcBuffer,
            /* [size_is][out] */ WCHAR *awcBuffer) = 0;
        
        virtual SCODE STDMETHODCALLTYPE GetValue( 
            /* [out] */ PROPVARIANT **ppPropValue) = 0;
        
        virtual SCODE STDMETHODCALLTYPE BindRegion( 
            /* [in] */ FILTERREGION origPos,
            /* [in] */ REFIID riid,
            /* [out] */ void **ppunk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IFilter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IFilter * This);
        
        DECLSPEC_XFGVIRT(IFilter, Init)
        SCODE ( STDMETHODCALLTYPE *Init )( 
            IFilter * This,
            /* [in] */ ULONG grfFlags,
            /* [in] */ ULONG cAttributes,
            /* [size_is][in] */ const FULLPROPSPEC *aAttributes,
            /* [out] */ ULONG *pFlags);
        
        DECLSPEC_XFGVIRT(IFilter, GetChunk)
        SCODE ( STDMETHODCALLTYPE *GetChunk )( 
            IFilter * This,
            /* [out] */ STAT_CHUNK *pStat);
        
        DECLSPEC_XFGVIRT(IFilter, GetText)
        SCODE ( STDMETHODCALLTYPE *GetText )( 
            IFilter * This,
            /* [out][in] */ ULONG *pcwcBuffer,
            /* [size_is][out] */ WCHAR *awcBuffer);
        
        DECLSPEC_XFGVIRT(IFilter, GetValue)
        SCODE ( STDMETHODCALLTYPE *GetValue )( 
            IFilter * This,
            /* [out] */ PROPVARIANT **ppPropValue);
        
        DECLSPEC_XFGVIRT(IFilter, BindRegion)
        SCODE ( STDMETHODCALLTYPE *BindRegion )( 
            IFilter * This,
            /* [in] */ FILTERREGION origPos,
            /* [in] */ REFIID riid,
            /* [out] */ void **ppunk);
        
        END_INTERFACE
    } IFilterVtbl;

    interface IFilter
    {
        CONST_VTBL struct IFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFilter_Init(This,grfFlags,cAttributes,aAttributes,pFlags)	\
    ( (This)->lpVtbl -> Init(This,grfFlags,cAttributes,aAttributes,pFlags) ) 

#define IFilter_GetChunk(This,pStat)	\
    ( (This)->lpVtbl -> GetChunk(This,pStat) ) 

#define IFilter_GetText(This,pcwcBuffer,awcBuffer)	\
    ( (This)->lpVtbl -> GetText(This,pcwcBuffer,awcBuffer) ) 

#define IFilter_GetValue(This,ppPropValue)	\
    ( (This)->lpVtbl -> GetValue(This,ppPropValue) ) 

#define IFilter_BindRegion(This,origPos,riid,ppunk)	\
    ( (This)->lpVtbl -> BindRegion(This,origPos,riid,ppunk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFilter_INTERFACE_DEFINED__ */


#ifndef __IPixelFilter_INTERFACE_DEFINED__
#define __IPixelFilter_INTERFACE_DEFINED__

/* interface IPixelFilter */
/* [unique][uuid][object] */ 

typedef 
enum IMAGE_PIXELFORMAT
    {
        FILTER_PIXELFORMAT_BGRA8	= 0,
        FILTER_PIXELFORMAT_PBGRA8	= 1,
        FILTER_PIXELFORMAT_BGR8	= 2
    } 	IMAGE_PIXELFORMAT;

typedef struct IMAGE_INFO
    {
    UINT Width;
    UINT Height;
    IMAGE_PIXELFORMAT Format;
    } 	IMAGE_INFO;


EXTERN_C const IID IID_IPixelFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3d7df9a7-8da6-4fbf-a45b-7592f06d93a9")
    IPixelFilter : public IFilter
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetImageInfo( 
            /* [out] */ __RPC__out IMAGE_INFO *imageInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPixelsForImage( 
            /* [in] */ FLOAT scalingFactor,
            /* [unique][in] */ __RPC__in_opt const RECT *sourceRect,
            /* [in] */ ULONG pixelBufferSize,
            /* [size_is][out] */ __RPC__out_ecount_full(pixelBufferSize) BYTE *pixelBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPixelFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPixelFilter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPixelFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPixelFilter * This);
        
        DECLSPEC_XFGVIRT(IFilter, Init)
        SCODE ( STDMETHODCALLTYPE *Init )( 
            __RPC__in IPixelFilter * This,
            /* [in] */ ULONG grfFlags,
            /* [in] */ ULONG cAttributes,
            /* [size_is][in] */ __RPC__in_ecount_full(cAttributes) const FULLPROPSPEC *aAttributes,
            /* [out] */ __RPC__out ULONG *pFlags);
        
        DECLSPEC_XFGVIRT(IFilter, GetChunk)
        SCODE ( STDMETHODCALLTYPE *GetChunk )( 
            __RPC__in IPixelFilter * This,
            /* [out] */ __RPC__out STAT_CHUNK *pStat);
        
        DECLSPEC_XFGVIRT(IFilter, GetText)
        SCODE ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in IPixelFilter * This,
            /* [out][in] */ __RPC__inout ULONG *pcwcBuffer,
            /* [size_is][out] */ __RPC__out_ecount_full(*pcwcBuffer) WCHAR *awcBuffer);
        
        DECLSPEC_XFGVIRT(IFilter, GetValue)
        SCODE ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in IPixelFilter * This,
            /* [out] */ __RPC__deref_out_opt PROPVARIANT **ppPropValue);
        
        DECLSPEC_XFGVIRT(IFilter, BindRegion)
        SCODE ( STDMETHODCALLTYPE *BindRegion )( 
            __RPC__in IPixelFilter * This,
            /* [in] */ FILTERREGION origPos,
            /* [in] */ __RPC__in REFIID riid,
            /* [out] */ __RPC__deref_out_opt void **ppunk);
        
        DECLSPEC_XFGVIRT(IPixelFilter, GetImageInfo)
        HRESULT ( STDMETHODCALLTYPE *GetImageInfo )( 
            __RPC__in IPixelFilter * This,
            /* [out] */ __RPC__out IMAGE_INFO *imageInfo);
        
        DECLSPEC_XFGVIRT(IPixelFilter, GetPixelsForImage)
        HRESULT ( STDMETHODCALLTYPE *GetPixelsForImage )( 
            __RPC__in IPixelFilter * This,
            /* [in] */ FLOAT scalingFactor,
            /* [unique][in] */ __RPC__in_opt const RECT *sourceRect,
            /* [in] */ ULONG pixelBufferSize,
            /* [size_is][out] */ __RPC__out_ecount_full(pixelBufferSize) BYTE *pixelBuffer);
        
        END_INTERFACE
    } IPixelFilterVtbl;

    interface IPixelFilter
    {
        CONST_VTBL struct IPixelFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPixelFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPixelFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPixelFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPixelFilter_Init(This,grfFlags,cAttributes,aAttributes,pFlags)	\
    ( (This)->lpVtbl -> Init(This,grfFlags,cAttributes,aAttributes,pFlags) ) 

#define IPixelFilter_GetChunk(This,pStat)	\
    ( (This)->lpVtbl -> GetChunk(This,pStat) ) 

#define IPixelFilter_GetText(This,pcwcBuffer,awcBuffer)	\
    ( (This)->lpVtbl -> GetText(This,pcwcBuffer,awcBuffer) ) 

#define IPixelFilter_GetValue(This,ppPropValue)	\
    ( (This)->lpVtbl -> GetValue(This,ppPropValue) ) 

#define IPixelFilter_BindRegion(This,origPos,riid,ppunk)	\
    ( (This)->lpVtbl -> BindRegion(This,origPos,riid,ppunk) ) 


#define IPixelFilter_GetImageInfo(This,imageInfo)	\
    ( (This)->lpVtbl -> GetImageInfo(This,imageInfo) ) 

#define IPixelFilter_GetPixelsForImage(This,scalingFactor,sourceRect,pixelBufferSize,pixelBuffer)	\
    ( (This)->lpVtbl -> GetPixelsForImage(This,scalingFactor,sourceRect,pixelBufferSize,pixelBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPixelFilter_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_filter_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_filter_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_filter_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


