

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */

#pragma warning( disable: 4049 )  /* more than 64k source lines */


/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 475
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

#ifndef __effectspri_h__
#define __effectspri_h__

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

#ifndef __IWMPEffects_FWD_DEFINED__
#define __IWMPEffects_FWD_DEFINED__
typedef interface IWMPEffects IWMPEffects;

#endif 	/* __IWMPEffects_FWD_DEFINED__ */


#ifndef __IWMPEffects2_FWD_DEFINED__
#define __IWMPEffects2_FWD_DEFINED__
typedef interface IWMPEffects2 IWMPEffects2;

#endif 	/* __IWMPEffects2_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "wmp.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_effectspri_0000_0000 */
/* [local] */ 

//=========================================================================
//
// Microsoft Windows Media Technologies
// Copyright (C) Microsoft Corporation. All rights reserved.
//
//=========================================================================
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define	EFFECT_CANGOFULLSCREEN	( 0x1 )

#define	EFFECT_HASPROPERTYPAGE	( 0x2 )

#define	EFFECT_VARIABLEFREQSTEP	( 0x4 )

#define	EFFECT_WINDOWEDONLY	( 0x8 )

#define	EFFECT2_FULLSCREENEXCLUSIVE	( 0x10 )

#define	SA_BUFFER_SIZE	( 1024 )


enum PlayerState
    {
        stop_state	= 0,
        pause_state	= 1,
        play_state	= 2
    } ;

//**********************************************************************
// Define the minimum and maximum frequency ranges returned in our
// TimedLevel frequency array (i.e. first index in TimedLevel.frequency
// is at 20Hz and last is at 22050Hz).
//**********************************************************************
static const float kfltTimedLevelMaximumFrequency = 22050.0F;
static const float kfltTimedLevelMinimumFrequency = 20.0F;

/*
 * FREQUENCY_INDEX() returns the index into TimedLevel.frequency[] where 
 * the specified frequency is located in the power spectrum
 */
#define FREQUENCY_INDEX(FREQ)\
  (int)(((FREQ) - kfltTimedLevelMinimumFrequency) /\
    (((kfltTimedLevelMaximumFrequency - kfltTimedLevelMinimumFrequency) / SA_BUFFER_SIZE)))

typedef struct tagTimedLevel
    {
    unsigned char frequency[ 2 ][ 1024 ];
    unsigned char waveform[ 2 ][ 1024 ];
    int state;
    hyper timeStamp;
    } 	TimedLevel;



extern RPC_IF_HANDLE __MIDL_itf_effectspri_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_effectspri_0000_0000_v0_0_s_ifspec;

#ifndef __IWMPEffects_INTERFACE_DEFINED__
#define __IWMPEffects_INTERFACE_DEFINED__

/* interface IWMPEffects */
/* [oleautomation][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPEffects;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D3984C13-C3CB-48e2-8BE5-5168340B4F35")
    IWMPEffects : public IUnknown
    {
    public:
        virtual /* [helpstring][local] */ HRESULT STDMETHODCALLTYPE Render( 
            /* [in] */ TimedLevel *pLevels,
            /* [in] */ HDC hdc,
            /* [in] */ RECT *prc) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE MediaInfo( 
            /* [in] */ LONG lChannelCount,
            /* [in] */ LONG lSampleRate,
            /* [in] */ BSTR bstrTitle) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetCapabilities( 
            /* [out] */ DWORD *pdwCapabilities) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetTitle( 
            /* [out] */ BSTR *bstrTitle) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPresetTitle( 
            /* [in] */ LONG nPreset,
            /* [out] */ BSTR *bstrPresetTitle) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPresetCount( 
            /* [out] */ LONG *pnPresetCount) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetCurrentPreset( 
            /* [in] */ LONG nPreset) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetCurrentPreset( 
            /* [out] */ LONG *pnPreset) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DisplayPropertyPage( 
            /* [in] */ HWND hwndOwner) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GoFullscreen( 
            /* [in] */ BOOL fFullScreen) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RenderFullScreen( 
            /* [in] */ TimedLevel *pLevels) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPEffectsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPEffects * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPEffects * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPEffects * This);
        
        DECLSPEC_XFGVIRT(IWMPEffects, Render)
        /* [helpstring][local] */ HRESULT ( STDMETHODCALLTYPE *Render )( 
            IWMPEffects * This,
            /* [in] */ TimedLevel *pLevels,
            /* [in] */ HDC hdc,
            /* [in] */ RECT *prc);
        
        DECLSPEC_XFGVIRT(IWMPEffects, MediaInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MediaInfo )( 
            IWMPEffects * This,
            /* [in] */ LONG lChannelCount,
            /* [in] */ LONG lSampleRate,
            /* [in] */ BSTR bstrTitle);
        
        DECLSPEC_XFGVIRT(IWMPEffects, GetCapabilities)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCapabilities )( 
            IWMPEffects * This,
            /* [out] */ DWORD *pdwCapabilities);
        
        DECLSPEC_XFGVIRT(IWMPEffects, GetTitle)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTitle )( 
            IWMPEffects * This,
            /* [out] */ BSTR *bstrTitle);
        
        DECLSPEC_XFGVIRT(IWMPEffects, GetPresetTitle)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPresetTitle )( 
            IWMPEffects * This,
            /* [in] */ LONG nPreset,
            /* [out] */ BSTR *bstrPresetTitle);
        
        DECLSPEC_XFGVIRT(IWMPEffects, GetPresetCount)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPresetCount )( 
            IWMPEffects * This,
            /* [out] */ LONG *pnPresetCount);
        
        DECLSPEC_XFGVIRT(IWMPEffects, SetCurrentPreset)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetCurrentPreset )( 
            IWMPEffects * This,
            /* [in] */ LONG nPreset);
        
        DECLSPEC_XFGVIRT(IWMPEffects, GetCurrentPreset)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCurrentPreset )( 
            IWMPEffects * This,
            /* [out] */ LONG *pnPreset);
        
        DECLSPEC_XFGVIRT(IWMPEffects, DisplayPropertyPage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DisplayPropertyPage )( 
            IWMPEffects * This,
            /* [in] */ HWND hwndOwner);
        
        DECLSPEC_XFGVIRT(IWMPEffects, GoFullscreen)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GoFullscreen )( 
            IWMPEffects * This,
            /* [in] */ BOOL fFullScreen);
        
        DECLSPEC_XFGVIRT(IWMPEffects, RenderFullScreen)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RenderFullScreen )( 
            IWMPEffects * This,
            /* [in] */ TimedLevel *pLevels);
        
        END_INTERFACE
    } IWMPEffectsVtbl;

    interface IWMPEffects
    {
        CONST_VTBL struct IWMPEffectsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPEffects_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPEffects_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPEffects_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPEffects_Render(This,pLevels,hdc,prc)	\
    ( (This)->lpVtbl -> Render(This,pLevels,hdc,prc) ) 

#define IWMPEffects_MediaInfo(This,lChannelCount,lSampleRate,bstrTitle)	\
    ( (This)->lpVtbl -> MediaInfo(This,lChannelCount,lSampleRate,bstrTitle) ) 

#define IWMPEffects_GetCapabilities(This,pdwCapabilities)	\
    ( (This)->lpVtbl -> GetCapabilities(This,pdwCapabilities) ) 

#define IWMPEffects_GetTitle(This,bstrTitle)	\
    ( (This)->lpVtbl -> GetTitle(This,bstrTitle) ) 

#define IWMPEffects_GetPresetTitle(This,nPreset,bstrPresetTitle)	\
    ( (This)->lpVtbl -> GetPresetTitle(This,nPreset,bstrPresetTitle) ) 

#define IWMPEffects_GetPresetCount(This,pnPresetCount)	\
    ( (This)->lpVtbl -> GetPresetCount(This,pnPresetCount) ) 

#define IWMPEffects_SetCurrentPreset(This,nPreset)	\
    ( (This)->lpVtbl -> SetCurrentPreset(This,nPreset) ) 

#define IWMPEffects_GetCurrentPreset(This,pnPreset)	\
    ( (This)->lpVtbl -> GetCurrentPreset(This,pnPreset) ) 

#define IWMPEffects_DisplayPropertyPage(This,hwndOwner)	\
    ( (This)->lpVtbl -> DisplayPropertyPage(This,hwndOwner) ) 

#define IWMPEffects_GoFullscreen(This,fFullScreen)	\
    ( (This)->lpVtbl -> GoFullscreen(This,fFullScreen) ) 

#define IWMPEffects_RenderFullScreen(This,pLevels)	\
    ( (This)->lpVtbl -> RenderFullScreen(This,pLevels) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPEffects_INTERFACE_DEFINED__ */


#ifndef __IWMPEffects2_INTERFACE_DEFINED__
#define __IWMPEffects2_INTERFACE_DEFINED__

/* interface IWMPEffects2 */
/* [oleautomation][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPEffects2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("695386EC-AA3C-4618-A5E1-DD9A8B987632")
    IWMPEffects2 : public IWMPEffects
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetCore( 
            /* [in] */ IWMPCore *pPlayer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ HWND hwndParent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Destroy( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyNewMedia( 
            /* [in] */ IWMPMedia *pMedia) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnWindowMessage( 
            /* [in] */ UINT msg,
            /* [in] */ WPARAM WParam,
            /* [in] */ LPARAM LParam,
            /* [in] */ LRESULT *plResultParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RenderWindowed( 
            /* [in] */ TimedLevel *pData,
            /* [in] */ BOOL fRequiredRender) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPEffects2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPEffects2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPEffects2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPEffects2 * This);
        
        DECLSPEC_XFGVIRT(IWMPEffects, Render)
        /* [helpstring][local] */ HRESULT ( STDMETHODCALLTYPE *Render )( 
            IWMPEffects2 * This,
            /* [in] */ TimedLevel *pLevels,
            /* [in] */ HDC hdc,
            /* [in] */ RECT *prc);
        
        DECLSPEC_XFGVIRT(IWMPEffects, MediaInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MediaInfo )( 
            IWMPEffects2 * This,
            /* [in] */ LONG lChannelCount,
            /* [in] */ LONG lSampleRate,
            /* [in] */ BSTR bstrTitle);
        
        DECLSPEC_XFGVIRT(IWMPEffects, GetCapabilities)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCapabilities )( 
            IWMPEffects2 * This,
            /* [out] */ DWORD *pdwCapabilities);
        
        DECLSPEC_XFGVIRT(IWMPEffects, GetTitle)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTitle )( 
            IWMPEffects2 * This,
            /* [out] */ BSTR *bstrTitle);
        
        DECLSPEC_XFGVIRT(IWMPEffects, GetPresetTitle)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPresetTitle )( 
            IWMPEffects2 * This,
            /* [in] */ LONG nPreset,
            /* [out] */ BSTR *bstrPresetTitle);
        
        DECLSPEC_XFGVIRT(IWMPEffects, GetPresetCount)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPresetCount )( 
            IWMPEffects2 * This,
            /* [out] */ LONG *pnPresetCount);
        
        DECLSPEC_XFGVIRT(IWMPEffects, SetCurrentPreset)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetCurrentPreset )( 
            IWMPEffects2 * This,
            /* [in] */ LONG nPreset);
        
        DECLSPEC_XFGVIRT(IWMPEffects, GetCurrentPreset)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCurrentPreset )( 
            IWMPEffects2 * This,
            /* [out] */ LONG *pnPreset);
        
        DECLSPEC_XFGVIRT(IWMPEffects, DisplayPropertyPage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DisplayPropertyPage )( 
            IWMPEffects2 * This,
            /* [in] */ HWND hwndOwner);
        
        DECLSPEC_XFGVIRT(IWMPEffects, GoFullscreen)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GoFullscreen )( 
            IWMPEffects2 * This,
            /* [in] */ BOOL fFullScreen);
        
        DECLSPEC_XFGVIRT(IWMPEffects, RenderFullScreen)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RenderFullScreen )( 
            IWMPEffects2 * This,
            /* [in] */ TimedLevel *pLevels);
        
        DECLSPEC_XFGVIRT(IWMPEffects2, SetCore)
        HRESULT ( STDMETHODCALLTYPE *SetCore )( 
            IWMPEffects2 * This,
            /* [in] */ IWMPCore *pPlayer);
        
        DECLSPEC_XFGVIRT(IWMPEffects2, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            IWMPEffects2 * This,
            /* [in] */ HWND hwndParent);
        
        DECLSPEC_XFGVIRT(IWMPEffects2, Destroy)
        HRESULT ( STDMETHODCALLTYPE *Destroy )( 
            IWMPEffects2 * This);
        
        DECLSPEC_XFGVIRT(IWMPEffects2, NotifyNewMedia)
        HRESULT ( STDMETHODCALLTYPE *NotifyNewMedia )( 
            IWMPEffects2 * This,
            /* [in] */ IWMPMedia *pMedia);
        
        DECLSPEC_XFGVIRT(IWMPEffects2, OnWindowMessage)
        HRESULT ( STDMETHODCALLTYPE *OnWindowMessage )( 
            IWMPEffects2 * This,
            /* [in] */ UINT msg,
            /* [in] */ WPARAM WParam,
            /* [in] */ LPARAM LParam,
            /* [in] */ LRESULT *plResultParam);
        
        DECLSPEC_XFGVIRT(IWMPEffects2, RenderWindowed)
        HRESULT ( STDMETHODCALLTYPE *RenderWindowed )( 
            IWMPEffects2 * This,
            /* [in] */ TimedLevel *pData,
            /* [in] */ BOOL fRequiredRender);
        
        END_INTERFACE
    } IWMPEffects2Vtbl;

    interface IWMPEffects2
    {
        CONST_VTBL struct IWMPEffects2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPEffects2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPEffects2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPEffects2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPEffects2_Render(This,pLevels,hdc,prc)	\
    ( (This)->lpVtbl -> Render(This,pLevels,hdc,prc) ) 

#define IWMPEffects2_MediaInfo(This,lChannelCount,lSampleRate,bstrTitle)	\
    ( (This)->lpVtbl -> MediaInfo(This,lChannelCount,lSampleRate,bstrTitle) ) 

#define IWMPEffects2_GetCapabilities(This,pdwCapabilities)	\
    ( (This)->lpVtbl -> GetCapabilities(This,pdwCapabilities) ) 

#define IWMPEffects2_GetTitle(This,bstrTitle)	\
    ( (This)->lpVtbl -> GetTitle(This,bstrTitle) ) 

#define IWMPEffects2_GetPresetTitle(This,nPreset,bstrPresetTitle)	\
    ( (This)->lpVtbl -> GetPresetTitle(This,nPreset,bstrPresetTitle) ) 

#define IWMPEffects2_GetPresetCount(This,pnPresetCount)	\
    ( (This)->lpVtbl -> GetPresetCount(This,pnPresetCount) ) 

#define IWMPEffects2_SetCurrentPreset(This,nPreset)	\
    ( (This)->lpVtbl -> SetCurrentPreset(This,nPreset) ) 

#define IWMPEffects2_GetCurrentPreset(This,pnPreset)	\
    ( (This)->lpVtbl -> GetCurrentPreset(This,pnPreset) ) 

#define IWMPEffects2_DisplayPropertyPage(This,hwndOwner)	\
    ( (This)->lpVtbl -> DisplayPropertyPage(This,hwndOwner) ) 

#define IWMPEffects2_GoFullscreen(This,fFullScreen)	\
    ( (This)->lpVtbl -> GoFullscreen(This,fFullScreen) ) 

#define IWMPEffects2_RenderFullScreen(This,pLevels)	\
    ( (This)->lpVtbl -> RenderFullScreen(This,pLevels) ) 


#define IWMPEffects2_SetCore(This,pPlayer)	\
    ( (This)->lpVtbl -> SetCore(This,pPlayer) ) 

#define IWMPEffects2_Create(This,hwndParent)	\
    ( (This)->lpVtbl -> Create(This,hwndParent) ) 

#define IWMPEffects2_Destroy(This)	\
    ( (This)->lpVtbl -> Destroy(This) ) 

#define IWMPEffects2_NotifyNewMedia(This,pMedia)	\
    ( (This)->lpVtbl -> NotifyNewMedia(This,pMedia) ) 

#define IWMPEffects2_OnWindowMessage(This,msg,WParam,LParam,plResultParam)	\
    ( (This)->lpVtbl -> OnWindowMessage(This,msg,WParam,LParam,plResultParam) ) 

#define IWMPEffects2_RenderWindowed(This,pData,fRequiredRender)	\
    ( (This)->lpVtbl -> RenderWindowed(This,pData,fRequiredRender) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPEffects2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_effectspri_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_effectspri_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_effectspri_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     unsigned long *, unsigned long            , BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  unsigned long *, unsigned char *, BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(unsigned long *, unsigned char *, BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     unsigned long *, BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize(     unsigned long *, unsigned long            , HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  unsigned long *, unsigned char *, HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(unsigned long *, unsigned char *, HWND * ); 
void                      __RPC_USER  HWND_UserFree(     unsigned long *, HWND * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


