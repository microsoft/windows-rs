

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

#ifndef __evr_h__
#define __evr_h__

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

#ifndef __IMFVideoPositionMapper_FWD_DEFINED__
#define __IMFVideoPositionMapper_FWD_DEFINED__
typedef interface IMFVideoPositionMapper IMFVideoPositionMapper;

#endif 	/* __IMFVideoPositionMapper_FWD_DEFINED__ */


#ifndef __IMFVideoDeviceID_FWD_DEFINED__
#define __IMFVideoDeviceID_FWD_DEFINED__
typedef interface IMFVideoDeviceID IMFVideoDeviceID;

#endif 	/* __IMFVideoDeviceID_FWD_DEFINED__ */


#ifndef __IMFVideoDisplayControl_FWD_DEFINED__
#define __IMFVideoDisplayControl_FWD_DEFINED__
typedef interface IMFVideoDisplayControl IMFVideoDisplayControl;

#endif 	/* __IMFVideoDisplayControl_FWD_DEFINED__ */


#ifndef __IMFVideoPresenter_FWD_DEFINED__
#define __IMFVideoPresenter_FWD_DEFINED__
typedef interface IMFVideoPresenter IMFVideoPresenter;

#endif 	/* __IMFVideoPresenter_FWD_DEFINED__ */


#ifndef __IMFDesiredSample_FWD_DEFINED__
#define __IMFDesiredSample_FWD_DEFINED__
typedef interface IMFDesiredSample IMFDesiredSample;

#endif 	/* __IMFDesiredSample_FWD_DEFINED__ */


#ifndef __IMFVideoMixerControl_FWD_DEFINED__
#define __IMFVideoMixerControl_FWD_DEFINED__
typedef interface IMFVideoMixerControl IMFVideoMixerControl;

#endif 	/* __IMFVideoMixerControl_FWD_DEFINED__ */


#ifndef __IMFVideoMixerControl2_FWD_DEFINED__
#define __IMFVideoMixerControl2_FWD_DEFINED__
typedef interface IMFVideoMixerControl2 IMFVideoMixerControl2;

#endif 	/* __IMFVideoMixerControl2_FWD_DEFINED__ */


#ifndef __IMFVideoRenderer_FWD_DEFINED__
#define __IMFVideoRenderer_FWD_DEFINED__
typedef interface IMFVideoRenderer IMFVideoRenderer;

#endif 	/* __IMFVideoRenderer_FWD_DEFINED__ */


#ifndef __IEVRFilterConfig_FWD_DEFINED__
#define __IEVRFilterConfig_FWD_DEFINED__
typedef interface IEVRFilterConfig IEVRFilterConfig;

#endif 	/* __IEVRFilterConfig_FWD_DEFINED__ */


#ifndef __IEVRFilterConfigEx_FWD_DEFINED__
#define __IEVRFilterConfigEx_FWD_DEFINED__
typedef interface IEVRFilterConfigEx IEVRFilterConfigEx;

#endif 	/* __IEVRFilterConfigEx_FWD_DEFINED__ */


#ifndef __IMFTopologyServiceLookup_FWD_DEFINED__
#define __IMFTopologyServiceLookup_FWD_DEFINED__
typedef interface IMFTopologyServiceLookup IMFTopologyServiceLookup;

#endif 	/* __IMFTopologyServiceLookup_FWD_DEFINED__ */


#ifndef __IMFTopologyServiceLookupClient_FWD_DEFINED__
#define __IMFTopologyServiceLookupClient_FWD_DEFINED__
typedef interface IMFTopologyServiceLookupClient IMFTopologyServiceLookupClient;

#endif 	/* __IMFTopologyServiceLookupClient_FWD_DEFINED__ */


#ifndef __IEVRTrustedVideoPlugin_FWD_DEFINED__
#define __IEVRTrustedVideoPlugin_FWD_DEFINED__
typedef interface IEVRTrustedVideoPlugin IEVRTrustedVideoPlugin;

#endif 	/* __IEVRTrustedVideoPlugin_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "propidl.h"
#include "mfidl.h"
#include "strmif.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_evr_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if defined(__midl)
typedef 
enum _D3DFORMAT
    {
        D3DFMT_UNKNOWN	= 0,
        D3DFMT_R8G8B8	= 20,
        D3DFMT_A8R8G8B8	= 21,
        D3DFMT_X8R8G8B8	= 22,
        D3DFMT_R5G6B5	= 23,
        D3DFMT_X1R5G5B5	= 24,
        D3DFMT_A1R5G5B5	= 25,
        D3DFMT_A4R4G4B4	= 26,
        D3DFMT_R3G3B2	= 27,
        D3DFMT_A8	= 28,
        D3DFMT_A8R3G3B2	= 29,
        D3DFMT_X4R4G4B4	= 30,
        D3DFMT_A2B10G10R10	= 31,
        D3DFMT_G16R16	= 34,
        D3DFMT_A8P8	= 40,
        D3DFMT_P8	= 41,
        D3DFMT_L8	= 50,
        D3DFMT_A8L8	= 51,
        D3DFMT_A4L4	= 52,
        D3DFMT_V8U8	= 60,
        D3DFMT_L6V5U5	= 61,
        D3DFMT_X8L8V8U8	= 62,
        D3DFMT_Q8W8V8U8	= 63,
        D3DFMT_V16U16	= 64,
        D3DFMT_W11V11U10	= 65,
        D3DFMT_A2W10V10U10	= 67,
        D3DFMT_D16_LOCKABLE	= 70,
        D3DFMT_D32	= 71,
        D3DFMT_D15S1	= 73,
        D3DFMT_D24S8	= 75,
        D3DFMT_D16	= 80,
        D3DFMT_D24X8	= 77,
        D3DFMT_D24X4S4	= 79,
        D3DFMT_VERTEXDATA	= 100,
        D3DFMT_INDEX16	= 101,
        D3DFMT_INDEX32	= 102,
        D3DFMT_FORCE_DWORD	= 0x7fffffff
    } 	D3DFORMAT;

#endif // __midl
DEFINE_GUID(MR_VIDEO_RENDER_SERVICE, 
    0x1092a86c, 
    0xab1a, 
    0x459a, 
    0xa3, 0x36, 0x83, 0x1f, 0xbc, 0x4d, 0x11, 0xff 
);
DEFINE_GUID(MR_VIDEO_MIXER_SERVICE, 
    0x73cd2fc, 
    0x6cf4, 
    0x40b7, 
    0x88, 0x59, 0xe8, 0x95, 0x52, 0xc8, 0x41, 0xf8 
);
DEFINE_GUID(MR_VIDEO_ACCELERATION_SERVICE, 
    0xefef5175, 
    0x5c7d, 
    0x4ce2, 
    0xbb, 0xbd, 0x34, 0xff, 0x8b, 0xca, 0x65, 0x54 
); 
DEFINE_GUID(MR_BUFFER_SERVICE, 
    0xa562248c, 
    0x9ac6, 
    0x4ffc, 
    0x9f, 0xba, 0x3a, 0xf8, 0xf8, 0xad, 0x1a, 0x4d 
);
DEFINE_GUID(VIDEO_ZOOM_RECT, 
    0x7aaa1638, 
    0x1b7f, 
    0x4c93, 
    0xbd, 0x89, 0x5b, 0x9c, 0x9f, 0xb6, 0xfc, 0xf0
);











extern RPC_IF_HANDLE __MIDL_itf_evr_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_evr_0000_0000_v0_0_s_ifspec;

#ifndef __IMFVideoPositionMapper_INTERFACE_DEFINED__
#define __IMFVideoPositionMapper_INTERFACE_DEFINED__

/* interface IMFVideoPositionMapper */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFVideoPositionMapper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1F6A9F17-E70B-4e24-8AE4-0B2C3BA7A4AE")
    IMFVideoPositionMapper : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MapOutputCoordinateToInputStream( 
            /* [in] */ float xOut,
            /* [in] */ float yOut,
            /* [in] */ DWORD dwOutputStreamIndex,
            /* [in] */ DWORD dwInputStreamIndex,
            /* [annotation][out] */ 
            _Out_  float *pxIn,
            /* [annotation][out] */ 
            _Out_  float *pyIn) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoPositionMapperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFVideoPositionMapper * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFVideoPositionMapper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFVideoPositionMapper * This);
        
        DECLSPEC_XFGVIRT(IMFVideoPositionMapper, MapOutputCoordinateToInputStream)
        HRESULT ( STDMETHODCALLTYPE *MapOutputCoordinateToInputStream )( 
            IMFVideoPositionMapper * This,
            /* [in] */ float xOut,
            /* [in] */ float yOut,
            /* [in] */ DWORD dwOutputStreamIndex,
            /* [in] */ DWORD dwInputStreamIndex,
            /* [annotation][out] */ 
            _Out_  float *pxIn,
            /* [annotation][out] */ 
            _Out_  float *pyIn);
        
        END_INTERFACE
    } IMFVideoPositionMapperVtbl;

    interface IMFVideoPositionMapper
    {
        CONST_VTBL struct IMFVideoPositionMapperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoPositionMapper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoPositionMapper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoPositionMapper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoPositionMapper_MapOutputCoordinateToInputStream(This,xOut,yOut,dwOutputStreamIndex,dwInputStreamIndex,pxIn,pyIn)	\
    ( (This)->lpVtbl -> MapOutputCoordinateToInputStream(This,xOut,yOut,dwOutputStreamIndex,dwInputStreamIndex,pxIn,pyIn) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoPositionMapper_INTERFACE_DEFINED__ */


#ifndef __IMFVideoDeviceID_INTERFACE_DEFINED__
#define __IMFVideoDeviceID_INTERFACE_DEFINED__

/* interface IMFVideoDeviceID */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFVideoDeviceID;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A38D9567-5A9C-4f3c-B293-8EB415B279BA")
    IMFVideoDeviceID : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDeviceID( 
            /* [annotation][out] */ 
            _Out_  IID *pDeviceID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoDeviceIDVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFVideoDeviceID * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFVideoDeviceID * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFVideoDeviceID * This);
        
        DECLSPEC_XFGVIRT(IMFVideoDeviceID, GetDeviceID)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceID )( 
            IMFVideoDeviceID * This,
            /* [annotation][out] */ 
            _Out_  IID *pDeviceID);
        
        END_INTERFACE
    } IMFVideoDeviceIDVtbl;

    interface IMFVideoDeviceID
    {
        CONST_VTBL struct IMFVideoDeviceIDVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoDeviceID_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoDeviceID_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoDeviceID_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoDeviceID_GetDeviceID(This,pDeviceID)	\
    ( (This)->lpVtbl -> GetDeviceID(This,pDeviceID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoDeviceID_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_evr_0000_0002 */
/* [local] */ 

typedef 
enum MFVideoAspectRatioMode
    {
        MFVideoARMode_None	= 0,
        MFVideoARMode_PreservePicture	= 0x1,
        MFVideoARMode_PreservePixel	= 0x2,
        MFVideoARMode_NonLinearStretch	= 0x4,
        MFVideoARMode_Mask	= 0x7
    } 	MFVideoAspectRatioMode;

typedef 
enum MFVideoRenderPrefs
    {
        MFVideoRenderPrefs_DoNotRenderBorder	= 0x1,
        MFVideoRenderPrefs_DoNotClipToDevice	= 0x2,
        MFVideoRenderPrefs_AllowOutputThrottling	= 0x4,
        MFVideoRenderPrefs_ForceOutputThrottling	= 0x8,
        MFVideoRenderPrefs_ForceBatching	= 0x10,
        MFVideoRenderPrefs_AllowBatching	= 0x20,
        MFVideoRenderPrefs_ForceScaling	= 0x40,
        MFVideoRenderPrefs_AllowScaling	= 0x80,
        MFVideoRenderPrefs_DoNotRepaintOnStop	= 0x100,
        MFVideoRenderPrefs_Mask	= 0x1ff
    } 	MFVideoRenderPrefs;

#ifndef _MFVideoNormalizedRect_
#define _MFVideoNormalizedRect_
typedef struct MFVideoNormalizedRect
    {
    float left;
    float top;
    float right;
    float bottom;
    } 	MFVideoNormalizedRect;

#endif


extern RPC_IF_HANDLE __MIDL_itf_evr_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_evr_0000_0002_v0_0_s_ifspec;

#ifndef __IMFVideoDisplayControl_INTERFACE_DEFINED__
#define __IMFVideoDisplayControl_INTERFACE_DEFINED__

/* interface IMFVideoDisplayControl */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFVideoDisplayControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a490b1e4-ab84-4d31-a1b2-181e03b1077a")
    IMFVideoDisplayControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNativeVideoSize( 
            /* [unique][out][in] */ __RPC__inout_opt SIZE *pszVideo,
            /* [unique][out][in] */ __RPC__inout_opt SIZE *pszARVideo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIdealVideoSize( 
            /* [unique][out][in] */ __RPC__inout_opt SIZE *pszMin,
            /* [unique][out][in] */ __RPC__inout_opt SIZE *pszMax) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVideoPosition( 
            /* [unique][in] */ __RPC__in_opt const MFVideoNormalizedRect *pnrcSource,
            /* [unique][in] */ __RPC__in_opt const LPRECT prcDest) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVideoPosition( 
            /* [out] */ __RPC__out MFVideoNormalizedRect *pnrcSource,
            /* [out] */ __RPC__out LPRECT prcDest) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAspectRatioMode( 
            /* [in] */ DWORD dwAspectRatioMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAspectRatioMode( 
            /* [out] */ __RPC__out DWORD *pdwAspectRatioMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVideoWindow( 
            /* [in] */ __RPC__in HWND hwndVideo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVideoWindow( 
            /* [out] */ __RPC__deref_out_opt HWND *phwndVideo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RepaintVideo( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentImage( 
            /* [out][in] */ __RPC__inout BITMAPINFOHEADER *pBih,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbDib) BYTE **pDib,
            /* [out] */ __RPC__out DWORD *pcbDib,
            /* [unique][out][in] */ __RPC__inout_opt LONGLONG *pTimeStamp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBorderColor( 
            /* [in] */ COLORREF Clr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBorderColor( 
            /* [out] */ __RPC__out COLORREF *pClr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRenderingPrefs( 
            /* [in] */ DWORD dwRenderFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRenderingPrefs( 
            /* [out] */ __RPC__out DWORD *pdwRenderFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFullscreen( 
            /* [in] */ BOOL fFullscreen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFullscreen( 
            /* [out] */ __RPC__out BOOL *pfFullscreen) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoDisplayControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFVideoDisplayControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFVideoDisplayControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFVideoDisplayControl * This);
        
        DECLSPEC_XFGVIRT(IMFVideoDisplayControl, GetNativeVideoSize)
        HRESULT ( STDMETHODCALLTYPE *GetNativeVideoSize )( 
            __RPC__in IMFVideoDisplayControl * This,
            /* [unique][out][in] */ __RPC__inout_opt SIZE *pszVideo,
            /* [unique][out][in] */ __RPC__inout_opt SIZE *pszARVideo);
        
        DECLSPEC_XFGVIRT(IMFVideoDisplayControl, GetIdealVideoSize)
        HRESULT ( STDMETHODCALLTYPE *GetIdealVideoSize )( 
            __RPC__in IMFVideoDisplayControl * This,
            /* [unique][out][in] */ __RPC__inout_opt SIZE *pszMin,
            /* [unique][out][in] */ __RPC__inout_opt SIZE *pszMax);
        
        DECLSPEC_XFGVIRT(IMFVideoDisplayControl, SetVideoPosition)
        HRESULT ( STDMETHODCALLTYPE *SetVideoPosition )( 
            __RPC__in IMFVideoDisplayControl * This,
            /* [unique][in] */ __RPC__in_opt const MFVideoNormalizedRect *pnrcSource,
            /* [unique][in] */ __RPC__in_opt const LPRECT prcDest);
        
        DECLSPEC_XFGVIRT(IMFVideoDisplayControl, GetVideoPosition)
        HRESULT ( STDMETHODCALLTYPE *GetVideoPosition )( 
            __RPC__in IMFVideoDisplayControl * This,
            /* [out] */ __RPC__out MFVideoNormalizedRect *pnrcSource,
            /* [out] */ __RPC__out LPRECT prcDest);
        
        DECLSPEC_XFGVIRT(IMFVideoDisplayControl, SetAspectRatioMode)
        HRESULT ( STDMETHODCALLTYPE *SetAspectRatioMode )( 
            __RPC__in IMFVideoDisplayControl * This,
            /* [in] */ DWORD dwAspectRatioMode);
        
        DECLSPEC_XFGVIRT(IMFVideoDisplayControl, GetAspectRatioMode)
        HRESULT ( STDMETHODCALLTYPE *GetAspectRatioMode )( 
            __RPC__in IMFVideoDisplayControl * This,
            /* [out] */ __RPC__out DWORD *pdwAspectRatioMode);
        
        DECLSPEC_XFGVIRT(IMFVideoDisplayControl, SetVideoWindow)
        HRESULT ( STDMETHODCALLTYPE *SetVideoWindow )( 
            __RPC__in IMFVideoDisplayControl * This,
            /* [in] */ __RPC__in HWND hwndVideo);
        
        DECLSPEC_XFGVIRT(IMFVideoDisplayControl, GetVideoWindow)
        HRESULT ( STDMETHODCALLTYPE *GetVideoWindow )( 
            __RPC__in IMFVideoDisplayControl * This,
            /* [out] */ __RPC__deref_out_opt HWND *phwndVideo);
        
        DECLSPEC_XFGVIRT(IMFVideoDisplayControl, RepaintVideo)
        HRESULT ( STDMETHODCALLTYPE *RepaintVideo )( 
            __RPC__in IMFVideoDisplayControl * This);
        
        DECLSPEC_XFGVIRT(IMFVideoDisplayControl, GetCurrentImage)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentImage )( 
            __RPC__in IMFVideoDisplayControl * This,
            /* [out][in] */ __RPC__inout BITMAPINFOHEADER *pBih,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbDib) BYTE **pDib,
            /* [out] */ __RPC__out DWORD *pcbDib,
            /* [unique][out][in] */ __RPC__inout_opt LONGLONG *pTimeStamp);
        
        DECLSPEC_XFGVIRT(IMFVideoDisplayControl, SetBorderColor)
        HRESULT ( STDMETHODCALLTYPE *SetBorderColor )( 
            __RPC__in IMFVideoDisplayControl * This,
            /* [in] */ COLORREF Clr);
        
        DECLSPEC_XFGVIRT(IMFVideoDisplayControl, GetBorderColor)
        HRESULT ( STDMETHODCALLTYPE *GetBorderColor )( 
            __RPC__in IMFVideoDisplayControl * This,
            /* [out] */ __RPC__out COLORREF *pClr);
        
        DECLSPEC_XFGVIRT(IMFVideoDisplayControl, SetRenderingPrefs)
        HRESULT ( STDMETHODCALLTYPE *SetRenderingPrefs )( 
            __RPC__in IMFVideoDisplayControl * This,
            /* [in] */ DWORD dwRenderFlags);
        
        DECLSPEC_XFGVIRT(IMFVideoDisplayControl, GetRenderingPrefs)
        HRESULT ( STDMETHODCALLTYPE *GetRenderingPrefs )( 
            __RPC__in IMFVideoDisplayControl * This,
            /* [out] */ __RPC__out DWORD *pdwRenderFlags);
        
        DECLSPEC_XFGVIRT(IMFVideoDisplayControl, SetFullscreen)
        HRESULT ( STDMETHODCALLTYPE *SetFullscreen )( 
            __RPC__in IMFVideoDisplayControl * This,
            /* [in] */ BOOL fFullscreen);
        
        DECLSPEC_XFGVIRT(IMFVideoDisplayControl, GetFullscreen)
        HRESULT ( STDMETHODCALLTYPE *GetFullscreen )( 
            __RPC__in IMFVideoDisplayControl * This,
            /* [out] */ __RPC__out BOOL *pfFullscreen);
        
        END_INTERFACE
    } IMFVideoDisplayControlVtbl;

    interface IMFVideoDisplayControl
    {
        CONST_VTBL struct IMFVideoDisplayControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoDisplayControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoDisplayControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoDisplayControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoDisplayControl_GetNativeVideoSize(This,pszVideo,pszARVideo)	\
    ( (This)->lpVtbl -> GetNativeVideoSize(This,pszVideo,pszARVideo) ) 

#define IMFVideoDisplayControl_GetIdealVideoSize(This,pszMin,pszMax)	\
    ( (This)->lpVtbl -> GetIdealVideoSize(This,pszMin,pszMax) ) 

#define IMFVideoDisplayControl_SetVideoPosition(This,pnrcSource,prcDest)	\
    ( (This)->lpVtbl -> SetVideoPosition(This,pnrcSource,prcDest) ) 

#define IMFVideoDisplayControl_GetVideoPosition(This,pnrcSource,prcDest)	\
    ( (This)->lpVtbl -> GetVideoPosition(This,pnrcSource,prcDest) ) 

#define IMFVideoDisplayControl_SetAspectRatioMode(This,dwAspectRatioMode)	\
    ( (This)->lpVtbl -> SetAspectRatioMode(This,dwAspectRatioMode) ) 

#define IMFVideoDisplayControl_GetAspectRatioMode(This,pdwAspectRatioMode)	\
    ( (This)->lpVtbl -> GetAspectRatioMode(This,pdwAspectRatioMode) ) 

#define IMFVideoDisplayControl_SetVideoWindow(This,hwndVideo)	\
    ( (This)->lpVtbl -> SetVideoWindow(This,hwndVideo) ) 

#define IMFVideoDisplayControl_GetVideoWindow(This,phwndVideo)	\
    ( (This)->lpVtbl -> GetVideoWindow(This,phwndVideo) ) 

#define IMFVideoDisplayControl_RepaintVideo(This)	\
    ( (This)->lpVtbl -> RepaintVideo(This) ) 

#define IMFVideoDisplayControl_GetCurrentImage(This,pBih,pDib,pcbDib,pTimeStamp)	\
    ( (This)->lpVtbl -> GetCurrentImage(This,pBih,pDib,pcbDib,pTimeStamp) ) 

#define IMFVideoDisplayControl_SetBorderColor(This,Clr)	\
    ( (This)->lpVtbl -> SetBorderColor(This,Clr) ) 

#define IMFVideoDisplayControl_GetBorderColor(This,pClr)	\
    ( (This)->lpVtbl -> GetBorderColor(This,pClr) ) 

#define IMFVideoDisplayControl_SetRenderingPrefs(This,dwRenderFlags)	\
    ( (This)->lpVtbl -> SetRenderingPrefs(This,dwRenderFlags) ) 

#define IMFVideoDisplayControl_GetRenderingPrefs(This,pdwRenderFlags)	\
    ( (This)->lpVtbl -> GetRenderingPrefs(This,pdwRenderFlags) ) 

#define IMFVideoDisplayControl_SetFullscreen(This,fFullscreen)	\
    ( (This)->lpVtbl -> SetFullscreen(This,fFullscreen) ) 

#define IMFVideoDisplayControl_GetFullscreen(This,pfFullscreen)	\
    ( (This)->lpVtbl -> GetFullscreen(This,pfFullscreen) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoDisplayControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_evr_0000_0003 */
/* [local] */ 

typedef 
enum MFVP_MESSAGE_TYPE
    {
        MFVP_MESSAGE_FLUSH	= 0,
        MFVP_MESSAGE_INVALIDATEMEDIATYPE	= 0x1,
        MFVP_MESSAGE_PROCESSINPUTNOTIFY	= 0x2,
        MFVP_MESSAGE_BEGINSTREAMING	= 0x3,
        MFVP_MESSAGE_ENDSTREAMING	= 0x4,
        MFVP_MESSAGE_ENDOFSTREAM	= 0x5,
        MFVP_MESSAGE_STEP	= 0x6,
        MFVP_MESSAGE_CANCELSTEP	= 0x7
    } 	MFVP_MESSAGE_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_evr_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_evr_0000_0003_v0_0_s_ifspec;

#ifndef __IMFVideoPresenter_INTERFACE_DEFINED__
#define __IMFVideoPresenter_INTERFACE_DEFINED__

/* interface IMFVideoPresenter */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFVideoPresenter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("29AFF080-182A-4a5d-AF3B-448F3A6346CB")
    IMFVideoPresenter : public IMFClockStateSink
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ProcessMessage( 
            MFVP_MESSAGE_TYPE eMessage,
            ULONG_PTR ulParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentMediaType( 
            /* [annotation][out] */ 
            _Outptr_  IMFVideoMediaType **ppMediaType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoPresenterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFVideoPresenter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFVideoPresenter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFVideoPresenter * This);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockStart)
        HRESULT ( STDMETHODCALLTYPE *OnClockStart )( 
            IMFVideoPresenter * This,
            /* [in] */ MFTIME hnsSystemTime,
            /* [in] */ LONGLONG llClockStartOffset);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockStop)
        HRESULT ( STDMETHODCALLTYPE *OnClockStop )( 
            IMFVideoPresenter * This,
            /* [in] */ MFTIME hnsSystemTime);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockPause)
        HRESULT ( STDMETHODCALLTYPE *OnClockPause )( 
            IMFVideoPresenter * This,
            /* [in] */ MFTIME hnsSystemTime);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockRestart)
        HRESULT ( STDMETHODCALLTYPE *OnClockRestart )( 
            IMFVideoPresenter * This,
            /* [in] */ MFTIME hnsSystemTime);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockSetRate)
        HRESULT ( STDMETHODCALLTYPE *OnClockSetRate )( 
            IMFVideoPresenter * This,
            /* [in] */ MFTIME hnsSystemTime,
            /* [in] */ float flRate);
        
        DECLSPEC_XFGVIRT(IMFVideoPresenter, ProcessMessage)
        HRESULT ( STDMETHODCALLTYPE *ProcessMessage )( 
            IMFVideoPresenter * This,
            MFVP_MESSAGE_TYPE eMessage,
            ULONG_PTR ulParam);
        
        DECLSPEC_XFGVIRT(IMFVideoPresenter, GetCurrentMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentMediaType )( 
            IMFVideoPresenter * This,
            /* [annotation][out] */ 
            _Outptr_  IMFVideoMediaType **ppMediaType);
        
        END_INTERFACE
    } IMFVideoPresenterVtbl;

    interface IMFVideoPresenter
    {
        CONST_VTBL struct IMFVideoPresenterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoPresenter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoPresenter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoPresenter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoPresenter_OnClockStart(This,hnsSystemTime,llClockStartOffset)	\
    ( (This)->lpVtbl -> OnClockStart(This,hnsSystemTime,llClockStartOffset) ) 

#define IMFVideoPresenter_OnClockStop(This,hnsSystemTime)	\
    ( (This)->lpVtbl -> OnClockStop(This,hnsSystemTime) ) 

#define IMFVideoPresenter_OnClockPause(This,hnsSystemTime)	\
    ( (This)->lpVtbl -> OnClockPause(This,hnsSystemTime) ) 

#define IMFVideoPresenter_OnClockRestart(This,hnsSystemTime)	\
    ( (This)->lpVtbl -> OnClockRestart(This,hnsSystemTime) ) 

#define IMFVideoPresenter_OnClockSetRate(This,hnsSystemTime,flRate)	\
    ( (This)->lpVtbl -> OnClockSetRate(This,hnsSystemTime,flRate) ) 


#define IMFVideoPresenter_ProcessMessage(This,eMessage,ulParam)	\
    ( (This)->lpVtbl -> ProcessMessage(This,eMessage,ulParam) ) 

#define IMFVideoPresenter_GetCurrentMediaType(This,ppMediaType)	\
    ( (This)->lpVtbl -> GetCurrentMediaType(This,ppMediaType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoPresenter_INTERFACE_DEFINED__ */


#ifndef __IMFDesiredSample_INTERFACE_DEFINED__
#define __IMFDesiredSample_INTERFACE_DEFINED__

/* interface IMFDesiredSample */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFDesiredSample;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56C294D0-753E-4260-8D61-A3D8820B1D54")
    IMFDesiredSample : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDesiredSampleTimeAndDuration( 
            /* [annotation][out] */ 
            _Out_  LONGLONG *phnsSampleTime,
            /* [annotation][out] */ 
            _Out_  LONGLONG *phnsSampleDuration) = 0;
        
        virtual void STDMETHODCALLTYPE SetDesiredSampleTimeAndDuration( 
            /* [in] */ LONGLONG hnsSampleTime,
            /* [in] */ LONGLONG hnsSampleDuration) = 0;
        
        virtual void STDMETHODCALLTYPE Clear( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFDesiredSampleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFDesiredSample * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFDesiredSample * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFDesiredSample * This);
        
        DECLSPEC_XFGVIRT(IMFDesiredSample, GetDesiredSampleTimeAndDuration)
        HRESULT ( STDMETHODCALLTYPE *GetDesiredSampleTimeAndDuration )( 
            IMFDesiredSample * This,
            /* [annotation][out] */ 
            _Out_  LONGLONG *phnsSampleTime,
            /* [annotation][out] */ 
            _Out_  LONGLONG *phnsSampleDuration);
        
        DECLSPEC_XFGVIRT(IMFDesiredSample, SetDesiredSampleTimeAndDuration)
        void ( STDMETHODCALLTYPE *SetDesiredSampleTimeAndDuration )( 
            IMFDesiredSample * This,
            /* [in] */ LONGLONG hnsSampleTime,
            /* [in] */ LONGLONG hnsSampleDuration);
        
        DECLSPEC_XFGVIRT(IMFDesiredSample, Clear)
        void ( STDMETHODCALLTYPE *Clear )( 
            IMFDesiredSample * This);
        
        END_INTERFACE
    } IMFDesiredSampleVtbl;

    interface IMFDesiredSample
    {
        CONST_VTBL struct IMFDesiredSampleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFDesiredSample_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFDesiredSample_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFDesiredSample_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFDesiredSample_GetDesiredSampleTimeAndDuration(This,phnsSampleTime,phnsSampleDuration)	\
    ( (This)->lpVtbl -> GetDesiredSampleTimeAndDuration(This,phnsSampleTime,phnsSampleDuration) ) 

#define IMFDesiredSample_SetDesiredSampleTimeAndDuration(This,hnsSampleTime,hnsSampleDuration)	\
    ( (This)->lpVtbl -> SetDesiredSampleTimeAndDuration(This,hnsSampleTime,hnsSampleDuration) ) 

#define IMFDesiredSample_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFDesiredSample_INTERFACE_DEFINED__ */


#ifndef __IMFVideoMixerControl_INTERFACE_DEFINED__
#define __IMFVideoMixerControl_INTERFACE_DEFINED__

/* interface IMFVideoMixerControl */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFVideoMixerControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A5C6C53F-C202-4aa5-9695-175BA8C508A5")
    IMFVideoMixerControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetStreamZOrder( 
            /* [in] */ DWORD dwStreamID,
            /* [in] */ DWORD dwZ) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamZOrder( 
            /* [in] */ DWORD dwStreamID,
            /* [out] */ __RPC__out DWORD *pdwZ) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStreamOutputRect( 
            /* [in] */ DWORD dwStreamID,
            /* [in] */ __RPC__in const MFVideoNormalizedRect *pnrcOutput) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamOutputRect( 
            /* [in] */ DWORD dwStreamID,
            /* [out] */ __RPC__out MFVideoNormalizedRect *pnrcOutput) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoMixerControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFVideoMixerControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFVideoMixerControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFVideoMixerControl * This);
        
        DECLSPEC_XFGVIRT(IMFVideoMixerControl, SetStreamZOrder)
        HRESULT ( STDMETHODCALLTYPE *SetStreamZOrder )( 
            __RPC__in IMFVideoMixerControl * This,
            /* [in] */ DWORD dwStreamID,
            /* [in] */ DWORD dwZ);
        
        DECLSPEC_XFGVIRT(IMFVideoMixerControl, GetStreamZOrder)
        HRESULT ( STDMETHODCALLTYPE *GetStreamZOrder )( 
            __RPC__in IMFVideoMixerControl * This,
            /* [in] */ DWORD dwStreamID,
            /* [out] */ __RPC__out DWORD *pdwZ);
        
        DECLSPEC_XFGVIRT(IMFVideoMixerControl, SetStreamOutputRect)
        HRESULT ( STDMETHODCALLTYPE *SetStreamOutputRect )( 
            __RPC__in IMFVideoMixerControl * This,
            /* [in] */ DWORD dwStreamID,
            /* [in] */ __RPC__in const MFVideoNormalizedRect *pnrcOutput);
        
        DECLSPEC_XFGVIRT(IMFVideoMixerControl, GetStreamOutputRect)
        HRESULT ( STDMETHODCALLTYPE *GetStreamOutputRect )( 
            __RPC__in IMFVideoMixerControl * This,
            /* [in] */ DWORD dwStreamID,
            /* [out] */ __RPC__out MFVideoNormalizedRect *pnrcOutput);
        
        END_INTERFACE
    } IMFVideoMixerControlVtbl;

    interface IMFVideoMixerControl
    {
        CONST_VTBL struct IMFVideoMixerControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoMixerControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoMixerControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoMixerControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoMixerControl_SetStreamZOrder(This,dwStreamID,dwZ)	\
    ( (This)->lpVtbl -> SetStreamZOrder(This,dwStreamID,dwZ) ) 

#define IMFVideoMixerControl_GetStreamZOrder(This,dwStreamID,pdwZ)	\
    ( (This)->lpVtbl -> GetStreamZOrder(This,dwStreamID,pdwZ) ) 

#define IMFVideoMixerControl_SetStreamOutputRect(This,dwStreamID,pnrcOutput)	\
    ( (This)->lpVtbl -> SetStreamOutputRect(This,dwStreamID,pnrcOutput) ) 

#define IMFVideoMixerControl_GetStreamOutputRect(This,dwStreamID,pnrcOutput)	\
    ( (This)->lpVtbl -> GetStreamOutputRect(This,dwStreamID,pnrcOutput) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoMixerControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_evr_0000_0006 */
/* [local] */ 

typedef 
enum _MFVideoMixPrefs
    {
        MFVideoMixPrefs_ForceHalfInterlace	= 0x1,
        MFVideoMixPrefs_AllowDropToHalfInterlace	= 0x2,
        MFVideoMixPrefs_AllowDropToBob	= 0x4,
        MFVideoMixPrefs_ForceBob	= 0x8,
        MFVideoMixPrefs_EnableRotation	= 0x10,
        MFVideoMixPrefs_Mask	= 0x1f
    } 	MFVideoMixPrefs;



extern RPC_IF_HANDLE __MIDL_itf_evr_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_evr_0000_0006_v0_0_s_ifspec;

#ifndef __IMFVideoMixerControl2_INTERFACE_DEFINED__
#define __IMFVideoMixerControl2_INTERFACE_DEFINED__

/* interface IMFVideoMixerControl2 */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFVideoMixerControl2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8459616d-966e-4930-b658-54fa7e5a16d3")
    IMFVideoMixerControl2 : public IMFVideoMixerControl
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetMixingPrefs( 
            /* [in] */ DWORD dwMixFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMixingPrefs( 
            /* [out] */ __RPC__out DWORD *pdwMixFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoMixerControl2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFVideoMixerControl2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFVideoMixerControl2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFVideoMixerControl2 * This);
        
        DECLSPEC_XFGVIRT(IMFVideoMixerControl, SetStreamZOrder)
        HRESULT ( STDMETHODCALLTYPE *SetStreamZOrder )( 
            __RPC__in IMFVideoMixerControl2 * This,
            /* [in] */ DWORD dwStreamID,
            /* [in] */ DWORD dwZ);
        
        DECLSPEC_XFGVIRT(IMFVideoMixerControl, GetStreamZOrder)
        HRESULT ( STDMETHODCALLTYPE *GetStreamZOrder )( 
            __RPC__in IMFVideoMixerControl2 * This,
            /* [in] */ DWORD dwStreamID,
            /* [out] */ __RPC__out DWORD *pdwZ);
        
        DECLSPEC_XFGVIRT(IMFVideoMixerControl, SetStreamOutputRect)
        HRESULT ( STDMETHODCALLTYPE *SetStreamOutputRect )( 
            __RPC__in IMFVideoMixerControl2 * This,
            /* [in] */ DWORD dwStreamID,
            /* [in] */ __RPC__in const MFVideoNormalizedRect *pnrcOutput);
        
        DECLSPEC_XFGVIRT(IMFVideoMixerControl, GetStreamOutputRect)
        HRESULT ( STDMETHODCALLTYPE *GetStreamOutputRect )( 
            __RPC__in IMFVideoMixerControl2 * This,
            /* [in] */ DWORD dwStreamID,
            /* [out] */ __RPC__out MFVideoNormalizedRect *pnrcOutput);
        
        DECLSPEC_XFGVIRT(IMFVideoMixerControl2, SetMixingPrefs)
        HRESULT ( STDMETHODCALLTYPE *SetMixingPrefs )( 
            __RPC__in IMFVideoMixerControl2 * This,
            /* [in] */ DWORD dwMixFlags);
        
        DECLSPEC_XFGVIRT(IMFVideoMixerControl2, GetMixingPrefs)
        HRESULT ( STDMETHODCALLTYPE *GetMixingPrefs )( 
            __RPC__in IMFVideoMixerControl2 * This,
            /* [out] */ __RPC__out DWORD *pdwMixFlags);
        
        END_INTERFACE
    } IMFVideoMixerControl2Vtbl;

    interface IMFVideoMixerControl2
    {
        CONST_VTBL struct IMFVideoMixerControl2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoMixerControl2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoMixerControl2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoMixerControl2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoMixerControl2_SetStreamZOrder(This,dwStreamID,dwZ)	\
    ( (This)->lpVtbl -> SetStreamZOrder(This,dwStreamID,dwZ) ) 

#define IMFVideoMixerControl2_GetStreamZOrder(This,dwStreamID,pdwZ)	\
    ( (This)->lpVtbl -> GetStreamZOrder(This,dwStreamID,pdwZ) ) 

#define IMFVideoMixerControl2_SetStreamOutputRect(This,dwStreamID,pnrcOutput)	\
    ( (This)->lpVtbl -> SetStreamOutputRect(This,dwStreamID,pnrcOutput) ) 

#define IMFVideoMixerControl2_GetStreamOutputRect(This,dwStreamID,pnrcOutput)	\
    ( (This)->lpVtbl -> GetStreamOutputRect(This,dwStreamID,pnrcOutput) ) 


#define IMFVideoMixerControl2_SetMixingPrefs(This,dwMixFlags)	\
    ( (This)->lpVtbl -> SetMixingPrefs(This,dwMixFlags) ) 

#define IMFVideoMixerControl2_GetMixingPrefs(This,pdwMixFlags)	\
    ( (This)->lpVtbl -> GetMixingPrefs(This,pdwMixFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoMixerControl2_INTERFACE_DEFINED__ */


#ifndef __IMFVideoRenderer_INTERFACE_DEFINED__
#define __IMFVideoRenderer_INTERFACE_DEFINED__

/* interface IMFVideoRenderer */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFVideoRenderer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DFDFD197-A9CA-43d8-B341-6AF3503792CD")
    IMFVideoRenderer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitializeRenderer( 
            /* [annotation][unique][in] */ 
            _In_opt_  IMFTransform *pVideoMixer,
            /* [annotation][unique][in] */ 
            _In_opt_  IMFVideoPresenter *pVideoPresenter) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoRendererVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFVideoRenderer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFVideoRenderer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFVideoRenderer * This);
        
        DECLSPEC_XFGVIRT(IMFVideoRenderer, InitializeRenderer)
        HRESULT ( STDMETHODCALLTYPE *InitializeRenderer )( 
            IMFVideoRenderer * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IMFTransform *pVideoMixer,
            /* [annotation][unique][in] */ 
            _In_opt_  IMFVideoPresenter *pVideoPresenter);
        
        END_INTERFACE
    } IMFVideoRendererVtbl;

    interface IMFVideoRenderer
    {
        CONST_VTBL struct IMFVideoRendererVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoRenderer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoRenderer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoRenderer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoRenderer_InitializeRenderer(This,pVideoMixer,pVideoPresenter)	\
    ( (This)->lpVtbl -> InitializeRenderer(This,pVideoMixer,pVideoPresenter) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoRenderer_INTERFACE_DEFINED__ */


#ifndef __IEVRFilterConfig_INTERFACE_DEFINED__
#define __IEVRFilterConfig_INTERFACE_DEFINED__

/* interface IEVRFilterConfig */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEVRFilterConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("83E91E85-82C1-4ea7-801D-85DC50B75086")
    IEVRFilterConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetNumberOfStreams( 
            /* [in] */ DWORD dwMaxStreams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumberOfStreams( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwMaxStreams) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEVRFilterConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEVRFilterConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEVRFilterConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEVRFilterConfig * This);
        
        DECLSPEC_XFGVIRT(IEVRFilterConfig, SetNumberOfStreams)
        HRESULT ( STDMETHODCALLTYPE *SetNumberOfStreams )( 
            IEVRFilterConfig * This,
            /* [in] */ DWORD dwMaxStreams);
        
        DECLSPEC_XFGVIRT(IEVRFilterConfig, GetNumberOfStreams)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfStreams )( 
            IEVRFilterConfig * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwMaxStreams);
        
        END_INTERFACE
    } IEVRFilterConfigVtbl;

    interface IEVRFilterConfig
    {
        CONST_VTBL struct IEVRFilterConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEVRFilterConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEVRFilterConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEVRFilterConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEVRFilterConfig_SetNumberOfStreams(This,dwMaxStreams)	\
    ( (This)->lpVtbl -> SetNumberOfStreams(This,dwMaxStreams) ) 

#define IEVRFilterConfig_GetNumberOfStreams(This,pdwMaxStreams)	\
    ( (This)->lpVtbl -> GetNumberOfStreams(This,pdwMaxStreams) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEVRFilterConfig_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_evr_0000_0009 */
/* [local] */ 

typedef 
enum _EVRFilterConfig_Prefs
    {
        EVRFilterConfigPrefs_EnableQoS	= 0x1,
        EVRFilterConfigPrefs_Mask	= 0x1
    } 	EVRFilterConfigPrefs;



extern RPC_IF_HANDLE __MIDL_itf_evr_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_evr_0000_0009_v0_0_s_ifspec;

#ifndef __IEVRFilterConfigEx_INTERFACE_DEFINED__
#define __IEVRFilterConfigEx_INTERFACE_DEFINED__

/* interface IEVRFilterConfigEx */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEVRFilterConfigEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aea36028-796d-454f-beee-b48071e24304")
    IEVRFilterConfigEx : public IEVRFilterConfig
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetConfigPrefs( 
            /* [in] */ DWORD dwConfigFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConfigPrefs( 
            /* [out] */ __RPC__out DWORD *pdwConfigFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEVRFilterConfigExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEVRFilterConfigEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEVRFilterConfigEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEVRFilterConfigEx * This);
        
        DECLSPEC_XFGVIRT(IEVRFilterConfig, SetNumberOfStreams)
        HRESULT ( STDMETHODCALLTYPE *SetNumberOfStreams )( 
            __RPC__in IEVRFilterConfigEx * This,
            /* [in] */ DWORD dwMaxStreams);
        
        DECLSPEC_XFGVIRT(IEVRFilterConfig, GetNumberOfStreams)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfStreams )( 
            __RPC__in IEVRFilterConfigEx * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwMaxStreams);
        
        DECLSPEC_XFGVIRT(IEVRFilterConfigEx, SetConfigPrefs)
        HRESULT ( STDMETHODCALLTYPE *SetConfigPrefs )( 
            __RPC__in IEVRFilterConfigEx * This,
            /* [in] */ DWORD dwConfigFlags);
        
        DECLSPEC_XFGVIRT(IEVRFilterConfigEx, GetConfigPrefs)
        HRESULT ( STDMETHODCALLTYPE *GetConfigPrefs )( 
            __RPC__in IEVRFilterConfigEx * This,
            /* [out] */ __RPC__out DWORD *pdwConfigFlags);
        
        END_INTERFACE
    } IEVRFilterConfigExVtbl;

    interface IEVRFilterConfigEx
    {
        CONST_VTBL struct IEVRFilterConfigExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEVRFilterConfigEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEVRFilterConfigEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEVRFilterConfigEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEVRFilterConfigEx_SetNumberOfStreams(This,dwMaxStreams)	\
    ( (This)->lpVtbl -> SetNumberOfStreams(This,dwMaxStreams) ) 

#define IEVRFilterConfigEx_GetNumberOfStreams(This,pdwMaxStreams)	\
    ( (This)->lpVtbl -> GetNumberOfStreams(This,pdwMaxStreams) ) 


#define IEVRFilterConfigEx_SetConfigPrefs(This,dwConfigFlags)	\
    ( (This)->lpVtbl -> SetConfigPrefs(This,dwConfigFlags) ) 

#define IEVRFilterConfigEx_GetConfigPrefs(This,pdwConfigFlags)	\
    ( (This)->lpVtbl -> GetConfigPrefs(This,pdwConfigFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEVRFilterConfigEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_evr_0000_0010 */
/* [local] */ 

typedef 
enum _MF_SERVICE_LOOKUP_TYPE
    {
        MF_SERVICE_LOOKUP_UPSTREAM	= 0,
        MF_SERVICE_LOOKUP_UPSTREAM_DIRECT	= ( MF_SERVICE_LOOKUP_UPSTREAM + 1 ) ,
        MF_SERVICE_LOOKUP_DOWNSTREAM	= ( MF_SERVICE_LOOKUP_UPSTREAM_DIRECT + 1 ) ,
        MF_SERVICE_LOOKUP_DOWNSTREAM_DIRECT	= ( MF_SERVICE_LOOKUP_DOWNSTREAM + 1 ) ,
        MF_SERVICE_LOOKUP_ALL	= ( MF_SERVICE_LOOKUP_DOWNSTREAM_DIRECT + 1 ) ,
        MF_SERVICE_LOOKUP_GLOBAL	= ( MF_SERVICE_LOOKUP_ALL + 1 ) 
    } 	MF_SERVICE_LOOKUP_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_evr_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_evr_0000_0010_v0_0_s_ifspec;

#ifndef __IMFTopologyServiceLookup_INTERFACE_DEFINED__
#define __IMFTopologyServiceLookup_INTERFACE_DEFINED__

/* interface IMFTopologyServiceLookup */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFTopologyServiceLookup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fa993889-4383-415a-a930-dd472a8cf6f7")
    IMFTopologyServiceLookup : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE LookupService( 
            /* [in] */ MF_SERVICE_LOOKUP_TYPE Type,
            /* [in] */ DWORD dwIndex,
            /* [in] */ REFGUID guidService,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Out_writes_to_(1,*pnObjects)  LPVOID *ppvObjects,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pnObjects) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTopologyServiceLookupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTopologyServiceLookup * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTopologyServiceLookup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTopologyServiceLookup * This);
        
        DECLSPEC_XFGVIRT(IMFTopologyServiceLookup, LookupService)
        HRESULT ( STDMETHODCALLTYPE *LookupService )( 
            IMFTopologyServiceLookup * This,
            /* [in] */ MF_SERVICE_LOOKUP_TYPE Type,
            /* [in] */ DWORD dwIndex,
            /* [in] */ REFGUID guidService,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Out_writes_to_(1,*pnObjects)  LPVOID *ppvObjects,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pnObjects);
        
        END_INTERFACE
    } IMFTopologyServiceLookupVtbl;

    interface IMFTopologyServiceLookup
    {
        CONST_VTBL struct IMFTopologyServiceLookupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTopologyServiceLookup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTopologyServiceLookup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTopologyServiceLookup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTopologyServiceLookup_LookupService(This,Type,dwIndex,guidService,riid,ppvObjects,pnObjects)	\
    ( (This)->lpVtbl -> LookupService(This,Type,dwIndex,guidService,riid,ppvObjects,pnObjects) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTopologyServiceLookup_INTERFACE_DEFINED__ */


#ifndef __IMFTopologyServiceLookupClient_INTERFACE_DEFINED__
#define __IMFTopologyServiceLookupClient_INTERFACE_DEFINED__

/* interface IMFTopologyServiceLookupClient */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IMFTopologyServiceLookupClient;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fa99388a-4383-415a-a930-dd472a8cf6f7")
    IMFTopologyServiceLookupClient : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitServicePointers( 
            /* [annotation][in] */ 
            _In_  IMFTopologyServiceLookup *pLookup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseServicePointers( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTopologyServiceLookupClientVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTopologyServiceLookupClient * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTopologyServiceLookupClient * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTopologyServiceLookupClient * This);
        
        DECLSPEC_XFGVIRT(IMFTopologyServiceLookupClient, InitServicePointers)
        HRESULT ( STDMETHODCALLTYPE *InitServicePointers )( 
            IMFTopologyServiceLookupClient * This,
            /* [annotation][in] */ 
            _In_  IMFTopologyServiceLookup *pLookup);
        
        DECLSPEC_XFGVIRT(IMFTopologyServiceLookupClient, ReleaseServicePointers)
        HRESULT ( STDMETHODCALLTYPE *ReleaseServicePointers )( 
            IMFTopologyServiceLookupClient * This);
        
        END_INTERFACE
    } IMFTopologyServiceLookupClientVtbl;

    interface IMFTopologyServiceLookupClient
    {
        CONST_VTBL struct IMFTopologyServiceLookupClientVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTopologyServiceLookupClient_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTopologyServiceLookupClient_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTopologyServiceLookupClient_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTopologyServiceLookupClient_InitServicePointers(This,pLookup)	\
    ( (This)->lpVtbl -> InitServicePointers(This,pLookup) ) 

#define IMFTopologyServiceLookupClient_ReleaseServicePointers(This)	\
    ( (This)->lpVtbl -> ReleaseServicePointers(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTopologyServiceLookupClient_INTERFACE_DEFINED__ */


#ifndef __IEVRTrustedVideoPlugin_INTERFACE_DEFINED__
#define __IEVRTrustedVideoPlugin_INTERFACE_DEFINED__

/* interface IEVRTrustedVideoPlugin */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEVRTrustedVideoPlugin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("83A4CE40-7710-494b-A893-A472049AF630")
    IEVRTrustedVideoPlugin : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsInTrustedVideoMode( 
            /* [out] */ BOOL *pYes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CanConstrict( 
            /* [out] */ BOOL *pYes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetConstriction( 
            DWORD dwKPix) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisableImageExport( 
            BOOL bDisable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEVRTrustedVideoPluginVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEVRTrustedVideoPlugin * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEVRTrustedVideoPlugin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEVRTrustedVideoPlugin * This);
        
        DECLSPEC_XFGVIRT(IEVRTrustedVideoPlugin, IsInTrustedVideoMode)
        HRESULT ( STDMETHODCALLTYPE *IsInTrustedVideoMode )( 
            IEVRTrustedVideoPlugin * This,
            /* [out] */ BOOL *pYes);
        
        DECLSPEC_XFGVIRT(IEVRTrustedVideoPlugin, CanConstrict)
        HRESULT ( STDMETHODCALLTYPE *CanConstrict )( 
            IEVRTrustedVideoPlugin * This,
            /* [out] */ BOOL *pYes);
        
        DECLSPEC_XFGVIRT(IEVRTrustedVideoPlugin, SetConstriction)
        HRESULT ( STDMETHODCALLTYPE *SetConstriction )( 
            IEVRTrustedVideoPlugin * This,
            DWORD dwKPix);
        
        DECLSPEC_XFGVIRT(IEVRTrustedVideoPlugin, DisableImageExport)
        HRESULT ( STDMETHODCALLTYPE *DisableImageExport )( 
            IEVRTrustedVideoPlugin * This,
            BOOL bDisable);
        
        END_INTERFACE
    } IEVRTrustedVideoPluginVtbl;

    interface IEVRTrustedVideoPlugin
    {
        CONST_VTBL struct IEVRTrustedVideoPluginVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEVRTrustedVideoPlugin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEVRTrustedVideoPlugin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEVRTrustedVideoPlugin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEVRTrustedVideoPlugin_IsInTrustedVideoMode(This,pYes)	\
    ( (This)->lpVtbl -> IsInTrustedVideoMode(This,pYes) ) 

#define IEVRTrustedVideoPlugin_CanConstrict(This,pYes)	\
    ( (This)->lpVtbl -> CanConstrict(This,pYes) ) 

#define IEVRTrustedVideoPlugin_SetConstriction(This,dwKPix)	\
    ( (This)->lpVtbl -> SetConstriction(This,dwKPix) ) 

#define IEVRTrustedVideoPlugin_DisableImageExport(This,bDisable)	\
    ( (This)->lpVtbl -> DisableImageExport(This,bDisable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEVRTrustedVideoPlugin_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_evr_0000_0013 */
/* [local] */ 

#ifndef MFEVRDLL
#define MFEVRDLL 0
#endif
#if MFEVRDLL
#define EVRPUBLIC(func) STDAPI _##func
#else
#define EVRPUBLIC(func) STDAPI func
#endif
EVRPUBLIC(MFCreateVideoPresenter)(
    _In_opt_ IUnknown * pOwner,
    REFIID riidDevice,
    REFIID riid,
    _Outptr_result_maybenull_ void ** ppVideoPresenter
    );
EVRPUBLIC(MFCreateVideoMixer)(
    _In_opt_ IUnknown * pOwner,
    REFIID riidDevice,
    REFIID riid,
    _Outptr_ void ** ppv
    );
EVRPUBLIC(MFCreateVideoMixerAndPresenter)(
    _In_opt_ IUnknown * pMixerOwner,
    _In_opt_ IUnknown * pPresenterOwner,
    REFIID riidMixer,
    _Outptr_ void ** ppvVideoMixer, 
    REFIID riidPresenter,
    _Outptr_ void ** ppvVideoPresenter
    );
EVRPUBLIC(MFCreateVideoRenderer)(
    REFIID riidRenderer,
    _Outptr_result_maybenull_ void ** ppVideoRenderer
    );
EVRPUBLIC(MFCreateVideoSampleFromSurface)(
    _In_ IUnknown* pUnkSurface,
    _Out_ IMFSample** ppSample
    );
EVRPUBLIC(MFCreateVideoSampleAllocator)(
    _In_ REFIID riid,
    _Outptr_ void** ppSampleAllocator
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_evr_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_evr_0000_0013_v0_0_s_ifspec;

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


