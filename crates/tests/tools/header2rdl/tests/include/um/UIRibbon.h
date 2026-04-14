

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

#ifndef __UIRibbon_h__
#define __UIRibbon_h__

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

#ifndef __IUISimplePropertySet_FWD_DEFINED__
#define __IUISimplePropertySet_FWD_DEFINED__
typedef interface IUISimplePropertySet IUISimplePropertySet;

#endif 	/* __IUISimplePropertySet_FWD_DEFINED__ */


#ifndef __IUIRibbon_FWD_DEFINED__
#define __IUIRibbon_FWD_DEFINED__
typedef interface IUIRibbon IUIRibbon;

#endif 	/* __IUIRibbon_FWD_DEFINED__ */


#ifndef __IUIFramework_FWD_DEFINED__
#define __IUIFramework_FWD_DEFINED__
typedef interface IUIFramework IUIFramework;

#endif 	/* __IUIFramework_FWD_DEFINED__ */


#ifndef __IUIEventLogger_FWD_DEFINED__
#define __IUIEventLogger_FWD_DEFINED__
typedef interface IUIEventLogger IUIEventLogger;

#endif 	/* __IUIEventLogger_FWD_DEFINED__ */


#ifndef __IUIEventingManager_FWD_DEFINED__
#define __IUIEventingManager_FWD_DEFINED__
typedef interface IUIEventingManager IUIEventingManager;

#endif 	/* __IUIEventingManager_FWD_DEFINED__ */


#ifndef __IUIContextualUI_FWD_DEFINED__
#define __IUIContextualUI_FWD_DEFINED__
typedef interface IUIContextualUI IUIContextualUI;

#endif 	/* __IUIContextualUI_FWD_DEFINED__ */


#ifndef __IUICollection_FWD_DEFINED__
#define __IUICollection_FWD_DEFINED__
typedef interface IUICollection IUICollection;

#endif 	/* __IUICollection_FWD_DEFINED__ */


#ifndef __IUICollectionChangedEvent_FWD_DEFINED__
#define __IUICollectionChangedEvent_FWD_DEFINED__
typedef interface IUICollectionChangedEvent IUICollectionChangedEvent;

#endif 	/* __IUICollectionChangedEvent_FWD_DEFINED__ */


#ifndef __IUICommandHandler_FWD_DEFINED__
#define __IUICommandHandler_FWD_DEFINED__
typedef interface IUICommandHandler IUICommandHandler;

#endif 	/* __IUICommandHandler_FWD_DEFINED__ */


#ifndef __IUIApplication_FWD_DEFINED__
#define __IUIApplication_FWD_DEFINED__
typedef interface IUIApplication IUIApplication;

#endif 	/* __IUIApplication_FWD_DEFINED__ */


#ifndef __IUIImage_FWD_DEFINED__
#define __IUIImage_FWD_DEFINED__
typedef interface IUIImage IUIImage;

#endif 	/* __IUIImage_FWD_DEFINED__ */


#ifndef __IUIImageFromBitmap_FWD_DEFINED__
#define __IUIImageFromBitmap_FWD_DEFINED__
typedef interface IUIImageFromBitmap IUIImageFromBitmap;

#endif 	/* __IUIImageFromBitmap_FWD_DEFINED__ */


#ifndef __UIRibbonFramework_FWD_DEFINED__
#define __UIRibbonFramework_FWD_DEFINED__

#ifdef __cplusplus
typedef class UIRibbonFramework UIRibbonFramework;
#else
typedef struct UIRibbonFramework UIRibbonFramework;
#endif /* __cplusplus */

#endif 	/* __UIRibbonFramework_FWD_DEFINED__ */


#ifndef __UIRibbonImageFromBitmapFactory_FWD_DEFINED__
#define __UIRibbonImageFromBitmapFactory_FWD_DEFINED__

#ifdef __cplusplus
typedef class UIRibbonImageFromBitmapFactory UIRibbonImageFromBitmapFactory;
#else
typedef struct UIRibbonImageFromBitmapFactory UIRibbonImageFromBitmapFactory;
#endif /* __cplusplus */

#endif 	/* __UIRibbonImageFromBitmapFactory_FWD_DEFINED__ */


/* header files for imported files */
#include "propsys.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_UIRibbon_0000_0000 */
/* [local] */ 

//****************************************************************************
//
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//  File:       UIRibbon.h (Generated from UIRibbon.idl)
//
//  Contents:   Interfaces for the Windows Ribbon Framework
//
//****************************************************************************
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma warning(push)
#pragma warning(disable:4668) 
#pragma warning(disable:4001) 
#pragma once
#pragma warning(pop)
#include <UIRibbonKeyDef.h>
DEFINE_UIPROPERTYKEY(UI_PKEY_Enabled,                      VT_BOOL,                1); 
DEFINE_UIPROPERTYKEY(UI_PKEY_LabelDescription,             VT_LPWSTR,              2); 
DEFINE_UIPROPERTYKEY(UI_PKEY_Keytip,                       VT_LPWSTR,              3); 
DEFINE_UIPROPERTYKEY(UI_PKEY_Label,                        VT_LPWSTR,              4); 
DEFINE_UIPROPERTYKEY(UI_PKEY_TooltipDescription,           VT_LPWSTR,              5); 
DEFINE_UIPROPERTYKEY(UI_PKEY_TooltipTitle,                 VT_LPWSTR,              6); 
DEFINE_UIPROPERTYKEY(UI_PKEY_LargeImage,                   VT_UNKNOWN,             7); 
DEFINE_UIPROPERTYKEY(UI_PKEY_LargeHighContrastImage,       VT_UNKNOWN,             8); 
DEFINE_UIPROPERTYKEY(UI_PKEY_SmallImage,                   VT_UNKNOWN,             9); 
DEFINE_UIPROPERTYKEY(UI_PKEY_SmallHighContrastImage,       VT_UNKNOWN,             10); 
DEFINE_UIPROPERTYKEY(UI_PKEY_CommandId,                    VT_UI4,                 100); 
DEFINE_UIPROPERTYKEY(UI_PKEY_ItemsSource,                  VT_UNKNOWN,             101); 
DEFINE_UIPROPERTYKEY(UI_PKEY_Categories,                   VT_UNKNOWN,             102); 
DEFINE_UIPROPERTYKEY(UI_PKEY_CategoryId,                   VT_UI4,                 103); 
DEFINE_UIPROPERTYKEY(UI_PKEY_SelectedItem,                 VT_UI4,                 104); 
DEFINE_UIPROPERTYKEY(UI_PKEY_CommandType,                  VT_UI4,                 105); 
DEFINE_UIPROPERTYKEY(UI_PKEY_ItemImage,                    VT_UNKNOWN,             106); 
DEFINE_UIPROPERTYKEY(UI_PKEY_BooleanValue,                 VT_BOOL,                200); 
DEFINE_UIPROPERTYKEY(UI_PKEY_DecimalValue,                 VT_DECIMAL,             201); 
DEFINE_UIPROPERTYKEY(UI_PKEY_StringValue,                  VT_LPWSTR,              202); 
DEFINE_UIPROPERTYKEY(UI_PKEY_MaxValue,                     VT_DECIMAL,             203); 
DEFINE_UIPROPERTYKEY(UI_PKEY_MinValue,                     VT_DECIMAL,             204); 
DEFINE_UIPROPERTYKEY(UI_PKEY_Increment,                    VT_DECIMAL,             205); 
DEFINE_UIPROPERTYKEY(UI_PKEY_DecimalPlaces,                VT_UI4,                 206); 
DEFINE_UIPROPERTYKEY(UI_PKEY_FormatString,                 VT_LPWSTR,              207); 
DEFINE_UIPROPERTYKEY(UI_PKEY_RepresentativeString,         VT_LPWSTR,              208); 
DEFINE_UIPROPERTYKEY(UI_PKEY_FontProperties,                     VT_UNKNOWN,             300); 
DEFINE_UIPROPERTYKEY(UI_PKEY_FontProperties_Family,              VT_LPWSTR,              301); 
DEFINE_UIPROPERTYKEY(UI_PKEY_FontProperties_Size,                VT_DECIMAL,             302); 
DEFINE_UIPROPERTYKEY(UI_PKEY_FontProperties_Bold,                VT_UI4,                 303); 
DEFINE_UIPROPERTYKEY(UI_PKEY_FontProperties_Italic,              VT_UI4,                 304); 
DEFINE_UIPROPERTYKEY(UI_PKEY_FontProperties_Underline,           VT_UI4,                 305); 
DEFINE_UIPROPERTYKEY(UI_PKEY_FontProperties_Strikethrough,       VT_UI4,                 306); 
DEFINE_UIPROPERTYKEY(UI_PKEY_FontProperties_VerticalPositioning, VT_UI4,                 307); 
DEFINE_UIPROPERTYKEY(UI_PKEY_FontProperties_ForegroundColor,     VT_UI4,                 308); 
DEFINE_UIPROPERTYKEY(UI_PKEY_FontProperties_BackgroundColor,     VT_UI4,                 309); 
DEFINE_UIPROPERTYKEY(UI_PKEY_FontProperties_ForegroundColorType, VT_UI4,                 310); 
DEFINE_UIPROPERTYKEY(UI_PKEY_FontProperties_BackgroundColorType, VT_UI4,                 311); 
DEFINE_UIPROPERTYKEY(UI_PKEY_FontProperties_ChangedProperties,   VT_UNKNOWN,             312); 
DEFINE_UIPROPERTYKEY(UI_PKEY_FontProperties_DeltaSize,           VT_UI4,                 313); 
DEFINE_UIPROPERTYKEY(UI_PKEY_RecentItems,                  VT_ARRAY|VT_UNKNOWN,    350); 
DEFINE_UIPROPERTYKEY(UI_PKEY_Pinned,                       VT_BOOL,                351); 
DEFINE_UIPROPERTYKEY(UI_PKEY_Color,                        VT_UI4,                 400); 
DEFINE_UIPROPERTYKEY(UI_PKEY_ColorType,                    VT_UI4,                 401); 
DEFINE_UIPROPERTYKEY(UI_PKEY_ColorMode,                    VT_UI4,                 402); 
DEFINE_UIPROPERTYKEY(UI_PKEY_ThemeColorsCategoryLabel,     VT_LPWSTR,              403); 
DEFINE_UIPROPERTYKEY(UI_PKEY_StandardColorsCategoryLabel,  VT_LPWSTR,              404); 
DEFINE_UIPROPERTYKEY(UI_PKEY_RecentColorsCategoryLabel,    VT_LPWSTR,              405); 
DEFINE_UIPROPERTYKEY(UI_PKEY_AutomaticColorLabel,          VT_LPWSTR,              406); 
DEFINE_UIPROPERTYKEY(UI_PKEY_NoColorLabel,                 VT_LPWSTR,              407); 
DEFINE_UIPROPERTYKEY(UI_PKEY_MoreColorsLabel,              VT_LPWSTR,              408); 
DEFINE_UIPROPERTYKEY(UI_PKEY_ThemeColors,                  VT_VECTOR|VT_UI4,       409); 
DEFINE_UIPROPERTYKEY(UI_PKEY_StandardColors,               VT_VECTOR|VT_UI4,       410); 
DEFINE_UIPROPERTYKEY(UI_PKEY_ThemeColorsTooltips,          VT_VECTOR|VT_LPWSTR,    411); 
DEFINE_UIPROPERTYKEY(UI_PKEY_StandardColorsTooltips,       VT_VECTOR|VT_LPWSTR,    412); 
DEFINE_UIPROPERTYKEY(UI_PKEY_Viewable,                     VT_BOOL,                1000); 
DEFINE_UIPROPERTYKEY(UI_PKEY_Minimized,                    VT_BOOL,                1001); 
DEFINE_UIPROPERTYKEY(UI_PKEY_QuickAccessToolbarDock,       VT_UI4,                 1002); 
DEFINE_UIPROPERTYKEY(UI_PKEY_ContextAvailable,             VT_UI4,                 1100); 
DEFINE_UIPROPERTYKEY(UI_PKEY_GlobalBackgroundColor,        VT_UI4,                 2000); 
DEFINE_UIPROPERTYKEY(UI_PKEY_GlobalHighlightColor,         VT_UI4,                 2001); 
DEFINE_UIPROPERTYKEY(UI_PKEY_GlobalTextColor,              VT_UI4,                 2002); 
typedef DWORD UI_HSBCOLOR;
__inline UI_HSBCOLOR UI_HSB(BYTE hue, BYTE saturation, BYTE brightness)
{
    return hue | (saturation << 8) | (brightness << 16);
}
typedef /* [v1_enum] */ 
enum UI_CONTEXTAVAILABILITY
    {
        UI_CONTEXTAVAILABILITY_NOTAVAILABLE	= 0,
        UI_CONTEXTAVAILABILITY_AVAILABLE	= 1,
        UI_CONTEXTAVAILABILITY_ACTIVE	= 2
    } 	UI_CONTEXTAVAILABILITY;

typedef /* [v1_enum] */ 
enum UI_FONTPROPERTIES
    {
        UI_FONTPROPERTIES_NOTAVAILABLE	= 0,
        UI_FONTPROPERTIES_NOTSET	= 1,
        UI_FONTPROPERTIES_SET	= 2
    } 	UI_FONTPROPERTIES;

typedef /* [v1_enum] */ 
enum UI_FONTVERTICALPOSITION
    {
        UI_FONTVERTICALPOSITION_NOTAVAILABLE	= 0,
        UI_FONTVERTICALPOSITION_NOTSET	= 1,
        UI_FONTVERTICALPOSITION_SUPERSCRIPT	= 2,
        UI_FONTVERTICALPOSITION_SUBSCRIPT	= 3
    } 	UI_FONTVERTICALPOSITION;

typedef /* [v1_enum] */ 
enum UI_FONTUNDERLINE
    {
        UI_FONTUNDERLINE_NOTAVAILABLE	= 0,
        UI_FONTUNDERLINE_NOTSET	= 1,
        UI_FONTUNDERLINE_SET	= 2
    } 	UI_FONTUNDERLINE;

typedef /* [v1_enum] */ 
enum UI_FONTDELTASIZE
    {
        UI_FONTDELTASIZE_GROW	= 0,
        UI_FONTDELTASIZE_SHRINK	= 1
    } 	UI_FONTDELTASIZE;

typedef /* [v1_enum] */ 
enum UI_CONTROLDOCK
    {
        UI_CONTROLDOCK_TOP	= 1,
        UI_CONTROLDOCK_BOTTOM	= 3
    } 	UI_CONTROLDOCK;

typedef /* [v1_enum] */ 
enum UI_SWATCHCOLORTYPE
    {
        UI_SWATCHCOLORTYPE_NOCOLOR	= 0,
        UI_SWATCHCOLORTYPE_AUTOMATIC	= 1,
        UI_SWATCHCOLORTYPE_RGB	= 2
    } 	UI_SWATCHCOLORTYPE;

typedef /* [v1_enum] */ 
enum UI_SWATCHCOLORMODE
    {
        UI_SWATCHCOLORMODE_NORMAL	= 0,
        UI_SWATCHCOLORMODE_MONOCHROME	= 1
    } 	UI_SWATCHCOLORMODE;

typedef /* [v1_enum] */ 
enum UI_EVENTTYPE
    {
        UI_EVENTTYPE_ApplicationMenuOpened	= 0,
        UI_EVENTTYPE_RibbonMinimized	= 1,
        UI_EVENTTYPE_RibbonExpanded	= 2,
        UI_EVENTTYPE_ApplicationModeSwitched	= 3,
        UI_EVENTTYPE_TabActivated	= 4,
        UI_EVENTTYPE_MenuOpened	= 5,
        UI_EVENTTYPE_CommandExecuted	= 6,
        UI_EVENTTYPE_TooltipShown	= 7
    } 	UI_EVENTTYPE;

typedef /* [v1_enum] */ 
enum UI_EVENTLOCATION
    {
        UI_EVENTLOCATION_Ribbon	= 0,
        UI_EVENTLOCATION_QAT	= 1,
        UI_EVENTLOCATION_ApplicationMenu	= 2,
        UI_EVENTLOCATION_ContextPopup	= 3
    } 	UI_EVENTLOCATION;



extern RPC_IF_HANDLE __MIDL_itf_UIRibbon_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIRibbon_0000_0000_v0_0_s_ifspec;

#ifndef __IUISimplePropertySet_INTERFACE_DEFINED__
#define __IUISimplePropertySet_INTERFACE_DEFINED__

/* interface IUISimplePropertySet */
/* [local][unique][object][uuid][helpstring] */ 


EXTERN_C const IID IID_IUISimplePropertySet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c205bb48-5b1c-4219-a106-15bd0a5f24e2")
    IUISimplePropertySet : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetValue( 
            /* [in] */ REFPROPERTYKEY key,
            /* [out] */ PROPVARIANT *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUISimplePropertySetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUISimplePropertySet * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUISimplePropertySet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUISimplePropertySet * This);
        
        DECLSPEC_XFGVIRT(IUISimplePropertySet, GetValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            IUISimplePropertySet * This,
            /* [in] */ REFPROPERTYKEY key,
            /* [out] */ PROPVARIANT *value);
        
        END_INTERFACE
    } IUISimplePropertySetVtbl;

    interface IUISimplePropertySet
    {
        CONST_VTBL struct IUISimplePropertySetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUISimplePropertySet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUISimplePropertySet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUISimplePropertySet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUISimplePropertySet_GetValue(This,key,value)	\
    ( (This)->lpVtbl -> GetValue(This,key,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUISimplePropertySet_INTERFACE_DEFINED__ */


#ifndef __IUIRibbon_INTERFACE_DEFINED__
#define __IUIRibbon_INTERFACE_DEFINED__

/* interface IUIRibbon */
/* [local][unique][object][uuid][helpstring] */ 


EXTERN_C const IID IID_IUIRibbon;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("803982ab-370a-4f7e-a9e7-8784036a6e26")
    IUIRibbon : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetHeight( 
            /* [out] */ UINT32 *cy) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE LoadSettingsFromStream( 
            /* [in] */ IStream *pStream) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SaveSettingsToStream( 
            /* [in] */ IStream *pStream) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIRibbonVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIRibbon * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIRibbon * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIRibbon * This);
        
        DECLSPEC_XFGVIRT(IUIRibbon, GetHeight)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetHeight )( 
            IUIRibbon * This,
            /* [out] */ UINT32 *cy);
        
        DECLSPEC_XFGVIRT(IUIRibbon, LoadSettingsFromStream)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *LoadSettingsFromStream )( 
            IUIRibbon * This,
            /* [in] */ IStream *pStream);
        
        DECLSPEC_XFGVIRT(IUIRibbon, SaveSettingsToStream)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SaveSettingsToStream )( 
            IUIRibbon * This,
            /* [in] */ IStream *pStream);
        
        END_INTERFACE
    } IUIRibbonVtbl;

    interface IUIRibbon
    {
        CONST_VTBL struct IUIRibbonVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIRibbon_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIRibbon_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIRibbon_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIRibbon_GetHeight(This,cy)	\
    ( (This)->lpVtbl -> GetHeight(This,cy) ) 

#define IUIRibbon_LoadSettingsFromStream(This,pStream)	\
    ( (This)->lpVtbl -> LoadSettingsFromStream(This,pStream) ) 

#define IUIRibbon_SaveSettingsToStream(This,pStream)	\
    ( (This)->lpVtbl -> SaveSettingsToStream(This,pStream) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIRibbon_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_UIRibbon_0000_0002 */
/* [local] */ 




extern RPC_IF_HANDLE __MIDL_itf_UIRibbon_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIRibbon_0000_0002_v0_0_s_ifspec;

#ifndef __IUIFramework_INTERFACE_DEFINED__
#define __IUIFramework_INTERFACE_DEFINED__

/* interface IUIFramework */
/* [local][unique][object][uuid][helpstring] */ 

typedef /* [v1_enum] */ 
enum UI_INVALIDATIONS
    {
        UI_INVALIDATIONS_STATE	= 0x1,
        UI_INVALIDATIONS_VALUE	= 0x2,
        UI_INVALIDATIONS_PROPERTY	= 0x4,
        UI_INVALIDATIONS_ALLPROPERTIES	= 0x8
    } 	UI_INVALIDATIONS;

DEFINE_ENUM_FLAG_OPERATORS(UI_INVALIDATIONS)
#define	UI_ALL_COMMANDS	( 0 )


EXTERN_C const IID IID_IUIFramework;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F4F0385D-6872-43a8-AD09-4C339CB3F5C5")
    IUIFramework : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ HWND frameWnd,
            /* [in] */ IUIApplication *application) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Destroy( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE LoadUI( 
            /* [in] */ HINSTANCE instance,
            /* [in] */ LPCWSTR resourceName) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetView( 
            UINT32 viewId,
            REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetUICommandProperty( 
            UINT32 commandId,
            /* [in] */ REFPROPERTYKEY key,
            /* [out] */ PROPVARIANT *value) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetUICommandProperty( 
            UINT32 commandId,
            /* [in] */ REFPROPERTYKEY key,
            /* [in] */ REFPROPVARIANT value) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE InvalidateUICommand( 
            UINT32 commandId,
            UI_INVALIDATIONS flags,
            /* [annotation][in] */ 
            _In_opt_  const PROPERTYKEY *key) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FlushPendingInvalidations( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetModes( 
            INT32 iModes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIFrameworkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIFramework * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIFramework * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIFramework * This);
        
        DECLSPEC_XFGVIRT(IUIFramework, Initialize)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IUIFramework * This,
            /* [in] */ HWND frameWnd,
            /* [in] */ IUIApplication *application);
        
        DECLSPEC_XFGVIRT(IUIFramework, Destroy)
        HRESULT ( STDMETHODCALLTYPE *Destroy )( 
            IUIFramework * This);
        
        DECLSPEC_XFGVIRT(IUIFramework, LoadUI)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *LoadUI )( 
            IUIFramework * This,
            /* [in] */ HINSTANCE instance,
            /* [in] */ LPCWSTR resourceName);
        
        DECLSPEC_XFGVIRT(IUIFramework, GetView)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetView )( 
            IUIFramework * This,
            UINT32 viewId,
            REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv);
        
        DECLSPEC_XFGVIRT(IUIFramework, GetUICommandProperty)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetUICommandProperty )( 
            IUIFramework * This,
            UINT32 commandId,
            /* [in] */ REFPROPERTYKEY key,
            /* [out] */ PROPVARIANT *value);
        
        DECLSPEC_XFGVIRT(IUIFramework, SetUICommandProperty)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetUICommandProperty )( 
            IUIFramework * This,
            UINT32 commandId,
            /* [in] */ REFPROPERTYKEY key,
            /* [in] */ REFPROPVARIANT value);
        
        DECLSPEC_XFGVIRT(IUIFramework, InvalidateUICommand)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *InvalidateUICommand )( 
            IUIFramework * This,
            UINT32 commandId,
            UI_INVALIDATIONS flags,
            /* [annotation][in] */ 
            _In_opt_  const PROPERTYKEY *key);
        
        DECLSPEC_XFGVIRT(IUIFramework, FlushPendingInvalidations)
        HRESULT ( STDMETHODCALLTYPE *FlushPendingInvalidations )( 
            IUIFramework * This);
        
        DECLSPEC_XFGVIRT(IUIFramework, SetModes)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetModes )( 
            IUIFramework * This,
            INT32 iModes);
        
        END_INTERFACE
    } IUIFrameworkVtbl;

    interface IUIFramework
    {
        CONST_VTBL struct IUIFrameworkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIFramework_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIFramework_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIFramework_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIFramework_Initialize(This,frameWnd,application)	\
    ( (This)->lpVtbl -> Initialize(This,frameWnd,application) ) 

#define IUIFramework_Destroy(This)	\
    ( (This)->lpVtbl -> Destroy(This) ) 

#define IUIFramework_LoadUI(This,instance,resourceName)	\
    ( (This)->lpVtbl -> LoadUI(This,instance,resourceName) ) 

#define IUIFramework_GetView(This,viewId,riid,ppv)	\
    ( (This)->lpVtbl -> GetView(This,viewId,riid,ppv) ) 

#define IUIFramework_GetUICommandProperty(This,commandId,key,value)	\
    ( (This)->lpVtbl -> GetUICommandProperty(This,commandId,key,value) ) 

#define IUIFramework_SetUICommandProperty(This,commandId,key,value)	\
    ( (This)->lpVtbl -> SetUICommandProperty(This,commandId,key,value) ) 

#define IUIFramework_InvalidateUICommand(This,commandId,flags,key)	\
    ( (This)->lpVtbl -> InvalidateUICommand(This,commandId,flags,key) ) 

#define IUIFramework_FlushPendingInvalidations(This)	\
    ( (This)->lpVtbl -> FlushPendingInvalidations(This) ) 

#define IUIFramework_SetModes(This,iModes)	\
    ( (This)->lpVtbl -> SetModes(This,iModes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIFramework_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_UIRibbon_0000_0003 */
/* [local] */ 

#ifdef __midl
typedef void *UI_EVENTPARAMS;

#else //!__midl
typedef struct _UI_EVENTPARAMS_COMMAND {
    UINT CommandID;
    PCWSTR CommandName;
    UINT ParentCommandID;
    PCWSTR ParentCommandName;
    UINT SelectionIndex;
    UI_EVENTLOCATION Location;
} UI_EVENTPARAMS_COMMAND;

typedef struct _UI_EVENTPARAMS {
    UI_EVENTTYPE EventType;
    union {
        INT32 Modes;
        UI_EVENTPARAMS_COMMAND Params;
    };
} UI_EVENTPARAMS;
#endif //__midl


extern RPC_IF_HANDLE __MIDL_itf_UIRibbon_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIRibbon_0000_0003_v0_0_s_ifspec;

#ifndef __IUIEventLogger_INTERFACE_DEFINED__
#define __IUIEventLogger_INTERFACE_DEFINED__

/* interface IUIEventLogger */
/* [local][unique][object][uuid][helpstring] */ 


EXTERN_C const IID IID_IUIEventLogger;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ec3e1034-dbf4-41a1-95d5-03e0f1026e05")
    IUIEventLogger : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE OnUIEvent( 
            /* [in] */ UI_EVENTPARAMS *pEventParams) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIEventLoggerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIEventLogger * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIEventLogger * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIEventLogger * This);
        
        DECLSPEC_XFGVIRT(IUIEventLogger, OnUIEvent)
        void ( STDMETHODCALLTYPE *OnUIEvent )( 
            IUIEventLogger * This,
            /* [in] */ UI_EVENTPARAMS *pEventParams);
        
        END_INTERFACE
    } IUIEventLoggerVtbl;

    interface IUIEventLogger
    {
        CONST_VTBL struct IUIEventLoggerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIEventLogger_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIEventLogger_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIEventLogger_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIEventLogger_OnUIEvent(This,pEventParams)	\
    ( (This)->lpVtbl -> OnUIEvent(This,pEventParams) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIEventLogger_INTERFACE_DEFINED__ */


#ifndef __IUIEventingManager_INTERFACE_DEFINED__
#define __IUIEventingManager_INTERFACE_DEFINED__

/* interface IUIEventingManager */
/* [local][unique][object][uuid][helpstring] */ 


EXTERN_C const IID IID_IUIEventingManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3BE6EA7F-9A9B-4198-9368-9B0F923BD534")
    IUIEventingManager : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetEventLogger( 
            /* [in] */ IUIEventLogger *eventLogger) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIEventingManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIEventingManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIEventingManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIEventingManager * This);
        
        DECLSPEC_XFGVIRT(IUIEventingManager, SetEventLogger)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetEventLogger )( 
            IUIEventingManager * This,
            /* [in] */ IUIEventLogger *eventLogger);
        
        END_INTERFACE
    } IUIEventingManagerVtbl;

    interface IUIEventingManager
    {
        CONST_VTBL struct IUIEventingManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIEventingManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIEventingManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIEventingManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIEventingManager_SetEventLogger(This,eventLogger)	\
    ( (This)->lpVtbl -> SetEventLogger(This,eventLogger) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIEventingManager_INTERFACE_DEFINED__ */


#ifndef __IUIContextualUI_INTERFACE_DEFINED__
#define __IUIContextualUI_INTERFACE_DEFINED__

/* interface IUIContextualUI */
/* [local][unique][object][uuid][helpstring] */ 


EXTERN_C const IID IID_IUIContextualUI;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EEA11F37-7C46-437c-8E55-B52122B29293")
    IUIContextualUI : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE ShowAtLocation( 
            INT32 x,
            INT32 y) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIContextualUIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIContextualUI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIContextualUI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIContextualUI * This);
        
        DECLSPEC_XFGVIRT(IUIContextualUI, ShowAtLocation)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *ShowAtLocation )( 
            IUIContextualUI * This,
            INT32 x,
            INT32 y);
        
        END_INTERFACE
    } IUIContextualUIVtbl;

    interface IUIContextualUI
    {
        CONST_VTBL struct IUIContextualUIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIContextualUI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIContextualUI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIContextualUI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIContextualUI_ShowAtLocation(This,x,y)	\
    ( (This)->lpVtbl -> ShowAtLocation(This,x,y) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIContextualUI_INTERFACE_DEFINED__ */


#ifndef __IUICollection_INTERFACE_DEFINED__
#define __IUICollection_INTERFACE_DEFINED__

/* interface IUICollection */
/* [local][unique][object][uuid][helpstring] */ 


EXTERN_C const IID IID_IUICollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DF4F45BF-6F9D-4dd7-9D68-D8F9CD18C4DB")
    IUICollection : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ UINT32 *count) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetItem( 
            UINT32 index,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **item) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ IUnknown *item) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Insert( 
            UINT32 index,
            /* [in] */ IUnknown *item) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE RemoveAt( 
            UINT32 index) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Replace( 
            UINT32 indexReplaced,
            /* [in] */ IUnknown *itemReplaceWith) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUICollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUICollection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUICollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUICollection * This);
        
        DECLSPEC_XFGVIRT(IUICollection, GetCount)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IUICollection * This,
            /* [out] */ UINT32 *count);
        
        DECLSPEC_XFGVIRT(IUICollection, GetItem)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            IUICollection * This,
            UINT32 index,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **item);
        
        DECLSPEC_XFGVIRT(IUICollection, Add)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            IUICollection * This,
            /* [in] */ IUnknown *item);
        
        DECLSPEC_XFGVIRT(IUICollection, Insert)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Insert )( 
            IUICollection * This,
            UINT32 index,
            /* [in] */ IUnknown *item);
        
        DECLSPEC_XFGVIRT(IUICollection, RemoveAt)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            IUICollection * This,
            UINT32 index);
        
        DECLSPEC_XFGVIRT(IUICollection, Replace)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Replace )( 
            IUICollection * This,
            UINT32 indexReplaced,
            /* [in] */ IUnknown *itemReplaceWith);
        
        DECLSPEC_XFGVIRT(IUICollection, Clear)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Clear )( 
            IUICollection * This);
        
        END_INTERFACE
    } IUICollectionVtbl;

    interface IUICollection
    {
        CONST_VTBL struct IUICollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUICollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUICollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUICollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUICollection_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IUICollection_GetItem(This,index,item)	\
    ( (This)->lpVtbl -> GetItem(This,index,item) ) 

#define IUICollection_Add(This,item)	\
    ( (This)->lpVtbl -> Add(This,item) ) 

#define IUICollection_Insert(This,index,item)	\
    ( (This)->lpVtbl -> Insert(This,index,item) ) 

#define IUICollection_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define IUICollection_Replace(This,indexReplaced,itemReplaceWith)	\
    ( (This)->lpVtbl -> Replace(This,indexReplaced,itemReplaceWith) ) 

#define IUICollection_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUICollection_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_UIRibbon_0000_0007 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum UI_COLLECTIONCHANGE
    {
        UI_COLLECTIONCHANGE_INSERT	= 0,
        UI_COLLECTIONCHANGE_REMOVE	= 1,
        UI_COLLECTIONCHANGE_REPLACE	= 2,
        UI_COLLECTIONCHANGE_RESET	= 3
    } 	UI_COLLECTIONCHANGE;

#define	UI_COLLECTION_INVALIDINDEX	( 0xffffffff )



extern RPC_IF_HANDLE __MIDL_itf_UIRibbon_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIRibbon_0000_0007_v0_0_s_ifspec;

#ifndef __IUICollectionChangedEvent_INTERFACE_DEFINED__
#define __IUICollectionChangedEvent_INTERFACE_DEFINED__

/* interface IUICollectionChangedEvent */
/* [local][unique][object][uuid][helpstring] */ 


EXTERN_C const IID IID_IUICollectionChangedEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6502AE91-A14D-44b5-BBD0-62AACC581D52")
    IUICollectionChangedEvent : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnChanged( 
            UI_COLLECTIONCHANGE action,
            UINT32 oldIndex,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *oldItem,
            UINT32 newIndex,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *newItem) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUICollectionChangedEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUICollectionChangedEvent * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUICollectionChangedEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUICollectionChangedEvent * This);
        
        DECLSPEC_XFGVIRT(IUICollectionChangedEvent, OnChanged)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnChanged )( 
            IUICollectionChangedEvent * This,
            UI_COLLECTIONCHANGE action,
            UINT32 oldIndex,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *oldItem,
            UINT32 newIndex,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *newItem);
        
        END_INTERFACE
    } IUICollectionChangedEventVtbl;

    interface IUICollectionChangedEvent
    {
        CONST_VTBL struct IUICollectionChangedEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUICollectionChangedEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUICollectionChangedEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUICollectionChangedEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUICollectionChangedEvent_OnChanged(This,action,oldIndex,oldItem,newIndex,newItem)	\
    ( (This)->lpVtbl -> OnChanged(This,action,oldIndex,oldItem,newIndex,newItem) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUICollectionChangedEvent_INTERFACE_DEFINED__ */


#ifndef __IUICommandHandler_INTERFACE_DEFINED__
#define __IUICommandHandler_INTERFACE_DEFINED__

/* interface IUICommandHandler */
/* [local][unique][object][uuid][helpstring] */ 

typedef /* [v1_enum] */ 
enum UI_EXECUTIONVERB
    {
        UI_EXECUTIONVERB_EXECUTE	= 0,
        UI_EXECUTIONVERB_PREVIEW	= 1,
        UI_EXECUTIONVERB_CANCELPREVIEW	= 2
    } 	UI_EXECUTIONVERB;


EXTERN_C const IID IID_IUICommandHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("75ae0a2d-dc03-4c9f-8883-069660d0beb6")
    IUICommandHandler : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Execute( 
            UINT32 commandId,
            UI_EXECUTIONVERB verb,
            /* [annotation][in] */ 
            _In_opt_  const PROPERTYKEY *key,
            /* [annotation][in] */ 
            _In_opt_  const PROPVARIANT *currentValue,
            /* [annotation][in] */ 
            _In_opt_  IUISimplePropertySet *commandExecutionProperties) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE UpdateProperty( 
            UINT32 commandId,
            /* [in] */ REFPROPERTYKEY key,
            /* [annotation][in] */ 
            _In_opt_  const PROPVARIANT *currentValue,
            /* [out] */ PROPVARIANT *newValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUICommandHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUICommandHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUICommandHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUICommandHandler * This);
        
        DECLSPEC_XFGVIRT(IUICommandHandler, Execute)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Execute )( 
            IUICommandHandler * This,
            UINT32 commandId,
            UI_EXECUTIONVERB verb,
            /* [annotation][in] */ 
            _In_opt_  const PROPERTYKEY *key,
            /* [annotation][in] */ 
            _In_opt_  const PROPVARIANT *currentValue,
            /* [annotation][in] */ 
            _In_opt_  IUISimplePropertySet *commandExecutionProperties);
        
        DECLSPEC_XFGVIRT(IUICommandHandler, UpdateProperty)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *UpdateProperty )( 
            IUICommandHandler * This,
            UINT32 commandId,
            /* [in] */ REFPROPERTYKEY key,
            /* [annotation][in] */ 
            _In_opt_  const PROPVARIANT *currentValue,
            /* [out] */ PROPVARIANT *newValue);
        
        END_INTERFACE
    } IUICommandHandlerVtbl;

    interface IUICommandHandler
    {
        CONST_VTBL struct IUICommandHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUICommandHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUICommandHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUICommandHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUICommandHandler_Execute(This,commandId,verb,key,currentValue,commandExecutionProperties)	\
    ( (This)->lpVtbl -> Execute(This,commandId,verb,key,currentValue,commandExecutionProperties) ) 

#define IUICommandHandler_UpdateProperty(This,commandId,key,currentValue,newValue)	\
    ( (This)->lpVtbl -> UpdateProperty(This,commandId,key,currentValue,newValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUICommandHandler_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_UIRibbon_0000_0009 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum UI_COMMANDTYPE
    {
        UI_COMMANDTYPE_UNKNOWN	= 0,
        UI_COMMANDTYPE_GROUP	= 1,
        UI_COMMANDTYPE_ACTION	= 2,
        UI_COMMANDTYPE_ANCHOR	= 3,
        UI_COMMANDTYPE_CONTEXT	= 4,
        UI_COMMANDTYPE_COLLECTION	= 5,
        UI_COMMANDTYPE_COMMANDCOLLECTION	= 6,
        UI_COMMANDTYPE_DECIMAL	= 7,
        UI_COMMANDTYPE_BOOLEAN	= 8,
        UI_COMMANDTYPE_FONT	= 9,
        UI_COMMANDTYPE_RECENTITEMS	= 10,
        UI_COMMANDTYPE_COLORANCHOR	= 11,
        UI_COMMANDTYPE_COLORCOLLECTION	= 12
    } 	UI_COMMANDTYPE;

typedef /* [v1_enum] */ 
enum UI_VIEWTYPE
    {
        UI_VIEWTYPE_RIBBON	= 1
    } 	UI_VIEWTYPE;



extern RPC_IF_HANDLE __MIDL_itf_UIRibbon_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIRibbon_0000_0009_v0_0_s_ifspec;

#ifndef __IUIApplication_INTERFACE_DEFINED__
#define __IUIApplication_INTERFACE_DEFINED__

/* interface IUIApplication */
/* [local][unique][object][uuid][helpstring] */ 

typedef /* [v1_enum] */ 
enum UI_VIEWVERB
    {
        UI_VIEWVERB_CREATE	= 0,
        UI_VIEWVERB_DESTROY	= 1,
        UI_VIEWVERB_SIZE	= 2,
        UI_VIEWVERB_ERROR	= 3
    } 	UI_VIEWVERB;


EXTERN_C const IID IID_IUIApplication;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D428903C-729A-491d-910D-682A08FF2522")
    IUIApplication : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnViewChanged( 
            UINT32 viewId,
            UI_VIEWTYPE typeID,
            /* [in] */ IUnknown *view,
            UI_VIEWVERB verb,
            INT32 uReasonCode) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnCreateUICommand( 
            UINT32 commandId,
            UI_COMMANDTYPE typeID,
            /* [out] */ IUICommandHandler **commandHandler) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnDestroyUICommand( 
            UINT32 commandId,
            UI_COMMANDTYPE typeID,
            /* [annotation][in] */ 
            _In_opt_  IUICommandHandler *commandHandler) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIApplicationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIApplication * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIApplication * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIApplication * This);
        
        DECLSPEC_XFGVIRT(IUIApplication, OnViewChanged)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnViewChanged )( 
            IUIApplication * This,
            UINT32 viewId,
            UI_VIEWTYPE typeID,
            /* [in] */ IUnknown *view,
            UI_VIEWVERB verb,
            INT32 uReasonCode);
        
        DECLSPEC_XFGVIRT(IUIApplication, OnCreateUICommand)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnCreateUICommand )( 
            IUIApplication * This,
            UINT32 commandId,
            UI_COMMANDTYPE typeID,
            /* [out] */ IUICommandHandler **commandHandler);
        
        DECLSPEC_XFGVIRT(IUIApplication, OnDestroyUICommand)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnDestroyUICommand )( 
            IUIApplication * This,
            UINT32 commandId,
            UI_COMMANDTYPE typeID,
            /* [annotation][in] */ 
            _In_opt_  IUICommandHandler *commandHandler);
        
        END_INTERFACE
    } IUIApplicationVtbl;

    interface IUIApplication
    {
        CONST_VTBL struct IUIApplicationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIApplication_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIApplication_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIApplication_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIApplication_OnViewChanged(This,viewId,typeID,view,verb,uReasonCode)	\
    ( (This)->lpVtbl -> OnViewChanged(This,viewId,typeID,view,verb,uReasonCode) ) 

#define IUIApplication_OnCreateUICommand(This,commandId,typeID,commandHandler)	\
    ( (This)->lpVtbl -> OnCreateUICommand(This,commandId,typeID,commandHandler) ) 

#define IUIApplication_OnDestroyUICommand(This,commandId,typeID,commandHandler)	\
    ( (This)->lpVtbl -> OnDestroyUICommand(This,commandId,typeID,commandHandler) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIApplication_INTERFACE_DEFINED__ */


#ifndef __IUIImage_INTERFACE_DEFINED__
#define __IUIImage_INTERFACE_DEFINED__

/* interface IUIImage */
/* [local][unique][object][uuid][helpstring] */ 


EXTERN_C const IID IID_IUIImage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("23c8c838-4de6-436b-ab01-5554bb7c30dd")
    IUIImage : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetBitmap( 
            /* [out] */ HBITMAP *bitmap) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIImageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIImage * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIImage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIImage * This);
        
        DECLSPEC_XFGVIRT(IUIImage, GetBitmap)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetBitmap )( 
            IUIImage * This,
            /* [out] */ HBITMAP *bitmap);
        
        END_INTERFACE
    } IUIImageVtbl;

    interface IUIImage
    {
        CONST_VTBL struct IUIImageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIImage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIImage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIImage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIImage_GetBitmap(This,bitmap)	\
    ( (This)->lpVtbl -> GetBitmap(This,bitmap) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIImage_INTERFACE_DEFINED__ */


#ifndef __IUIImageFromBitmap_INTERFACE_DEFINED__
#define __IUIImageFromBitmap_INTERFACE_DEFINED__

/* interface IUIImageFromBitmap */
/* [local][unique][object][uuid][helpstring] */ 

typedef /* [v1_enum] */ 
enum UI_OWNERSHIP
    {
        UI_OWNERSHIP_TRANSFER	= 0,
        UI_OWNERSHIP_COPY	= 1
    } 	UI_OWNERSHIP;


EXTERN_C const IID IID_IUIImageFromBitmap;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("18aba7f3-4c1c-4ba2-bf6c-f5c3326fa816")
    IUIImageFromBitmap : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateImage( 
            /* [in] */ HBITMAP bitmap,
            UI_OWNERSHIP options,
            /* [out] */ IUIImage **image) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIImageFromBitmapVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIImageFromBitmap * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIImageFromBitmap * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIImageFromBitmap * This);
        
        DECLSPEC_XFGVIRT(IUIImageFromBitmap, CreateImage)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateImage )( 
            IUIImageFromBitmap * This,
            /* [in] */ HBITMAP bitmap,
            UI_OWNERSHIP options,
            /* [out] */ IUIImage **image);
        
        END_INTERFACE
    } IUIImageFromBitmapVtbl;

    interface IUIImageFromBitmap
    {
        CONST_VTBL struct IUIImageFromBitmapVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIImageFromBitmap_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIImageFromBitmap_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIImageFromBitmap_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIImageFromBitmap_CreateImage(This,bitmap,options,image)	\
    ( (This)->lpVtbl -> CreateImage(This,bitmap,options,image) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIImageFromBitmap_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_UIRibbon_0000_0012 */
/* [local] */ 

#define UI_MAKEAPPMODE(x) (1 << (x))
DEFINE_GUID(CLSID_UIRibbonFramework,0x926749fa, 0x2615, 0x4987, 0x88, 0x45, 0xc3, 0x3e, 0x65, 0xf2, 0xb9, 0x57);
DEFINE_GUID(CLSID_UIRibbonImageFromBitmapFactory,
                                    0x0f7434b6, 0x59b6, 0x4250, 0x99, 0x9e, 0xd1, 0x68, 0xd6, 0xae, 0x42, 0x93);
DEFINE_GUID(LIBID_UIRibbon,         0x942f35c2, 0xe83b, 0x45ef, 0xb0, 0x85, 0xac, 0x29, 0x5d, 0xd6, 0x3d, 0x5b);


extern RPC_IF_HANDLE __MIDL_itf_UIRibbon_0000_0012_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIRibbon_0000_0012_v0_0_s_ifspec;


#ifndef __UIRibbon_LIBRARY_DEFINED__
#define __UIRibbon_LIBRARY_DEFINED__

/* library UIRibbon */
/* [version][lcid][uuid] */ 


EXTERN_C const IID LIBID_UIRibbon;

EXTERN_C const CLSID CLSID_UIRibbonFramework;

#ifdef __cplusplus

class DECLSPEC_UUID("926749fa-2615-4987-8845-c33e65f2b957")
UIRibbonFramework;
#endif

EXTERN_C const CLSID CLSID_UIRibbonImageFromBitmapFactory;

#ifdef __cplusplus

class DECLSPEC_UUID("0F7434B6-59B6-4250-999E-D168D6AE4293")
UIRibbonImageFromBitmapFactory;
#endif
#endif /* __UIRibbon_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_UIRibbon_0000_0013 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_UIRibbon_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIRibbon_0000_0013_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


