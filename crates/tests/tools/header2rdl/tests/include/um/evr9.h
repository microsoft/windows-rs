

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

#ifndef __evr9_h__
#define __evr9_h__

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

#ifndef __IEVRVideoStreamControl_FWD_DEFINED__
#define __IEVRVideoStreamControl_FWD_DEFINED__
typedef interface IEVRVideoStreamControl IEVRVideoStreamControl;

#endif 	/* __IEVRVideoStreamControl_FWD_DEFINED__ */


#ifndef __IMFVideoProcessor_FWD_DEFINED__
#define __IMFVideoProcessor_FWD_DEFINED__
typedef interface IMFVideoProcessor IMFVideoProcessor;

#endif 	/* __IMFVideoProcessor_FWD_DEFINED__ */


#ifndef __IMFVideoMixerBitmap_FWD_DEFINED__
#define __IMFVideoMixerBitmap_FWD_DEFINED__
typedef interface IMFVideoMixerBitmap IMFVideoMixerBitmap;

#endif 	/* __IMFVideoMixerBitmap_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "mfobjects.h"
#include "mftransform.h"
#include "evr.h"
#include "dxva2api.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_evr9_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)




extern RPC_IF_HANDLE __MIDL_itf_evr9_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_evr9_0000_0000_v0_0_s_ifspec;

#ifndef __IEVRVideoStreamControl_INTERFACE_DEFINED__
#define __IEVRVideoStreamControl_INTERFACE_DEFINED__

/* interface IEVRVideoStreamControl */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IEVRVideoStreamControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d0cfe38b-93e7-4772-8957-0400c49a4485")
    IEVRVideoStreamControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetStreamActiveState( 
            /* [in] */ BOOL fActive) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamActiveState( 
            /* [annotation][out] */ 
            _Out_  BOOL *lpfActive) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEVRVideoStreamControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEVRVideoStreamControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEVRVideoStreamControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEVRVideoStreamControl * This);
        
        DECLSPEC_XFGVIRT(IEVRVideoStreamControl, SetStreamActiveState)
        HRESULT ( STDMETHODCALLTYPE *SetStreamActiveState )( 
            IEVRVideoStreamControl * This,
            /* [in] */ BOOL fActive);
        
        DECLSPEC_XFGVIRT(IEVRVideoStreamControl, GetStreamActiveState)
        HRESULT ( STDMETHODCALLTYPE *GetStreamActiveState )( 
            IEVRVideoStreamControl * This,
            /* [annotation][out] */ 
            _Out_  BOOL *lpfActive);
        
        END_INTERFACE
    } IEVRVideoStreamControlVtbl;

    interface IEVRVideoStreamControl
    {
        CONST_VTBL struct IEVRVideoStreamControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEVRVideoStreamControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEVRVideoStreamControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEVRVideoStreamControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEVRVideoStreamControl_SetStreamActiveState(This,fActive)	\
    ( (This)->lpVtbl -> SetStreamActiveState(This,fActive) ) 

#define IEVRVideoStreamControl_GetStreamActiveState(This,lpfActive)	\
    ( (This)->lpVtbl -> GetStreamActiveState(This,lpfActive) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEVRVideoStreamControl_INTERFACE_DEFINED__ */


#ifndef __IMFVideoProcessor_INTERFACE_DEFINED__
#define __IMFVideoProcessor_INTERFACE_DEFINED__

/* interface IMFVideoProcessor */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFVideoProcessor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6AB0000C-FECE-4d1f-A2AC-A9573530656E")
    IMFVideoProcessor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAvailableVideoProcessorModes( 
            /* [out][in] */ __RPC__inout UINT *lpdwNumProcessingModes,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*lpdwNumProcessingModes) GUID **ppVideoProcessingModes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVideoProcessorCaps( 
            /* [in] */ __RPC__in LPGUID lpVideoProcessorMode,
            /* [out] */ __RPC__out DXVA2_VideoProcessorCaps *lpVideoProcessorCaps) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVideoProcessorMode( 
            /* [out] */ __RPC__out LPGUID lpMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVideoProcessorMode( 
            /* [in] */ __RPC__in LPGUID lpMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProcAmpRange( 
            DWORD dwProperty,
            /* [out] */ __RPC__out DXVA2_ValueRange *pPropRange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProcAmpValues( 
            DWORD dwFlags,
            /* [out] */ __RPC__out DXVA2_ProcAmpValues *Values) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProcAmpValues( 
            DWORD dwFlags,
            /* [in] */ __RPC__in DXVA2_ProcAmpValues *pValues) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFilteringRange( 
            DWORD dwProperty,
            /* [out] */ __RPC__out DXVA2_ValueRange *pPropRange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFilteringValue( 
            DWORD dwProperty,
            /* [out] */ __RPC__out DXVA2_Fixed32 *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFilteringValue( 
            DWORD dwProperty,
            /* [in] */ __RPC__in DXVA2_Fixed32 *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBackgroundColor( 
            /* [out] */ __RPC__out COLORREF *lpClrBkg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBackgroundColor( 
            COLORREF ClrBkg) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoProcessorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFVideoProcessor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFVideoProcessor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFVideoProcessor * This);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessor, GetAvailableVideoProcessorModes)
        HRESULT ( STDMETHODCALLTYPE *GetAvailableVideoProcessorModes )( 
            __RPC__in IMFVideoProcessor * This,
            /* [out][in] */ __RPC__inout UINT *lpdwNumProcessingModes,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*lpdwNumProcessingModes) GUID **ppVideoProcessingModes);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessor, GetVideoProcessorCaps)
        HRESULT ( STDMETHODCALLTYPE *GetVideoProcessorCaps )( 
            __RPC__in IMFVideoProcessor * This,
            /* [in] */ __RPC__in LPGUID lpVideoProcessorMode,
            /* [out] */ __RPC__out DXVA2_VideoProcessorCaps *lpVideoProcessorCaps);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessor, GetVideoProcessorMode)
        HRESULT ( STDMETHODCALLTYPE *GetVideoProcessorMode )( 
            __RPC__in IMFVideoProcessor * This,
            /* [out] */ __RPC__out LPGUID lpMode);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessor, SetVideoProcessorMode)
        HRESULT ( STDMETHODCALLTYPE *SetVideoProcessorMode )( 
            __RPC__in IMFVideoProcessor * This,
            /* [in] */ __RPC__in LPGUID lpMode);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessor, GetProcAmpRange)
        HRESULT ( STDMETHODCALLTYPE *GetProcAmpRange )( 
            __RPC__in IMFVideoProcessor * This,
            DWORD dwProperty,
            /* [out] */ __RPC__out DXVA2_ValueRange *pPropRange);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessor, GetProcAmpValues)
        HRESULT ( STDMETHODCALLTYPE *GetProcAmpValues )( 
            __RPC__in IMFVideoProcessor * This,
            DWORD dwFlags,
            /* [out] */ __RPC__out DXVA2_ProcAmpValues *Values);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessor, SetProcAmpValues)
        HRESULT ( STDMETHODCALLTYPE *SetProcAmpValues )( 
            __RPC__in IMFVideoProcessor * This,
            DWORD dwFlags,
            /* [in] */ __RPC__in DXVA2_ProcAmpValues *pValues);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessor, GetFilteringRange)
        HRESULT ( STDMETHODCALLTYPE *GetFilteringRange )( 
            __RPC__in IMFVideoProcessor * This,
            DWORD dwProperty,
            /* [out] */ __RPC__out DXVA2_ValueRange *pPropRange);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessor, GetFilteringValue)
        HRESULT ( STDMETHODCALLTYPE *GetFilteringValue )( 
            __RPC__in IMFVideoProcessor * This,
            DWORD dwProperty,
            /* [out] */ __RPC__out DXVA2_Fixed32 *pValue);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessor, SetFilteringValue)
        HRESULT ( STDMETHODCALLTYPE *SetFilteringValue )( 
            __RPC__in IMFVideoProcessor * This,
            DWORD dwProperty,
            /* [in] */ __RPC__in DXVA2_Fixed32 *pValue);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessor, GetBackgroundColor)
        HRESULT ( STDMETHODCALLTYPE *GetBackgroundColor )( 
            __RPC__in IMFVideoProcessor * This,
            /* [out] */ __RPC__out COLORREF *lpClrBkg);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessor, SetBackgroundColor)
        HRESULT ( STDMETHODCALLTYPE *SetBackgroundColor )( 
            __RPC__in IMFVideoProcessor * This,
            COLORREF ClrBkg);
        
        END_INTERFACE
    } IMFVideoProcessorVtbl;

    interface IMFVideoProcessor
    {
        CONST_VTBL struct IMFVideoProcessorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoProcessor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoProcessor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoProcessor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoProcessor_GetAvailableVideoProcessorModes(This,lpdwNumProcessingModes,ppVideoProcessingModes)	\
    ( (This)->lpVtbl -> GetAvailableVideoProcessorModes(This,lpdwNumProcessingModes,ppVideoProcessingModes) ) 

#define IMFVideoProcessor_GetVideoProcessorCaps(This,lpVideoProcessorMode,lpVideoProcessorCaps)	\
    ( (This)->lpVtbl -> GetVideoProcessorCaps(This,lpVideoProcessorMode,lpVideoProcessorCaps) ) 

#define IMFVideoProcessor_GetVideoProcessorMode(This,lpMode)	\
    ( (This)->lpVtbl -> GetVideoProcessorMode(This,lpMode) ) 

#define IMFVideoProcessor_SetVideoProcessorMode(This,lpMode)	\
    ( (This)->lpVtbl -> SetVideoProcessorMode(This,lpMode) ) 

#define IMFVideoProcessor_GetProcAmpRange(This,dwProperty,pPropRange)	\
    ( (This)->lpVtbl -> GetProcAmpRange(This,dwProperty,pPropRange) ) 

#define IMFVideoProcessor_GetProcAmpValues(This,dwFlags,Values)	\
    ( (This)->lpVtbl -> GetProcAmpValues(This,dwFlags,Values) ) 

#define IMFVideoProcessor_SetProcAmpValues(This,dwFlags,pValues)	\
    ( (This)->lpVtbl -> SetProcAmpValues(This,dwFlags,pValues) ) 

#define IMFVideoProcessor_GetFilteringRange(This,dwProperty,pPropRange)	\
    ( (This)->lpVtbl -> GetFilteringRange(This,dwProperty,pPropRange) ) 

#define IMFVideoProcessor_GetFilteringValue(This,dwProperty,pValue)	\
    ( (This)->lpVtbl -> GetFilteringValue(This,dwProperty,pValue) ) 

#define IMFVideoProcessor_SetFilteringValue(This,dwProperty,pValue)	\
    ( (This)->lpVtbl -> SetFilteringValue(This,dwProperty,pValue) ) 

#define IMFVideoProcessor_GetBackgroundColor(This,lpClrBkg)	\
    ( (This)->lpVtbl -> GetBackgroundColor(This,lpClrBkg) ) 

#define IMFVideoProcessor_SetBackgroundColor(This,ClrBkg)	\
    ( (This)->lpVtbl -> SetBackgroundColor(This,ClrBkg) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoProcessor_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_evr9_0000_0002 */
/* [local] */ 

typedef struct MFVideoAlphaBitmapParams
    {
    DWORD dwFlags;
    COLORREF clrSrcKey;
    RECT rcSrc;
    MFVideoNormalizedRect nrcDest;
    FLOAT fAlpha;
    DWORD dwFilterMode;
    } 	MFVideoAlphaBitmapParams;

typedef struct MFVideoAlphaBitmap
    {
    BOOL GetBitmapFromDC;
    union 
        {
        HDC hdc;
        IDirect3DSurface9 *pDDS;
        } 	bitmap;
    MFVideoAlphaBitmapParams params;
    } 	MFVideoAlphaBitmap;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_evr9_0000_0002_0002
    {
        MFVideoAlphaBitmap_EntireDDS	= 0x1,
        MFVideoAlphaBitmap_SrcColorKey	= 0x2,
        MFVideoAlphaBitmap_SrcRect	= 0x4,
        MFVideoAlphaBitmap_DestRect	= 0x8,
        MFVideoAlphaBitmap_FilterMode	= 0x10,
        MFVideoAlphaBitmap_Alpha	= 0x20,
        MFVideoAlphaBitmap_BitMask	= 0x3f
    } 	MFVideoAlphaBitmapFlags;



extern RPC_IF_HANDLE __MIDL_itf_evr9_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_evr9_0000_0002_v0_0_s_ifspec;

#ifndef __IMFVideoMixerBitmap_INTERFACE_DEFINED__
#define __IMFVideoMixerBitmap_INTERFACE_DEFINED__

/* interface IMFVideoMixerBitmap */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFVideoMixerBitmap;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("814C7B20-0FDB-4eec-AF8F-F957C8F69EDC")
    IMFVideoMixerBitmap : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetAlphaBitmap( 
            /* [in] */ const MFVideoAlphaBitmap *pBmpParms) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearAlphaBitmap( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateAlphaBitmapParameters( 
            /* [in] */ const MFVideoAlphaBitmapParams *pBmpParms) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAlphaBitmapParameters( 
            /* [annotation][out] */ 
            _Out_  MFVideoAlphaBitmapParams *pBmpParms) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoMixerBitmapVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFVideoMixerBitmap * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFVideoMixerBitmap * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFVideoMixerBitmap * This);
        
        DECLSPEC_XFGVIRT(IMFVideoMixerBitmap, SetAlphaBitmap)
        HRESULT ( STDMETHODCALLTYPE *SetAlphaBitmap )( 
            IMFVideoMixerBitmap * This,
            /* [in] */ const MFVideoAlphaBitmap *pBmpParms);
        
        DECLSPEC_XFGVIRT(IMFVideoMixerBitmap, ClearAlphaBitmap)
        HRESULT ( STDMETHODCALLTYPE *ClearAlphaBitmap )( 
            IMFVideoMixerBitmap * This);
        
        DECLSPEC_XFGVIRT(IMFVideoMixerBitmap, UpdateAlphaBitmapParameters)
        HRESULT ( STDMETHODCALLTYPE *UpdateAlphaBitmapParameters )( 
            IMFVideoMixerBitmap * This,
            /* [in] */ const MFVideoAlphaBitmapParams *pBmpParms);
        
        DECLSPEC_XFGVIRT(IMFVideoMixerBitmap, GetAlphaBitmapParameters)
        HRESULT ( STDMETHODCALLTYPE *GetAlphaBitmapParameters )( 
            IMFVideoMixerBitmap * This,
            /* [annotation][out] */ 
            _Out_  MFVideoAlphaBitmapParams *pBmpParms);
        
        END_INTERFACE
    } IMFVideoMixerBitmapVtbl;

    interface IMFVideoMixerBitmap
    {
        CONST_VTBL struct IMFVideoMixerBitmapVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoMixerBitmap_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoMixerBitmap_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoMixerBitmap_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoMixerBitmap_SetAlphaBitmap(This,pBmpParms)	\
    ( (This)->lpVtbl -> SetAlphaBitmap(This,pBmpParms) ) 

#define IMFVideoMixerBitmap_ClearAlphaBitmap(This)	\
    ( (This)->lpVtbl -> ClearAlphaBitmap(This) ) 

#define IMFVideoMixerBitmap_UpdateAlphaBitmapParameters(This,pBmpParms)	\
    ( (This)->lpVtbl -> UpdateAlphaBitmapParameters(This,pBmpParms) ) 

#define IMFVideoMixerBitmap_GetAlphaBitmapParameters(This,pBmpParms)	\
    ( (This)->lpVtbl -> GetAlphaBitmapParameters(This,pBmpParms) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoMixerBitmap_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_evr9_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_evr9_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_evr9_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


