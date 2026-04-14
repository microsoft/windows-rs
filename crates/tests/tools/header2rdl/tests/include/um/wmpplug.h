

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

#ifndef __wmpplugpri_h__
#define __wmpplugpri_h__

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

#ifndef __IWMPPluginUI_FWD_DEFINED__
#define __IWMPPluginUI_FWD_DEFINED__
typedef interface IWMPPluginUI IWMPPluginUI;

#endif 	/* __IWMPPluginUI_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "wmp.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wmpplugpri_0000_0000 */
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
#define PLUGIN_INSTALLREGKEY                L"Software\\Microsoft\\MediaPlayer\\UIPlugins"
#define PLUGIN_INSTALLREGKEY_FRIENDLYNAME   L"FriendlyName"
#define PLUGIN_INSTALLREGKEY_DESCRIPTION    L"Description"
#define PLUGIN_INSTALLREGKEY_CAPABILITIES   L"Capabilities"
#define PLUGIN_INSTALLREGKEY_UNINSTALL      L"UninstallPath"
#define	PLUGIN_TYPE_BACKGROUND	( 0x1 )

#define	PLUGIN_TYPE_SEPARATEWINDOW	( 0x2 )

#define	PLUGIN_TYPE_DISPLAYAREA	( 0x3 )

#define	PLUGIN_TYPE_SETTINGSAREA	( 0x4 )

#define	PLUGIN_TYPE_METADATAAREA	( 0x5 )

#define	PLUGIN_FLAGS_HASPROPERTYPAGE	( 0x80000000 )

#define	PLUGIN_FLAGS_INSTALLAUTORUN	( 0x40000000 )

#define	PLUGIN_FLAGS_LAUNCHPROPERTYPAGE	( 0x20000000 )

#define	PLUGIN_FLAGS_ACCEPTSMEDIA	( 0x10000000 )

#define	PLUGIN_FLAGS_ACCEPTSPLAYLISTS	( 0x8000000 )

#define	PLUGIN_FLAGS_HASPRESETS	( 0x4000000 )

#define	PLUGIN_FLAGS_HIDDEN	( 0x2000000 )

#define PLUGIN_MISC_PRESETCOUNT      L"PresetCount"
#define PLUGIN_MISC_PRESETNAMES      L"PresetNames"
#define PLUGIN_MISC_CURRENTPRESET    L"CurrentPreset"
#define PLUGIN_SEPARATEWINDOW_RESIZABLE     L"Resizable"
#define PLUGIN_SEPARATEWINDOW_DEFAULTWIDTH  L"DefaultWidth"
#define PLUGIN_SEPARATEWINDOW_DEFAULTHEIGHT L"DefaultHeight"
#define PLUGIN_SEPARATEWINDOW_MINWIDTH      L"MinWidth"
#define PLUGIN_SEPARATEWINDOW_MINHEIGHT     L"MinHeight"
#define PLUGIN_SEPARATEWINDOW_MAXWIDTH      L"MaxWidth"
#define PLUGIN_SEPARATEWINDOW_MAXHEIGHT     L"MaxHeight"
#define PLUGIN_MISC_QUERYDESTROY            L"QueryDestroy"
#define PLUGIN_ALL_MEDIASENDTO              L"MediaSendTo"
#define PLUGIN_ALL_PLAYLISTSENDTO           L"PlaylistSendTo"
__inline BOOL WMPNotifyPluginAddRemove()
{
    return( ::PostMessage( HWND_BROADCAST, ::RegisterWindowMessageA( "WMPlayer_PluginAddRemove" ), 0, 0 ) );
}


extern RPC_IF_HANDLE __MIDL_itf_wmpplugpri_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmpplugpri_0000_0000_v0_0_s_ifspec;

#ifndef __IWMPPluginUI_INTERFACE_DEFINED__
#define __IWMPPluginUI_INTERFACE_DEFINED__

/* interface IWMPPluginUI */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPPluginUI;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4C5E8F9F-AD3E-4bf9-9753-FCD30D6D38DD")
    IWMPPluginUI : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetCore( 
            /* [in] */ IWMPCore *pCore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ HWND hwndParent,
            /* [out] */ HWND *phwndWindow) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Destroy( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisplayPropertyPage( 
            /* [in] */ HWND hwndParent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ const WCHAR *pwszName,
            /* [out] */ VARIANT *pvarProperty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ const WCHAR *pwszName,
            /* [in] */ const VARIANT *pvarProperty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TranslateAccelerator( 
            /* [in] */ LPMSG lpmsg) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPPluginUIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPPluginUI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPPluginUI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPPluginUI * This);
        
        DECLSPEC_XFGVIRT(IWMPPluginUI, SetCore)
        HRESULT ( STDMETHODCALLTYPE *SetCore )( 
            IWMPPluginUI * This,
            /* [in] */ IWMPCore *pCore);
        
        DECLSPEC_XFGVIRT(IWMPPluginUI, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            IWMPPluginUI * This,
            /* [in] */ HWND hwndParent,
            /* [out] */ HWND *phwndWindow);
        
        DECLSPEC_XFGVIRT(IWMPPluginUI, Destroy)
        HRESULT ( STDMETHODCALLTYPE *Destroy )( 
            IWMPPluginUI * This);
        
        DECLSPEC_XFGVIRT(IWMPPluginUI, DisplayPropertyPage)
        HRESULT ( STDMETHODCALLTYPE *DisplayPropertyPage )( 
            IWMPPluginUI * This,
            /* [in] */ HWND hwndParent);
        
        DECLSPEC_XFGVIRT(IWMPPluginUI, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            IWMPPluginUI * This,
            /* [in] */ const WCHAR *pwszName,
            /* [out] */ VARIANT *pvarProperty);
        
        DECLSPEC_XFGVIRT(IWMPPluginUI, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            IWMPPluginUI * This,
            /* [in] */ const WCHAR *pwszName,
            /* [in] */ const VARIANT *pvarProperty);
        
        DECLSPEC_XFGVIRT(IWMPPluginUI, TranslateAccelerator)
        HRESULT ( STDMETHODCALLTYPE *TranslateAccelerator )( 
            IWMPPluginUI * This,
            /* [in] */ LPMSG lpmsg);
        
        END_INTERFACE
    } IWMPPluginUIVtbl;

    interface IWMPPluginUI
    {
        CONST_VTBL struct IWMPPluginUIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPPluginUI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPPluginUI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPPluginUI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPPluginUI_SetCore(This,pCore)	\
    ( (This)->lpVtbl -> SetCore(This,pCore) ) 

#define IWMPPluginUI_Create(This,hwndParent,phwndWindow)	\
    ( (This)->lpVtbl -> Create(This,hwndParent,phwndWindow) ) 

#define IWMPPluginUI_Destroy(This)	\
    ( (This)->lpVtbl -> Destroy(This) ) 

#define IWMPPluginUI_DisplayPropertyPage(This,hwndParent)	\
    ( (This)->lpVtbl -> DisplayPropertyPage(This,hwndParent) ) 

#define IWMPPluginUI_GetProperty(This,pwszName,pvarProperty)	\
    ( (This)->lpVtbl -> GetProperty(This,pwszName,pvarProperty) ) 

#define IWMPPluginUI_SetProperty(This,pwszName,pvarProperty)	\
    ( (This)->lpVtbl -> SetProperty(This,pwszName,pvarProperty) ) 

#define IWMPPluginUI_TranslateAccelerator(This,lpmsg)	\
    ( (This)->lpVtbl -> TranslateAccelerator(This,lpmsg) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPPluginUI_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wmpplugpri_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wmpplugpri_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmpplugpri_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  HWND_UserSize(     unsigned long *, unsigned long            , HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  unsigned long *, unsigned char *, HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(unsigned long *, unsigned char *, HWND * ); 
void                      __RPC_USER  HWND_UserFree(     unsigned long *, HWND * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     unsigned long *, unsigned long            , VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  unsigned long *, unsigned char *, VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(unsigned long *, unsigned char *, VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     unsigned long *, VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


