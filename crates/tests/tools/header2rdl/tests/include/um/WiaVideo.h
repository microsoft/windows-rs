

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

#ifndef __wiavideo_xp_h__
#define __wiavideo_xp_h__

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

#ifndef __IWiaVideo_FWD_DEFINED__
#define __IWiaVideo_FWD_DEFINED__
typedef interface IWiaVideo IWiaVideo;

#endif 	/* __IWiaVideo_FWD_DEFINED__ */


#ifndef __WiaVideo_FWD_DEFINED__
#define __WiaVideo_FWD_DEFINED__

#ifdef __cplusplus
typedef class WiaVideo WiaVideo;
#else
typedef struct WiaVideo WiaVideo;
#endif /* __cplusplus */

#endif 	/* __WiaVideo_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wiavideo_xp_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_wiavideo_xp_0000_0000_0001
    {
        WIAVIDEO_NO_VIDEO	= 1,
        WIAVIDEO_CREATING_VIDEO	= 2,
        WIAVIDEO_VIDEO_CREATED	= 3,
        WIAVIDEO_VIDEO_PLAYING	= 4,
        WIAVIDEO_VIDEO_PAUSED	= 5,
        WIAVIDEO_DESTROYING_VIDEO	= 6
    } 	WIAVIDEO_STATE;



extern RPC_IF_HANDLE __MIDL_itf_wiavideo_xp_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wiavideo_xp_0000_0000_v0_0_s_ifspec;

#ifndef __IWiaVideo_INTERFACE_DEFINED__
#define __IWiaVideo_INTERFACE_DEFINED__

/* interface IWiaVideo */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWiaVideo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D52920AA-DB88-41F0-946C-E00DC0A19CFA")
    IWiaVideo : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PreviewVisible( 
            /* [retval][out] */ __RPC__out BOOL *pbPreviewVisible) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PreviewVisible( 
            /* [in] */ BOOL bPreviewVisible) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ImagesDirectory( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrImageDirectory) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ImagesDirectory( 
            /* [in] */ __RPC__in BSTR bstrImageDirectory) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateVideoByWiaDevID( 
            /* [in] */ __RPC__in BSTR bstrWiaDeviceID,
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ BOOL bStretchToFitParent,
            /* [in] */ BOOL bAutoBeginPlayback) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateVideoByDevNum( 
            /* [in] */ UINT uiDeviceNumber,
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ BOOL bStretchToFitParent,
            /* [in] */ BOOL bAutoBeginPlayback) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateVideoByName( 
            /* [in] */ __RPC__in BSTR bstrFriendlyName,
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ BOOL bStretchToFitParent,
            /* [in] */ BOOL bAutoBeginPlayback) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DestroyVideo( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Play( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE TakePicture( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrNewImageFilename) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ResizeVideo( 
            /* [in] */ BOOL bStretchToFitParent) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCurrentState( 
            /* [retval][out] */ __RPC__out WIAVIDEO_STATE *pState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWiaVideoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWiaVideo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWiaVideo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWiaVideo * This);
        
        DECLSPEC_XFGVIRT(IWiaVideo, get_PreviewVisible)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreviewVisible )( 
            __RPC__in IWiaVideo * This,
            /* [retval][out] */ __RPC__out BOOL *pbPreviewVisible);
        
        DECLSPEC_XFGVIRT(IWiaVideo, put_PreviewVisible)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PreviewVisible )( 
            __RPC__in IWiaVideo * This,
            /* [in] */ BOOL bPreviewVisible);
        
        DECLSPEC_XFGVIRT(IWiaVideo, get_ImagesDirectory)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ImagesDirectory )( 
            __RPC__in IWiaVideo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrImageDirectory);
        
        DECLSPEC_XFGVIRT(IWiaVideo, put_ImagesDirectory)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ImagesDirectory )( 
            __RPC__in IWiaVideo * This,
            /* [in] */ __RPC__in BSTR bstrImageDirectory);
        
        DECLSPEC_XFGVIRT(IWiaVideo, CreateVideoByWiaDevID)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateVideoByWiaDevID )( 
            __RPC__in IWiaVideo * This,
            /* [in] */ __RPC__in BSTR bstrWiaDeviceID,
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ BOOL bStretchToFitParent,
            /* [in] */ BOOL bAutoBeginPlayback);
        
        DECLSPEC_XFGVIRT(IWiaVideo, CreateVideoByDevNum)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateVideoByDevNum )( 
            __RPC__in IWiaVideo * This,
            /* [in] */ UINT uiDeviceNumber,
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ BOOL bStretchToFitParent,
            /* [in] */ BOOL bAutoBeginPlayback);
        
        DECLSPEC_XFGVIRT(IWiaVideo, CreateVideoByName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateVideoByName )( 
            __RPC__in IWiaVideo * This,
            /* [in] */ __RPC__in BSTR bstrFriendlyName,
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ BOOL bStretchToFitParent,
            /* [in] */ BOOL bAutoBeginPlayback);
        
        DECLSPEC_XFGVIRT(IWiaVideo, DestroyVideo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DestroyVideo )( 
            __RPC__in IWiaVideo * This);
        
        DECLSPEC_XFGVIRT(IWiaVideo, Play)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Play )( 
            __RPC__in IWiaVideo * This);
        
        DECLSPEC_XFGVIRT(IWiaVideo, Pause)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in IWiaVideo * This);
        
        DECLSPEC_XFGVIRT(IWiaVideo, TakePicture)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *TakePicture )( 
            __RPC__in IWiaVideo * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrNewImageFilename);
        
        DECLSPEC_XFGVIRT(IWiaVideo, ResizeVideo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ResizeVideo )( 
            __RPC__in IWiaVideo * This,
            /* [in] */ BOOL bStretchToFitParent);
        
        DECLSPEC_XFGVIRT(IWiaVideo, GetCurrentState)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCurrentState )( 
            __RPC__in IWiaVideo * This,
            /* [retval][out] */ __RPC__out WIAVIDEO_STATE *pState);
        
        END_INTERFACE
    } IWiaVideoVtbl;

    interface IWiaVideo
    {
        CONST_VTBL struct IWiaVideoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWiaVideo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWiaVideo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWiaVideo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWiaVideo_get_PreviewVisible(This,pbPreviewVisible)	\
    ( (This)->lpVtbl -> get_PreviewVisible(This,pbPreviewVisible) ) 

#define IWiaVideo_put_PreviewVisible(This,bPreviewVisible)	\
    ( (This)->lpVtbl -> put_PreviewVisible(This,bPreviewVisible) ) 

#define IWiaVideo_get_ImagesDirectory(This,pbstrImageDirectory)	\
    ( (This)->lpVtbl -> get_ImagesDirectory(This,pbstrImageDirectory) ) 

#define IWiaVideo_put_ImagesDirectory(This,bstrImageDirectory)	\
    ( (This)->lpVtbl -> put_ImagesDirectory(This,bstrImageDirectory) ) 

#define IWiaVideo_CreateVideoByWiaDevID(This,bstrWiaDeviceID,hwndParent,bStretchToFitParent,bAutoBeginPlayback)	\
    ( (This)->lpVtbl -> CreateVideoByWiaDevID(This,bstrWiaDeviceID,hwndParent,bStretchToFitParent,bAutoBeginPlayback) ) 

#define IWiaVideo_CreateVideoByDevNum(This,uiDeviceNumber,hwndParent,bStretchToFitParent,bAutoBeginPlayback)	\
    ( (This)->lpVtbl -> CreateVideoByDevNum(This,uiDeviceNumber,hwndParent,bStretchToFitParent,bAutoBeginPlayback) ) 

#define IWiaVideo_CreateVideoByName(This,bstrFriendlyName,hwndParent,bStretchToFitParent,bAutoBeginPlayback)	\
    ( (This)->lpVtbl -> CreateVideoByName(This,bstrFriendlyName,hwndParent,bStretchToFitParent,bAutoBeginPlayback) ) 

#define IWiaVideo_DestroyVideo(This)	\
    ( (This)->lpVtbl -> DestroyVideo(This) ) 

#define IWiaVideo_Play(This)	\
    ( (This)->lpVtbl -> Play(This) ) 

#define IWiaVideo_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IWiaVideo_TakePicture(This,pbstrNewImageFilename)	\
    ( (This)->lpVtbl -> TakePicture(This,pbstrNewImageFilename) ) 

#define IWiaVideo_ResizeVideo(This,bStretchToFitParent)	\
    ( (This)->lpVtbl -> ResizeVideo(This,bStretchToFitParent) ) 

#define IWiaVideo_GetCurrentState(This,pState)	\
    ( (This)->lpVtbl -> GetCurrentState(This,pState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWiaVideo_INTERFACE_DEFINED__ */



#ifndef __WIAVIDEOLib_LIBRARY_DEFINED__
#define __WIAVIDEOLib_LIBRARY_DEFINED__

/* library WIAVIDEOLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_WIAVIDEOLib;

EXTERN_C const CLSID CLSID_WiaVideo;

#ifdef __cplusplus

class DECLSPEC_UUID("3908C3CD-4478-4536-AF2F-10C25D4EF89A")
WiaVideo;
#endif
#endif /* __WIAVIDEOLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_wiavideo_xp_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wiavideo_xp_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wiavideo_xp_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


