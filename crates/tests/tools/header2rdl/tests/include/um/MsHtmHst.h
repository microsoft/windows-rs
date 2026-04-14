

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

#ifndef __mshtmhst_h__
#define __mshtmhst_h__

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

#ifndef __IHostDialogHelper_FWD_DEFINED__
#define __IHostDialogHelper_FWD_DEFINED__
typedef interface IHostDialogHelper IHostDialogHelper;

#endif 	/* __IHostDialogHelper_FWD_DEFINED__ */


#ifndef __HostDialogHelper_FWD_DEFINED__
#define __HostDialogHelper_FWD_DEFINED__

#ifdef __cplusplus
typedef class HostDialogHelper HostDialogHelper;
#else
typedef struct HostDialogHelper HostDialogHelper;
#endif /* __cplusplus */

#endif 	/* __HostDialogHelper_FWD_DEFINED__ */


#ifndef __IDocHostUIHandler_FWD_DEFINED__
#define __IDocHostUIHandler_FWD_DEFINED__
typedef interface IDocHostUIHandler IDocHostUIHandler;

#endif 	/* __IDocHostUIHandler_FWD_DEFINED__ */


#ifndef __IDocHostUIHandler2_FWD_DEFINED__
#define __IDocHostUIHandler2_FWD_DEFINED__
typedef interface IDocHostUIHandler2 IDocHostUIHandler2;

#endif 	/* __IDocHostUIHandler2_FWD_DEFINED__ */


#ifndef __ICustomDoc_FWD_DEFINED__
#define __ICustomDoc_FWD_DEFINED__
typedef interface ICustomDoc ICustomDoc;

#endif 	/* __ICustomDoc_FWD_DEFINED__ */


#ifndef __IDocHostShowUI_FWD_DEFINED__
#define __IDocHostShowUI_FWD_DEFINED__
typedef interface IDocHostShowUI IDocHostShowUI;

#endif 	/* __IDocHostShowUI_FWD_DEFINED__ */


#ifndef __IClassFactoryEx_FWD_DEFINED__
#define __IClassFactoryEx_FWD_DEFINED__
typedef interface IClassFactoryEx IClassFactoryEx;

#endif 	/* __IClassFactoryEx_FWD_DEFINED__ */


#ifndef __IHTMLOMWindowServices_FWD_DEFINED__
#define __IHTMLOMWindowServices_FWD_DEFINED__
typedef interface IHTMLOMWindowServices IHTMLOMWindowServices;

#endif 	/* __IHTMLOMWindowServices_FWD_DEFINED__ */


/* header files for imported files */
#include "ocidl.h"
#include "docobj.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mshtmhst_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// mshtmhst.h
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
#pragma comment(lib,"uuid.lib")

//--------------------------------------------------------------------------
// MSTHML Advanced Host Interfaces.

#ifndef MSHTMHST_H
#define MSHTMHST_H
#define CONTEXT_MENU_DEFAULT        0
#define CONTEXT_MENU_IMAGE          1
#define CONTEXT_MENU_CONTROL        2
#define CONTEXT_MENU_TABLE          3
// in browse mode
#define CONTEXT_MENU_TEXTSELECT     4
#define CONTEXT_MENU_ANCHOR         5
#define CONTEXT_MENU_UNKNOWN        6
//;begin_internal
// These 2 are mapped to IMAGE for the public
#define CONTEXT_MENU_IMGDYNSRC      7
#define CONTEXT_MENU_DEBUG          8
//;end_internal
#define CONTEXT_MENU_VSCROLL        9
#define CONTEXT_MENU_HSCROLL        10
#define CONTEXT_MENU_MEDIA          11
//;begin_internal
#define CONTEXT_MENU_ENTITY         12
#define CONTEXT_MENU_PDF            13
#define CONTEXT_MENU_DISABLEDFLASH  14
//;end_internal
#define MENUEXT_SHOWDIALOG           0x1
#define DOCHOSTUIFLAG_BROWSER       (DOCHOSTUIFLAG_DISABLE_HELP_MENU | DOCHOSTUIFLAG_DISABLE_SCRIPT_INACTIVE) 
#define HTMLDLG_NOUI                     0x0010
#define HTMLDLG_MODAL                    0x0020
#define HTMLDLG_MODELESS                 0x0040
#define HTMLDLG_PRINT_TEMPLATE           0x0080
#define HTMLDLG_VERIFY                   0x0100
#define HTMLDLG_ALLOW_UNKNOWN_THREAD     0x0200
#define PRINT_DONTBOTHERUSER             0x01
#define PRINT_WAITFORCOMPLETION          0x02
#define CMDID_SCRIPTSITE_URL                0
#define CMDID_SCRIPTSITE_HTMLDLGTRUST       1
#define CMDID_SCRIPTSITE_SECSTATE           2
#define CMDID_SCRIPTSITE_SID                3
#define CMDID_SCRIPTSITE_TRUSTEDDOC         4
#define CMDID_SCRIPTSITE_SECURITY_WINDOW    5
#define CMDID_SCRIPTSITE_NAMESPACE          6
#define CMDID_SCRIPTSITE_IURI               7
#define CMDID_HOSTCONTEXT_URL               8
#define CMDID_SCRIPTSITE_ALLOWRECOVERY      9
#define CMDID_SCRIPTSITE_BASEIURI          10
EXTERN_C const GUID CGID_ScriptSite;
EXTERN_C const GUID CGID_MSHTML;
#define CMDSETID_Forms3 CGID_MSHTML
#define SZ_HTML_CLIENTSITE_OBJECTPARAM L"{d4db6850-5385-11d0-89e9-00a0c90a90ac}"
#ifndef __IHTMLWindow2_FWD_DEFINED__
#define __IHTMLWindow2_FWD_DEFINED__
typedef interface IHTMLWindow2 IHTMLWindow2;
#endif
typedef HRESULT STDAPICALLTYPE SHOWHTMLDIALOGFN (HWND hwndParent, IMoniker *pmk, VARIANT *pvarArgIn, WCHAR* pchOptions, VARIANT *pvArgOut);
typedef HRESULT STDAPICALLTYPE SHOWHTMLDIALOGEXFN (HWND hwndParent, IMoniker *pmk, DWORD dwDialogFlags, VARIANT *pvarArgIn, WCHAR* pchOptions, VARIANT *pvArgOut);
typedef HRESULT STDAPICALLTYPE SHOWMODELESSHTMLDIALOGFN (HWND hwndParent, IMoniker *pmk, VARIANT *pvarArgIn, VARIANT* pvarOptions, IHTMLWindow2 ** ppWindow);
typedef HRESULT STDAPICALLTYPE IEREGISTERXMLNSFN (_In_ LPCWSTR lpszURI, _In_ GUID clsid, _In_ BOOL fMachine);
typedef HRESULT STDAPICALLTYPE IEISXMLNSREGISTEREDFN (_In_ LPCWSTR lpszURI, _Out_ GUID *pCLSID);
//;begin_internal
STDAPI ShowHTMLDialog(                   
    HWND        hwndParent,              
    IMoniker *  pMk,                     
    VARIANT *   pvarArgIn,               
    _In_z_ LPWSTR pchOptions,            
    VARIANT *   pvarArgOut               
    );                                   
STDAPI ShowHTMLDialogEx(                 
    HWND        hwndParent,              
    IMoniker *  pMk,                     
    DWORD       dwDialogFlags,           
    VARIANT *   pvarArgIn,               
    _In_ LPWSTR pchOptions,              
    VARIANT *   pvarArgOut               
    );                                   
STDAPI ShowModelessHTMLDialog(           
    HWND        hwndParent,              
    IMoniker *  pMk,                     
    VARIANT *   pvarArgIn,               
    VARIANT *   pvarOptions,             
    IHTMLWindow2 ** ppWindow);           
//;end_internal
//;begin_internal
#ifndef _ARM_
STDAPI RunHTMLApplication(               
    HINSTANCE hinst,                     
    HINSTANCE hPrevInst,                 
    _In_ LPSTR szCmdLine,                
    int nCmdShow                         
    );                                   
#endif // !ARM
//;end_internal
//;begin_internal
STDAPI CreateHTMLPropertyPage(           
    IMoniker *          pmk,             
    IPropertyPage **    ppPP             
    );                                   
//;end_internal
//;begin_internal
STDAPI EarlyStartDisplaySystem();        
//;end_internal
STDAPI IERegisterXMLNS(                  
     _In_ LPCWSTR lpszURI,               
     _In_ GUID clsid,                    
     _In_ BOOL fMachine                  
     );                                  
STDAPI IEIsXMLNSRegistered(              
     LPCWSTR lpszURI,                    
     _Out_ GUID *pCLSID                  
     );                                  
//;begin_internal
STDAPI GetColorValueFromString(          
     _In_ LPCWSTR lpszColor,             
     _In_ BOOL fStrictCSS1,              
     _In_ BOOL fIsStandardsCSS,          
     _Out_ COLORREF *pColor              
     );                                  
//;end_internal
//;begin_internal
STDAPI InitializeLocalHtmlEngine();      
STDAPI UninitializeLocalHtmlEngine();    
//;end_internal


extern RPC_IF_HANDLE __MIDL_itf_mshtmhst_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mshtmhst_0000_0000_v0_0_s_ifspec;

#ifndef __IHostDialogHelper_INTERFACE_DEFINED__
#define __IHostDialogHelper_INTERFACE_DEFINED__

/* interface IHostDialogHelper */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IHostDialogHelper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("53DEC138-A51E-11d2-861E-00C04FA35C89")
    IHostDialogHelper : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ShowHTMLDialog( 
            HWND hwndParent,
            IMoniker *pMk,
            VARIANT *pvarArgIn,
            /* [annotation][in] */ 
            _In_z_  WCHAR *pchOptions,
            VARIANT *pvarArgOut,
            IUnknown *punkHost) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHostDialogHelperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IHostDialogHelper * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IHostDialogHelper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IHostDialogHelper * This);
        
        DECLSPEC_XFGVIRT(IHostDialogHelper, ShowHTMLDialog)
        HRESULT ( STDMETHODCALLTYPE *ShowHTMLDialog )( 
            IHostDialogHelper * This,
            HWND hwndParent,
            IMoniker *pMk,
            VARIANT *pvarArgIn,
            /* [annotation][in] */ 
            _In_z_  WCHAR *pchOptions,
            VARIANT *pvarArgOut,
            IUnknown *punkHost);
        
        END_INTERFACE
    } IHostDialogHelperVtbl;

    interface IHostDialogHelper
    {
        CONST_VTBL struct IHostDialogHelperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHostDialogHelper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHostDialogHelper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHostDialogHelper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHostDialogHelper_ShowHTMLDialog(This,hwndParent,pMk,pvarArgIn,pchOptions,pvarArgOut,punkHost)	\
    ( (This)->lpVtbl -> ShowHTMLDialog(This,hwndParent,pMk,pvarArgIn,pchOptions,pvarArgOut,punkHost) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHostDialogHelper_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mshtmhst_0000_0002 */
/* [local] */ 

EXTERN_C const GUID CLSID_HostDialogHelper;
//;begin_internal
typedef 
enum tagDOCHOSTUITYPE
    {
        DOCHOSTUITYPE_BROWSE	= 0,
        DOCHOSTUITYPE_AUTHOR	= 1
    } 	DOCHOSTUITYPE;

//;end_internal
typedef 
enum tagDOCHOSTUIDBLCLK
    {
        DOCHOSTUIDBLCLK_DEFAULT	= 0,
        DOCHOSTUIDBLCLK_SHOWPROPERTIES	= 1,
        DOCHOSTUIDBLCLK_SHOWCODE	= 2
    } 	DOCHOSTUIDBLCLK;

typedef 
enum tagDOCHOSTUIFLAG
    {
        DOCHOSTUIFLAG_DIALOG	= 0x1,
        DOCHOSTUIFLAG_DISABLE_HELP_MENU	= 0x2,
        DOCHOSTUIFLAG_NO3DBORDER	= 0x4,
        DOCHOSTUIFLAG_SCROLL_NO	= 0x8,
        DOCHOSTUIFLAG_DISABLE_SCRIPT_INACTIVE	= 0x10,
        DOCHOSTUIFLAG_OPENNEWWIN	= 0x20,
        DOCHOSTUIFLAG_DISABLE_OFFSCREEN	= 0x40,
        DOCHOSTUIFLAG_FLAT_SCROLLBAR	= 0x80,
        DOCHOSTUIFLAG_DIV_BLOCKDEFAULT	= 0x100,
        DOCHOSTUIFLAG_ACTIVATE_CLIENTHIT_ONLY	= 0x200,
        DOCHOSTUIFLAG_OVERRIDEBEHAVIORFACTORY	= 0x400,
        DOCHOSTUIFLAG_CODEPAGELINKEDFONTS	= 0x800,
        DOCHOSTUIFLAG_URL_ENCODING_DISABLE_UTF8	= 0x1000,
        DOCHOSTUIFLAG_URL_ENCODING_ENABLE_UTF8	= 0x2000,
        DOCHOSTUIFLAG_ENABLE_FORMS_AUTOCOMPLETE	= 0x4000,
        DOCHOSTUIFLAG_ENABLE_INPLACE_NAVIGATION	= 0x10000,
        DOCHOSTUIFLAG_IME_ENABLE_RECONVERSION	= 0x20000,
        DOCHOSTUIFLAG_THEME	= 0x40000,
        DOCHOSTUIFLAG_NOTHEME	= 0x80000,
        DOCHOSTUIFLAG_NOPICS	= 0x100000,
        DOCHOSTUIFLAG_NO3DOUTERBORDER	= 0x200000,
        DOCHOSTUIFLAG_DISABLE_EDIT_NS_FIXUP	= 0x400000,
        DOCHOSTUIFLAG_LOCAL_MACHINE_ACCESS_CHECK	= 0x800000,
        DOCHOSTUIFLAG_DISABLE_UNTRUSTEDPROTOCOL	= 0x1000000,
        DOCHOSTUIFLAG_HOST_NAVIGATES	= 0x2000000,
        DOCHOSTUIFLAG_ENABLE_REDIRECT_NOTIFICATION	= 0x4000000,
        DOCHOSTUIFLAG_USE_WINDOWLESS_SELECTCONTROL	= 0x8000000,
        DOCHOSTUIFLAG_USE_WINDOWED_SELECTCONTROL	= 0x10000000,
        DOCHOSTUIFLAG_ENABLE_ACTIVEX_INACTIVATE_MODE	= 0x20000000,
        DOCHOSTUIFLAG_DPI_AWARE	= 0x40000000
    } 	DOCHOSTUIFLAG;



extern RPC_IF_HANDLE __MIDL_itf_mshtmhst_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mshtmhst_0000_0002_v0_0_s_ifspec;

#ifndef __IDocHostUIHandler_INTERFACE_DEFINED__
#define __IDocHostUIHandler_INTERFACE_DEFINED__

/* interface IDocHostUIHandler */
/* [local][unique][uuid][object] */ 

typedef struct _DOCHOSTUIINFO
    {
    ULONG cbSize;
    DWORD dwFlags;
    DWORD dwDoubleClick;
    OLECHAR *pchHostCss;
    OLECHAR *pchHostNS;
    } 	DOCHOSTUIINFO;


EXTERN_C const IID IID_IDocHostUIHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bd3f23c0-d43e-11cf-893b-00aa00bdce1a")
    IDocHostUIHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ShowContextMenu( 
            /* [annotation][in] */ 
            _In_  DWORD dwID,
            /* [annotation][in] */ 
            _In_  POINT *ppt,
            /* [annotation][in] */ 
            _In_  IUnknown *pcmdtReserved,
            /* [annotation][in] */ 
            _In_  IDispatch *pdispReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHostInfo( 
            /* [annotation][out][in] */ 
            _Inout_  DOCHOSTUIINFO *pInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShowUI( 
            /* [annotation][in] */ 
            _In_  DWORD dwID,
            /* [annotation][in] */ 
            _In_  IOleInPlaceActiveObject *pActiveObject,
            /* [annotation][in] */ 
            _In_  IOleCommandTarget *pCommandTarget,
            /* [annotation][in] */ 
            _In_  IOleInPlaceFrame *pFrame,
            /* [annotation][in] */ 
            _In_  IOleInPlaceUIWindow *pDoc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HideUI( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateUI( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnableModeless( 
            /* [in] */ BOOL fEnable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDocWindowActivate( 
            /* [in] */ BOOL fActivate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnFrameWindowActivate( 
            /* [in] */ BOOL fActivate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResizeBorder( 
            /* [annotation][in] */ 
            _In_  LPCRECT prcBorder,
            /* [annotation][in] */ 
            _In_  IOleInPlaceUIWindow *pUIWindow,
            /* [annotation][in] */ 
            _In_  BOOL fRameWindow) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TranslateAccelerator( 
            /* [in] */ LPMSG lpMsg,
            /* [in] */ const GUID *pguidCmdGroup,
            /* [in] */ DWORD nCmdID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOptionKeyPath( 
            /* [annotation][out] */ 
            _Out_  LPOLESTR *pchKey,
            /* [in] */ DWORD dw) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDropTarget( 
            /* [annotation][in] */ 
            _In_  IDropTarget *pDropTarget,
            /* [annotation][out] */ 
            _Outptr_  IDropTarget **ppDropTarget) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExternal( 
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IDispatch **ppDispatch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TranslateUrl( 
            /* [in] */ DWORD dwTranslate,
            /* [annotation][in] */ 
            _In_  LPWSTR pchURLIn,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *ppchURLOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FilterDataObject( 
            /* [annotation][in] */ 
            _In_  IDataObject *pDO,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IDataObject **ppDORet) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDocHostUIHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDocHostUIHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDocHostUIHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDocHostUIHandler * This);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, ShowContextMenu)
        HRESULT ( STDMETHODCALLTYPE *ShowContextMenu )( 
            IDocHostUIHandler * This,
            /* [annotation][in] */ 
            _In_  DWORD dwID,
            /* [annotation][in] */ 
            _In_  POINT *ppt,
            /* [annotation][in] */ 
            _In_  IUnknown *pcmdtReserved,
            /* [annotation][in] */ 
            _In_  IDispatch *pdispReserved);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, GetHostInfo)
        HRESULT ( STDMETHODCALLTYPE *GetHostInfo )( 
            IDocHostUIHandler * This,
            /* [annotation][out][in] */ 
            _Inout_  DOCHOSTUIINFO *pInfo);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, ShowUI)
        HRESULT ( STDMETHODCALLTYPE *ShowUI )( 
            IDocHostUIHandler * This,
            /* [annotation][in] */ 
            _In_  DWORD dwID,
            /* [annotation][in] */ 
            _In_  IOleInPlaceActiveObject *pActiveObject,
            /* [annotation][in] */ 
            _In_  IOleCommandTarget *pCommandTarget,
            /* [annotation][in] */ 
            _In_  IOleInPlaceFrame *pFrame,
            /* [annotation][in] */ 
            _In_  IOleInPlaceUIWindow *pDoc);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, HideUI)
        HRESULT ( STDMETHODCALLTYPE *HideUI )( 
            IDocHostUIHandler * This);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, UpdateUI)
        HRESULT ( STDMETHODCALLTYPE *UpdateUI )( 
            IDocHostUIHandler * This);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, EnableModeless)
        HRESULT ( STDMETHODCALLTYPE *EnableModeless )( 
            IDocHostUIHandler * This,
            /* [in] */ BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, OnDocWindowActivate)
        HRESULT ( STDMETHODCALLTYPE *OnDocWindowActivate )( 
            IDocHostUIHandler * This,
            /* [in] */ BOOL fActivate);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, OnFrameWindowActivate)
        HRESULT ( STDMETHODCALLTYPE *OnFrameWindowActivate )( 
            IDocHostUIHandler * This,
            /* [in] */ BOOL fActivate);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, ResizeBorder)
        HRESULT ( STDMETHODCALLTYPE *ResizeBorder )( 
            IDocHostUIHandler * This,
            /* [annotation][in] */ 
            _In_  LPCRECT prcBorder,
            /* [annotation][in] */ 
            _In_  IOleInPlaceUIWindow *pUIWindow,
            /* [annotation][in] */ 
            _In_  BOOL fRameWindow);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, TranslateAccelerator)
        HRESULT ( STDMETHODCALLTYPE *TranslateAccelerator )( 
            IDocHostUIHandler * This,
            /* [in] */ LPMSG lpMsg,
            /* [in] */ const GUID *pguidCmdGroup,
            /* [in] */ DWORD nCmdID);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, GetOptionKeyPath)
        HRESULT ( STDMETHODCALLTYPE *GetOptionKeyPath )( 
            IDocHostUIHandler * This,
            /* [annotation][out] */ 
            _Out_  LPOLESTR *pchKey,
            /* [in] */ DWORD dw);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, GetDropTarget)
        HRESULT ( STDMETHODCALLTYPE *GetDropTarget )( 
            IDocHostUIHandler * This,
            /* [annotation][in] */ 
            _In_  IDropTarget *pDropTarget,
            /* [annotation][out] */ 
            _Outptr_  IDropTarget **ppDropTarget);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, GetExternal)
        HRESULT ( STDMETHODCALLTYPE *GetExternal )( 
            IDocHostUIHandler * This,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IDispatch **ppDispatch);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, TranslateUrl)
        HRESULT ( STDMETHODCALLTYPE *TranslateUrl )( 
            IDocHostUIHandler * This,
            /* [in] */ DWORD dwTranslate,
            /* [annotation][in] */ 
            _In_  LPWSTR pchURLIn,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *ppchURLOut);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, FilterDataObject)
        HRESULT ( STDMETHODCALLTYPE *FilterDataObject )( 
            IDocHostUIHandler * This,
            /* [annotation][in] */ 
            _In_  IDataObject *pDO,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IDataObject **ppDORet);
        
        END_INTERFACE
    } IDocHostUIHandlerVtbl;

    interface IDocHostUIHandler
    {
        CONST_VTBL struct IDocHostUIHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDocHostUIHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDocHostUIHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDocHostUIHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDocHostUIHandler_ShowContextMenu(This,dwID,ppt,pcmdtReserved,pdispReserved)	\
    ( (This)->lpVtbl -> ShowContextMenu(This,dwID,ppt,pcmdtReserved,pdispReserved) ) 

#define IDocHostUIHandler_GetHostInfo(This,pInfo)	\
    ( (This)->lpVtbl -> GetHostInfo(This,pInfo) ) 

#define IDocHostUIHandler_ShowUI(This,dwID,pActiveObject,pCommandTarget,pFrame,pDoc)	\
    ( (This)->lpVtbl -> ShowUI(This,dwID,pActiveObject,pCommandTarget,pFrame,pDoc) ) 

#define IDocHostUIHandler_HideUI(This)	\
    ( (This)->lpVtbl -> HideUI(This) ) 

#define IDocHostUIHandler_UpdateUI(This)	\
    ( (This)->lpVtbl -> UpdateUI(This) ) 

#define IDocHostUIHandler_EnableModeless(This,fEnable)	\
    ( (This)->lpVtbl -> EnableModeless(This,fEnable) ) 

#define IDocHostUIHandler_OnDocWindowActivate(This,fActivate)	\
    ( (This)->lpVtbl -> OnDocWindowActivate(This,fActivate) ) 

#define IDocHostUIHandler_OnFrameWindowActivate(This,fActivate)	\
    ( (This)->lpVtbl -> OnFrameWindowActivate(This,fActivate) ) 

#define IDocHostUIHandler_ResizeBorder(This,prcBorder,pUIWindow,fRameWindow)	\
    ( (This)->lpVtbl -> ResizeBorder(This,prcBorder,pUIWindow,fRameWindow) ) 

#define IDocHostUIHandler_TranslateAccelerator(This,lpMsg,pguidCmdGroup,nCmdID)	\
    ( (This)->lpVtbl -> TranslateAccelerator(This,lpMsg,pguidCmdGroup,nCmdID) ) 

#define IDocHostUIHandler_GetOptionKeyPath(This,pchKey,dw)	\
    ( (This)->lpVtbl -> GetOptionKeyPath(This,pchKey,dw) ) 

#define IDocHostUIHandler_GetDropTarget(This,pDropTarget,ppDropTarget)	\
    ( (This)->lpVtbl -> GetDropTarget(This,pDropTarget,ppDropTarget) ) 

#define IDocHostUIHandler_GetExternal(This,ppDispatch)	\
    ( (This)->lpVtbl -> GetExternal(This,ppDispatch) ) 

#define IDocHostUIHandler_TranslateUrl(This,dwTranslate,pchURLIn,ppchURLOut)	\
    ( (This)->lpVtbl -> TranslateUrl(This,dwTranslate,pchURLIn,ppchURLOut) ) 

#define IDocHostUIHandler_FilterDataObject(This,pDO,ppDORet)	\
    ( (This)->lpVtbl -> FilterDataObject(This,pDO,ppDORet) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDocHostUIHandler_INTERFACE_DEFINED__ */


#ifndef __IDocHostUIHandler2_INTERFACE_DEFINED__
#define __IDocHostUIHandler2_INTERFACE_DEFINED__

/* interface IDocHostUIHandler2 */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IDocHostUIHandler2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3050f6d0-98b5-11cf-bb82-00aa00bdce0b")
    IDocHostUIHandler2 : public IDocHostUIHandler
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOverrideKeyPath( 
            /* [annotation][out] */ 
            _Outptr_  LPOLESTR *pchKey,
            /* [in] */ DWORD dw) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDocHostUIHandler2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDocHostUIHandler2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDocHostUIHandler2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDocHostUIHandler2 * This);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, ShowContextMenu)
        HRESULT ( STDMETHODCALLTYPE *ShowContextMenu )( 
            IDocHostUIHandler2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwID,
            /* [annotation][in] */ 
            _In_  POINT *ppt,
            /* [annotation][in] */ 
            _In_  IUnknown *pcmdtReserved,
            /* [annotation][in] */ 
            _In_  IDispatch *pdispReserved);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, GetHostInfo)
        HRESULT ( STDMETHODCALLTYPE *GetHostInfo )( 
            IDocHostUIHandler2 * This,
            /* [annotation][out][in] */ 
            _Inout_  DOCHOSTUIINFO *pInfo);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, ShowUI)
        HRESULT ( STDMETHODCALLTYPE *ShowUI )( 
            IDocHostUIHandler2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwID,
            /* [annotation][in] */ 
            _In_  IOleInPlaceActiveObject *pActiveObject,
            /* [annotation][in] */ 
            _In_  IOleCommandTarget *pCommandTarget,
            /* [annotation][in] */ 
            _In_  IOleInPlaceFrame *pFrame,
            /* [annotation][in] */ 
            _In_  IOleInPlaceUIWindow *pDoc);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, HideUI)
        HRESULT ( STDMETHODCALLTYPE *HideUI )( 
            IDocHostUIHandler2 * This);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, UpdateUI)
        HRESULT ( STDMETHODCALLTYPE *UpdateUI )( 
            IDocHostUIHandler2 * This);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, EnableModeless)
        HRESULT ( STDMETHODCALLTYPE *EnableModeless )( 
            IDocHostUIHandler2 * This,
            /* [in] */ BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, OnDocWindowActivate)
        HRESULT ( STDMETHODCALLTYPE *OnDocWindowActivate )( 
            IDocHostUIHandler2 * This,
            /* [in] */ BOOL fActivate);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, OnFrameWindowActivate)
        HRESULT ( STDMETHODCALLTYPE *OnFrameWindowActivate )( 
            IDocHostUIHandler2 * This,
            /* [in] */ BOOL fActivate);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, ResizeBorder)
        HRESULT ( STDMETHODCALLTYPE *ResizeBorder )( 
            IDocHostUIHandler2 * This,
            /* [annotation][in] */ 
            _In_  LPCRECT prcBorder,
            /* [annotation][in] */ 
            _In_  IOleInPlaceUIWindow *pUIWindow,
            /* [annotation][in] */ 
            _In_  BOOL fRameWindow);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, TranslateAccelerator)
        HRESULT ( STDMETHODCALLTYPE *TranslateAccelerator )( 
            IDocHostUIHandler2 * This,
            /* [in] */ LPMSG lpMsg,
            /* [in] */ const GUID *pguidCmdGroup,
            /* [in] */ DWORD nCmdID);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, GetOptionKeyPath)
        HRESULT ( STDMETHODCALLTYPE *GetOptionKeyPath )( 
            IDocHostUIHandler2 * This,
            /* [annotation][out] */ 
            _Out_  LPOLESTR *pchKey,
            /* [in] */ DWORD dw);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, GetDropTarget)
        HRESULT ( STDMETHODCALLTYPE *GetDropTarget )( 
            IDocHostUIHandler2 * This,
            /* [annotation][in] */ 
            _In_  IDropTarget *pDropTarget,
            /* [annotation][out] */ 
            _Outptr_  IDropTarget **ppDropTarget);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, GetExternal)
        HRESULT ( STDMETHODCALLTYPE *GetExternal )( 
            IDocHostUIHandler2 * This,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IDispatch **ppDispatch);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, TranslateUrl)
        HRESULT ( STDMETHODCALLTYPE *TranslateUrl )( 
            IDocHostUIHandler2 * This,
            /* [in] */ DWORD dwTranslate,
            /* [annotation][in] */ 
            _In_  LPWSTR pchURLIn,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *ppchURLOut);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler, FilterDataObject)
        HRESULT ( STDMETHODCALLTYPE *FilterDataObject )( 
            IDocHostUIHandler2 * This,
            /* [annotation][in] */ 
            _In_  IDataObject *pDO,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IDataObject **ppDORet);
        
        DECLSPEC_XFGVIRT(IDocHostUIHandler2, GetOverrideKeyPath)
        HRESULT ( STDMETHODCALLTYPE *GetOverrideKeyPath )( 
            IDocHostUIHandler2 * This,
            /* [annotation][out] */ 
            _Outptr_  LPOLESTR *pchKey,
            /* [in] */ DWORD dw);
        
        END_INTERFACE
    } IDocHostUIHandler2Vtbl;

    interface IDocHostUIHandler2
    {
        CONST_VTBL struct IDocHostUIHandler2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDocHostUIHandler2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDocHostUIHandler2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDocHostUIHandler2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDocHostUIHandler2_ShowContextMenu(This,dwID,ppt,pcmdtReserved,pdispReserved)	\
    ( (This)->lpVtbl -> ShowContextMenu(This,dwID,ppt,pcmdtReserved,pdispReserved) ) 

#define IDocHostUIHandler2_GetHostInfo(This,pInfo)	\
    ( (This)->lpVtbl -> GetHostInfo(This,pInfo) ) 

#define IDocHostUIHandler2_ShowUI(This,dwID,pActiveObject,pCommandTarget,pFrame,pDoc)	\
    ( (This)->lpVtbl -> ShowUI(This,dwID,pActiveObject,pCommandTarget,pFrame,pDoc) ) 

#define IDocHostUIHandler2_HideUI(This)	\
    ( (This)->lpVtbl -> HideUI(This) ) 

#define IDocHostUIHandler2_UpdateUI(This)	\
    ( (This)->lpVtbl -> UpdateUI(This) ) 

#define IDocHostUIHandler2_EnableModeless(This,fEnable)	\
    ( (This)->lpVtbl -> EnableModeless(This,fEnable) ) 

#define IDocHostUIHandler2_OnDocWindowActivate(This,fActivate)	\
    ( (This)->lpVtbl -> OnDocWindowActivate(This,fActivate) ) 

#define IDocHostUIHandler2_OnFrameWindowActivate(This,fActivate)	\
    ( (This)->lpVtbl -> OnFrameWindowActivate(This,fActivate) ) 

#define IDocHostUIHandler2_ResizeBorder(This,prcBorder,pUIWindow,fRameWindow)	\
    ( (This)->lpVtbl -> ResizeBorder(This,prcBorder,pUIWindow,fRameWindow) ) 

#define IDocHostUIHandler2_TranslateAccelerator(This,lpMsg,pguidCmdGroup,nCmdID)	\
    ( (This)->lpVtbl -> TranslateAccelerator(This,lpMsg,pguidCmdGroup,nCmdID) ) 

#define IDocHostUIHandler2_GetOptionKeyPath(This,pchKey,dw)	\
    ( (This)->lpVtbl -> GetOptionKeyPath(This,pchKey,dw) ) 

#define IDocHostUIHandler2_GetDropTarget(This,pDropTarget,ppDropTarget)	\
    ( (This)->lpVtbl -> GetDropTarget(This,pDropTarget,ppDropTarget) ) 

#define IDocHostUIHandler2_GetExternal(This,ppDispatch)	\
    ( (This)->lpVtbl -> GetExternal(This,ppDispatch) ) 

#define IDocHostUIHandler2_TranslateUrl(This,dwTranslate,pchURLIn,ppchURLOut)	\
    ( (This)->lpVtbl -> TranslateUrl(This,dwTranslate,pchURLIn,ppchURLOut) ) 

#define IDocHostUIHandler2_FilterDataObject(This,pDO,ppDORet)	\
    ( (This)->lpVtbl -> FilterDataObject(This,pDO,ppDORet) ) 


#define IDocHostUIHandler2_GetOverrideKeyPath(This,pchKey,dw)	\
    ( (This)->lpVtbl -> GetOverrideKeyPath(This,pchKey,dw) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDocHostUIHandler2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mshtmhst_0000_0004 */
/* [local] */ 

DEFINE_GUID(CGID_DocHostCommandHandler,0xf38bc242,0xb950,0x11d1,0x89,0x18,0x00,0xc0,0x4f,0xc2,0xc8,0x36);


extern RPC_IF_HANDLE __MIDL_itf_mshtmhst_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mshtmhst_0000_0004_v0_0_s_ifspec;

#ifndef __ICustomDoc_INTERFACE_DEFINED__
#define __ICustomDoc_INTERFACE_DEFINED__

/* interface ICustomDoc */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ICustomDoc;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3050f3f0-98b5-11cf-bb82-00aa00bdce0b")
    ICustomDoc : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetUIHandler( 
            /* [in] */ IDocHostUIHandler *pUIHandler) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICustomDocVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICustomDoc * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICustomDoc * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICustomDoc * This);
        
        DECLSPEC_XFGVIRT(ICustomDoc, SetUIHandler)
        HRESULT ( STDMETHODCALLTYPE *SetUIHandler )( 
            ICustomDoc * This,
            /* [in] */ IDocHostUIHandler *pUIHandler);
        
        END_INTERFACE
    } ICustomDocVtbl;

    interface ICustomDoc
    {
        CONST_VTBL struct ICustomDocVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICustomDoc_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICustomDoc_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICustomDoc_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICustomDoc_SetUIHandler(This,pUIHandler)	\
    ( (This)->lpVtbl -> SetUIHandler(This,pUIHandler) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICustomDoc_INTERFACE_DEFINED__ */


#ifndef __IDocHostShowUI_INTERFACE_DEFINED__
#define __IDocHostShowUI_INTERFACE_DEFINED__

/* interface IDocHostShowUI */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IDocHostShowUI;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c4d244b0-d43e-11cf-893b-00aa00bdce1a")
    IDocHostShowUI : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ShowMessage( 
            /* [in] */ HWND hwnd,
            /* [annotation][in] */ 
            _In_  LPOLESTR lpstrText,
            /* [annotation][in] */ 
            _In_  LPOLESTR lpstrCaption,
            /* [in] */ DWORD dwType,
            /* [annotation][in] */ 
            _In_  LPOLESTR lpstrHelpFile,
            /* [in] */ DWORD dwHelpContext,
            /* [out] */ LRESULT *plResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShowHelp( 
            /* [in] */ HWND hwnd,
            /* [annotation][in] */ 
            _In_  LPOLESTR pszHelpFile,
            /* [in] */ UINT uCommand,
            /* [in] */ DWORD dwData,
            /* [in] */ POINT ptMouse,
            /* [out] */ IDispatch *pDispatchObjectHit) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDocHostShowUIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDocHostShowUI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDocHostShowUI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDocHostShowUI * This);
        
        DECLSPEC_XFGVIRT(IDocHostShowUI, ShowMessage)
        HRESULT ( STDMETHODCALLTYPE *ShowMessage )( 
            IDocHostShowUI * This,
            /* [in] */ HWND hwnd,
            /* [annotation][in] */ 
            _In_  LPOLESTR lpstrText,
            /* [annotation][in] */ 
            _In_  LPOLESTR lpstrCaption,
            /* [in] */ DWORD dwType,
            /* [annotation][in] */ 
            _In_  LPOLESTR lpstrHelpFile,
            /* [in] */ DWORD dwHelpContext,
            /* [out] */ LRESULT *plResult);
        
        DECLSPEC_XFGVIRT(IDocHostShowUI, ShowHelp)
        HRESULT ( STDMETHODCALLTYPE *ShowHelp )( 
            IDocHostShowUI * This,
            /* [in] */ HWND hwnd,
            /* [annotation][in] */ 
            _In_  LPOLESTR pszHelpFile,
            /* [in] */ UINT uCommand,
            /* [in] */ DWORD dwData,
            /* [in] */ POINT ptMouse,
            /* [out] */ IDispatch *pDispatchObjectHit);
        
        END_INTERFACE
    } IDocHostShowUIVtbl;

    interface IDocHostShowUI
    {
        CONST_VTBL struct IDocHostShowUIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDocHostShowUI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDocHostShowUI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDocHostShowUI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDocHostShowUI_ShowMessage(This,hwnd,lpstrText,lpstrCaption,dwType,lpstrHelpFile,dwHelpContext,plResult)	\
    ( (This)->lpVtbl -> ShowMessage(This,hwnd,lpstrText,lpstrCaption,dwType,lpstrHelpFile,dwHelpContext,plResult) ) 

#define IDocHostShowUI_ShowHelp(This,hwnd,pszHelpFile,uCommand,dwData,ptMouse,pDispatchObjectHit)	\
    ( (This)->lpVtbl -> ShowHelp(This,hwnd,pszHelpFile,uCommand,dwData,ptMouse,pDispatchObjectHit) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDocHostShowUI_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mshtmhst_0000_0006 */
/* [local] */ 

#define IClassFactory3      IClassFactoryEx
#define IID_IClassFactory3  IID_IClassFactoryEx


extern RPC_IF_HANDLE __MIDL_itf_mshtmhst_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mshtmhst_0000_0006_v0_0_s_ifspec;

#ifndef __IClassFactoryEx_INTERFACE_DEFINED__
#define __IClassFactoryEx_INTERFACE_DEFINED__

/* interface IClassFactoryEx */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IClassFactoryEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("342D1EA0-AE25-11D1-89C5-006008C3FBFC")
    IClassFactoryEx : public IClassFactory
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateInstanceWithContext( 
            IUnknown *punkContext,
            IUnknown *punkOuter,
            REFIID riid,
            /* [out] */ void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IClassFactoryExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IClassFactoryEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IClassFactoryEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IClassFactoryEx * This);
        
        DECLSPEC_XFGVIRT(IClassFactory, CreateInstance)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            IClassFactoryEx * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *pUnkOuter,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IClassFactory, LockServer)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *LockServer )( 
            IClassFactoryEx * This,
            /* [in] */ BOOL fLock);
        
        DECLSPEC_XFGVIRT(IClassFactoryEx, CreateInstanceWithContext)
        HRESULT ( STDMETHODCALLTYPE *CreateInstanceWithContext )( 
            IClassFactoryEx * This,
            IUnknown *punkContext,
            IUnknown *punkOuter,
            REFIID riid,
            /* [out] */ void **ppv);
        
        END_INTERFACE
    } IClassFactoryExVtbl;

    interface IClassFactoryEx
    {
        CONST_VTBL struct IClassFactoryExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IClassFactoryEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IClassFactoryEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IClassFactoryEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IClassFactoryEx_CreateInstance(This,pUnkOuter,riid,ppvObject)	\
    ( (This)->lpVtbl -> CreateInstance(This,pUnkOuter,riid,ppvObject) ) 

#define IClassFactoryEx_LockServer(This,fLock)	\
    ( (This)->lpVtbl -> LockServer(This,fLock) ) 


#define IClassFactoryEx_CreateInstanceWithContext(This,punkContext,punkOuter,riid,ppv)	\
    ( (This)->lpVtbl -> CreateInstanceWithContext(This,punkContext,punkOuter,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IClassFactoryEx_INTERFACE_DEFINED__ */


#ifndef __IHTMLOMWindowServices_INTERFACE_DEFINED__
#define __IHTMLOMWindowServices_INTERFACE_DEFINED__

/* interface IHTMLOMWindowServices */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IHTMLOMWindowServices;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3050f5fc-98b5-11cf-bb82-00aa00bdce0b")
    IHTMLOMWindowServices : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE moveTo( 
            /* [in] */ LONG x,
            /* [in] */ LONG y) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE moveBy( 
            /* [in] */ LONG x,
            /* [in] */ LONG y) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE resizeTo( 
            /* [in] */ LONG x,
            /* [in] */ LONG y) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE resizeBy( 
            /* [in] */ LONG x,
            /* [in] */ LONG y) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHTMLOMWindowServicesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IHTMLOMWindowServices * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IHTMLOMWindowServices * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IHTMLOMWindowServices * This);
        
        DECLSPEC_XFGVIRT(IHTMLOMWindowServices, moveTo)
        HRESULT ( STDMETHODCALLTYPE *moveTo )( 
            __RPC__in IHTMLOMWindowServices * This,
            /* [in] */ LONG x,
            /* [in] */ LONG y);
        
        DECLSPEC_XFGVIRT(IHTMLOMWindowServices, moveBy)
        HRESULT ( STDMETHODCALLTYPE *moveBy )( 
            __RPC__in IHTMLOMWindowServices * This,
            /* [in] */ LONG x,
            /* [in] */ LONG y);
        
        DECLSPEC_XFGVIRT(IHTMLOMWindowServices, resizeTo)
        HRESULT ( STDMETHODCALLTYPE *resizeTo )( 
            __RPC__in IHTMLOMWindowServices * This,
            /* [in] */ LONG x,
            /* [in] */ LONG y);
        
        DECLSPEC_XFGVIRT(IHTMLOMWindowServices, resizeBy)
        HRESULT ( STDMETHODCALLTYPE *resizeBy )( 
            __RPC__in IHTMLOMWindowServices * This,
            /* [in] */ LONG x,
            /* [in] */ LONG y);
        
        END_INTERFACE
    } IHTMLOMWindowServicesVtbl;

    interface IHTMLOMWindowServices
    {
        CONST_VTBL struct IHTMLOMWindowServicesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHTMLOMWindowServices_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHTMLOMWindowServices_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHTMLOMWindowServices_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHTMLOMWindowServices_moveTo(This,x,y)	\
    ( (This)->lpVtbl -> moveTo(This,x,y) ) 

#define IHTMLOMWindowServices_moveBy(This,x,y)	\
    ( (This)->lpVtbl -> moveBy(This,x,y) ) 

#define IHTMLOMWindowServices_resizeTo(This,x,y)	\
    ( (This)->lpVtbl -> resizeTo(This,x,y) ) 

#define IHTMLOMWindowServices_resizeBy(This,x,y)	\
    ( (This)->lpVtbl -> resizeBy(This,x,y) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHTMLOMWindowServices_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mshtmhst_0000_0008 */
/* [local] */ 

#define SID_SHTMLOMWindowServices  IID_IHTMLOMWindowServices
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mshtmhst_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mshtmhst_0000_0008_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


