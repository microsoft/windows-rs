

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


#ifndef __exdisp_h__
#define __exdisp_h__

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

#ifndef __IWebBrowser_FWD_DEFINED__
#define __IWebBrowser_FWD_DEFINED__
typedef interface IWebBrowser IWebBrowser;

#endif 	/* __IWebBrowser_FWD_DEFINED__ */


#ifndef __DWebBrowserEvents_FWD_DEFINED__
#define __DWebBrowserEvents_FWD_DEFINED__
typedef interface DWebBrowserEvents DWebBrowserEvents;

#endif 	/* __DWebBrowserEvents_FWD_DEFINED__ */


#ifndef __IWebBrowserApp_FWD_DEFINED__
#define __IWebBrowserApp_FWD_DEFINED__
typedef interface IWebBrowserApp IWebBrowserApp;

#endif 	/* __IWebBrowserApp_FWD_DEFINED__ */


#ifndef __IWebBrowser2_FWD_DEFINED__
#define __IWebBrowser2_FWD_DEFINED__
typedef interface IWebBrowser2 IWebBrowser2;

#endif 	/* __IWebBrowser2_FWD_DEFINED__ */


#ifndef __DWebBrowserEvents2_FWD_DEFINED__
#define __DWebBrowserEvents2_FWD_DEFINED__
typedef interface DWebBrowserEvents2 DWebBrowserEvents2;

#endif 	/* __DWebBrowserEvents2_FWD_DEFINED__ */


#ifndef __WebBrowser_V1_FWD_DEFINED__
#define __WebBrowser_V1_FWD_DEFINED__

#ifdef __cplusplus
typedef class WebBrowser_V1 WebBrowser_V1;
#else
typedef struct WebBrowser_V1 WebBrowser_V1;
#endif /* __cplusplus */

#endif 	/* __WebBrowser_V1_FWD_DEFINED__ */


#ifndef __WebBrowser_FWD_DEFINED__
#define __WebBrowser_FWD_DEFINED__

#ifdef __cplusplus
typedef class WebBrowser WebBrowser;
#else
typedef struct WebBrowser WebBrowser;
#endif /* __cplusplus */

#endif 	/* __WebBrowser_FWD_DEFINED__ */


#ifndef __InternetExplorer_FWD_DEFINED__
#define __InternetExplorer_FWD_DEFINED__

#ifdef __cplusplus
typedef class InternetExplorer InternetExplorer;
#else
typedef struct InternetExplorer InternetExplorer;
#endif /* __cplusplus */

#endif 	/* __InternetExplorer_FWD_DEFINED__ */


#ifndef __InternetExplorerMedium_FWD_DEFINED__
#define __InternetExplorerMedium_FWD_DEFINED__

#ifdef __cplusplus
typedef class InternetExplorerMedium InternetExplorerMedium;
#else
typedef struct InternetExplorerMedium InternetExplorerMedium;
#endif /* __cplusplus */

#endif 	/* __InternetExplorerMedium_FWD_DEFINED__ */


#ifndef __ShellBrowserWindow_FWD_DEFINED__
#define __ShellBrowserWindow_FWD_DEFINED__

#ifdef __cplusplus
typedef class ShellBrowserWindow ShellBrowserWindow;
#else
typedef struct ShellBrowserWindow ShellBrowserWindow;
#endif /* __cplusplus */

#endif 	/* __ShellBrowserWindow_FWD_DEFINED__ */


#ifndef __DShellWindowsEvents_FWD_DEFINED__
#define __DShellWindowsEvents_FWD_DEFINED__
typedef interface DShellWindowsEvents DShellWindowsEvents;

#endif 	/* __DShellWindowsEvents_FWD_DEFINED__ */


#ifndef __IShellWindows_FWD_DEFINED__
#define __IShellWindows_FWD_DEFINED__
typedef interface IShellWindows IShellWindows;

#endif 	/* __IShellWindows_FWD_DEFINED__ */


#ifndef __ShellWindows_FWD_DEFINED__
#define __ShellWindows_FWD_DEFINED__

#ifdef __cplusplus
typedef class ShellWindows ShellWindows;
#else
typedef struct ShellWindows ShellWindows;
#endif /* __cplusplus */

#endif 	/* __ShellWindows_FWD_DEFINED__ */


#ifndef __IShellUIHelper_FWD_DEFINED__
#define __IShellUIHelper_FWD_DEFINED__
typedef interface IShellUIHelper IShellUIHelper;

#endif 	/* __IShellUIHelper_FWD_DEFINED__ */


#ifndef __IShellUIHelper2_FWD_DEFINED__
#define __IShellUIHelper2_FWD_DEFINED__
typedef interface IShellUIHelper2 IShellUIHelper2;

#endif 	/* __IShellUIHelper2_FWD_DEFINED__ */


#ifndef __IShellUIHelper3_FWD_DEFINED__
#define __IShellUIHelper3_FWD_DEFINED__
typedef interface IShellUIHelper3 IShellUIHelper3;

#endif 	/* __IShellUIHelper3_FWD_DEFINED__ */


#ifndef __IShellUIHelper4_FWD_DEFINED__
#define __IShellUIHelper4_FWD_DEFINED__
typedef interface IShellUIHelper4 IShellUIHelper4;

#endif 	/* __IShellUIHelper4_FWD_DEFINED__ */


#ifndef __IShellUIHelper5_FWD_DEFINED__
#define __IShellUIHelper5_FWD_DEFINED__
typedef interface IShellUIHelper5 IShellUIHelper5;

#endif 	/* __IShellUIHelper5_FWD_DEFINED__ */


#ifndef __IShellUIHelper6_FWD_DEFINED__
#define __IShellUIHelper6_FWD_DEFINED__
typedef interface IShellUIHelper6 IShellUIHelper6;

#endif 	/* __IShellUIHelper6_FWD_DEFINED__ */


#ifndef __IShellUIHelper7_FWD_DEFINED__
#define __IShellUIHelper7_FWD_DEFINED__
typedef interface IShellUIHelper7 IShellUIHelper7;

#endif 	/* __IShellUIHelper7_FWD_DEFINED__ */


#ifndef __IShellUIHelper8_FWD_DEFINED__
#define __IShellUIHelper8_FWD_DEFINED__
typedef interface IShellUIHelper8 IShellUIHelper8;

#endif 	/* __IShellUIHelper8_FWD_DEFINED__ */


#ifndef __IShellUIHelper9_FWD_DEFINED__
#define __IShellUIHelper9_FWD_DEFINED__
typedef interface IShellUIHelper9 IShellUIHelper9;

#endif 	/* __IShellUIHelper9_FWD_DEFINED__ */


#ifndef __ShellUIHelper_FWD_DEFINED__
#define __ShellUIHelper_FWD_DEFINED__

#ifdef __cplusplus
typedef class ShellUIHelper ShellUIHelper;
#else
typedef struct ShellUIHelper ShellUIHelper;
#endif /* __cplusplus */

#endif 	/* __ShellUIHelper_FWD_DEFINED__ */


#ifndef __DShellNameSpaceEvents_FWD_DEFINED__
#define __DShellNameSpaceEvents_FWD_DEFINED__
typedef interface DShellNameSpaceEvents DShellNameSpaceEvents;

#endif 	/* __DShellNameSpaceEvents_FWD_DEFINED__ */


#ifndef __IShellFavoritesNameSpace_FWD_DEFINED__
#define __IShellFavoritesNameSpace_FWD_DEFINED__
typedef interface IShellFavoritesNameSpace IShellFavoritesNameSpace;

#endif 	/* __IShellFavoritesNameSpace_FWD_DEFINED__ */


#ifndef __IShellNameSpace_FWD_DEFINED__
#define __IShellNameSpace_FWD_DEFINED__
typedef interface IShellNameSpace IShellNameSpace;

#endif 	/* __IShellNameSpace_FWD_DEFINED__ */


#ifndef __ShellNameSpace_FWD_DEFINED__
#define __ShellNameSpace_FWD_DEFINED__

#ifdef __cplusplus
typedef class ShellNameSpace ShellNameSpace;
#else
typedef struct ShellNameSpace ShellNameSpace;
#endif /* __cplusplus */

#endif 	/* __ShellNameSpace_FWD_DEFINED__ */


#ifndef __IScriptErrorList_FWD_DEFINED__
#define __IScriptErrorList_FWD_DEFINED__
typedef interface IScriptErrorList IScriptErrorList;

#endif 	/* __IScriptErrorList_FWD_DEFINED__ */


#ifndef __CScriptErrorList_FWD_DEFINED__
#define __CScriptErrorList_FWD_DEFINED__

#ifdef __cplusplus
typedef class CScriptErrorList CScriptErrorList;
#else
typedef struct CScriptErrorList CScriptErrorList;
#endif /* __cplusplus */

#endif 	/* __CScriptErrorList_FWD_DEFINED__ */


/* header files for imported files */
#include "ocidl.h"
#include "docobj.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_exdisp_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// exdisp.h
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
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(pop)
#pragma warning(pop)
#pragma region Desktop Family
#pragma endregion
#define SID_SkipHung IID_IEnumVARIANT


extern RPC_IF_HANDLE __MIDL_itf_exdisp_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_exdisp_0000_0000_v0_0_s_ifspec;


#ifndef __SHDocVw_LIBRARY_DEFINED__
#define __SHDocVw_LIBRARY_DEFINED__

/* library SHDocVw */
/* [version][lcid][helpstring][uuid] */ 

typedef /* [helpstring][uuid] */  DECLSPEC_UUID("34A226E0-DF30-11CF-89A9-00A0C9054129") 
enum CommandStateChangeConstants
    {
        CSC_UPDATECOMMANDS	= ( int  )0xffffffff,
        CSC_NAVIGATEFORWARD	= 0x1,
        CSC_NAVIGATEBACK	= 0x2
    } 	CommandStateChangeConstants;

typedef /* [helpstring][uuid] */  DECLSPEC_UUID("65507BE0-91A8-11d3-A845-009027220E6D") 
enum SecureLockIconConstants
    {
        secureLockIconUnsecure	= 0,
        secureLockIconMixed	= 0x1,
        secureLockIconSecureUnknownBits	= 0x2,
        secureLockIconSecure40Bit	= 0x3,
        secureLockIconSecure56Bit	= 0x4,
        secureLockIconSecureFortezza	= 0x5,
        secureLockIconSecure128Bit	= 0x6
    } 	SecureLockIconConstants;

typedef /* [helpstring][uuid] */  DECLSPEC_UUID("a8317d46-03cb-4975-ae94-85e9f2e1d020") 
enum NewProcessCauseConstants
    {
        ProtectedModeRedirect	= 0x1
    } 	NewProcessCauseConstants;

typedef /* [helpstring][uuid] */  DECLSPEC_UUID("F41E6981-28E5-11d0-82B4-00A0C90C29C5") 
enum ShellWindowTypeConstants
    {
        SWC_EXPLORER	= 0,
        SWC_BROWSER	= 0x1,
        SWC_3RDPARTY	= 0x2,
        SWC_CALLBACK	= 0x4,
        SWC_DESKTOP	= 0x8
    } 	ShellWindowTypeConstants;

typedef /* [hidden][helpstring][uuid] */  DECLSPEC_UUID("7716a370-38ca-11d0-a48b-00a0c90a8f39") 
enum ShellWindowFindWindowOptions
    {
        SWFO_NEEDDISPATCH	= 0x1,
        SWFO_INCLUDEPENDING	= 0x2,
        SWFO_COOKIEPASSED	= 0x4
    } 	ShellWindowFindWindowOptions;


EXTERN_C const IID LIBID_SHDocVw;

#ifndef __IWebBrowser_INTERFACE_DEFINED__
#define __IWebBrowser_INTERFACE_DEFINED__

/* interface IWebBrowser */
/* [object][oleautomation][dual][hidden][helpcontext][helpstring][uuid] */ 

typedef /* [helpstring][uuid] */  DECLSPEC_UUID("14EE5380-A378-11cf-A731-00A0C9082637") 
enum BrowserNavConstants
    {
        navOpenInNewWindow	= 0x1,
        navNoHistory	= 0x2,
        navNoReadFromCache	= 0x4,
        navNoWriteToCache	= 0x8,
        navAllowAutosearch	= 0x10,
        navBrowserBar	= 0x20,
        navHyperlink	= 0x40,
        navEnforceRestricted	= 0x80,
        navNewWindowsManaged	= 0x100,
        navUntrustedForDownload	= 0x200,
        navTrustedForActiveX	= 0x400,
        navOpenInNewTab	= 0x800,
        navOpenInBackgroundTab	= 0x1000,
        navKeepWordWheelText	= 0x2000,
        navVirtualTab	= 0x4000,
        navBlockRedirectsXDomain	= 0x8000,
        navOpenNewForegroundTab	= 0x10000,
        navTravelLogScreenshot	= 0x20000,
        navDeferUnload	= 0x40000,
        navSpeculative	= 0x80000,
        navSuggestNewWindow	= 0x100000,
        navSuggestNewTab	= 0x200000,
        navReserved1	= 0x400000,
        navHomepageNavigate	= 0x800000,
        navRefresh	= 0x1000000,
        navHostNavigation	= 0x2000000,
        navReserved2	= 0x4000000,
        navReserved3	= 0x8000000,
        navReserved4	= 0x10000000,
        navReserved5	= 0x20000000,
        navReserved6	= 0x40000000,
        navReserved7	= 0x80000000
    } 	BrowserNavConstants;

//;begin_internal
#define navUserInitiatedAction navReserved1
#define navDisableDownloadSave navReserved2
#define navServerRedirectedVtabSwitch navReserved3
#define navVtabSwitchNotUserInitiatedAction navReserved4
#define navCheckDontShowNeedIE navReserved5
#define navCheckDontShowNeedHVSI navReserved6
#define navCheckDontShowHVSINeedHost navReserved7
//;end_internal
typedef /* [helpstring][uuid] */  DECLSPEC_UUID("C317C261-A991-11cf-A731-00A0C9082637") 
enum RefreshConstants
    {
        REFRESH_NORMAL	= 0,
        REFRESH_IFEXPIRED	= 1,
        REFRESH_COMPLETELY	= 3
    } 	RefreshConstants;


EXTERN_C const IID IID_IWebBrowser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EAB22AC1-30C1-11CF-A7EB-0000C05BAE0B")
    IWebBrowser : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GoBack( void) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GoForward( void) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GoHome( void) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GoSearch( void) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Navigate( 
            /* [in] */ __RPC__in BSTR URL,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Flags,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *TargetFrameName,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *PostData,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Headers) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh2( 
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Level) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Application( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Parent( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Container( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Document( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_TopLevelContainer( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Type) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Left( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Left( 
            /* [in] */ long Left) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Top( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Top( 
            /* [in] */ long Top) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Width( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Width( 
            /* [in] */ long Width) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Height( 
            /* [retval][out] */ __RPC__out long *pl) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Height( 
            /* [in] */ long Height) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LocationName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *LocationName) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_LocationURL( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *LocationURL) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Busy( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebBrowserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWebBrowser * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWebBrowser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWebBrowser * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWebBrowser * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWebBrowser * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWebBrowser * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWebBrowser * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWebBrowser, GoBack)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GoBack )( 
            __RPC__in IWebBrowser * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, GoForward)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GoForward )( 
            __RPC__in IWebBrowser * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, GoHome)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GoHome )( 
            __RPC__in IWebBrowser * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, GoSearch)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GoSearch )( 
            __RPC__in IWebBrowser * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, Navigate)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Navigate )( 
            __RPC__in IWebBrowser * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Flags,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *TargetFrameName,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *PostData,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Headers);
        
        DECLSPEC_XFGVIRT(IWebBrowser, Refresh)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IWebBrowser * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, Refresh2)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh2 )( 
            __RPC__in IWebBrowser * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Level);
        
        DECLSPEC_XFGVIRT(IWebBrowser, Stop)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IWebBrowser * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Application)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in IWebBrowser * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Parent)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IWebBrowser * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Container)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Container )( 
            __RPC__in IWebBrowser * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Document)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Document )( 
            __RPC__in IWebBrowser * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_TopLevelContainer)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_TopLevelContainer )( 
            __RPC__in IWebBrowser * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Type)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IWebBrowser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Type);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Left)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Left )( 
            __RPC__in IWebBrowser * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(IWebBrowser, put_Left)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Left )( 
            __RPC__in IWebBrowser * This,
            /* [in] */ long Left);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Top)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Top )( 
            __RPC__in IWebBrowser * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(IWebBrowser, put_Top)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Top )( 
            __RPC__in IWebBrowser * This,
            /* [in] */ long Top);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Width)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Width )( 
            __RPC__in IWebBrowser * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(IWebBrowser, put_Width)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Width )( 
            __RPC__in IWebBrowser * This,
            /* [in] */ long Width);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Height)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Height )( 
            __RPC__in IWebBrowser * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(IWebBrowser, put_Height)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Height )( 
            __RPC__in IWebBrowser * This,
            /* [in] */ long Height);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_LocationName)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LocationName )( 
            __RPC__in IWebBrowser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *LocationName);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_LocationURL)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LocationURL )( 
            __RPC__in IWebBrowser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *LocationURL);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Busy)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Busy )( 
            __RPC__in IWebBrowser * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        END_INTERFACE
    } IWebBrowserVtbl;

    interface IWebBrowser
    {
        CONST_VTBL struct IWebBrowserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebBrowser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebBrowser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebBrowser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebBrowser_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWebBrowser_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWebBrowser_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWebBrowser_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWebBrowser_GoBack(This)	\
    ( (This)->lpVtbl -> GoBack(This) ) 

#define IWebBrowser_GoForward(This)	\
    ( (This)->lpVtbl -> GoForward(This) ) 

#define IWebBrowser_GoHome(This)	\
    ( (This)->lpVtbl -> GoHome(This) ) 

#define IWebBrowser_GoSearch(This)	\
    ( (This)->lpVtbl -> GoSearch(This) ) 

#define IWebBrowser_Navigate(This,URL,Flags,TargetFrameName,PostData,Headers)	\
    ( (This)->lpVtbl -> Navigate(This,URL,Flags,TargetFrameName,PostData,Headers) ) 

#define IWebBrowser_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IWebBrowser_Refresh2(This,Level)	\
    ( (This)->lpVtbl -> Refresh2(This,Level) ) 

#define IWebBrowser_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IWebBrowser_get_Application(This,ppDisp)	\
    ( (This)->lpVtbl -> get_Application(This,ppDisp) ) 

#define IWebBrowser_get_Parent(This,ppDisp)	\
    ( (This)->lpVtbl -> get_Parent(This,ppDisp) ) 

#define IWebBrowser_get_Container(This,ppDisp)	\
    ( (This)->lpVtbl -> get_Container(This,ppDisp) ) 

#define IWebBrowser_get_Document(This,ppDisp)	\
    ( (This)->lpVtbl -> get_Document(This,ppDisp) ) 

#define IWebBrowser_get_TopLevelContainer(This,pBool)	\
    ( (This)->lpVtbl -> get_TopLevelContainer(This,pBool) ) 

#define IWebBrowser_get_Type(This,Type)	\
    ( (This)->lpVtbl -> get_Type(This,Type) ) 

#define IWebBrowser_get_Left(This,pl)	\
    ( (This)->lpVtbl -> get_Left(This,pl) ) 

#define IWebBrowser_put_Left(This,Left)	\
    ( (This)->lpVtbl -> put_Left(This,Left) ) 

#define IWebBrowser_get_Top(This,pl)	\
    ( (This)->lpVtbl -> get_Top(This,pl) ) 

#define IWebBrowser_put_Top(This,Top)	\
    ( (This)->lpVtbl -> put_Top(This,Top) ) 

#define IWebBrowser_get_Width(This,pl)	\
    ( (This)->lpVtbl -> get_Width(This,pl) ) 

#define IWebBrowser_put_Width(This,Width)	\
    ( (This)->lpVtbl -> put_Width(This,Width) ) 

#define IWebBrowser_get_Height(This,pl)	\
    ( (This)->lpVtbl -> get_Height(This,pl) ) 

#define IWebBrowser_put_Height(This,Height)	\
    ( (This)->lpVtbl -> put_Height(This,Height) ) 

#define IWebBrowser_get_LocationName(This,LocationName)	\
    ( (This)->lpVtbl -> get_LocationName(This,LocationName) ) 

#define IWebBrowser_get_LocationURL(This,LocationURL)	\
    ( (This)->lpVtbl -> get_LocationURL(This,LocationURL) ) 

#define IWebBrowser_get_Busy(This,pBool)	\
    ( (This)->lpVtbl -> get_Busy(This,pBool) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebBrowser_INTERFACE_DEFINED__ */


#ifndef __DWebBrowserEvents_DISPINTERFACE_DEFINED__
#define __DWebBrowserEvents_DISPINTERFACE_DEFINED__

/* dispinterface DWebBrowserEvents */
/* [hidden][helpstring][uuid] */ 


EXTERN_C const IID DIID_DWebBrowserEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("EAB22AC2-30C1-11CF-A7EB-0000C05BAE0B")
    DWebBrowserEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct DWebBrowserEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DWebBrowserEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DWebBrowserEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DWebBrowserEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DWebBrowserEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DWebBrowserEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DWebBrowserEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DWebBrowserEvents * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } DWebBrowserEventsVtbl;

    interface DWebBrowserEvents
    {
        CONST_VTBL struct DWebBrowserEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DWebBrowserEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DWebBrowserEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DWebBrowserEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DWebBrowserEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DWebBrowserEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DWebBrowserEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DWebBrowserEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __DWebBrowserEvents_DISPINTERFACE_DEFINED__ */


#ifndef __IWebBrowserApp_INTERFACE_DEFINED__
#define __IWebBrowserApp_INTERFACE_DEFINED__

/* interface IWebBrowserApp */
/* [object][dual][oleautomation][hidden][helpcontext][helpstring][uuid] */ 


EXTERN_C const IID IID_IWebBrowserApp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0002DF05-0000-0000-C000-000000000046")
    IWebBrowserApp : public IWebBrowser
    {
    public:
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Quit( void) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ClientToWindow( 
            /* [out][in] */ __RPC__inout int *pcx,
            /* [out][in] */ __RPC__inout int *pcy) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE PutProperty( 
            /* [in] */ __RPC__in BSTR Property,
            /* [in] */ VARIANT vtValue) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ __RPC__in BSTR Property,
            /* [retval][out] */ __RPC__out VARIANT *pvtValue) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_HWND( 
            /* [retval][out] */ __RPC__out SHANDLE_PTR *pHWND) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_FullName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *FullName) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Path) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Visible( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Visible( 
            /* [in] */ VARIANT_BOOL Value) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_StatusBar( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_StatusBar( 
            /* [in] */ VARIANT_BOOL Value) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_StatusText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *StatusText) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_StatusText( 
            /* [in] */ __RPC__in BSTR StatusText) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ToolBar( 
            /* [retval][out] */ __RPC__out int *Value) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ToolBar( 
            /* [in] */ int Value) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MenuBar( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Value) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MenuBar( 
            /* [in] */ VARIANT_BOOL Value) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_FullScreen( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbFullScreen) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_FullScreen( 
            /* [in] */ VARIANT_BOOL bFullScreen) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebBrowserAppVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWebBrowserApp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWebBrowserApp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWebBrowserApp * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWebBrowserApp * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWebBrowserApp * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWebBrowserApp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWebBrowserApp * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWebBrowser, GoBack)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GoBack )( 
            __RPC__in IWebBrowserApp * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, GoForward)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GoForward )( 
            __RPC__in IWebBrowserApp * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, GoHome)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GoHome )( 
            __RPC__in IWebBrowserApp * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, GoSearch)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GoSearch )( 
            __RPC__in IWebBrowserApp * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, Navigate)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Navigate )( 
            __RPC__in IWebBrowserApp * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Flags,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *TargetFrameName,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *PostData,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Headers);
        
        DECLSPEC_XFGVIRT(IWebBrowser, Refresh)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IWebBrowserApp * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, Refresh2)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh2 )( 
            __RPC__in IWebBrowserApp * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Level);
        
        DECLSPEC_XFGVIRT(IWebBrowser, Stop)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IWebBrowserApp * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Application)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Parent)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Container)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Container )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Document)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Document )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_TopLevelContainer)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_TopLevelContainer )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Type)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Type);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Left)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Left )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(IWebBrowser, put_Left)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Left )( 
            __RPC__in IWebBrowserApp * This,
            /* [in] */ long Left);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Top)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Top )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(IWebBrowser, put_Top)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Top )( 
            __RPC__in IWebBrowserApp * This,
            /* [in] */ long Top);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Width)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Width )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(IWebBrowser, put_Width)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Width )( 
            __RPC__in IWebBrowserApp * This,
            /* [in] */ long Width);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Height)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Height )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(IWebBrowser, put_Height)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Height )( 
            __RPC__in IWebBrowserApp * This,
            /* [in] */ long Height);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_LocationName)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LocationName )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *LocationName);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_LocationURL)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LocationURL )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *LocationURL);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Busy)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Busy )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, Quit)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Quit )( 
            __RPC__in IWebBrowserApp * This);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, ClientToWindow)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ClientToWindow )( 
            __RPC__in IWebBrowserApp * This,
            /* [out][in] */ __RPC__inout int *pcx,
            /* [out][in] */ __RPC__inout int *pcy);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, PutProperty)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PutProperty )( 
            __RPC__in IWebBrowserApp * This,
            /* [in] */ __RPC__in BSTR Property,
            /* [in] */ VARIANT vtValue);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, GetProperty)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IWebBrowserApp * This,
            /* [in] */ __RPC__in BSTR Property,
            /* [retval][out] */ __RPC__out VARIANT *pvtValue);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_Name)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_HWND)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_HWND )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__out SHANDLE_PTR *pHWND);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_FullName)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_FullName )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *FullName);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_Path)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Path);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_Visible)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Visible )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, put_Visible)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Visible )( 
            __RPC__in IWebBrowserApp * This,
            /* [in] */ VARIANT_BOOL Value);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_StatusBar)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StatusBar )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, put_StatusBar)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StatusBar )( 
            __RPC__in IWebBrowserApp * This,
            /* [in] */ VARIANT_BOOL Value);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_StatusText)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StatusText )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *StatusText);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, put_StatusText)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StatusText )( 
            __RPC__in IWebBrowserApp * This,
            /* [in] */ __RPC__in BSTR StatusText);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_ToolBar)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ToolBar )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__out int *Value);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, put_ToolBar)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ToolBar )( 
            __RPC__in IWebBrowserApp * This,
            /* [in] */ int Value);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_MenuBar)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MenuBar )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Value);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, put_MenuBar)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MenuBar )( 
            __RPC__in IWebBrowserApp * This,
            /* [in] */ VARIANT_BOOL Value);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_FullScreen)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_FullScreen )( 
            __RPC__in IWebBrowserApp * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbFullScreen);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, put_FullScreen)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_FullScreen )( 
            __RPC__in IWebBrowserApp * This,
            /* [in] */ VARIANT_BOOL bFullScreen);
        
        END_INTERFACE
    } IWebBrowserAppVtbl;

    interface IWebBrowserApp
    {
        CONST_VTBL struct IWebBrowserAppVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebBrowserApp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebBrowserApp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebBrowserApp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebBrowserApp_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWebBrowserApp_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWebBrowserApp_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWebBrowserApp_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWebBrowserApp_GoBack(This)	\
    ( (This)->lpVtbl -> GoBack(This) ) 

#define IWebBrowserApp_GoForward(This)	\
    ( (This)->lpVtbl -> GoForward(This) ) 

#define IWebBrowserApp_GoHome(This)	\
    ( (This)->lpVtbl -> GoHome(This) ) 

#define IWebBrowserApp_GoSearch(This)	\
    ( (This)->lpVtbl -> GoSearch(This) ) 

#define IWebBrowserApp_Navigate(This,URL,Flags,TargetFrameName,PostData,Headers)	\
    ( (This)->lpVtbl -> Navigate(This,URL,Flags,TargetFrameName,PostData,Headers) ) 

#define IWebBrowserApp_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IWebBrowserApp_Refresh2(This,Level)	\
    ( (This)->lpVtbl -> Refresh2(This,Level) ) 

#define IWebBrowserApp_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IWebBrowserApp_get_Application(This,ppDisp)	\
    ( (This)->lpVtbl -> get_Application(This,ppDisp) ) 

#define IWebBrowserApp_get_Parent(This,ppDisp)	\
    ( (This)->lpVtbl -> get_Parent(This,ppDisp) ) 

#define IWebBrowserApp_get_Container(This,ppDisp)	\
    ( (This)->lpVtbl -> get_Container(This,ppDisp) ) 

#define IWebBrowserApp_get_Document(This,ppDisp)	\
    ( (This)->lpVtbl -> get_Document(This,ppDisp) ) 

#define IWebBrowserApp_get_TopLevelContainer(This,pBool)	\
    ( (This)->lpVtbl -> get_TopLevelContainer(This,pBool) ) 

#define IWebBrowserApp_get_Type(This,Type)	\
    ( (This)->lpVtbl -> get_Type(This,Type) ) 

#define IWebBrowserApp_get_Left(This,pl)	\
    ( (This)->lpVtbl -> get_Left(This,pl) ) 

#define IWebBrowserApp_put_Left(This,Left)	\
    ( (This)->lpVtbl -> put_Left(This,Left) ) 

#define IWebBrowserApp_get_Top(This,pl)	\
    ( (This)->lpVtbl -> get_Top(This,pl) ) 

#define IWebBrowserApp_put_Top(This,Top)	\
    ( (This)->lpVtbl -> put_Top(This,Top) ) 

#define IWebBrowserApp_get_Width(This,pl)	\
    ( (This)->lpVtbl -> get_Width(This,pl) ) 

#define IWebBrowserApp_put_Width(This,Width)	\
    ( (This)->lpVtbl -> put_Width(This,Width) ) 

#define IWebBrowserApp_get_Height(This,pl)	\
    ( (This)->lpVtbl -> get_Height(This,pl) ) 

#define IWebBrowserApp_put_Height(This,Height)	\
    ( (This)->lpVtbl -> put_Height(This,Height) ) 

#define IWebBrowserApp_get_LocationName(This,LocationName)	\
    ( (This)->lpVtbl -> get_LocationName(This,LocationName) ) 

#define IWebBrowserApp_get_LocationURL(This,LocationURL)	\
    ( (This)->lpVtbl -> get_LocationURL(This,LocationURL) ) 

#define IWebBrowserApp_get_Busy(This,pBool)	\
    ( (This)->lpVtbl -> get_Busy(This,pBool) ) 


#define IWebBrowserApp_Quit(This)	\
    ( (This)->lpVtbl -> Quit(This) ) 

#define IWebBrowserApp_ClientToWindow(This,pcx,pcy)	\
    ( (This)->lpVtbl -> ClientToWindow(This,pcx,pcy) ) 

#define IWebBrowserApp_PutProperty(This,Property,vtValue)	\
    ( (This)->lpVtbl -> PutProperty(This,Property,vtValue) ) 

#define IWebBrowserApp_GetProperty(This,Property,pvtValue)	\
    ( (This)->lpVtbl -> GetProperty(This,Property,pvtValue) ) 

#define IWebBrowserApp_get_Name(This,Name)	\
    ( (This)->lpVtbl -> get_Name(This,Name) ) 

#define IWebBrowserApp_get_HWND(This,pHWND)	\
    ( (This)->lpVtbl -> get_HWND(This,pHWND) ) 

#define IWebBrowserApp_get_FullName(This,FullName)	\
    ( (This)->lpVtbl -> get_FullName(This,FullName) ) 

#define IWebBrowserApp_get_Path(This,Path)	\
    ( (This)->lpVtbl -> get_Path(This,Path) ) 

#define IWebBrowserApp_get_Visible(This,pBool)	\
    ( (This)->lpVtbl -> get_Visible(This,pBool) ) 

#define IWebBrowserApp_put_Visible(This,Value)	\
    ( (This)->lpVtbl -> put_Visible(This,Value) ) 

#define IWebBrowserApp_get_StatusBar(This,pBool)	\
    ( (This)->lpVtbl -> get_StatusBar(This,pBool) ) 

#define IWebBrowserApp_put_StatusBar(This,Value)	\
    ( (This)->lpVtbl -> put_StatusBar(This,Value) ) 

#define IWebBrowserApp_get_StatusText(This,StatusText)	\
    ( (This)->lpVtbl -> get_StatusText(This,StatusText) ) 

#define IWebBrowserApp_put_StatusText(This,StatusText)	\
    ( (This)->lpVtbl -> put_StatusText(This,StatusText) ) 

#define IWebBrowserApp_get_ToolBar(This,Value)	\
    ( (This)->lpVtbl -> get_ToolBar(This,Value) ) 

#define IWebBrowserApp_put_ToolBar(This,Value)	\
    ( (This)->lpVtbl -> put_ToolBar(This,Value) ) 

#define IWebBrowserApp_get_MenuBar(This,Value)	\
    ( (This)->lpVtbl -> get_MenuBar(This,Value) ) 

#define IWebBrowserApp_put_MenuBar(This,Value)	\
    ( (This)->lpVtbl -> put_MenuBar(This,Value) ) 

#define IWebBrowserApp_get_FullScreen(This,pbFullScreen)	\
    ( (This)->lpVtbl -> get_FullScreen(This,pbFullScreen) ) 

#define IWebBrowserApp_put_FullScreen(This,bFullScreen)	\
    ( (This)->lpVtbl -> put_FullScreen(This,bFullScreen) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebBrowserApp_INTERFACE_DEFINED__ */


#ifndef __IWebBrowser2_INTERFACE_DEFINED__
#define __IWebBrowser2_INTERFACE_DEFINED__

/* interface IWebBrowser2 */
/* [object][dual][oleautomation][hidden][helpcontext][helpstring][uuid] */ 


EXTERN_C const IID IID_IWebBrowser2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D30C1661-CDAF-11d0-8A3E-00C04FC9E26E")
    IWebBrowser2 : public IWebBrowserApp
    {
    public:
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Navigate2( 
            /* [in] */ __RPC__in VARIANT *URL,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Flags,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *TargetFrameName,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *PostData,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Headers) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE QueryStatusWB( 
            /* [in] */ OLECMDID cmdID,
            /* [retval][out] */ __RPC__out OLECMDF *pcmdf) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ExecWB( 
            /* [in] */ OLECMDID cmdID,
            /* [in] */ OLECMDEXECOPT cmdexecopt,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *pvaIn,
            /* [unique][optional][out][in] */ __RPC__inout_opt VARIANT *pvaOut) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ShowBrowserBar( 
            /* [in] */ __RPC__in VARIANT *pvaClsid,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *pvarShow,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *pvarSize) = 0;
        
        virtual /* [bindable][propget][id] */ HRESULT STDMETHODCALLTYPE get_ReadyState( 
            /* [out][retval] */ __RPC__out READYSTATE *plReadyState) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Offline( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbOffline) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Offline( 
            /* [in] */ VARIANT_BOOL bOffline) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Silent( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbSilent) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Silent( 
            /* [in] */ VARIANT_BOOL bSilent) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RegisterAsBrowser( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRegister) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_RegisterAsBrowser( 
            /* [in] */ VARIANT_BOOL bRegister) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RegisterAsDropTarget( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRegister) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_RegisterAsDropTarget( 
            /* [in] */ VARIANT_BOOL bRegister) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_TheaterMode( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRegister) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_TheaterMode( 
            /* [in] */ VARIANT_BOOL bRegister) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_AddressBar( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Value) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_AddressBar( 
            /* [in] */ VARIANT_BOOL Value) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Resizable( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Value) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Resizable( 
            /* [in] */ VARIANT_BOOL Value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebBrowser2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWebBrowser2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWebBrowser2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWebBrowser2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWebBrowser2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IWebBrowser, GoBack)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GoBack )( 
            __RPC__in IWebBrowser2 * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, GoForward)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GoForward )( 
            __RPC__in IWebBrowser2 * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, GoHome)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GoHome )( 
            __RPC__in IWebBrowser2 * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, GoSearch)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GoSearch )( 
            __RPC__in IWebBrowser2 * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, Navigate)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Navigate )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Flags,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *TargetFrameName,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *PostData,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Headers);
        
        DECLSPEC_XFGVIRT(IWebBrowser, Refresh)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IWebBrowser2 * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, Refresh2)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh2 )( 
            __RPC__in IWebBrowser2 * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Level);
        
        DECLSPEC_XFGVIRT(IWebBrowser, Stop)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IWebBrowser2 * This);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Application)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Application )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Parent)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Container)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Container )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Document)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Document )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppDisp);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_TopLevelContainer)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_TopLevelContainer )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Type)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Type);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Left)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Left )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(IWebBrowser, put_Left)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Left )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ long Left);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Top)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Top )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(IWebBrowser, put_Top)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Top )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ long Top);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Width)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Width )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(IWebBrowser, put_Width)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Width )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ long Width);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Height)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Height )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out long *pl);
        
        DECLSPEC_XFGVIRT(IWebBrowser, put_Height)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Height )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ long Height);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_LocationName)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LocationName )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *LocationName);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_LocationURL)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_LocationURL )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *LocationURL);
        
        DECLSPEC_XFGVIRT(IWebBrowser, get_Busy)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Busy )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, Quit)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Quit )( 
            __RPC__in IWebBrowser2 * This);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, ClientToWindow)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ClientToWindow )( 
            __RPC__in IWebBrowser2 * This,
            /* [out][in] */ __RPC__inout int *pcx,
            /* [out][in] */ __RPC__inout int *pcy);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, PutProperty)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PutProperty )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ __RPC__in BSTR Property,
            /* [in] */ VARIANT vtValue);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, GetProperty)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ __RPC__in BSTR Property,
            /* [retval][out] */ __RPC__out VARIANT *pvtValue);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_Name)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_HWND)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_HWND )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out SHANDLE_PTR *pHWND);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_FullName)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_FullName )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *FullName);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_Path)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Path);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_Visible)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Visible )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, put_Visible)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Visible )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ VARIANT_BOOL Value);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_StatusBar)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StatusBar )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, put_StatusBar)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StatusBar )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ VARIANT_BOOL Value);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_StatusText)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_StatusText )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *StatusText);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, put_StatusText)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_StatusText )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ __RPC__in BSTR StatusText);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_ToolBar)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ToolBar )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out int *Value);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, put_ToolBar)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ToolBar )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ int Value);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_MenuBar)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MenuBar )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Value);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, put_MenuBar)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MenuBar )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ VARIANT_BOOL Value);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, get_FullScreen)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_FullScreen )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbFullScreen);
        
        DECLSPEC_XFGVIRT(IWebBrowserApp, put_FullScreen)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_FullScreen )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ VARIANT_BOOL bFullScreen);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, Navigate2)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Navigate2 )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ __RPC__in VARIANT *URL,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Flags,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *TargetFrameName,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *PostData,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Headers);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, QueryStatusWB)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *QueryStatusWB )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ OLECMDID cmdID,
            /* [retval][out] */ __RPC__out OLECMDF *pcmdf);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, ExecWB)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExecWB )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ OLECMDID cmdID,
            /* [in] */ OLECMDEXECOPT cmdexecopt,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *pvaIn,
            /* [unique][optional][out][in] */ __RPC__inout_opt VARIANT *pvaOut);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, ShowBrowserBar)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ShowBrowserBar )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ __RPC__in VARIANT *pvaClsid,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *pvarShow,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *pvarSize);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, get_ReadyState)
        /* [bindable][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ReadyState )( 
            __RPC__in IWebBrowser2 * This,
            /* [out][retval] */ __RPC__out READYSTATE *plReadyState);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, get_Offline)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Offline )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbOffline);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, put_Offline)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Offline )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ VARIANT_BOOL bOffline);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, get_Silent)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Silent )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbSilent);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, put_Silent)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Silent )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ VARIANT_BOOL bSilent);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, get_RegisterAsBrowser)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RegisterAsBrowser )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRegister);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, put_RegisterAsBrowser)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RegisterAsBrowser )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ VARIANT_BOOL bRegister);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, get_RegisterAsDropTarget)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RegisterAsDropTarget )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRegister);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, put_RegisterAsDropTarget)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RegisterAsDropTarget )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ VARIANT_BOOL bRegister);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, get_TheaterMode)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_TheaterMode )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRegister);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, put_TheaterMode)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_TheaterMode )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ VARIANT_BOOL bRegister);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, get_AddressBar)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AddressBar )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Value);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, put_AddressBar)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AddressBar )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ VARIANT_BOOL Value);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, get_Resizable)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Resizable )( 
            __RPC__in IWebBrowser2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Value);
        
        DECLSPEC_XFGVIRT(IWebBrowser2, put_Resizable)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Resizable )( 
            __RPC__in IWebBrowser2 * This,
            /* [in] */ VARIANT_BOOL Value);
        
        END_INTERFACE
    } IWebBrowser2Vtbl;

    interface IWebBrowser2
    {
        CONST_VTBL struct IWebBrowser2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebBrowser2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebBrowser2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebBrowser2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebBrowser2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWebBrowser2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWebBrowser2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWebBrowser2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWebBrowser2_GoBack(This)	\
    ( (This)->lpVtbl -> GoBack(This) ) 

#define IWebBrowser2_GoForward(This)	\
    ( (This)->lpVtbl -> GoForward(This) ) 

#define IWebBrowser2_GoHome(This)	\
    ( (This)->lpVtbl -> GoHome(This) ) 

#define IWebBrowser2_GoSearch(This)	\
    ( (This)->lpVtbl -> GoSearch(This) ) 

#define IWebBrowser2_Navigate(This,URL,Flags,TargetFrameName,PostData,Headers)	\
    ( (This)->lpVtbl -> Navigate(This,URL,Flags,TargetFrameName,PostData,Headers) ) 

#define IWebBrowser2_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IWebBrowser2_Refresh2(This,Level)	\
    ( (This)->lpVtbl -> Refresh2(This,Level) ) 

#define IWebBrowser2_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IWebBrowser2_get_Application(This,ppDisp)	\
    ( (This)->lpVtbl -> get_Application(This,ppDisp) ) 

#define IWebBrowser2_get_Parent(This,ppDisp)	\
    ( (This)->lpVtbl -> get_Parent(This,ppDisp) ) 

#define IWebBrowser2_get_Container(This,ppDisp)	\
    ( (This)->lpVtbl -> get_Container(This,ppDisp) ) 

#define IWebBrowser2_get_Document(This,ppDisp)	\
    ( (This)->lpVtbl -> get_Document(This,ppDisp) ) 

#define IWebBrowser2_get_TopLevelContainer(This,pBool)	\
    ( (This)->lpVtbl -> get_TopLevelContainer(This,pBool) ) 

#define IWebBrowser2_get_Type(This,Type)	\
    ( (This)->lpVtbl -> get_Type(This,Type) ) 

#define IWebBrowser2_get_Left(This,pl)	\
    ( (This)->lpVtbl -> get_Left(This,pl) ) 

#define IWebBrowser2_put_Left(This,Left)	\
    ( (This)->lpVtbl -> put_Left(This,Left) ) 

#define IWebBrowser2_get_Top(This,pl)	\
    ( (This)->lpVtbl -> get_Top(This,pl) ) 

#define IWebBrowser2_put_Top(This,Top)	\
    ( (This)->lpVtbl -> put_Top(This,Top) ) 

#define IWebBrowser2_get_Width(This,pl)	\
    ( (This)->lpVtbl -> get_Width(This,pl) ) 

#define IWebBrowser2_put_Width(This,Width)	\
    ( (This)->lpVtbl -> put_Width(This,Width) ) 

#define IWebBrowser2_get_Height(This,pl)	\
    ( (This)->lpVtbl -> get_Height(This,pl) ) 

#define IWebBrowser2_put_Height(This,Height)	\
    ( (This)->lpVtbl -> put_Height(This,Height) ) 

#define IWebBrowser2_get_LocationName(This,LocationName)	\
    ( (This)->lpVtbl -> get_LocationName(This,LocationName) ) 

#define IWebBrowser2_get_LocationURL(This,LocationURL)	\
    ( (This)->lpVtbl -> get_LocationURL(This,LocationURL) ) 

#define IWebBrowser2_get_Busy(This,pBool)	\
    ( (This)->lpVtbl -> get_Busy(This,pBool) ) 


#define IWebBrowser2_Quit(This)	\
    ( (This)->lpVtbl -> Quit(This) ) 

#define IWebBrowser2_ClientToWindow(This,pcx,pcy)	\
    ( (This)->lpVtbl -> ClientToWindow(This,pcx,pcy) ) 

#define IWebBrowser2_PutProperty(This,Property,vtValue)	\
    ( (This)->lpVtbl -> PutProperty(This,Property,vtValue) ) 

#define IWebBrowser2_GetProperty(This,Property,pvtValue)	\
    ( (This)->lpVtbl -> GetProperty(This,Property,pvtValue) ) 

#define IWebBrowser2_get_Name(This,Name)	\
    ( (This)->lpVtbl -> get_Name(This,Name) ) 

#define IWebBrowser2_get_HWND(This,pHWND)	\
    ( (This)->lpVtbl -> get_HWND(This,pHWND) ) 

#define IWebBrowser2_get_FullName(This,FullName)	\
    ( (This)->lpVtbl -> get_FullName(This,FullName) ) 

#define IWebBrowser2_get_Path(This,Path)	\
    ( (This)->lpVtbl -> get_Path(This,Path) ) 

#define IWebBrowser2_get_Visible(This,pBool)	\
    ( (This)->lpVtbl -> get_Visible(This,pBool) ) 

#define IWebBrowser2_put_Visible(This,Value)	\
    ( (This)->lpVtbl -> put_Visible(This,Value) ) 

#define IWebBrowser2_get_StatusBar(This,pBool)	\
    ( (This)->lpVtbl -> get_StatusBar(This,pBool) ) 

#define IWebBrowser2_put_StatusBar(This,Value)	\
    ( (This)->lpVtbl -> put_StatusBar(This,Value) ) 

#define IWebBrowser2_get_StatusText(This,StatusText)	\
    ( (This)->lpVtbl -> get_StatusText(This,StatusText) ) 

#define IWebBrowser2_put_StatusText(This,StatusText)	\
    ( (This)->lpVtbl -> put_StatusText(This,StatusText) ) 

#define IWebBrowser2_get_ToolBar(This,Value)	\
    ( (This)->lpVtbl -> get_ToolBar(This,Value) ) 

#define IWebBrowser2_put_ToolBar(This,Value)	\
    ( (This)->lpVtbl -> put_ToolBar(This,Value) ) 

#define IWebBrowser2_get_MenuBar(This,Value)	\
    ( (This)->lpVtbl -> get_MenuBar(This,Value) ) 

#define IWebBrowser2_put_MenuBar(This,Value)	\
    ( (This)->lpVtbl -> put_MenuBar(This,Value) ) 

#define IWebBrowser2_get_FullScreen(This,pbFullScreen)	\
    ( (This)->lpVtbl -> get_FullScreen(This,pbFullScreen) ) 

#define IWebBrowser2_put_FullScreen(This,bFullScreen)	\
    ( (This)->lpVtbl -> put_FullScreen(This,bFullScreen) ) 


#define IWebBrowser2_Navigate2(This,URL,Flags,TargetFrameName,PostData,Headers)	\
    ( (This)->lpVtbl -> Navigate2(This,URL,Flags,TargetFrameName,PostData,Headers) ) 

#define IWebBrowser2_QueryStatusWB(This,cmdID,pcmdf)	\
    ( (This)->lpVtbl -> QueryStatusWB(This,cmdID,pcmdf) ) 

#define IWebBrowser2_ExecWB(This,cmdID,cmdexecopt,pvaIn,pvaOut)	\
    ( (This)->lpVtbl -> ExecWB(This,cmdID,cmdexecopt,pvaIn,pvaOut) ) 

#define IWebBrowser2_ShowBrowserBar(This,pvaClsid,pvarShow,pvarSize)	\
    ( (This)->lpVtbl -> ShowBrowserBar(This,pvaClsid,pvarShow,pvarSize) ) 

#define IWebBrowser2_get_ReadyState(This,plReadyState)	\
    ( (This)->lpVtbl -> get_ReadyState(This,plReadyState) ) 

#define IWebBrowser2_get_Offline(This,pbOffline)	\
    ( (This)->lpVtbl -> get_Offline(This,pbOffline) ) 

#define IWebBrowser2_put_Offline(This,bOffline)	\
    ( (This)->lpVtbl -> put_Offline(This,bOffline) ) 

#define IWebBrowser2_get_Silent(This,pbSilent)	\
    ( (This)->lpVtbl -> get_Silent(This,pbSilent) ) 

#define IWebBrowser2_put_Silent(This,bSilent)	\
    ( (This)->lpVtbl -> put_Silent(This,bSilent) ) 

#define IWebBrowser2_get_RegisterAsBrowser(This,pbRegister)	\
    ( (This)->lpVtbl -> get_RegisterAsBrowser(This,pbRegister) ) 

#define IWebBrowser2_put_RegisterAsBrowser(This,bRegister)	\
    ( (This)->lpVtbl -> put_RegisterAsBrowser(This,bRegister) ) 

#define IWebBrowser2_get_RegisterAsDropTarget(This,pbRegister)	\
    ( (This)->lpVtbl -> get_RegisterAsDropTarget(This,pbRegister) ) 

#define IWebBrowser2_put_RegisterAsDropTarget(This,bRegister)	\
    ( (This)->lpVtbl -> put_RegisterAsDropTarget(This,bRegister) ) 

#define IWebBrowser2_get_TheaterMode(This,pbRegister)	\
    ( (This)->lpVtbl -> get_TheaterMode(This,pbRegister) ) 

#define IWebBrowser2_put_TheaterMode(This,bRegister)	\
    ( (This)->lpVtbl -> put_TheaterMode(This,bRegister) ) 

#define IWebBrowser2_get_AddressBar(This,Value)	\
    ( (This)->lpVtbl -> get_AddressBar(This,Value) ) 

#define IWebBrowser2_put_AddressBar(This,Value)	\
    ( (This)->lpVtbl -> put_AddressBar(This,Value) ) 

#define IWebBrowser2_get_Resizable(This,Value)	\
    ( (This)->lpVtbl -> get_Resizable(This,Value) ) 

#define IWebBrowser2_put_Resizable(This,Value)	\
    ( (This)->lpVtbl -> put_Resizable(This,Value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebBrowser2_INTERFACE_DEFINED__ */


#ifndef __DWebBrowserEvents2_DISPINTERFACE_DEFINED__
#define __DWebBrowserEvents2_DISPINTERFACE_DEFINED__

/* dispinterface DWebBrowserEvents2 */
/* [hidden][helpstring][uuid] */ 


EXTERN_C const IID DIID_DWebBrowserEvents2;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("34A715A0-6587-11D0-924A-0020AFC7AC4D")
    DWebBrowserEvents2 : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct DWebBrowserEvents2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DWebBrowserEvents2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DWebBrowserEvents2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DWebBrowserEvents2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DWebBrowserEvents2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DWebBrowserEvents2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DWebBrowserEvents2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DWebBrowserEvents2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } DWebBrowserEvents2Vtbl;

    interface DWebBrowserEvents2
    {
        CONST_VTBL struct DWebBrowserEvents2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DWebBrowserEvents2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DWebBrowserEvents2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DWebBrowserEvents2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DWebBrowserEvents2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DWebBrowserEvents2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DWebBrowserEvents2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DWebBrowserEvents2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __DWebBrowserEvents2_DISPINTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_WebBrowser_V1;

#ifdef __cplusplus

class DECLSPEC_UUID("EAB22AC3-30C1-11CF-A7EB-0000C05BAE0B")
WebBrowser_V1;
#endif

EXTERN_C const CLSID CLSID_WebBrowser;

#ifdef __cplusplus

class DECLSPEC_UUID("8856F961-340A-11D0-A96B-00C04FD705A2")
WebBrowser;
#endif

EXTERN_C const CLSID CLSID_InternetExplorer;

#ifdef __cplusplus

class DECLSPEC_UUID("0002DF01-0000-0000-C000-000000000046")
InternetExplorer;
#endif

EXTERN_C const CLSID CLSID_InternetExplorerMedium;

#ifdef __cplusplus

class DECLSPEC_UUID("D5E8041D-920F-45e9-B8FB-B1DEB82C6E5E")
InternetExplorerMedium;
#endif

EXTERN_C const CLSID CLSID_ShellBrowserWindow;

#ifdef __cplusplus

class DECLSPEC_UUID("c08afd90-f2a1-11d1-8455-00a0c91f3880")
ShellBrowserWindow;
#endif

#ifndef __DShellWindowsEvents_DISPINTERFACE_DEFINED__
#define __DShellWindowsEvents_DISPINTERFACE_DEFINED__

/* dispinterface DShellWindowsEvents */
/* [helpstring][uuid] */ 


EXTERN_C const IID DIID_DShellWindowsEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("fe4106e0-399a-11d0-a48c-00a0c90a8f39")
    DShellWindowsEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct DShellWindowsEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DShellWindowsEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DShellWindowsEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DShellWindowsEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DShellWindowsEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DShellWindowsEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DShellWindowsEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DShellWindowsEvents * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } DShellWindowsEventsVtbl;

    interface DShellWindowsEvents
    {
        CONST_VTBL struct DShellWindowsEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DShellWindowsEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DShellWindowsEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DShellWindowsEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DShellWindowsEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DShellWindowsEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DShellWindowsEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DShellWindowsEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __DShellWindowsEvents_DISPINTERFACE_DEFINED__ */


#ifndef __IShellWindows_INTERFACE_DEFINED__
#define __IShellWindows_INTERFACE_DEFINED__

/* interface IShellWindows */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellWindows;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("85CB6900-4D95-11CF-960C-0080C7F4EE85")
    IShellWindows : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *Count) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [optional][in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **Folder) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE _NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
        virtual /* [hidden][helpstring] */ HRESULT STDMETHODCALLTYPE Register( 
            /* [in] */ __RPC__in_opt IDispatch *pid,
            /* [in] */ long hwnd,
            /* [in] */ int swClass,
            /* [out] */ __RPC__out long *plCookie) = 0;
        
        virtual /* [hidden][helpstring] */ HRESULT STDMETHODCALLTYPE RegisterPending( 
            /* [in] */ long lThreadId,
            /* [in] */ __RPC__in VARIANT *pvarloc,
            /* [in] */ __RPC__in VARIANT *pvarlocRoot,
            /* [in] */ int swClass,
            /* [out] */ __RPC__out long *plCookie) = 0;
        
        virtual /* [hidden][helpstring] */ HRESULT STDMETHODCALLTYPE Revoke( 
            /* [in] */ long lCookie) = 0;
        
        virtual /* [hidden][helpstring] */ HRESULT STDMETHODCALLTYPE OnNavigate( 
            /* [in] */ long lCookie,
            /* [in] */ __RPC__in VARIANT *pvarLoc) = 0;
        
        virtual /* [hidden][helpstring] */ HRESULT STDMETHODCALLTYPE OnActivated( 
            /* [in] */ long lCookie,
            /* [in] */ VARIANT_BOOL fActive) = 0;
        
        virtual /* [hidden][helpstring] */ HRESULT STDMETHODCALLTYPE FindWindowSW( 
            /* [in] */ __RPC__in VARIANT *pvarLoc,
            /* [in] */ __RPC__in VARIANT *pvarLocRoot,
            /* [in] */ int swClass,
            /* [out] */ __RPC__out long *phwnd,
            /* [in] */ int swfwOptions,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdispOut) = 0;
        
        virtual /* [hidden][helpstring] */ HRESULT STDMETHODCALLTYPE OnCreated( 
            /* [in] */ long lCookie,
            /* [in] */ __RPC__in_opt IUnknown *punk) = 0;
        
        virtual /* [hidden][helpstring] */ HRESULT STDMETHODCALLTYPE ProcessAttachDetach( 
            /* [in] */ VARIANT_BOOL fAttach) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellWindowsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellWindows * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellWindows * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellWindows * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellWindows * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellWindows * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellWindows * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellWindows * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IShellWindows, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IShellWindows * This,
            /* [retval][out] */ __RPC__out long *Count);
        
        DECLSPEC_XFGVIRT(IShellWindows, Item)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IShellWindows * This,
            /* [optional][in] */ VARIANT index,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **Folder);
        
        DECLSPEC_XFGVIRT(IShellWindows, _NewEnum)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in IShellWindows * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(IShellWindows, Register)
        /* [hidden][helpstring] */ HRESULT ( STDMETHODCALLTYPE *Register )( 
            __RPC__in IShellWindows * This,
            /* [in] */ __RPC__in_opt IDispatch *pid,
            /* [in] */ long hwnd,
            /* [in] */ int swClass,
            /* [out] */ __RPC__out long *plCookie);
        
        DECLSPEC_XFGVIRT(IShellWindows, RegisterPending)
        /* [hidden][helpstring] */ HRESULT ( STDMETHODCALLTYPE *RegisterPending )( 
            __RPC__in IShellWindows * This,
            /* [in] */ long lThreadId,
            /* [in] */ __RPC__in VARIANT *pvarloc,
            /* [in] */ __RPC__in VARIANT *pvarlocRoot,
            /* [in] */ int swClass,
            /* [out] */ __RPC__out long *plCookie);
        
        DECLSPEC_XFGVIRT(IShellWindows, Revoke)
        /* [hidden][helpstring] */ HRESULT ( STDMETHODCALLTYPE *Revoke )( 
            __RPC__in IShellWindows * This,
            /* [in] */ long lCookie);
        
        DECLSPEC_XFGVIRT(IShellWindows, OnNavigate)
        /* [hidden][helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnNavigate )( 
            __RPC__in IShellWindows * This,
            /* [in] */ long lCookie,
            /* [in] */ __RPC__in VARIANT *pvarLoc);
        
        DECLSPEC_XFGVIRT(IShellWindows, OnActivated)
        /* [hidden][helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnActivated )( 
            __RPC__in IShellWindows * This,
            /* [in] */ long lCookie,
            /* [in] */ VARIANT_BOOL fActive);
        
        DECLSPEC_XFGVIRT(IShellWindows, FindWindowSW)
        /* [hidden][helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindWindowSW )( 
            __RPC__in IShellWindows * This,
            /* [in] */ __RPC__in VARIANT *pvarLoc,
            /* [in] */ __RPC__in VARIANT *pvarLocRoot,
            /* [in] */ int swClass,
            /* [out] */ __RPC__out long *phwnd,
            /* [in] */ int swfwOptions,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdispOut);
        
        DECLSPEC_XFGVIRT(IShellWindows, OnCreated)
        /* [hidden][helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnCreated )( 
            __RPC__in IShellWindows * This,
            /* [in] */ long lCookie,
            /* [in] */ __RPC__in_opt IUnknown *punk);
        
        DECLSPEC_XFGVIRT(IShellWindows, ProcessAttachDetach)
        /* [hidden][helpstring] */ HRESULT ( STDMETHODCALLTYPE *ProcessAttachDetach )( 
            __RPC__in IShellWindows * This,
            /* [in] */ VARIANT_BOOL fAttach);
        
        END_INTERFACE
    } IShellWindowsVtbl;

    interface IShellWindows
    {
        CONST_VTBL struct IShellWindowsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellWindows_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellWindows_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellWindows_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellWindows_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellWindows_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellWindows_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellWindows_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellWindows_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define IShellWindows_Item(This,index,Folder)	\
    ( (This)->lpVtbl -> Item(This,index,Folder) ) 

#define IShellWindows__NewEnum(This,ppunk)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppunk) ) 

#define IShellWindows_Register(This,pid,hwnd,swClass,plCookie)	\
    ( (This)->lpVtbl -> Register(This,pid,hwnd,swClass,plCookie) ) 

#define IShellWindows_RegisterPending(This,lThreadId,pvarloc,pvarlocRoot,swClass,plCookie)	\
    ( (This)->lpVtbl -> RegisterPending(This,lThreadId,pvarloc,pvarlocRoot,swClass,plCookie) ) 

#define IShellWindows_Revoke(This,lCookie)	\
    ( (This)->lpVtbl -> Revoke(This,lCookie) ) 

#define IShellWindows_OnNavigate(This,lCookie,pvarLoc)	\
    ( (This)->lpVtbl -> OnNavigate(This,lCookie,pvarLoc) ) 

#define IShellWindows_OnActivated(This,lCookie,fActive)	\
    ( (This)->lpVtbl -> OnActivated(This,lCookie,fActive) ) 

#define IShellWindows_FindWindowSW(This,pvarLoc,pvarLocRoot,swClass,phwnd,swfwOptions,ppdispOut)	\
    ( (This)->lpVtbl -> FindWindowSW(This,pvarLoc,pvarLocRoot,swClass,phwnd,swfwOptions,ppdispOut) ) 

#define IShellWindows_OnCreated(This,lCookie,punk)	\
    ( (This)->lpVtbl -> OnCreated(This,lCookie,punk) ) 

#define IShellWindows_ProcessAttachDetach(This,fAttach)	\
    ( (This)->lpVtbl -> ProcessAttachDetach(This,fAttach) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellWindows_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_ShellWindows;

#ifdef __cplusplus

class DECLSPEC_UUID("9BA05972-F6A8-11CF-A442-00A0C90A8F39")
ShellWindows;
#endif

#ifndef __IShellUIHelper_INTERFACE_DEFINED__
#define __IShellUIHelper_INTERFACE_DEFINED__

/* interface IShellUIHelper */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellUIHelper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("729FE2F8-1EA8-11d1-8F85-00C04FC2FBE1")
    IShellUIHelper : public IDispatch
    {
    public:
        virtual /* [id][hidden] */ HRESULT STDMETHODCALLTYPE ResetFirstBootMode( void) = 0;
        
        virtual /* [id][hidden] */ HRESULT STDMETHODCALLTYPE ResetSafeMode( void) = 0;
        
        virtual /* [id][hidden] */ HRESULT STDMETHODCALLTYPE RefreshOfflineDesktop( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AddFavorite( 
            /* [in] */ __RPC__in BSTR URL,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Title) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AddChannel( 
            /* [in] */ __RPC__in BSTR URL) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AddDesktopComponent( 
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Type,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Left,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Top,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Width,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Height) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE IsSubscribed( 
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE NavigateAndFind( 
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR strQuery,
            /* [in] */ __RPC__in VARIANT *varTargetFrame) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ImportExportFavorites( 
            /* [in] */ VARIANT_BOOL fImport,
            /* [in] */ __RPC__in BSTR strImpExpPath) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AutoCompleteSaveForm( 
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Form) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AutoScan( 
            /* [in] */ __RPC__in BSTR strSearch,
            /* [in] */ __RPC__in BSTR strFailureUrl,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *pvarTargetFrame) = 0;
        
        virtual /* [id][hidden] */ HRESULT STDMETHODCALLTYPE AutoCompleteAttach( 
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Reserved) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ShowBrowserUI( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in VARIANT *pvarIn,
            /* [retval][out] */ __RPC__out VARIANT *pvarOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellUIHelperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellUIHelper * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellUIHelper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellUIHelper * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellUIHelper * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellUIHelper * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellUIHelper * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellUIHelper * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetFirstBootMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetFirstBootMode )( 
            __RPC__in IShellUIHelper * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetSafeMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetSafeMode )( 
            __RPC__in IShellUIHelper * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, RefreshOfflineDesktop)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *RefreshOfflineDesktop )( 
            __RPC__in IShellUIHelper * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddFavorite)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddFavorite )( 
            __RPC__in IShellUIHelper * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Title);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddChannel)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddChannel )( 
            __RPC__in IShellUIHelper * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddDesktopComponent)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddDesktopComponent )( 
            __RPC__in IShellUIHelper * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Type,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Left,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Top,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Width,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Height);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, IsSubscribed)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSubscribed )( 
            __RPC__in IShellUIHelper * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, NavigateAndFind)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NavigateAndFind )( 
            __RPC__in IShellUIHelper * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR strQuery,
            /* [in] */ __RPC__in VARIANT *varTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ImportExportFavorites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ImportExportFavorites )( 
            __RPC__in IShellUIHelper * This,
            /* [in] */ VARIANT_BOOL fImport,
            /* [in] */ __RPC__in BSTR strImpExpPath);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteSaveForm)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteSaveForm )( 
            __RPC__in IShellUIHelper * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Form);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoScan)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoScan )( 
            __RPC__in IShellUIHelper * This,
            /* [in] */ __RPC__in BSTR strSearch,
            /* [in] */ __RPC__in BSTR strFailureUrl,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *pvarTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteAttach)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteAttach )( 
            __RPC__in IShellUIHelper * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Reserved);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ShowBrowserUI)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowBrowserUI )( 
            __RPC__in IShellUIHelper * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in VARIANT *pvarIn,
            /* [retval][out] */ __RPC__out VARIANT *pvarOut);
        
        END_INTERFACE
    } IShellUIHelperVtbl;

    interface IShellUIHelper
    {
        CONST_VTBL struct IShellUIHelperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellUIHelper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellUIHelper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellUIHelper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellUIHelper_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellUIHelper_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellUIHelper_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellUIHelper_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellUIHelper_ResetFirstBootMode(This)	\
    ( (This)->lpVtbl -> ResetFirstBootMode(This) ) 

#define IShellUIHelper_ResetSafeMode(This)	\
    ( (This)->lpVtbl -> ResetSafeMode(This) ) 

#define IShellUIHelper_RefreshOfflineDesktop(This)	\
    ( (This)->lpVtbl -> RefreshOfflineDesktop(This) ) 

#define IShellUIHelper_AddFavorite(This,URL,Title)	\
    ( (This)->lpVtbl -> AddFavorite(This,URL,Title) ) 

#define IShellUIHelper_AddChannel(This,URL)	\
    ( (This)->lpVtbl -> AddChannel(This,URL) ) 

#define IShellUIHelper_AddDesktopComponent(This,URL,Type,Left,Top,Width,Height)	\
    ( (This)->lpVtbl -> AddDesktopComponent(This,URL,Type,Left,Top,Width,Height) ) 

#define IShellUIHelper_IsSubscribed(This,URL,pBool)	\
    ( (This)->lpVtbl -> IsSubscribed(This,URL,pBool) ) 

#define IShellUIHelper_NavigateAndFind(This,URL,strQuery,varTargetFrame)	\
    ( (This)->lpVtbl -> NavigateAndFind(This,URL,strQuery,varTargetFrame) ) 

#define IShellUIHelper_ImportExportFavorites(This,fImport,strImpExpPath)	\
    ( (This)->lpVtbl -> ImportExportFavorites(This,fImport,strImpExpPath) ) 

#define IShellUIHelper_AutoCompleteSaveForm(This,Form)	\
    ( (This)->lpVtbl -> AutoCompleteSaveForm(This,Form) ) 

#define IShellUIHelper_AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame)	\
    ( (This)->lpVtbl -> AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame) ) 

#define IShellUIHelper_AutoCompleteAttach(This,Reserved)	\
    ( (This)->lpVtbl -> AutoCompleteAttach(This,Reserved) ) 

#define IShellUIHelper_ShowBrowserUI(This,bstrName,pvarIn,pvarOut)	\
    ( (This)->lpVtbl -> ShowBrowserUI(This,bstrName,pvarIn,pvarOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellUIHelper_INTERFACE_DEFINED__ */


#ifndef __IShellUIHelper2_INTERFACE_DEFINED__
#define __IShellUIHelper2_INTERFACE_DEFINED__

/* interface IShellUIHelper2 */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellUIHelper2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a7fe6eda-1932-4281-b881-87b31b8bc52c")
    IShellUIHelper2 : public IShellUIHelper
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AddSearchProvider( 
            /* [in] */ __RPC__in BSTR URL) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE RunOnceShown( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SkipRunOnce( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CustomizeSettings( 
            /* [in] */ VARIANT_BOOL fSQM,
            /* [in] */ VARIANT_BOOL fPhishing,
            /* [in] */ __RPC__in BSTR bstrLocale) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SqmEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE PhishingEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE BrandImageUri( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUri) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SkipTabsWelcome( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DiagnoseConnection( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CustomizeClearType( 
            /* [in] */ VARIANT_BOOL fSet) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE IsSearchProviderInstalled( 
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out DWORD *pdwResult) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE IsSearchMigrated( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfMigrated) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DefaultSearchProvider( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE RunOnceRequiredSettingsComplete( 
            /* [in] */ VARIANT_BOOL fComplete) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE RunOnceHasShown( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfShown) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SearchGuideUrl( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUrl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellUIHelper2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellUIHelper2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellUIHelper2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellUIHelper2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellUIHelper2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellUIHelper2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellUIHelper2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellUIHelper2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetFirstBootMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetFirstBootMode )( 
            __RPC__in IShellUIHelper2 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetSafeMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetSafeMode )( 
            __RPC__in IShellUIHelper2 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, RefreshOfflineDesktop)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *RefreshOfflineDesktop )( 
            __RPC__in IShellUIHelper2 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddFavorite)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddFavorite )( 
            __RPC__in IShellUIHelper2 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Title);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddChannel)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddChannel )( 
            __RPC__in IShellUIHelper2 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddDesktopComponent)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddDesktopComponent )( 
            __RPC__in IShellUIHelper2 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Type,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Left,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Top,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Width,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Height);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, IsSubscribed)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSubscribed )( 
            __RPC__in IShellUIHelper2 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, NavigateAndFind)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NavigateAndFind )( 
            __RPC__in IShellUIHelper2 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR strQuery,
            /* [in] */ __RPC__in VARIANT *varTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ImportExportFavorites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ImportExportFavorites )( 
            __RPC__in IShellUIHelper2 * This,
            /* [in] */ VARIANT_BOOL fImport,
            /* [in] */ __RPC__in BSTR strImpExpPath);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteSaveForm)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteSaveForm )( 
            __RPC__in IShellUIHelper2 * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Form);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoScan)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoScan )( 
            __RPC__in IShellUIHelper2 * This,
            /* [in] */ __RPC__in BSTR strSearch,
            /* [in] */ __RPC__in BSTR strFailureUrl,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *pvarTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteAttach)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteAttach )( 
            __RPC__in IShellUIHelper2 * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Reserved);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ShowBrowserUI)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowBrowserUI )( 
            __RPC__in IShellUIHelper2 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in VARIANT *pvarIn,
            /* [retval][out] */ __RPC__out VARIANT *pvarOut);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, AddSearchProvider)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddSearchProvider )( 
            __RPC__in IShellUIHelper2 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceShown)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceShown )( 
            __RPC__in IShellUIHelper2 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SkipRunOnce)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SkipRunOnce )( 
            __RPC__in IShellUIHelper2 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, CustomizeSettings)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CustomizeSettings )( 
            __RPC__in IShellUIHelper2 * This,
            /* [in] */ VARIANT_BOOL fSQM,
            /* [in] */ VARIANT_BOOL fPhishing,
            /* [in] */ __RPC__in BSTR bstrLocale);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SqmEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SqmEnabled )( 
            __RPC__in IShellUIHelper2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, PhishingEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PhishingEnabled )( 
            __RPC__in IShellUIHelper2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, BrandImageUri)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BrandImageUri )( 
            __RPC__in IShellUIHelper2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUri);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SkipTabsWelcome)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SkipTabsWelcome )( 
            __RPC__in IShellUIHelper2 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, DiagnoseConnection)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DiagnoseConnection )( 
            __RPC__in IShellUIHelper2 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, CustomizeClearType)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CustomizeClearType )( 
            __RPC__in IShellUIHelper2 * This,
            /* [in] */ VARIANT_BOOL fSet);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, IsSearchProviderInstalled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSearchProviderInstalled )( 
            __RPC__in IShellUIHelper2 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, IsSearchMigrated)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSearchMigrated )( 
            __RPC__in IShellUIHelper2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfMigrated);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, DefaultSearchProvider)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DefaultSearchProvider )( 
            __RPC__in IShellUIHelper2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceRequiredSettingsComplete)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceRequiredSettingsComplete )( 
            __RPC__in IShellUIHelper2 * This,
            /* [in] */ VARIANT_BOOL fComplete);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceHasShown)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceHasShown )( 
            __RPC__in IShellUIHelper2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfShown);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SearchGuideUrl)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SearchGuideUrl )( 
            __RPC__in IShellUIHelper2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUrl);
        
        END_INTERFACE
    } IShellUIHelper2Vtbl;

    interface IShellUIHelper2
    {
        CONST_VTBL struct IShellUIHelper2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellUIHelper2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellUIHelper2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellUIHelper2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellUIHelper2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellUIHelper2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellUIHelper2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellUIHelper2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellUIHelper2_ResetFirstBootMode(This)	\
    ( (This)->lpVtbl -> ResetFirstBootMode(This) ) 

#define IShellUIHelper2_ResetSafeMode(This)	\
    ( (This)->lpVtbl -> ResetSafeMode(This) ) 

#define IShellUIHelper2_RefreshOfflineDesktop(This)	\
    ( (This)->lpVtbl -> RefreshOfflineDesktop(This) ) 

#define IShellUIHelper2_AddFavorite(This,URL,Title)	\
    ( (This)->lpVtbl -> AddFavorite(This,URL,Title) ) 

#define IShellUIHelper2_AddChannel(This,URL)	\
    ( (This)->lpVtbl -> AddChannel(This,URL) ) 

#define IShellUIHelper2_AddDesktopComponent(This,URL,Type,Left,Top,Width,Height)	\
    ( (This)->lpVtbl -> AddDesktopComponent(This,URL,Type,Left,Top,Width,Height) ) 

#define IShellUIHelper2_IsSubscribed(This,URL,pBool)	\
    ( (This)->lpVtbl -> IsSubscribed(This,URL,pBool) ) 

#define IShellUIHelper2_NavigateAndFind(This,URL,strQuery,varTargetFrame)	\
    ( (This)->lpVtbl -> NavigateAndFind(This,URL,strQuery,varTargetFrame) ) 

#define IShellUIHelper2_ImportExportFavorites(This,fImport,strImpExpPath)	\
    ( (This)->lpVtbl -> ImportExportFavorites(This,fImport,strImpExpPath) ) 

#define IShellUIHelper2_AutoCompleteSaveForm(This,Form)	\
    ( (This)->lpVtbl -> AutoCompleteSaveForm(This,Form) ) 

#define IShellUIHelper2_AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame)	\
    ( (This)->lpVtbl -> AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame) ) 

#define IShellUIHelper2_AutoCompleteAttach(This,Reserved)	\
    ( (This)->lpVtbl -> AutoCompleteAttach(This,Reserved) ) 

#define IShellUIHelper2_ShowBrowserUI(This,bstrName,pvarIn,pvarOut)	\
    ( (This)->lpVtbl -> ShowBrowserUI(This,bstrName,pvarIn,pvarOut) ) 


#define IShellUIHelper2_AddSearchProvider(This,URL)	\
    ( (This)->lpVtbl -> AddSearchProvider(This,URL) ) 

#define IShellUIHelper2_RunOnceShown(This)	\
    ( (This)->lpVtbl -> RunOnceShown(This) ) 

#define IShellUIHelper2_SkipRunOnce(This)	\
    ( (This)->lpVtbl -> SkipRunOnce(This) ) 

#define IShellUIHelper2_CustomizeSettings(This,fSQM,fPhishing,bstrLocale)	\
    ( (This)->lpVtbl -> CustomizeSettings(This,fSQM,fPhishing,bstrLocale) ) 

#define IShellUIHelper2_SqmEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> SqmEnabled(This,pfEnabled) ) 

#define IShellUIHelper2_PhishingEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> PhishingEnabled(This,pfEnabled) ) 

#define IShellUIHelper2_BrandImageUri(This,pbstrUri)	\
    ( (This)->lpVtbl -> BrandImageUri(This,pbstrUri) ) 

#define IShellUIHelper2_SkipTabsWelcome(This)	\
    ( (This)->lpVtbl -> SkipTabsWelcome(This) ) 

#define IShellUIHelper2_DiagnoseConnection(This)	\
    ( (This)->lpVtbl -> DiagnoseConnection(This) ) 

#define IShellUIHelper2_CustomizeClearType(This,fSet)	\
    ( (This)->lpVtbl -> CustomizeClearType(This,fSet) ) 

#define IShellUIHelper2_IsSearchProviderInstalled(This,URL,pdwResult)	\
    ( (This)->lpVtbl -> IsSearchProviderInstalled(This,URL,pdwResult) ) 

#define IShellUIHelper2_IsSearchMigrated(This,pfMigrated)	\
    ( (This)->lpVtbl -> IsSearchMigrated(This,pfMigrated) ) 

#define IShellUIHelper2_DefaultSearchProvider(This,pbstrName)	\
    ( (This)->lpVtbl -> DefaultSearchProvider(This,pbstrName) ) 

#define IShellUIHelper2_RunOnceRequiredSettingsComplete(This,fComplete)	\
    ( (This)->lpVtbl -> RunOnceRequiredSettingsComplete(This,fComplete) ) 

#define IShellUIHelper2_RunOnceHasShown(This,pfShown)	\
    ( (This)->lpVtbl -> RunOnceHasShown(This,pfShown) ) 

#define IShellUIHelper2_SearchGuideUrl(This,pbstrUrl)	\
    ( (This)->lpVtbl -> SearchGuideUrl(This,pbstrUrl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellUIHelper2_INTERFACE_DEFINED__ */


#ifndef __IShellUIHelper3_INTERFACE_DEFINED__
#define __IShellUIHelper3_INTERFACE_DEFINED__

/* interface IShellUIHelper3 */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellUIHelper3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("528DF2EC-D419-40bc-9B6D-DCDBF9C1B25D")
    IShellUIHelper3 : public IShellUIHelper2
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AddService( 
            /* [in] */ __RPC__in BSTR URL) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE IsServiceInstalled( 
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Verb,
            /* [retval][out] */ __RPC__out DWORD *pdwResult) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE InPrivateFilteringEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AddToFavoritesBar( 
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Title,
            /* [in][optional] */ __RPC__in VARIANT *Type) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE BuildNewTabPage( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetRecentlyClosedVisible( 
            /* [in] */ VARIANT_BOOL fVisible) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetActivitiesVisible( 
            /* [in] */ VARIANT_BOOL fVisible) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ContentDiscoveryReset( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE IsSuggestedSitesEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EnableSuggestedSites( 
            /* [in] */ VARIANT_BOOL fEnable) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE NavigateToSuggestedSites( 
            /* [in] */ __RPC__in BSTR bstrRelativeUrl) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ShowTabsHelp( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ShowInPrivateHelp( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellUIHelper3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellUIHelper3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellUIHelper3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellUIHelper3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellUIHelper3 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetFirstBootMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetFirstBootMode )( 
            __RPC__in IShellUIHelper3 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetSafeMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetSafeMode )( 
            __RPC__in IShellUIHelper3 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, RefreshOfflineDesktop)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *RefreshOfflineDesktop )( 
            __RPC__in IShellUIHelper3 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddFavorite)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddFavorite )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Title);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddChannel)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddChannel )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddDesktopComponent)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddDesktopComponent )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Type,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Left,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Top,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Width,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Height);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, IsSubscribed)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSubscribed )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, NavigateAndFind)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NavigateAndFind )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR strQuery,
            /* [in] */ __RPC__in VARIANT *varTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ImportExportFavorites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ImportExportFavorites )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ VARIANT_BOOL fImport,
            /* [in] */ __RPC__in BSTR strImpExpPath);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteSaveForm)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteSaveForm )( 
            __RPC__in IShellUIHelper3 * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Form);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoScan)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoScan )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ __RPC__in BSTR strSearch,
            /* [in] */ __RPC__in BSTR strFailureUrl,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *pvarTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteAttach)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteAttach )( 
            __RPC__in IShellUIHelper3 * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Reserved);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ShowBrowserUI)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowBrowserUI )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in VARIANT *pvarIn,
            /* [retval][out] */ __RPC__out VARIANT *pvarOut);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, AddSearchProvider)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddSearchProvider )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceShown)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceShown )( 
            __RPC__in IShellUIHelper3 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SkipRunOnce)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SkipRunOnce )( 
            __RPC__in IShellUIHelper3 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, CustomizeSettings)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CustomizeSettings )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ VARIANT_BOOL fSQM,
            /* [in] */ VARIANT_BOOL fPhishing,
            /* [in] */ __RPC__in BSTR bstrLocale);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SqmEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SqmEnabled )( 
            __RPC__in IShellUIHelper3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, PhishingEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PhishingEnabled )( 
            __RPC__in IShellUIHelper3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, BrandImageUri)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BrandImageUri )( 
            __RPC__in IShellUIHelper3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUri);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SkipTabsWelcome)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SkipTabsWelcome )( 
            __RPC__in IShellUIHelper3 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, DiagnoseConnection)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DiagnoseConnection )( 
            __RPC__in IShellUIHelper3 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, CustomizeClearType)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CustomizeClearType )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ VARIANT_BOOL fSet);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, IsSearchProviderInstalled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSearchProviderInstalled )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, IsSearchMigrated)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSearchMigrated )( 
            __RPC__in IShellUIHelper3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfMigrated);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, DefaultSearchProvider)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DefaultSearchProvider )( 
            __RPC__in IShellUIHelper3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceRequiredSettingsComplete)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceRequiredSettingsComplete )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ VARIANT_BOOL fComplete);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceHasShown)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceHasShown )( 
            __RPC__in IShellUIHelper3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfShown);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SearchGuideUrl)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SearchGuideUrl )( 
            __RPC__in IShellUIHelper3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUrl);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, AddService)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddService )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, IsServiceInstalled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsServiceInstalled )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Verb,
            /* [retval][out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, InPrivateFilteringEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InPrivateFilteringEnabled )( 
            __RPC__in IShellUIHelper3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, AddToFavoritesBar)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddToFavoritesBar )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Title,
            /* [in][optional] */ __RPC__in VARIANT *Type);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, BuildNewTabPage)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BuildNewTabPage )( 
            __RPC__in IShellUIHelper3 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, SetRecentlyClosedVisible)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetRecentlyClosedVisible )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, SetActivitiesVisible)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetActivitiesVisible )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ContentDiscoveryReset)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ContentDiscoveryReset )( 
            __RPC__in IShellUIHelper3 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, IsSuggestedSitesEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSuggestedSitesEnabled )( 
            __RPC__in IShellUIHelper3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, EnableSuggestedSites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnableSuggestedSites )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ VARIANT_BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, NavigateToSuggestedSites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NavigateToSuggestedSites )( 
            __RPC__in IShellUIHelper3 * This,
            /* [in] */ __RPC__in BSTR bstrRelativeUrl);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ShowTabsHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowTabsHelp )( 
            __RPC__in IShellUIHelper3 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ShowInPrivateHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowInPrivateHelp )( 
            __RPC__in IShellUIHelper3 * This);
        
        END_INTERFACE
    } IShellUIHelper3Vtbl;

    interface IShellUIHelper3
    {
        CONST_VTBL struct IShellUIHelper3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellUIHelper3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellUIHelper3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellUIHelper3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellUIHelper3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellUIHelper3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellUIHelper3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellUIHelper3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellUIHelper3_ResetFirstBootMode(This)	\
    ( (This)->lpVtbl -> ResetFirstBootMode(This) ) 

#define IShellUIHelper3_ResetSafeMode(This)	\
    ( (This)->lpVtbl -> ResetSafeMode(This) ) 

#define IShellUIHelper3_RefreshOfflineDesktop(This)	\
    ( (This)->lpVtbl -> RefreshOfflineDesktop(This) ) 

#define IShellUIHelper3_AddFavorite(This,URL,Title)	\
    ( (This)->lpVtbl -> AddFavorite(This,URL,Title) ) 

#define IShellUIHelper3_AddChannel(This,URL)	\
    ( (This)->lpVtbl -> AddChannel(This,URL) ) 

#define IShellUIHelper3_AddDesktopComponent(This,URL,Type,Left,Top,Width,Height)	\
    ( (This)->lpVtbl -> AddDesktopComponent(This,URL,Type,Left,Top,Width,Height) ) 

#define IShellUIHelper3_IsSubscribed(This,URL,pBool)	\
    ( (This)->lpVtbl -> IsSubscribed(This,URL,pBool) ) 

#define IShellUIHelper3_NavigateAndFind(This,URL,strQuery,varTargetFrame)	\
    ( (This)->lpVtbl -> NavigateAndFind(This,URL,strQuery,varTargetFrame) ) 

#define IShellUIHelper3_ImportExportFavorites(This,fImport,strImpExpPath)	\
    ( (This)->lpVtbl -> ImportExportFavorites(This,fImport,strImpExpPath) ) 

#define IShellUIHelper3_AutoCompleteSaveForm(This,Form)	\
    ( (This)->lpVtbl -> AutoCompleteSaveForm(This,Form) ) 

#define IShellUIHelper3_AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame)	\
    ( (This)->lpVtbl -> AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame) ) 

#define IShellUIHelper3_AutoCompleteAttach(This,Reserved)	\
    ( (This)->lpVtbl -> AutoCompleteAttach(This,Reserved) ) 

#define IShellUIHelper3_ShowBrowserUI(This,bstrName,pvarIn,pvarOut)	\
    ( (This)->lpVtbl -> ShowBrowserUI(This,bstrName,pvarIn,pvarOut) ) 


#define IShellUIHelper3_AddSearchProvider(This,URL)	\
    ( (This)->lpVtbl -> AddSearchProvider(This,URL) ) 

#define IShellUIHelper3_RunOnceShown(This)	\
    ( (This)->lpVtbl -> RunOnceShown(This) ) 

#define IShellUIHelper3_SkipRunOnce(This)	\
    ( (This)->lpVtbl -> SkipRunOnce(This) ) 

#define IShellUIHelper3_CustomizeSettings(This,fSQM,fPhishing,bstrLocale)	\
    ( (This)->lpVtbl -> CustomizeSettings(This,fSQM,fPhishing,bstrLocale) ) 

#define IShellUIHelper3_SqmEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> SqmEnabled(This,pfEnabled) ) 

#define IShellUIHelper3_PhishingEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> PhishingEnabled(This,pfEnabled) ) 

#define IShellUIHelper3_BrandImageUri(This,pbstrUri)	\
    ( (This)->lpVtbl -> BrandImageUri(This,pbstrUri) ) 

#define IShellUIHelper3_SkipTabsWelcome(This)	\
    ( (This)->lpVtbl -> SkipTabsWelcome(This) ) 

#define IShellUIHelper3_DiagnoseConnection(This)	\
    ( (This)->lpVtbl -> DiagnoseConnection(This) ) 

#define IShellUIHelper3_CustomizeClearType(This,fSet)	\
    ( (This)->lpVtbl -> CustomizeClearType(This,fSet) ) 

#define IShellUIHelper3_IsSearchProviderInstalled(This,URL,pdwResult)	\
    ( (This)->lpVtbl -> IsSearchProviderInstalled(This,URL,pdwResult) ) 

#define IShellUIHelper3_IsSearchMigrated(This,pfMigrated)	\
    ( (This)->lpVtbl -> IsSearchMigrated(This,pfMigrated) ) 

#define IShellUIHelper3_DefaultSearchProvider(This,pbstrName)	\
    ( (This)->lpVtbl -> DefaultSearchProvider(This,pbstrName) ) 

#define IShellUIHelper3_RunOnceRequiredSettingsComplete(This,fComplete)	\
    ( (This)->lpVtbl -> RunOnceRequiredSettingsComplete(This,fComplete) ) 

#define IShellUIHelper3_RunOnceHasShown(This,pfShown)	\
    ( (This)->lpVtbl -> RunOnceHasShown(This,pfShown) ) 

#define IShellUIHelper3_SearchGuideUrl(This,pbstrUrl)	\
    ( (This)->lpVtbl -> SearchGuideUrl(This,pbstrUrl) ) 


#define IShellUIHelper3_AddService(This,URL)	\
    ( (This)->lpVtbl -> AddService(This,URL) ) 

#define IShellUIHelper3_IsServiceInstalled(This,URL,Verb,pdwResult)	\
    ( (This)->lpVtbl -> IsServiceInstalled(This,URL,Verb,pdwResult) ) 

#define IShellUIHelper3_InPrivateFilteringEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> InPrivateFilteringEnabled(This,pfEnabled) ) 

#define IShellUIHelper3_AddToFavoritesBar(This,URL,Title,Type)	\
    ( (This)->lpVtbl -> AddToFavoritesBar(This,URL,Title,Type) ) 

#define IShellUIHelper3_BuildNewTabPage(This)	\
    ( (This)->lpVtbl -> BuildNewTabPage(This) ) 

#define IShellUIHelper3_SetRecentlyClosedVisible(This,fVisible)	\
    ( (This)->lpVtbl -> SetRecentlyClosedVisible(This,fVisible) ) 

#define IShellUIHelper3_SetActivitiesVisible(This,fVisible)	\
    ( (This)->lpVtbl -> SetActivitiesVisible(This,fVisible) ) 

#define IShellUIHelper3_ContentDiscoveryReset(This)	\
    ( (This)->lpVtbl -> ContentDiscoveryReset(This) ) 

#define IShellUIHelper3_IsSuggestedSitesEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> IsSuggestedSitesEnabled(This,pfEnabled) ) 

#define IShellUIHelper3_EnableSuggestedSites(This,fEnable)	\
    ( (This)->lpVtbl -> EnableSuggestedSites(This,fEnable) ) 

#define IShellUIHelper3_NavigateToSuggestedSites(This,bstrRelativeUrl)	\
    ( (This)->lpVtbl -> NavigateToSuggestedSites(This,bstrRelativeUrl) ) 

#define IShellUIHelper3_ShowTabsHelp(This)	\
    ( (This)->lpVtbl -> ShowTabsHelp(This) ) 

#define IShellUIHelper3_ShowInPrivateHelp(This)	\
    ( (This)->lpVtbl -> ShowInPrivateHelp(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellUIHelper3_INTERFACE_DEFINED__ */


#ifndef __IShellUIHelper4_INTERFACE_DEFINED__
#define __IShellUIHelper4_INTERFACE_DEFINED__

/* interface IShellUIHelper4 */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellUIHelper4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B36E6A53-8073-499E-824C-D776330A333E")
    IShellUIHelper4 : public IShellUIHelper3
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msIsSiteMode( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfSiteMode) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msSiteModeShowThumbBar( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msSiteModeAddThumbBarButton( 
            /* [in] */ __RPC__in BSTR bstrIconURL,
            /* [in] */ __RPC__in BSTR bstrTooltip,
            /* [retval][out] */ __RPC__out VARIANT *pvarButtonID) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msSiteModeUpdateThumbBarButton( 
            /* [in] */ VARIANT ButtonID,
            /* [in] */ VARIANT_BOOL fEnabled,
            /* [in] */ VARIANT_BOOL fVisible) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msSiteModeSetIconOverlay( 
            /* [in] */ __RPC__in BSTR IconUrl,
            /* [in][optional] */ __RPC__in VARIANT *pvarDescription) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msSiteModeClearIconOverlay( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msAddSiteMode( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msSiteModeCreateJumpList( 
            /* [in] */ __RPC__in BSTR bstrHeader) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msSiteModeAddJumpListItem( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in BSTR bstrActionUri,
            /* [in] */ __RPC__in BSTR bstrIconUri,
            /* [in][optional] */ __RPC__in VARIANT *pvarWindowType) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msSiteModeClearJumpList( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msSiteModeShowJumpList( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msSiteModeAddButtonStyle( 
            /* [in] */ VARIANT uiButtonID,
            /* [in] */ __RPC__in BSTR bstrIconUrl,
            /* [in] */ __RPC__in BSTR bstrTooltip,
            /* [retval][out] */ __RPC__out VARIANT *pvarStyleID) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msSiteModeShowButtonStyle( 
            /* [in] */ VARIANT uiButtonID,
            /* [in] */ VARIANT uiStyleID) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msSiteModeActivate( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msIsSiteModeFirstRun( 
            /* [in] */ VARIANT_BOOL fPreserveState,
            /* [retval][out] */ __RPC__out VARIANT *puiFirstRun) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msAddTrackingProtectionList( 
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR bstrFilterName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msTrackingProtectionEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msActiveXFilteringEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellUIHelper4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellUIHelper4 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellUIHelper4 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetFirstBootMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetFirstBootMode )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetSafeMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetSafeMode )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, RefreshOfflineDesktop)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *RefreshOfflineDesktop )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddFavorite)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddFavorite )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Title);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddChannel)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddChannel )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddDesktopComponent)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddDesktopComponent )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Type,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Left,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Top,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Width,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Height);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, IsSubscribed)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSubscribed )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, NavigateAndFind)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NavigateAndFind )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR strQuery,
            /* [in] */ __RPC__in VARIANT *varTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ImportExportFavorites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ImportExportFavorites )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ VARIANT_BOOL fImport,
            /* [in] */ __RPC__in BSTR strImpExpPath);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteSaveForm)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteSaveForm )( 
            __RPC__in IShellUIHelper4 * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Form);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoScan)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoScan )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR strSearch,
            /* [in] */ __RPC__in BSTR strFailureUrl,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *pvarTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteAttach)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteAttach )( 
            __RPC__in IShellUIHelper4 * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Reserved);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ShowBrowserUI)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowBrowserUI )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in VARIANT *pvarIn,
            /* [retval][out] */ __RPC__out VARIANT *pvarOut);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, AddSearchProvider)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddSearchProvider )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceShown)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceShown )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SkipRunOnce)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SkipRunOnce )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, CustomizeSettings)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CustomizeSettings )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ VARIANT_BOOL fSQM,
            /* [in] */ VARIANT_BOOL fPhishing,
            /* [in] */ __RPC__in BSTR bstrLocale);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SqmEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SqmEnabled )( 
            __RPC__in IShellUIHelper4 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, PhishingEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PhishingEnabled )( 
            __RPC__in IShellUIHelper4 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, BrandImageUri)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BrandImageUri )( 
            __RPC__in IShellUIHelper4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUri);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SkipTabsWelcome)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SkipTabsWelcome )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, DiagnoseConnection)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DiagnoseConnection )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, CustomizeClearType)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CustomizeClearType )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ VARIANT_BOOL fSet);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, IsSearchProviderInstalled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSearchProviderInstalled )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, IsSearchMigrated)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSearchMigrated )( 
            __RPC__in IShellUIHelper4 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfMigrated);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, DefaultSearchProvider)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DefaultSearchProvider )( 
            __RPC__in IShellUIHelper4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceRequiredSettingsComplete)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceRequiredSettingsComplete )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ VARIANT_BOOL fComplete);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceHasShown)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceHasShown )( 
            __RPC__in IShellUIHelper4 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfShown);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SearchGuideUrl)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SearchGuideUrl )( 
            __RPC__in IShellUIHelper4 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUrl);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, AddService)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddService )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, IsServiceInstalled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsServiceInstalled )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Verb,
            /* [retval][out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, InPrivateFilteringEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InPrivateFilteringEnabled )( 
            __RPC__in IShellUIHelper4 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, AddToFavoritesBar)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddToFavoritesBar )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Title,
            /* [in][optional] */ __RPC__in VARIANT *Type);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, BuildNewTabPage)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BuildNewTabPage )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, SetRecentlyClosedVisible)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetRecentlyClosedVisible )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, SetActivitiesVisible)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetActivitiesVisible )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ContentDiscoveryReset)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ContentDiscoveryReset )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, IsSuggestedSitesEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSuggestedSitesEnabled )( 
            __RPC__in IShellUIHelper4 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, EnableSuggestedSites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnableSuggestedSites )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ VARIANT_BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, NavigateToSuggestedSites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NavigateToSuggestedSites )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR bstrRelativeUrl);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ShowTabsHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowTabsHelp )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ShowInPrivateHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowInPrivateHelp )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msIsSiteMode)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msIsSiteMode )( 
            __RPC__in IShellUIHelper4 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfSiteMode);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowThumbBar)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowThumbBar )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddThumbBarButton)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddThumbBarButton )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR bstrIconURL,
            /* [in] */ __RPC__in BSTR bstrTooltip,
            /* [retval][out] */ __RPC__out VARIANT *pvarButtonID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeUpdateThumbBarButton)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeUpdateThumbBarButton )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ VARIANT ButtonID,
            /* [in] */ VARIANT_BOOL fEnabled,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeSetIconOverlay)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeSetIconOverlay )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR IconUrl,
            /* [in][optional] */ __RPC__in VARIANT *pvarDescription);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeClearIconOverlay)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeClearIconOverlay )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msAddSiteMode)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msAddSiteMode )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeCreateJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeCreateJumpList )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR bstrHeader);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddJumpListItem)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddJumpListItem )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in BSTR bstrActionUri,
            /* [in] */ __RPC__in BSTR bstrIconUri,
            /* [in][optional] */ __RPC__in VARIANT *pvarWindowType);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeClearJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeClearJumpList )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowJumpList )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddButtonStyle)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddButtonStyle )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ VARIANT uiButtonID,
            /* [in] */ __RPC__in BSTR bstrIconUrl,
            /* [in] */ __RPC__in BSTR bstrTooltip,
            /* [retval][out] */ __RPC__out VARIANT *pvarStyleID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowButtonStyle)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowButtonStyle )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ VARIANT uiButtonID,
            /* [in] */ VARIANT uiStyleID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeActivate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeActivate )( 
            __RPC__in IShellUIHelper4 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msIsSiteModeFirstRun)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msIsSiteModeFirstRun )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ VARIANT_BOOL fPreserveState,
            /* [retval][out] */ __RPC__out VARIANT *puiFirstRun);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msAddTrackingProtectionList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msAddTrackingProtectionList )( 
            __RPC__in IShellUIHelper4 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR bstrFilterName);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msTrackingProtectionEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msTrackingProtectionEnabled )( 
            __RPC__in IShellUIHelper4 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msActiveXFilteringEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msActiveXFilteringEnabled )( 
            __RPC__in IShellUIHelper4 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        END_INTERFACE
    } IShellUIHelper4Vtbl;

    interface IShellUIHelper4
    {
        CONST_VTBL struct IShellUIHelper4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellUIHelper4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellUIHelper4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellUIHelper4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellUIHelper4_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellUIHelper4_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellUIHelper4_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellUIHelper4_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellUIHelper4_ResetFirstBootMode(This)	\
    ( (This)->lpVtbl -> ResetFirstBootMode(This) ) 

#define IShellUIHelper4_ResetSafeMode(This)	\
    ( (This)->lpVtbl -> ResetSafeMode(This) ) 

#define IShellUIHelper4_RefreshOfflineDesktop(This)	\
    ( (This)->lpVtbl -> RefreshOfflineDesktop(This) ) 

#define IShellUIHelper4_AddFavorite(This,URL,Title)	\
    ( (This)->lpVtbl -> AddFavorite(This,URL,Title) ) 

#define IShellUIHelper4_AddChannel(This,URL)	\
    ( (This)->lpVtbl -> AddChannel(This,URL) ) 

#define IShellUIHelper4_AddDesktopComponent(This,URL,Type,Left,Top,Width,Height)	\
    ( (This)->lpVtbl -> AddDesktopComponent(This,URL,Type,Left,Top,Width,Height) ) 

#define IShellUIHelper4_IsSubscribed(This,URL,pBool)	\
    ( (This)->lpVtbl -> IsSubscribed(This,URL,pBool) ) 

#define IShellUIHelper4_NavigateAndFind(This,URL,strQuery,varTargetFrame)	\
    ( (This)->lpVtbl -> NavigateAndFind(This,URL,strQuery,varTargetFrame) ) 

#define IShellUIHelper4_ImportExportFavorites(This,fImport,strImpExpPath)	\
    ( (This)->lpVtbl -> ImportExportFavorites(This,fImport,strImpExpPath) ) 

#define IShellUIHelper4_AutoCompleteSaveForm(This,Form)	\
    ( (This)->lpVtbl -> AutoCompleteSaveForm(This,Form) ) 

#define IShellUIHelper4_AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame)	\
    ( (This)->lpVtbl -> AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame) ) 

#define IShellUIHelper4_AutoCompleteAttach(This,Reserved)	\
    ( (This)->lpVtbl -> AutoCompleteAttach(This,Reserved) ) 

#define IShellUIHelper4_ShowBrowserUI(This,bstrName,pvarIn,pvarOut)	\
    ( (This)->lpVtbl -> ShowBrowserUI(This,bstrName,pvarIn,pvarOut) ) 


#define IShellUIHelper4_AddSearchProvider(This,URL)	\
    ( (This)->lpVtbl -> AddSearchProvider(This,URL) ) 

#define IShellUIHelper4_RunOnceShown(This)	\
    ( (This)->lpVtbl -> RunOnceShown(This) ) 

#define IShellUIHelper4_SkipRunOnce(This)	\
    ( (This)->lpVtbl -> SkipRunOnce(This) ) 

#define IShellUIHelper4_CustomizeSettings(This,fSQM,fPhishing,bstrLocale)	\
    ( (This)->lpVtbl -> CustomizeSettings(This,fSQM,fPhishing,bstrLocale) ) 

#define IShellUIHelper4_SqmEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> SqmEnabled(This,pfEnabled) ) 

#define IShellUIHelper4_PhishingEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> PhishingEnabled(This,pfEnabled) ) 

#define IShellUIHelper4_BrandImageUri(This,pbstrUri)	\
    ( (This)->lpVtbl -> BrandImageUri(This,pbstrUri) ) 

#define IShellUIHelper4_SkipTabsWelcome(This)	\
    ( (This)->lpVtbl -> SkipTabsWelcome(This) ) 

#define IShellUIHelper4_DiagnoseConnection(This)	\
    ( (This)->lpVtbl -> DiagnoseConnection(This) ) 

#define IShellUIHelper4_CustomizeClearType(This,fSet)	\
    ( (This)->lpVtbl -> CustomizeClearType(This,fSet) ) 

#define IShellUIHelper4_IsSearchProviderInstalled(This,URL,pdwResult)	\
    ( (This)->lpVtbl -> IsSearchProviderInstalled(This,URL,pdwResult) ) 

#define IShellUIHelper4_IsSearchMigrated(This,pfMigrated)	\
    ( (This)->lpVtbl -> IsSearchMigrated(This,pfMigrated) ) 

#define IShellUIHelper4_DefaultSearchProvider(This,pbstrName)	\
    ( (This)->lpVtbl -> DefaultSearchProvider(This,pbstrName) ) 

#define IShellUIHelper4_RunOnceRequiredSettingsComplete(This,fComplete)	\
    ( (This)->lpVtbl -> RunOnceRequiredSettingsComplete(This,fComplete) ) 

#define IShellUIHelper4_RunOnceHasShown(This,pfShown)	\
    ( (This)->lpVtbl -> RunOnceHasShown(This,pfShown) ) 

#define IShellUIHelper4_SearchGuideUrl(This,pbstrUrl)	\
    ( (This)->lpVtbl -> SearchGuideUrl(This,pbstrUrl) ) 


#define IShellUIHelper4_AddService(This,URL)	\
    ( (This)->lpVtbl -> AddService(This,URL) ) 

#define IShellUIHelper4_IsServiceInstalled(This,URL,Verb,pdwResult)	\
    ( (This)->lpVtbl -> IsServiceInstalled(This,URL,Verb,pdwResult) ) 

#define IShellUIHelper4_InPrivateFilteringEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> InPrivateFilteringEnabled(This,pfEnabled) ) 

#define IShellUIHelper4_AddToFavoritesBar(This,URL,Title,Type)	\
    ( (This)->lpVtbl -> AddToFavoritesBar(This,URL,Title,Type) ) 

#define IShellUIHelper4_BuildNewTabPage(This)	\
    ( (This)->lpVtbl -> BuildNewTabPage(This) ) 

#define IShellUIHelper4_SetRecentlyClosedVisible(This,fVisible)	\
    ( (This)->lpVtbl -> SetRecentlyClosedVisible(This,fVisible) ) 

#define IShellUIHelper4_SetActivitiesVisible(This,fVisible)	\
    ( (This)->lpVtbl -> SetActivitiesVisible(This,fVisible) ) 

#define IShellUIHelper4_ContentDiscoveryReset(This)	\
    ( (This)->lpVtbl -> ContentDiscoveryReset(This) ) 

#define IShellUIHelper4_IsSuggestedSitesEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> IsSuggestedSitesEnabled(This,pfEnabled) ) 

#define IShellUIHelper4_EnableSuggestedSites(This,fEnable)	\
    ( (This)->lpVtbl -> EnableSuggestedSites(This,fEnable) ) 

#define IShellUIHelper4_NavigateToSuggestedSites(This,bstrRelativeUrl)	\
    ( (This)->lpVtbl -> NavigateToSuggestedSites(This,bstrRelativeUrl) ) 

#define IShellUIHelper4_ShowTabsHelp(This)	\
    ( (This)->lpVtbl -> ShowTabsHelp(This) ) 

#define IShellUIHelper4_ShowInPrivateHelp(This)	\
    ( (This)->lpVtbl -> ShowInPrivateHelp(This) ) 


#define IShellUIHelper4_msIsSiteMode(This,pfSiteMode)	\
    ( (This)->lpVtbl -> msIsSiteMode(This,pfSiteMode) ) 

#define IShellUIHelper4_msSiteModeShowThumbBar(This)	\
    ( (This)->lpVtbl -> msSiteModeShowThumbBar(This) ) 

#define IShellUIHelper4_msSiteModeAddThumbBarButton(This,bstrIconURL,bstrTooltip,pvarButtonID)	\
    ( (This)->lpVtbl -> msSiteModeAddThumbBarButton(This,bstrIconURL,bstrTooltip,pvarButtonID) ) 

#define IShellUIHelper4_msSiteModeUpdateThumbBarButton(This,ButtonID,fEnabled,fVisible)	\
    ( (This)->lpVtbl -> msSiteModeUpdateThumbBarButton(This,ButtonID,fEnabled,fVisible) ) 

#define IShellUIHelper4_msSiteModeSetIconOverlay(This,IconUrl,pvarDescription)	\
    ( (This)->lpVtbl -> msSiteModeSetIconOverlay(This,IconUrl,pvarDescription) ) 

#define IShellUIHelper4_msSiteModeClearIconOverlay(This)	\
    ( (This)->lpVtbl -> msSiteModeClearIconOverlay(This) ) 

#define IShellUIHelper4_msAddSiteMode(This)	\
    ( (This)->lpVtbl -> msAddSiteMode(This) ) 

#define IShellUIHelper4_msSiteModeCreateJumpList(This,bstrHeader)	\
    ( (This)->lpVtbl -> msSiteModeCreateJumpList(This,bstrHeader) ) 

#define IShellUIHelper4_msSiteModeAddJumpListItem(This,bstrName,bstrActionUri,bstrIconUri,pvarWindowType)	\
    ( (This)->lpVtbl -> msSiteModeAddJumpListItem(This,bstrName,bstrActionUri,bstrIconUri,pvarWindowType) ) 

#define IShellUIHelper4_msSiteModeClearJumpList(This)	\
    ( (This)->lpVtbl -> msSiteModeClearJumpList(This) ) 

#define IShellUIHelper4_msSiteModeShowJumpList(This)	\
    ( (This)->lpVtbl -> msSiteModeShowJumpList(This) ) 

#define IShellUIHelper4_msSiteModeAddButtonStyle(This,uiButtonID,bstrIconUrl,bstrTooltip,pvarStyleID)	\
    ( (This)->lpVtbl -> msSiteModeAddButtonStyle(This,uiButtonID,bstrIconUrl,bstrTooltip,pvarStyleID) ) 

#define IShellUIHelper4_msSiteModeShowButtonStyle(This,uiButtonID,uiStyleID)	\
    ( (This)->lpVtbl -> msSiteModeShowButtonStyle(This,uiButtonID,uiStyleID) ) 

#define IShellUIHelper4_msSiteModeActivate(This)	\
    ( (This)->lpVtbl -> msSiteModeActivate(This) ) 

#define IShellUIHelper4_msIsSiteModeFirstRun(This,fPreserveState,puiFirstRun)	\
    ( (This)->lpVtbl -> msIsSiteModeFirstRun(This,fPreserveState,puiFirstRun) ) 

#define IShellUIHelper4_msAddTrackingProtectionList(This,URL,bstrFilterName)	\
    ( (This)->lpVtbl -> msAddTrackingProtectionList(This,URL,bstrFilterName) ) 

#define IShellUIHelper4_msTrackingProtectionEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> msTrackingProtectionEnabled(This,pfEnabled) ) 

#define IShellUIHelper4_msActiveXFilteringEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> msActiveXFilteringEnabled(This,pfEnabled) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellUIHelper4_INTERFACE_DEFINED__ */


#ifndef __IShellUIHelper5_INTERFACE_DEFINED__
#define __IShellUIHelper5_INTERFACE_DEFINED__

/* interface IShellUIHelper5 */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellUIHelper5;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A2A08B09-103D-4D3F-B91C-EA455CA82EFA")
    IShellUIHelper5 : public IShellUIHelper4
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msProvisionNetworks( 
            /* [in] */ __RPC__in BSTR bstrProvisioningXml,
            /* [retval][out] */ __RPC__out VARIANT *puiResult) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msReportSafeUrl( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msSiteModeRefreshBadge( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msSiteModeClearBadge( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msDiagnoseConnectionUILess( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msLaunchNetworkClientHelp( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msChangeDefaultBrowser( 
            /* [in] */ VARIANT_BOOL fChange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellUIHelper5Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellUIHelper5 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellUIHelper5 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetFirstBootMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetFirstBootMode )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetSafeMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetSafeMode )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, RefreshOfflineDesktop)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *RefreshOfflineDesktop )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddFavorite)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddFavorite )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Title);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddChannel)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddChannel )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddDesktopComponent)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddDesktopComponent )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Type,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Left,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Top,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Width,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Height);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, IsSubscribed)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSubscribed )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, NavigateAndFind)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NavigateAndFind )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR strQuery,
            /* [in] */ __RPC__in VARIANT *varTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ImportExportFavorites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ImportExportFavorites )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ VARIANT_BOOL fImport,
            /* [in] */ __RPC__in BSTR strImpExpPath);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteSaveForm)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteSaveForm )( 
            __RPC__in IShellUIHelper5 * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Form);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoScan)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoScan )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR strSearch,
            /* [in] */ __RPC__in BSTR strFailureUrl,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *pvarTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteAttach)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteAttach )( 
            __RPC__in IShellUIHelper5 * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Reserved);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ShowBrowserUI)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowBrowserUI )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in VARIANT *pvarIn,
            /* [retval][out] */ __RPC__out VARIANT *pvarOut);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, AddSearchProvider)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddSearchProvider )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceShown)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceShown )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SkipRunOnce)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SkipRunOnce )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, CustomizeSettings)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CustomizeSettings )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ VARIANT_BOOL fSQM,
            /* [in] */ VARIANT_BOOL fPhishing,
            /* [in] */ __RPC__in BSTR bstrLocale);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SqmEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SqmEnabled )( 
            __RPC__in IShellUIHelper5 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, PhishingEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PhishingEnabled )( 
            __RPC__in IShellUIHelper5 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, BrandImageUri)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BrandImageUri )( 
            __RPC__in IShellUIHelper5 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUri);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SkipTabsWelcome)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SkipTabsWelcome )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, DiagnoseConnection)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DiagnoseConnection )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, CustomizeClearType)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CustomizeClearType )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ VARIANT_BOOL fSet);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, IsSearchProviderInstalled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSearchProviderInstalled )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, IsSearchMigrated)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSearchMigrated )( 
            __RPC__in IShellUIHelper5 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfMigrated);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, DefaultSearchProvider)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DefaultSearchProvider )( 
            __RPC__in IShellUIHelper5 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceRequiredSettingsComplete)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceRequiredSettingsComplete )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ VARIANT_BOOL fComplete);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceHasShown)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceHasShown )( 
            __RPC__in IShellUIHelper5 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfShown);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SearchGuideUrl)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SearchGuideUrl )( 
            __RPC__in IShellUIHelper5 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUrl);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, AddService)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddService )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, IsServiceInstalled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsServiceInstalled )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Verb,
            /* [retval][out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, InPrivateFilteringEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InPrivateFilteringEnabled )( 
            __RPC__in IShellUIHelper5 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, AddToFavoritesBar)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddToFavoritesBar )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Title,
            /* [in][optional] */ __RPC__in VARIANT *Type);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, BuildNewTabPage)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BuildNewTabPage )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, SetRecentlyClosedVisible)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetRecentlyClosedVisible )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, SetActivitiesVisible)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetActivitiesVisible )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ContentDiscoveryReset)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ContentDiscoveryReset )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, IsSuggestedSitesEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSuggestedSitesEnabled )( 
            __RPC__in IShellUIHelper5 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, EnableSuggestedSites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnableSuggestedSites )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ VARIANT_BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, NavigateToSuggestedSites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NavigateToSuggestedSites )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR bstrRelativeUrl);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ShowTabsHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowTabsHelp )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ShowInPrivateHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowInPrivateHelp )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msIsSiteMode)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msIsSiteMode )( 
            __RPC__in IShellUIHelper5 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfSiteMode);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowThumbBar)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowThumbBar )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddThumbBarButton)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddThumbBarButton )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR bstrIconURL,
            /* [in] */ __RPC__in BSTR bstrTooltip,
            /* [retval][out] */ __RPC__out VARIANT *pvarButtonID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeUpdateThumbBarButton)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeUpdateThumbBarButton )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ VARIANT ButtonID,
            /* [in] */ VARIANT_BOOL fEnabled,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeSetIconOverlay)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeSetIconOverlay )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR IconUrl,
            /* [in][optional] */ __RPC__in VARIANT *pvarDescription);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeClearIconOverlay)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeClearIconOverlay )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msAddSiteMode)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msAddSiteMode )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeCreateJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeCreateJumpList )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR bstrHeader);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddJumpListItem)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddJumpListItem )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in BSTR bstrActionUri,
            /* [in] */ __RPC__in BSTR bstrIconUri,
            /* [in][optional] */ __RPC__in VARIANT *pvarWindowType);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeClearJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeClearJumpList )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowJumpList )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddButtonStyle)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddButtonStyle )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ VARIANT uiButtonID,
            /* [in] */ __RPC__in BSTR bstrIconUrl,
            /* [in] */ __RPC__in BSTR bstrTooltip,
            /* [retval][out] */ __RPC__out VARIANT *pvarStyleID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowButtonStyle)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowButtonStyle )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ VARIANT uiButtonID,
            /* [in] */ VARIANT uiStyleID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeActivate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeActivate )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msIsSiteModeFirstRun)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msIsSiteModeFirstRun )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ VARIANT_BOOL fPreserveState,
            /* [retval][out] */ __RPC__out VARIANT *puiFirstRun);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msAddTrackingProtectionList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msAddTrackingProtectionList )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR bstrFilterName);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msTrackingProtectionEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msTrackingProtectionEnabled )( 
            __RPC__in IShellUIHelper5 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msActiveXFilteringEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msActiveXFilteringEnabled )( 
            __RPC__in IShellUIHelper5 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msProvisionNetworks)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msProvisionNetworks )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ __RPC__in BSTR bstrProvisioningXml,
            /* [retval][out] */ __RPC__out VARIANT *puiResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msReportSafeUrl)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msReportSafeUrl )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msSiteModeRefreshBadge)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeRefreshBadge )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msSiteModeClearBadge)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeClearBadge )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msDiagnoseConnectionUILess)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msDiagnoseConnectionUILess )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msLaunchNetworkClientHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msLaunchNetworkClientHelp )( 
            __RPC__in IShellUIHelper5 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msChangeDefaultBrowser)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msChangeDefaultBrowser )( 
            __RPC__in IShellUIHelper5 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        END_INTERFACE
    } IShellUIHelper5Vtbl;

    interface IShellUIHelper5
    {
        CONST_VTBL struct IShellUIHelper5Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellUIHelper5_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellUIHelper5_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellUIHelper5_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellUIHelper5_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellUIHelper5_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellUIHelper5_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellUIHelper5_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellUIHelper5_ResetFirstBootMode(This)	\
    ( (This)->lpVtbl -> ResetFirstBootMode(This) ) 

#define IShellUIHelper5_ResetSafeMode(This)	\
    ( (This)->lpVtbl -> ResetSafeMode(This) ) 

#define IShellUIHelper5_RefreshOfflineDesktop(This)	\
    ( (This)->lpVtbl -> RefreshOfflineDesktop(This) ) 

#define IShellUIHelper5_AddFavorite(This,URL,Title)	\
    ( (This)->lpVtbl -> AddFavorite(This,URL,Title) ) 

#define IShellUIHelper5_AddChannel(This,URL)	\
    ( (This)->lpVtbl -> AddChannel(This,URL) ) 

#define IShellUIHelper5_AddDesktopComponent(This,URL,Type,Left,Top,Width,Height)	\
    ( (This)->lpVtbl -> AddDesktopComponent(This,URL,Type,Left,Top,Width,Height) ) 

#define IShellUIHelper5_IsSubscribed(This,URL,pBool)	\
    ( (This)->lpVtbl -> IsSubscribed(This,URL,pBool) ) 

#define IShellUIHelper5_NavigateAndFind(This,URL,strQuery,varTargetFrame)	\
    ( (This)->lpVtbl -> NavigateAndFind(This,URL,strQuery,varTargetFrame) ) 

#define IShellUIHelper5_ImportExportFavorites(This,fImport,strImpExpPath)	\
    ( (This)->lpVtbl -> ImportExportFavorites(This,fImport,strImpExpPath) ) 

#define IShellUIHelper5_AutoCompleteSaveForm(This,Form)	\
    ( (This)->lpVtbl -> AutoCompleteSaveForm(This,Form) ) 

#define IShellUIHelper5_AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame)	\
    ( (This)->lpVtbl -> AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame) ) 

#define IShellUIHelper5_AutoCompleteAttach(This,Reserved)	\
    ( (This)->lpVtbl -> AutoCompleteAttach(This,Reserved) ) 

#define IShellUIHelper5_ShowBrowserUI(This,bstrName,pvarIn,pvarOut)	\
    ( (This)->lpVtbl -> ShowBrowserUI(This,bstrName,pvarIn,pvarOut) ) 


#define IShellUIHelper5_AddSearchProvider(This,URL)	\
    ( (This)->lpVtbl -> AddSearchProvider(This,URL) ) 

#define IShellUIHelper5_RunOnceShown(This)	\
    ( (This)->lpVtbl -> RunOnceShown(This) ) 

#define IShellUIHelper5_SkipRunOnce(This)	\
    ( (This)->lpVtbl -> SkipRunOnce(This) ) 

#define IShellUIHelper5_CustomizeSettings(This,fSQM,fPhishing,bstrLocale)	\
    ( (This)->lpVtbl -> CustomizeSettings(This,fSQM,fPhishing,bstrLocale) ) 

#define IShellUIHelper5_SqmEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> SqmEnabled(This,pfEnabled) ) 

#define IShellUIHelper5_PhishingEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> PhishingEnabled(This,pfEnabled) ) 

#define IShellUIHelper5_BrandImageUri(This,pbstrUri)	\
    ( (This)->lpVtbl -> BrandImageUri(This,pbstrUri) ) 

#define IShellUIHelper5_SkipTabsWelcome(This)	\
    ( (This)->lpVtbl -> SkipTabsWelcome(This) ) 

#define IShellUIHelper5_DiagnoseConnection(This)	\
    ( (This)->lpVtbl -> DiagnoseConnection(This) ) 

#define IShellUIHelper5_CustomizeClearType(This,fSet)	\
    ( (This)->lpVtbl -> CustomizeClearType(This,fSet) ) 

#define IShellUIHelper5_IsSearchProviderInstalled(This,URL,pdwResult)	\
    ( (This)->lpVtbl -> IsSearchProviderInstalled(This,URL,pdwResult) ) 

#define IShellUIHelper5_IsSearchMigrated(This,pfMigrated)	\
    ( (This)->lpVtbl -> IsSearchMigrated(This,pfMigrated) ) 

#define IShellUIHelper5_DefaultSearchProvider(This,pbstrName)	\
    ( (This)->lpVtbl -> DefaultSearchProvider(This,pbstrName) ) 

#define IShellUIHelper5_RunOnceRequiredSettingsComplete(This,fComplete)	\
    ( (This)->lpVtbl -> RunOnceRequiredSettingsComplete(This,fComplete) ) 

#define IShellUIHelper5_RunOnceHasShown(This,pfShown)	\
    ( (This)->lpVtbl -> RunOnceHasShown(This,pfShown) ) 

#define IShellUIHelper5_SearchGuideUrl(This,pbstrUrl)	\
    ( (This)->lpVtbl -> SearchGuideUrl(This,pbstrUrl) ) 


#define IShellUIHelper5_AddService(This,URL)	\
    ( (This)->lpVtbl -> AddService(This,URL) ) 

#define IShellUIHelper5_IsServiceInstalled(This,URL,Verb,pdwResult)	\
    ( (This)->lpVtbl -> IsServiceInstalled(This,URL,Verb,pdwResult) ) 

#define IShellUIHelper5_InPrivateFilteringEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> InPrivateFilteringEnabled(This,pfEnabled) ) 

#define IShellUIHelper5_AddToFavoritesBar(This,URL,Title,Type)	\
    ( (This)->lpVtbl -> AddToFavoritesBar(This,URL,Title,Type) ) 

#define IShellUIHelper5_BuildNewTabPage(This)	\
    ( (This)->lpVtbl -> BuildNewTabPage(This) ) 

#define IShellUIHelper5_SetRecentlyClosedVisible(This,fVisible)	\
    ( (This)->lpVtbl -> SetRecentlyClosedVisible(This,fVisible) ) 

#define IShellUIHelper5_SetActivitiesVisible(This,fVisible)	\
    ( (This)->lpVtbl -> SetActivitiesVisible(This,fVisible) ) 

#define IShellUIHelper5_ContentDiscoveryReset(This)	\
    ( (This)->lpVtbl -> ContentDiscoveryReset(This) ) 

#define IShellUIHelper5_IsSuggestedSitesEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> IsSuggestedSitesEnabled(This,pfEnabled) ) 

#define IShellUIHelper5_EnableSuggestedSites(This,fEnable)	\
    ( (This)->lpVtbl -> EnableSuggestedSites(This,fEnable) ) 

#define IShellUIHelper5_NavigateToSuggestedSites(This,bstrRelativeUrl)	\
    ( (This)->lpVtbl -> NavigateToSuggestedSites(This,bstrRelativeUrl) ) 

#define IShellUIHelper5_ShowTabsHelp(This)	\
    ( (This)->lpVtbl -> ShowTabsHelp(This) ) 

#define IShellUIHelper5_ShowInPrivateHelp(This)	\
    ( (This)->lpVtbl -> ShowInPrivateHelp(This) ) 


#define IShellUIHelper5_msIsSiteMode(This,pfSiteMode)	\
    ( (This)->lpVtbl -> msIsSiteMode(This,pfSiteMode) ) 

#define IShellUIHelper5_msSiteModeShowThumbBar(This)	\
    ( (This)->lpVtbl -> msSiteModeShowThumbBar(This) ) 

#define IShellUIHelper5_msSiteModeAddThumbBarButton(This,bstrIconURL,bstrTooltip,pvarButtonID)	\
    ( (This)->lpVtbl -> msSiteModeAddThumbBarButton(This,bstrIconURL,bstrTooltip,pvarButtonID) ) 

#define IShellUIHelper5_msSiteModeUpdateThumbBarButton(This,ButtonID,fEnabled,fVisible)	\
    ( (This)->lpVtbl -> msSiteModeUpdateThumbBarButton(This,ButtonID,fEnabled,fVisible) ) 

#define IShellUIHelper5_msSiteModeSetIconOverlay(This,IconUrl,pvarDescription)	\
    ( (This)->lpVtbl -> msSiteModeSetIconOverlay(This,IconUrl,pvarDescription) ) 

#define IShellUIHelper5_msSiteModeClearIconOverlay(This)	\
    ( (This)->lpVtbl -> msSiteModeClearIconOverlay(This) ) 

#define IShellUIHelper5_msAddSiteMode(This)	\
    ( (This)->lpVtbl -> msAddSiteMode(This) ) 

#define IShellUIHelper5_msSiteModeCreateJumpList(This,bstrHeader)	\
    ( (This)->lpVtbl -> msSiteModeCreateJumpList(This,bstrHeader) ) 

#define IShellUIHelper5_msSiteModeAddJumpListItem(This,bstrName,bstrActionUri,bstrIconUri,pvarWindowType)	\
    ( (This)->lpVtbl -> msSiteModeAddJumpListItem(This,bstrName,bstrActionUri,bstrIconUri,pvarWindowType) ) 

#define IShellUIHelper5_msSiteModeClearJumpList(This)	\
    ( (This)->lpVtbl -> msSiteModeClearJumpList(This) ) 

#define IShellUIHelper5_msSiteModeShowJumpList(This)	\
    ( (This)->lpVtbl -> msSiteModeShowJumpList(This) ) 

#define IShellUIHelper5_msSiteModeAddButtonStyle(This,uiButtonID,bstrIconUrl,bstrTooltip,pvarStyleID)	\
    ( (This)->lpVtbl -> msSiteModeAddButtonStyle(This,uiButtonID,bstrIconUrl,bstrTooltip,pvarStyleID) ) 

#define IShellUIHelper5_msSiteModeShowButtonStyle(This,uiButtonID,uiStyleID)	\
    ( (This)->lpVtbl -> msSiteModeShowButtonStyle(This,uiButtonID,uiStyleID) ) 

#define IShellUIHelper5_msSiteModeActivate(This)	\
    ( (This)->lpVtbl -> msSiteModeActivate(This) ) 

#define IShellUIHelper5_msIsSiteModeFirstRun(This,fPreserveState,puiFirstRun)	\
    ( (This)->lpVtbl -> msIsSiteModeFirstRun(This,fPreserveState,puiFirstRun) ) 

#define IShellUIHelper5_msAddTrackingProtectionList(This,URL,bstrFilterName)	\
    ( (This)->lpVtbl -> msAddTrackingProtectionList(This,URL,bstrFilterName) ) 

#define IShellUIHelper5_msTrackingProtectionEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> msTrackingProtectionEnabled(This,pfEnabled) ) 

#define IShellUIHelper5_msActiveXFilteringEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> msActiveXFilteringEnabled(This,pfEnabled) ) 


#define IShellUIHelper5_msProvisionNetworks(This,bstrProvisioningXml,puiResult)	\
    ( (This)->lpVtbl -> msProvisionNetworks(This,bstrProvisioningXml,puiResult) ) 

#define IShellUIHelper5_msReportSafeUrl(This)	\
    ( (This)->lpVtbl -> msReportSafeUrl(This) ) 

#define IShellUIHelper5_msSiteModeRefreshBadge(This)	\
    ( (This)->lpVtbl -> msSiteModeRefreshBadge(This) ) 

#define IShellUIHelper5_msSiteModeClearBadge(This)	\
    ( (This)->lpVtbl -> msSiteModeClearBadge(This) ) 

#define IShellUIHelper5_msDiagnoseConnectionUILess(This)	\
    ( (This)->lpVtbl -> msDiagnoseConnectionUILess(This) ) 

#define IShellUIHelper5_msLaunchNetworkClientHelp(This)	\
    ( (This)->lpVtbl -> msLaunchNetworkClientHelp(This) ) 

#define IShellUIHelper5_msChangeDefaultBrowser(This,fChange)	\
    ( (This)->lpVtbl -> msChangeDefaultBrowser(This,fChange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellUIHelper5_INTERFACE_DEFINED__ */


#ifndef __IShellUIHelper6_INTERFACE_DEFINED__
#define __IShellUIHelper6_INTERFACE_DEFINED__

/* interface IShellUIHelper6 */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellUIHelper6;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("987A573E-46EE-4E89-96AB-DDF7F8FDC98C")
    IShellUIHelper6 : public IShellUIHelper5
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msStopPeriodicTileUpdate( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msStartPeriodicTileUpdate( 
            /* [in] */ VARIANT pollingUris,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT uiUpdateRecurrence) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msStartPeriodicTileUpdateBatch( 
            /* [in] */ VARIANT pollingUris,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT uiUpdateRecurrence) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msClearTile( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msEnableTileNotificationQueue( 
            /* [in] */ VARIANT_BOOL fChange) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msPinnedSiteState( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSiteState) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msEnableTileNotificationQueueForSquare150x150( 
            /* [in] */ VARIANT_BOOL fChange) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msEnableTileNotificationQueueForWide310x150( 
            /* [in] */ VARIANT_BOOL fChange) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msEnableTileNotificationQueueForSquare310x310( 
            /* [in] */ VARIANT_BOOL fChange) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msScheduledTileNotification( 
            /* [in] */ __RPC__in BSTR bstrNotificationXml,
            /* [in] */ __RPC__in BSTR bstrNotificationId,
            /* [in] */ __RPC__in BSTR bstrNotificationTag,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT expirationTime) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msRemoveScheduledTileNotification( 
            /* [in] */ __RPC__in BSTR bstrNotificationId) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msStartPeriodicBadgeUpdate( 
            /* [in] */ __RPC__in BSTR pollingUri,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT uiUpdateRecurrence) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msStopPeriodicBadgeUpdate( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE msLaunchInternetOptions( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellUIHelper6Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellUIHelper6 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellUIHelper6 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetFirstBootMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetFirstBootMode )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetSafeMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetSafeMode )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, RefreshOfflineDesktop)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *RefreshOfflineDesktop )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddFavorite)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddFavorite )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Title);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddChannel)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddChannel )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddDesktopComponent)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddDesktopComponent )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Type,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Left,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Top,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Width,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Height);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, IsSubscribed)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSubscribed )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, NavigateAndFind)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NavigateAndFind )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR strQuery,
            /* [in] */ __RPC__in VARIANT *varTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ImportExportFavorites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ImportExportFavorites )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT_BOOL fImport,
            /* [in] */ __RPC__in BSTR strImpExpPath);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteSaveForm)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteSaveForm )( 
            __RPC__in IShellUIHelper6 * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Form);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoScan)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoScan )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR strSearch,
            /* [in] */ __RPC__in BSTR strFailureUrl,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *pvarTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteAttach)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteAttach )( 
            __RPC__in IShellUIHelper6 * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Reserved);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ShowBrowserUI)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowBrowserUI )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in VARIANT *pvarIn,
            /* [retval][out] */ __RPC__out VARIANT *pvarOut);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, AddSearchProvider)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddSearchProvider )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceShown)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceShown )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SkipRunOnce)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SkipRunOnce )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, CustomizeSettings)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CustomizeSettings )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT_BOOL fSQM,
            /* [in] */ VARIANT_BOOL fPhishing,
            /* [in] */ __RPC__in BSTR bstrLocale);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SqmEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SqmEnabled )( 
            __RPC__in IShellUIHelper6 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, PhishingEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PhishingEnabled )( 
            __RPC__in IShellUIHelper6 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, BrandImageUri)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BrandImageUri )( 
            __RPC__in IShellUIHelper6 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUri);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SkipTabsWelcome)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SkipTabsWelcome )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, DiagnoseConnection)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DiagnoseConnection )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, CustomizeClearType)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CustomizeClearType )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT_BOOL fSet);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, IsSearchProviderInstalled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSearchProviderInstalled )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, IsSearchMigrated)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSearchMigrated )( 
            __RPC__in IShellUIHelper6 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfMigrated);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, DefaultSearchProvider)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DefaultSearchProvider )( 
            __RPC__in IShellUIHelper6 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceRequiredSettingsComplete)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceRequiredSettingsComplete )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT_BOOL fComplete);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceHasShown)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceHasShown )( 
            __RPC__in IShellUIHelper6 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfShown);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SearchGuideUrl)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SearchGuideUrl )( 
            __RPC__in IShellUIHelper6 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUrl);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, AddService)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddService )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, IsServiceInstalled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsServiceInstalled )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Verb,
            /* [retval][out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, InPrivateFilteringEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InPrivateFilteringEnabled )( 
            __RPC__in IShellUIHelper6 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, AddToFavoritesBar)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddToFavoritesBar )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Title,
            /* [in][optional] */ __RPC__in VARIANT *Type);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, BuildNewTabPage)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BuildNewTabPage )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, SetRecentlyClosedVisible)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetRecentlyClosedVisible )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, SetActivitiesVisible)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetActivitiesVisible )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ContentDiscoveryReset)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ContentDiscoveryReset )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, IsSuggestedSitesEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSuggestedSitesEnabled )( 
            __RPC__in IShellUIHelper6 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, EnableSuggestedSites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnableSuggestedSites )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT_BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, NavigateToSuggestedSites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NavigateToSuggestedSites )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR bstrRelativeUrl);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ShowTabsHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowTabsHelp )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ShowInPrivateHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowInPrivateHelp )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msIsSiteMode)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msIsSiteMode )( 
            __RPC__in IShellUIHelper6 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfSiteMode);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowThumbBar)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowThumbBar )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddThumbBarButton)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddThumbBarButton )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR bstrIconURL,
            /* [in] */ __RPC__in BSTR bstrTooltip,
            /* [retval][out] */ __RPC__out VARIANT *pvarButtonID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeUpdateThumbBarButton)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeUpdateThumbBarButton )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT ButtonID,
            /* [in] */ VARIANT_BOOL fEnabled,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeSetIconOverlay)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeSetIconOverlay )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR IconUrl,
            /* [in][optional] */ __RPC__in VARIANT *pvarDescription);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeClearIconOverlay)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeClearIconOverlay )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msAddSiteMode)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msAddSiteMode )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeCreateJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeCreateJumpList )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR bstrHeader);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddJumpListItem)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddJumpListItem )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in BSTR bstrActionUri,
            /* [in] */ __RPC__in BSTR bstrIconUri,
            /* [in][optional] */ __RPC__in VARIANT *pvarWindowType);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeClearJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeClearJumpList )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowJumpList )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddButtonStyle)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddButtonStyle )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT uiButtonID,
            /* [in] */ __RPC__in BSTR bstrIconUrl,
            /* [in] */ __RPC__in BSTR bstrTooltip,
            /* [retval][out] */ __RPC__out VARIANT *pvarStyleID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowButtonStyle)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowButtonStyle )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT uiButtonID,
            /* [in] */ VARIANT uiStyleID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeActivate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeActivate )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msIsSiteModeFirstRun)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msIsSiteModeFirstRun )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT_BOOL fPreserveState,
            /* [retval][out] */ __RPC__out VARIANT *puiFirstRun);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msAddTrackingProtectionList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msAddTrackingProtectionList )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR bstrFilterName);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msTrackingProtectionEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msTrackingProtectionEnabled )( 
            __RPC__in IShellUIHelper6 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msActiveXFilteringEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msActiveXFilteringEnabled )( 
            __RPC__in IShellUIHelper6 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msProvisionNetworks)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msProvisionNetworks )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR bstrProvisioningXml,
            /* [retval][out] */ __RPC__out VARIANT *puiResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msReportSafeUrl)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msReportSafeUrl )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msSiteModeRefreshBadge)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeRefreshBadge )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msSiteModeClearBadge)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeClearBadge )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msDiagnoseConnectionUILess)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msDiagnoseConnectionUILess )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msLaunchNetworkClientHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msLaunchNetworkClientHelp )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msChangeDefaultBrowser)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msChangeDefaultBrowser )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStopPeriodicTileUpdate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStopPeriodicTileUpdate )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStartPeriodicTileUpdate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStartPeriodicTileUpdate )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT pollingUris,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT uiUpdateRecurrence);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStartPeriodicTileUpdateBatch)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStartPeriodicTileUpdateBatch )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT pollingUris,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT uiUpdateRecurrence);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msClearTile)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msClearTile )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msEnableTileNotificationQueue)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msEnableTileNotificationQueue )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msPinnedSiteState)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msPinnedSiteState )( 
            __RPC__in IShellUIHelper6 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSiteState);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msEnableTileNotificationQueueForSquare150x150)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msEnableTileNotificationQueueForSquare150x150 )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msEnableTileNotificationQueueForWide310x150)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msEnableTileNotificationQueueForWide310x150 )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msEnableTileNotificationQueueForSquare310x310)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msEnableTileNotificationQueueForSquare310x310 )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msScheduledTileNotification)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msScheduledTileNotification )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR bstrNotificationXml,
            /* [in] */ __RPC__in BSTR bstrNotificationId,
            /* [in] */ __RPC__in BSTR bstrNotificationTag,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT expirationTime);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msRemoveScheduledTileNotification)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msRemoveScheduledTileNotification )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR bstrNotificationId);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStartPeriodicBadgeUpdate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStartPeriodicBadgeUpdate )( 
            __RPC__in IShellUIHelper6 * This,
            /* [in] */ __RPC__in BSTR pollingUri,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT uiUpdateRecurrence);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStopPeriodicBadgeUpdate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStopPeriodicBadgeUpdate )( 
            __RPC__in IShellUIHelper6 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msLaunchInternetOptions)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msLaunchInternetOptions )( 
            __RPC__in IShellUIHelper6 * This);
        
        END_INTERFACE
    } IShellUIHelper6Vtbl;

    interface IShellUIHelper6
    {
        CONST_VTBL struct IShellUIHelper6Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellUIHelper6_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellUIHelper6_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellUIHelper6_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellUIHelper6_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellUIHelper6_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellUIHelper6_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellUIHelper6_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellUIHelper6_ResetFirstBootMode(This)	\
    ( (This)->lpVtbl -> ResetFirstBootMode(This) ) 

#define IShellUIHelper6_ResetSafeMode(This)	\
    ( (This)->lpVtbl -> ResetSafeMode(This) ) 

#define IShellUIHelper6_RefreshOfflineDesktop(This)	\
    ( (This)->lpVtbl -> RefreshOfflineDesktop(This) ) 

#define IShellUIHelper6_AddFavorite(This,URL,Title)	\
    ( (This)->lpVtbl -> AddFavorite(This,URL,Title) ) 

#define IShellUIHelper6_AddChannel(This,URL)	\
    ( (This)->lpVtbl -> AddChannel(This,URL) ) 

#define IShellUIHelper6_AddDesktopComponent(This,URL,Type,Left,Top,Width,Height)	\
    ( (This)->lpVtbl -> AddDesktopComponent(This,URL,Type,Left,Top,Width,Height) ) 

#define IShellUIHelper6_IsSubscribed(This,URL,pBool)	\
    ( (This)->lpVtbl -> IsSubscribed(This,URL,pBool) ) 

#define IShellUIHelper6_NavigateAndFind(This,URL,strQuery,varTargetFrame)	\
    ( (This)->lpVtbl -> NavigateAndFind(This,URL,strQuery,varTargetFrame) ) 

#define IShellUIHelper6_ImportExportFavorites(This,fImport,strImpExpPath)	\
    ( (This)->lpVtbl -> ImportExportFavorites(This,fImport,strImpExpPath) ) 

#define IShellUIHelper6_AutoCompleteSaveForm(This,Form)	\
    ( (This)->lpVtbl -> AutoCompleteSaveForm(This,Form) ) 

#define IShellUIHelper6_AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame)	\
    ( (This)->lpVtbl -> AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame) ) 

#define IShellUIHelper6_AutoCompleteAttach(This,Reserved)	\
    ( (This)->lpVtbl -> AutoCompleteAttach(This,Reserved) ) 

#define IShellUIHelper6_ShowBrowserUI(This,bstrName,pvarIn,pvarOut)	\
    ( (This)->lpVtbl -> ShowBrowserUI(This,bstrName,pvarIn,pvarOut) ) 


#define IShellUIHelper6_AddSearchProvider(This,URL)	\
    ( (This)->lpVtbl -> AddSearchProvider(This,URL) ) 

#define IShellUIHelper6_RunOnceShown(This)	\
    ( (This)->lpVtbl -> RunOnceShown(This) ) 

#define IShellUIHelper6_SkipRunOnce(This)	\
    ( (This)->lpVtbl -> SkipRunOnce(This) ) 

#define IShellUIHelper6_CustomizeSettings(This,fSQM,fPhishing,bstrLocale)	\
    ( (This)->lpVtbl -> CustomizeSettings(This,fSQM,fPhishing,bstrLocale) ) 

#define IShellUIHelper6_SqmEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> SqmEnabled(This,pfEnabled) ) 

#define IShellUIHelper6_PhishingEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> PhishingEnabled(This,pfEnabled) ) 

#define IShellUIHelper6_BrandImageUri(This,pbstrUri)	\
    ( (This)->lpVtbl -> BrandImageUri(This,pbstrUri) ) 

#define IShellUIHelper6_SkipTabsWelcome(This)	\
    ( (This)->lpVtbl -> SkipTabsWelcome(This) ) 

#define IShellUIHelper6_DiagnoseConnection(This)	\
    ( (This)->lpVtbl -> DiagnoseConnection(This) ) 

#define IShellUIHelper6_CustomizeClearType(This,fSet)	\
    ( (This)->lpVtbl -> CustomizeClearType(This,fSet) ) 

#define IShellUIHelper6_IsSearchProviderInstalled(This,URL,pdwResult)	\
    ( (This)->lpVtbl -> IsSearchProviderInstalled(This,URL,pdwResult) ) 

#define IShellUIHelper6_IsSearchMigrated(This,pfMigrated)	\
    ( (This)->lpVtbl -> IsSearchMigrated(This,pfMigrated) ) 

#define IShellUIHelper6_DefaultSearchProvider(This,pbstrName)	\
    ( (This)->lpVtbl -> DefaultSearchProvider(This,pbstrName) ) 

#define IShellUIHelper6_RunOnceRequiredSettingsComplete(This,fComplete)	\
    ( (This)->lpVtbl -> RunOnceRequiredSettingsComplete(This,fComplete) ) 

#define IShellUIHelper6_RunOnceHasShown(This,pfShown)	\
    ( (This)->lpVtbl -> RunOnceHasShown(This,pfShown) ) 

#define IShellUIHelper6_SearchGuideUrl(This,pbstrUrl)	\
    ( (This)->lpVtbl -> SearchGuideUrl(This,pbstrUrl) ) 


#define IShellUIHelper6_AddService(This,URL)	\
    ( (This)->lpVtbl -> AddService(This,URL) ) 

#define IShellUIHelper6_IsServiceInstalled(This,URL,Verb,pdwResult)	\
    ( (This)->lpVtbl -> IsServiceInstalled(This,URL,Verb,pdwResult) ) 

#define IShellUIHelper6_InPrivateFilteringEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> InPrivateFilteringEnabled(This,pfEnabled) ) 

#define IShellUIHelper6_AddToFavoritesBar(This,URL,Title,Type)	\
    ( (This)->lpVtbl -> AddToFavoritesBar(This,URL,Title,Type) ) 

#define IShellUIHelper6_BuildNewTabPage(This)	\
    ( (This)->lpVtbl -> BuildNewTabPage(This) ) 

#define IShellUIHelper6_SetRecentlyClosedVisible(This,fVisible)	\
    ( (This)->lpVtbl -> SetRecentlyClosedVisible(This,fVisible) ) 

#define IShellUIHelper6_SetActivitiesVisible(This,fVisible)	\
    ( (This)->lpVtbl -> SetActivitiesVisible(This,fVisible) ) 

#define IShellUIHelper6_ContentDiscoveryReset(This)	\
    ( (This)->lpVtbl -> ContentDiscoveryReset(This) ) 

#define IShellUIHelper6_IsSuggestedSitesEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> IsSuggestedSitesEnabled(This,pfEnabled) ) 

#define IShellUIHelper6_EnableSuggestedSites(This,fEnable)	\
    ( (This)->lpVtbl -> EnableSuggestedSites(This,fEnable) ) 

#define IShellUIHelper6_NavigateToSuggestedSites(This,bstrRelativeUrl)	\
    ( (This)->lpVtbl -> NavigateToSuggestedSites(This,bstrRelativeUrl) ) 

#define IShellUIHelper6_ShowTabsHelp(This)	\
    ( (This)->lpVtbl -> ShowTabsHelp(This) ) 

#define IShellUIHelper6_ShowInPrivateHelp(This)	\
    ( (This)->lpVtbl -> ShowInPrivateHelp(This) ) 


#define IShellUIHelper6_msIsSiteMode(This,pfSiteMode)	\
    ( (This)->lpVtbl -> msIsSiteMode(This,pfSiteMode) ) 

#define IShellUIHelper6_msSiteModeShowThumbBar(This)	\
    ( (This)->lpVtbl -> msSiteModeShowThumbBar(This) ) 

#define IShellUIHelper6_msSiteModeAddThumbBarButton(This,bstrIconURL,bstrTooltip,pvarButtonID)	\
    ( (This)->lpVtbl -> msSiteModeAddThumbBarButton(This,bstrIconURL,bstrTooltip,pvarButtonID) ) 

#define IShellUIHelper6_msSiteModeUpdateThumbBarButton(This,ButtonID,fEnabled,fVisible)	\
    ( (This)->lpVtbl -> msSiteModeUpdateThumbBarButton(This,ButtonID,fEnabled,fVisible) ) 

#define IShellUIHelper6_msSiteModeSetIconOverlay(This,IconUrl,pvarDescription)	\
    ( (This)->lpVtbl -> msSiteModeSetIconOverlay(This,IconUrl,pvarDescription) ) 

#define IShellUIHelper6_msSiteModeClearIconOverlay(This)	\
    ( (This)->lpVtbl -> msSiteModeClearIconOverlay(This) ) 

#define IShellUIHelper6_msAddSiteMode(This)	\
    ( (This)->lpVtbl -> msAddSiteMode(This) ) 

#define IShellUIHelper6_msSiteModeCreateJumpList(This,bstrHeader)	\
    ( (This)->lpVtbl -> msSiteModeCreateJumpList(This,bstrHeader) ) 

#define IShellUIHelper6_msSiteModeAddJumpListItem(This,bstrName,bstrActionUri,bstrIconUri,pvarWindowType)	\
    ( (This)->lpVtbl -> msSiteModeAddJumpListItem(This,bstrName,bstrActionUri,bstrIconUri,pvarWindowType) ) 

#define IShellUIHelper6_msSiteModeClearJumpList(This)	\
    ( (This)->lpVtbl -> msSiteModeClearJumpList(This) ) 

#define IShellUIHelper6_msSiteModeShowJumpList(This)	\
    ( (This)->lpVtbl -> msSiteModeShowJumpList(This) ) 

#define IShellUIHelper6_msSiteModeAddButtonStyle(This,uiButtonID,bstrIconUrl,bstrTooltip,pvarStyleID)	\
    ( (This)->lpVtbl -> msSiteModeAddButtonStyle(This,uiButtonID,bstrIconUrl,bstrTooltip,pvarStyleID) ) 

#define IShellUIHelper6_msSiteModeShowButtonStyle(This,uiButtonID,uiStyleID)	\
    ( (This)->lpVtbl -> msSiteModeShowButtonStyle(This,uiButtonID,uiStyleID) ) 

#define IShellUIHelper6_msSiteModeActivate(This)	\
    ( (This)->lpVtbl -> msSiteModeActivate(This) ) 

#define IShellUIHelper6_msIsSiteModeFirstRun(This,fPreserveState,puiFirstRun)	\
    ( (This)->lpVtbl -> msIsSiteModeFirstRun(This,fPreserveState,puiFirstRun) ) 

#define IShellUIHelper6_msAddTrackingProtectionList(This,URL,bstrFilterName)	\
    ( (This)->lpVtbl -> msAddTrackingProtectionList(This,URL,bstrFilterName) ) 

#define IShellUIHelper6_msTrackingProtectionEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> msTrackingProtectionEnabled(This,pfEnabled) ) 

#define IShellUIHelper6_msActiveXFilteringEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> msActiveXFilteringEnabled(This,pfEnabled) ) 


#define IShellUIHelper6_msProvisionNetworks(This,bstrProvisioningXml,puiResult)	\
    ( (This)->lpVtbl -> msProvisionNetworks(This,bstrProvisioningXml,puiResult) ) 

#define IShellUIHelper6_msReportSafeUrl(This)	\
    ( (This)->lpVtbl -> msReportSafeUrl(This) ) 

#define IShellUIHelper6_msSiteModeRefreshBadge(This)	\
    ( (This)->lpVtbl -> msSiteModeRefreshBadge(This) ) 

#define IShellUIHelper6_msSiteModeClearBadge(This)	\
    ( (This)->lpVtbl -> msSiteModeClearBadge(This) ) 

#define IShellUIHelper6_msDiagnoseConnectionUILess(This)	\
    ( (This)->lpVtbl -> msDiagnoseConnectionUILess(This) ) 

#define IShellUIHelper6_msLaunchNetworkClientHelp(This)	\
    ( (This)->lpVtbl -> msLaunchNetworkClientHelp(This) ) 

#define IShellUIHelper6_msChangeDefaultBrowser(This,fChange)	\
    ( (This)->lpVtbl -> msChangeDefaultBrowser(This,fChange) ) 


#define IShellUIHelper6_msStopPeriodicTileUpdate(This)	\
    ( (This)->lpVtbl -> msStopPeriodicTileUpdate(This) ) 

#define IShellUIHelper6_msStartPeriodicTileUpdate(This,pollingUris,startTime,uiUpdateRecurrence)	\
    ( (This)->lpVtbl -> msStartPeriodicTileUpdate(This,pollingUris,startTime,uiUpdateRecurrence) ) 

#define IShellUIHelper6_msStartPeriodicTileUpdateBatch(This,pollingUris,startTime,uiUpdateRecurrence)	\
    ( (This)->lpVtbl -> msStartPeriodicTileUpdateBatch(This,pollingUris,startTime,uiUpdateRecurrence) ) 

#define IShellUIHelper6_msClearTile(This)	\
    ( (This)->lpVtbl -> msClearTile(This) ) 

#define IShellUIHelper6_msEnableTileNotificationQueue(This,fChange)	\
    ( (This)->lpVtbl -> msEnableTileNotificationQueue(This,fChange) ) 

#define IShellUIHelper6_msPinnedSiteState(This,pvarSiteState)	\
    ( (This)->lpVtbl -> msPinnedSiteState(This,pvarSiteState) ) 

#define IShellUIHelper6_msEnableTileNotificationQueueForSquare150x150(This,fChange)	\
    ( (This)->lpVtbl -> msEnableTileNotificationQueueForSquare150x150(This,fChange) ) 

#define IShellUIHelper6_msEnableTileNotificationQueueForWide310x150(This,fChange)	\
    ( (This)->lpVtbl -> msEnableTileNotificationQueueForWide310x150(This,fChange) ) 

#define IShellUIHelper6_msEnableTileNotificationQueueForSquare310x310(This,fChange)	\
    ( (This)->lpVtbl -> msEnableTileNotificationQueueForSquare310x310(This,fChange) ) 

#define IShellUIHelper6_msScheduledTileNotification(This,bstrNotificationXml,bstrNotificationId,bstrNotificationTag,startTime,expirationTime)	\
    ( (This)->lpVtbl -> msScheduledTileNotification(This,bstrNotificationXml,bstrNotificationId,bstrNotificationTag,startTime,expirationTime) ) 

#define IShellUIHelper6_msRemoveScheduledTileNotification(This,bstrNotificationId)	\
    ( (This)->lpVtbl -> msRemoveScheduledTileNotification(This,bstrNotificationId) ) 

#define IShellUIHelper6_msStartPeriodicBadgeUpdate(This,pollingUri,startTime,uiUpdateRecurrence)	\
    ( (This)->lpVtbl -> msStartPeriodicBadgeUpdate(This,pollingUri,startTime,uiUpdateRecurrence) ) 

#define IShellUIHelper6_msStopPeriodicBadgeUpdate(This)	\
    ( (This)->lpVtbl -> msStopPeriodicBadgeUpdate(This) ) 

#define IShellUIHelper6_msLaunchInternetOptions(This)	\
    ( (This)->lpVtbl -> msLaunchInternetOptions(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellUIHelper6_INTERFACE_DEFINED__ */


#ifndef __IShellUIHelper7_INTERFACE_DEFINED__
#define __IShellUIHelper7_INTERFACE_DEFINED__

/* interface IShellUIHelper7 */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellUIHelper7;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("60E567C8-9573-4AB2-A264-637C6C161CB1")
    IShellUIHelper7 : public IShellUIHelper6
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetExperimentalFlag( 
            /* [in] */ __RPC__in BSTR bstrFlagString,
            /* [in] */ VARIANT_BOOL vfFlag) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetExperimentalFlag( 
            /* [in] */ __RPC__in BSTR bstrFlagString,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *vfFlag) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetExperimentalValue( 
            /* [in] */ __RPC__in BSTR bstrValueString,
            /* [in] */ DWORD dwValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetExperimentalValue( 
            /* [in] */ __RPC__in BSTR bstrValueString,
            /* [retval][out] */ __RPC__out DWORD *pdwValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ResetAllExperimentalFlagsAndValues( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetNeedIEAutoLaunchFlag( 
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *flag) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetNeedIEAutoLaunchFlag( 
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [in] */ VARIANT_BOOL flag) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE HasNeedIEAutoLaunchFlag( 
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *exists) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE LaunchIE( 
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [in] */ VARIANT_BOOL automated) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellUIHelper7Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellUIHelper7 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellUIHelper7 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetFirstBootMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetFirstBootMode )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetSafeMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetSafeMode )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, RefreshOfflineDesktop)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *RefreshOfflineDesktop )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddFavorite)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddFavorite )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Title);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddChannel)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddChannel )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddDesktopComponent)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddDesktopComponent )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Type,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Left,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Top,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Width,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Height);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, IsSubscribed)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSubscribed )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, NavigateAndFind)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NavigateAndFind )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR strQuery,
            /* [in] */ __RPC__in VARIANT *varTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ImportExportFavorites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ImportExportFavorites )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT_BOOL fImport,
            /* [in] */ __RPC__in BSTR strImpExpPath);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteSaveForm)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteSaveForm )( 
            __RPC__in IShellUIHelper7 * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Form);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoScan)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoScan )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR strSearch,
            /* [in] */ __RPC__in BSTR strFailureUrl,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *pvarTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteAttach)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteAttach )( 
            __RPC__in IShellUIHelper7 * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Reserved);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ShowBrowserUI)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowBrowserUI )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in VARIANT *pvarIn,
            /* [retval][out] */ __RPC__out VARIANT *pvarOut);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, AddSearchProvider)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddSearchProvider )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceShown)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceShown )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SkipRunOnce)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SkipRunOnce )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, CustomizeSettings)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CustomizeSettings )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT_BOOL fSQM,
            /* [in] */ VARIANT_BOOL fPhishing,
            /* [in] */ __RPC__in BSTR bstrLocale);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SqmEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SqmEnabled )( 
            __RPC__in IShellUIHelper7 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, PhishingEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PhishingEnabled )( 
            __RPC__in IShellUIHelper7 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, BrandImageUri)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BrandImageUri )( 
            __RPC__in IShellUIHelper7 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUri);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SkipTabsWelcome)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SkipTabsWelcome )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, DiagnoseConnection)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DiagnoseConnection )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, CustomizeClearType)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CustomizeClearType )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT_BOOL fSet);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, IsSearchProviderInstalled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSearchProviderInstalled )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, IsSearchMigrated)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSearchMigrated )( 
            __RPC__in IShellUIHelper7 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfMigrated);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, DefaultSearchProvider)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DefaultSearchProvider )( 
            __RPC__in IShellUIHelper7 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceRequiredSettingsComplete)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceRequiredSettingsComplete )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT_BOOL fComplete);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceHasShown)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceHasShown )( 
            __RPC__in IShellUIHelper7 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfShown);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SearchGuideUrl)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SearchGuideUrl )( 
            __RPC__in IShellUIHelper7 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUrl);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, AddService)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddService )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, IsServiceInstalled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsServiceInstalled )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Verb,
            /* [retval][out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, InPrivateFilteringEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InPrivateFilteringEnabled )( 
            __RPC__in IShellUIHelper7 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, AddToFavoritesBar)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddToFavoritesBar )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Title,
            /* [in][optional] */ __RPC__in VARIANT *Type);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, BuildNewTabPage)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BuildNewTabPage )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, SetRecentlyClosedVisible)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetRecentlyClosedVisible )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, SetActivitiesVisible)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetActivitiesVisible )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ContentDiscoveryReset)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ContentDiscoveryReset )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, IsSuggestedSitesEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSuggestedSitesEnabled )( 
            __RPC__in IShellUIHelper7 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, EnableSuggestedSites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnableSuggestedSites )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT_BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, NavigateToSuggestedSites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NavigateToSuggestedSites )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR bstrRelativeUrl);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ShowTabsHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowTabsHelp )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ShowInPrivateHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowInPrivateHelp )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msIsSiteMode)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msIsSiteMode )( 
            __RPC__in IShellUIHelper7 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfSiteMode);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowThumbBar)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowThumbBar )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddThumbBarButton)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddThumbBarButton )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR bstrIconURL,
            /* [in] */ __RPC__in BSTR bstrTooltip,
            /* [retval][out] */ __RPC__out VARIANT *pvarButtonID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeUpdateThumbBarButton)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeUpdateThumbBarButton )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT ButtonID,
            /* [in] */ VARIANT_BOOL fEnabled,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeSetIconOverlay)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeSetIconOverlay )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR IconUrl,
            /* [in][optional] */ __RPC__in VARIANT *pvarDescription);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeClearIconOverlay)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeClearIconOverlay )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msAddSiteMode)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msAddSiteMode )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeCreateJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeCreateJumpList )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR bstrHeader);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddJumpListItem)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddJumpListItem )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in BSTR bstrActionUri,
            /* [in] */ __RPC__in BSTR bstrIconUri,
            /* [in][optional] */ __RPC__in VARIANT *pvarWindowType);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeClearJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeClearJumpList )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowJumpList )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddButtonStyle)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddButtonStyle )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT uiButtonID,
            /* [in] */ __RPC__in BSTR bstrIconUrl,
            /* [in] */ __RPC__in BSTR bstrTooltip,
            /* [retval][out] */ __RPC__out VARIANT *pvarStyleID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowButtonStyle)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowButtonStyle )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT uiButtonID,
            /* [in] */ VARIANT uiStyleID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeActivate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeActivate )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msIsSiteModeFirstRun)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msIsSiteModeFirstRun )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT_BOOL fPreserveState,
            /* [retval][out] */ __RPC__out VARIANT *puiFirstRun);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msAddTrackingProtectionList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msAddTrackingProtectionList )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR bstrFilterName);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msTrackingProtectionEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msTrackingProtectionEnabled )( 
            __RPC__in IShellUIHelper7 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msActiveXFilteringEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msActiveXFilteringEnabled )( 
            __RPC__in IShellUIHelper7 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msProvisionNetworks)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msProvisionNetworks )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR bstrProvisioningXml,
            /* [retval][out] */ __RPC__out VARIANT *puiResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msReportSafeUrl)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msReportSafeUrl )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msSiteModeRefreshBadge)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeRefreshBadge )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msSiteModeClearBadge)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeClearBadge )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msDiagnoseConnectionUILess)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msDiagnoseConnectionUILess )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msLaunchNetworkClientHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msLaunchNetworkClientHelp )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msChangeDefaultBrowser)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msChangeDefaultBrowser )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStopPeriodicTileUpdate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStopPeriodicTileUpdate )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStartPeriodicTileUpdate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStartPeriodicTileUpdate )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT pollingUris,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT uiUpdateRecurrence);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStartPeriodicTileUpdateBatch)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStartPeriodicTileUpdateBatch )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT pollingUris,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT uiUpdateRecurrence);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msClearTile)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msClearTile )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msEnableTileNotificationQueue)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msEnableTileNotificationQueue )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msPinnedSiteState)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msPinnedSiteState )( 
            __RPC__in IShellUIHelper7 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSiteState);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msEnableTileNotificationQueueForSquare150x150)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msEnableTileNotificationQueueForSquare150x150 )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msEnableTileNotificationQueueForWide310x150)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msEnableTileNotificationQueueForWide310x150 )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msEnableTileNotificationQueueForSquare310x310)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msEnableTileNotificationQueueForSquare310x310 )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msScheduledTileNotification)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msScheduledTileNotification )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR bstrNotificationXml,
            /* [in] */ __RPC__in BSTR bstrNotificationId,
            /* [in] */ __RPC__in BSTR bstrNotificationTag,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT expirationTime);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msRemoveScheduledTileNotification)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msRemoveScheduledTileNotification )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR bstrNotificationId);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStartPeriodicBadgeUpdate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStartPeriodicBadgeUpdate )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR pollingUri,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT uiUpdateRecurrence);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStopPeriodicBadgeUpdate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStopPeriodicBadgeUpdate )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msLaunchInternetOptions)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msLaunchInternetOptions )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, SetExperimentalFlag)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetExperimentalFlag )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR bstrFlagString,
            /* [in] */ VARIANT_BOOL vfFlag);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, GetExperimentalFlag)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetExperimentalFlag )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR bstrFlagString,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *vfFlag);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, SetExperimentalValue)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetExperimentalValue )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR bstrValueString,
            /* [in] */ DWORD dwValue);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, GetExperimentalValue)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetExperimentalValue )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR bstrValueString,
            /* [retval][out] */ __RPC__out DWORD *pdwValue);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, ResetAllExperimentalFlagsAndValues)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ResetAllExperimentalFlagsAndValues )( 
            __RPC__in IShellUIHelper7 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, GetNeedIEAutoLaunchFlag)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetNeedIEAutoLaunchFlag )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *flag);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, SetNeedIEAutoLaunchFlag)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetNeedIEAutoLaunchFlag )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [in] */ VARIANT_BOOL flag);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, HasNeedIEAutoLaunchFlag)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *HasNeedIEAutoLaunchFlag )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *exists);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, LaunchIE)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *LaunchIE )( 
            __RPC__in IShellUIHelper7 * This,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [in] */ VARIANT_BOOL automated);
        
        END_INTERFACE
    } IShellUIHelper7Vtbl;

    interface IShellUIHelper7
    {
        CONST_VTBL struct IShellUIHelper7Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellUIHelper7_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellUIHelper7_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellUIHelper7_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellUIHelper7_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellUIHelper7_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellUIHelper7_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellUIHelper7_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellUIHelper7_ResetFirstBootMode(This)	\
    ( (This)->lpVtbl -> ResetFirstBootMode(This) ) 

#define IShellUIHelper7_ResetSafeMode(This)	\
    ( (This)->lpVtbl -> ResetSafeMode(This) ) 

#define IShellUIHelper7_RefreshOfflineDesktop(This)	\
    ( (This)->lpVtbl -> RefreshOfflineDesktop(This) ) 

#define IShellUIHelper7_AddFavorite(This,URL,Title)	\
    ( (This)->lpVtbl -> AddFavorite(This,URL,Title) ) 

#define IShellUIHelper7_AddChannel(This,URL)	\
    ( (This)->lpVtbl -> AddChannel(This,URL) ) 

#define IShellUIHelper7_AddDesktopComponent(This,URL,Type,Left,Top,Width,Height)	\
    ( (This)->lpVtbl -> AddDesktopComponent(This,URL,Type,Left,Top,Width,Height) ) 

#define IShellUIHelper7_IsSubscribed(This,URL,pBool)	\
    ( (This)->lpVtbl -> IsSubscribed(This,URL,pBool) ) 

#define IShellUIHelper7_NavigateAndFind(This,URL,strQuery,varTargetFrame)	\
    ( (This)->lpVtbl -> NavigateAndFind(This,URL,strQuery,varTargetFrame) ) 

#define IShellUIHelper7_ImportExportFavorites(This,fImport,strImpExpPath)	\
    ( (This)->lpVtbl -> ImportExportFavorites(This,fImport,strImpExpPath) ) 

#define IShellUIHelper7_AutoCompleteSaveForm(This,Form)	\
    ( (This)->lpVtbl -> AutoCompleteSaveForm(This,Form) ) 

#define IShellUIHelper7_AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame)	\
    ( (This)->lpVtbl -> AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame) ) 

#define IShellUIHelper7_AutoCompleteAttach(This,Reserved)	\
    ( (This)->lpVtbl -> AutoCompleteAttach(This,Reserved) ) 

#define IShellUIHelper7_ShowBrowserUI(This,bstrName,pvarIn,pvarOut)	\
    ( (This)->lpVtbl -> ShowBrowserUI(This,bstrName,pvarIn,pvarOut) ) 


#define IShellUIHelper7_AddSearchProvider(This,URL)	\
    ( (This)->lpVtbl -> AddSearchProvider(This,URL) ) 

#define IShellUIHelper7_RunOnceShown(This)	\
    ( (This)->lpVtbl -> RunOnceShown(This) ) 

#define IShellUIHelper7_SkipRunOnce(This)	\
    ( (This)->lpVtbl -> SkipRunOnce(This) ) 

#define IShellUIHelper7_CustomizeSettings(This,fSQM,fPhishing,bstrLocale)	\
    ( (This)->lpVtbl -> CustomizeSettings(This,fSQM,fPhishing,bstrLocale) ) 

#define IShellUIHelper7_SqmEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> SqmEnabled(This,pfEnabled) ) 

#define IShellUIHelper7_PhishingEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> PhishingEnabled(This,pfEnabled) ) 

#define IShellUIHelper7_BrandImageUri(This,pbstrUri)	\
    ( (This)->lpVtbl -> BrandImageUri(This,pbstrUri) ) 

#define IShellUIHelper7_SkipTabsWelcome(This)	\
    ( (This)->lpVtbl -> SkipTabsWelcome(This) ) 

#define IShellUIHelper7_DiagnoseConnection(This)	\
    ( (This)->lpVtbl -> DiagnoseConnection(This) ) 

#define IShellUIHelper7_CustomizeClearType(This,fSet)	\
    ( (This)->lpVtbl -> CustomizeClearType(This,fSet) ) 

#define IShellUIHelper7_IsSearchProviderInstalled(This,URL,pdwResult)	\
    ( (This)->lpVtbl -> IsSearchProviderInstalled(This,URL,pdwResult) ) 

#define IShellUIHelper7_IsSearchMigrated(This,pfMigrated)	\
    ( (This)->lpVtbl -> IsSearchMigrated(This,pfMigrated) ) 

#define IShellUIHelper7_DefaultSearchProvider(This,pbstrName)	\
    ( (This)->lpVtbl -> DefaultSearchProvider(This,pbstrName) ) 

#define IShellUIHelper7_RunOnceRequiredSettingsComplete(This,fComplete)	\
    ( (This)->lpVtbl -> RunOnceRequiredSettingsComplete(This,fComplete) ) 

#define IShellUIHelper7_RunOnceHasShown(This,pfShown)	\
    ( (This)->lpVtbl -> RunOnceHasShown(This,pfShown) ) 

#define IShellUIHelper7_SearchGuideUrl(This,pbstrUrl)	\
    ( (This)->lpVtbl -> SearchGuideUrl(This,pbstrUrl) ) 


#define IShellUIHelper7_AddService(This,URL)	\
    ( (This)->lpVtbl -> AddService(This,URL) ) 

#define IShellUIHelper7_IsServiceInstalled(This,URL,Verb,pdwResult)	\
    ( (This)->lpVtbl -> IsServiceInstalled(This,URL,Verb,pdwResult) ) 

#define IShellUIHelper7_InPrivateFilteringEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> InPrivateFilteringEnabled(This,pfEnabled) ) 

#define IShellUIHelper7_AddToFavoritesBar(This,URL,Title,Type)	\
    ( (This)->lpVtbl -> AddToFavoritesBar(This,URL,Title,Type) ) 

#define IShellUIHelper7_BuildNewTabPage(This)	\
    ( (This)->lpVtbl -> BuildNewTabPage(This) ) 

#define IShellUIHelper7_SetRecentlyClosedVisible(This,fVisible)	\
    ( (This)->lpVtbl -> SetRecentlyClosedVisible(This,fVisible) ) 

#define IShellUIHelper7_SetActivitiesVisible(This,fVisible)	\
    ( (This)->lpVtbl -> SetActivitiesVisible(This,fVisible) ) 

#define IShellUIHelper7_ContentDiscoveryReset(This)	\
    ( (This)->lpVtbl -> ContentDiscoveryReset(This) ) 

#define IShellUIHelper7_IsSuggestedSitesEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> IsSuggestedSitesEnabled(This,pfEnabled) ) 

#define IShellUIHelper7_EnableSuggestedSites(This,fEnable)	\
    ( (This)->lpVtbl -> EnableSuggestedSites(This,fEnable) ) 

#define IShellUIHelper7_NavigateToSuggestedSites(This,bstrRelativeUrl)	\
    ( (This)->lpVtbl -> NavigateToSuggestedSites(This,bstrRelativeUrl) ) 

#define IShellUIHelper7_ShowTabsHelp(This)	\
    ( (This)->lpVtbl -> ShowTabsHelp(This) ) 

#define IShellUIHelper7_ShowInPrivateHelp(This)	\
    ( (This)->lpVtbl -> ShowInPrivateHelp(This) ) 


#define IShellUIHelper7_msIsSiteMode(This,pfSiteMode)	\
    ( (This)->lpVtbl -> msIsSiteMode(This,pfSiteMode) ) 

#define IShellUIHelper7_msSiteModeShowThumbBar(This)	\
    ( (This)->lpVtbl -> msSiteModeShowThumbBar(This) ) 

#define IShellUIHelper7_msSiteModeAddThumbBarButton(This,bstrIconURL,bstrTooltip,pvarButtonID)	\
    ( (This)->lpVtbl -> msSiteModeAddThumbBarButton(This,bstrIconURL,bstrTooltip,pvarButtonID) ) 

#define IShellUIHelper7_msSiteModeUpdateThumbBarButton(This,ButtonID,fEnabled,fVisible)	\
    ( (This)->lpVtbl -> msSiteModeUpdateThumbBarButton(This,ButtonID,fEnabled,fVisible) ) 

#define IShellUIHelper7_msSiteModeSetIconOverlay(This,IconUrl,pvarDescription)	\
    ( (This)->lpVtbl -> msSiteModeSetIconOverlay(This,IconUrl,pvarDescription) ) 

#define IShellUIHelper7_msSiteModeClearIconOverlay(This)	\
    ( (This)->lpVtbl -> msSiteModeClearIconOverlay(This) ) 

#define IShellUIHelper7_msAddSiteMode(This)	\
    ( (This)->lpVtbl -> msAddSiteMode(This) ) 

#define IShellUIHelper7_msSiteModeCreateJumpList(This,bstrHeader)	\
    ( (This)->lpVtbl -> msSiteModeCreateJumpList(This,bstrHeader) ) 

#define IShellUIHelper7_msSiteModeAddJumpListItem(This,bstrName,bstrActionUri,bstrIconUri,pvarWindowType)	\
    ( (This)->lpVtbl -> msSiteModeAddJumpListItem(This,bstrName,bstrActionUri,bstrIconUri,pvarWindowType) ) 

#define IShellUIHelper7_msSiteModeClearJumpList(This)	\
    ( (This)->lpVtbl -> msSiteModeClearJumpList(This) ) 

#define IShellUIHelper7_msSiteModeShowJumpList(This)	\
    ( (This)->lpVtbl -> msSiteModeShowJumpList(This) ) 

#define IShellUIHelper7_msSiteModeAddButtonStyle(This,uiButtonID,bstrIconUrl,bstrTooltip,pvarStyleID)	\
    ( (This)->lpVtbl -> msSiteModeAddButtonStyle(This,uiButtonID,bstrIconUrl,bstrTooltip,pvarStyleID) ) 

#define IShellUIHelper7_msSiteModeShowButtonStyle(This,uiButtonID,uiStyleID)	\
    ( (This)->lpVtbl -> msSiteModeShowButtonStyle(This,uiButtonID,uiStyleID) ) 

#define IShellUIHelper7_msSiteModeActivate(This)	\
    ( (This)->lpVtbl -> msSiteModeActivate(This) ) 

#define IShellUIHelper7_msIsSiteModeFirstRun(This,fPreserveState,puiFirstRun)	\
    ( (This)->lpVtbl -> msIsSiteModeFirstRun(This,fPreserveState,puiFirstRun) ) 

#define IShellUIHelper7_msAddTrackingProtectionList(This,URL,bstrFilterName)	\
    ( (This)->lpVtbl -> msAddTrackingProtectionList(This,URL,bstrFilterName) ) 

#define IShellUIHelper7_msTrackingProtectionEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> msTrackingProtectionEnabled(This,pfEnabled) ) 

#define IShellUIHelper7_msActiveXFilteringEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> msActiveXFilteringEnabled(This,pfEnabled) ) 


#define IShellUIHelper7_msProvisionNetworks(This,bstrProvisioningXml,puiResult)	\
    ( (This)->lpVtbl -> msProvisionNetworks(This,bstrProvisioningXml,puiResult) ) 

#define IShellUIHelper7_msReportSafeUrl(This)	\
    ( (This)->lpVtbl -> msReportSafeUrl(This) ) 

#define IShellUIHelper7_msSiteModeRefreshBadge(This)	\
    ( (This)->lpVtbl -> msSiteModeRefreshBadge(This) ) 

#define IShellUIHelper7_msSiteModeClearBadge(This)	\
    ( (This)->lpVtbl -> msSiteModeClearBadge(This) ) 

#define IShellUIHelper7_msDiagnoseConnectionUILess(This)	\
    ( (This)->lpVtbl -> msDiagnoseConnectionUILess(This) ) 

#define IShellUIHelper7_msLaunchNetworkClientHelp(This)	\
    ( (This)->lpVtbl -> msLaunchNetworkClientHelp(This) ) 

#define IShellUIHelper7_msChangeDefaultBrowser(This,fChange)	\
    ( (This)->lpVtbl -> msChangeDefaultBrowser(This,fChange) ) 


#define IShellUIHelper7_msStopPeriodicTileUpdate(This)	\
    ( (This)->lpVtbl -> msStopPeriodicTileUpdate(This) ) 

#define IShellUIHelper7_msStartPeriodicTileUpdate(This,pollingUris,startTime,uiUpdateRecurrence)	\
    ( (This)->lpVtbl -> msStartPeriodicTileUpdate(This,pollingUris,startTime,uiUpdateRecurrence) ) 

#define IShellUIHelper7_msStartPeriodicTileUpdateBatch(This,pollingUris,startTime,uiUpdateRecurrence)	\
    ( (This)->lpVtbl -> msStartPeriodicTileUpdateBatch(This,pollingUris,startTime,uiUpdateRecurrence) ) 

#define IShellUIHelper7_msClearTile(This)	\
    ( (This)->lpVtbl -> msClearTile(This) ) 

#define IShellUIHelper7_msEnableTileNotificationQueue(This,fChange)	\
    ( (This)->lpVtbl -> msEnableTileNotificationQueue(This,fChange) ) 

#define IShellUIHelper7_msPinnedSiteState(This,pvarSiteState)	\
    ( (This)->lpVtbl -> msPinnedSiteState(This,pvarSiteState) ) 

#define IShellUIHelper7_msEnableTileNotificationQueueForSquare150x150(This,fChange)	\
    ( (This)->lpVtbl -> msEnableTileNotificationQueueForSquare150x150(This,fChange) ) 

#define IShellUIHelper7_msEnableTileNotificationQueueForWide310x150(This,fChange)	\
    ( (This)->lpVtbl -> msEnableTileNotificationQueueForWide310x150(This,fChange) ) 

#define IShellUIHelper7_msEnableTileNotificationQueueForSquare310x310(This,fChange)	\
    ( (This)->lpVtbl -> msEnableTileNotificationQueueForSquare310x310(This,fChange) ) 

#define IShellUIHelper7_msScheduledTileNotification(This,bstrNotificationXml,bstrNotificationId,bstrNotificationTag,startTime,expirationTime)	\
    ( (This)->lpVtbl -> msScheduledTileNotification(This,bstrNotificationXml,bstrNotificationId,bstrNotificationTag,startTime,expirationTime) ) 

#define IShellUIHelper7_msRemoveScheduledTileNotification(This,bstrNotificationId)	\
    ( (This)->lpVtbl -> msRemoveScheduledTileNotification(This,bstrNotificationId) ) 

#define IShellUIHelper7_msStartPeriodicBadgeUpdate(This,pollingUri,startTime,uiUpdateRecurrence)	\
    ( (This)->lpVtbl -> msStartPeriodicBadgeUpdate(This,pollingUri,startTime,uiUpdateRecurrence) ) 

#define IShellUIHelper7_msStopPeriodicBadgeUpdate(This)	\
    ( (This)->lpVtbl -> msStopPeriodicBadgeUpdate(This) ) 

#define IShellUIHelper7_msLaunchInternetOptions(This)	\
    ( (This)->lpVtbl -> msLaunchInternetOptions(This) ) 


#define IShellUIHelper7_SetExperimentalFlag(This,bstrFlagString,vfFlag)	\
    ( (This)->lpVtbl -> SetExperimentalFlag(This,bstrFlagString,vfFlag) ) 

#define IShellUIHelper7_GetExperimentalFlag(This,bstrFlagString,vfFlag)	\
    ( (This)->lpVtbl -> GetExperimentalFlag(This,bstrFlagString,vfFlag) ) 

#define IShellUIHelper7_SetExperimentalValue(This,bstrValueString,dwValue)	\
    ( (This)->lpVtbl -> SetExperimentalValue(This,bstrValueString,dwValue) ) 

#define IShellUIHelper7_GetExperimentalValue(This,bstrValueString,pdwValue)	\
    ( (This)->lpVtbl -> GetExperimentalValue(This,bstrValueString,pdwValue) ) 

#define IShellUIHelper7_ResetAllExperimentalFlagsAndValues(This)	\
    ( (This)->lpVtbl -> ResetAllExperimentalFlagsAndValues(This) ) 

#define IShellUIHelper7_GetNeedIEAutoLaunchFlag(This,bstrUrl,flag)	\
    ( (This)->lpVtbl -> GetNeedIEAutoLaunchFlag(This,bstrUrl,flag) ) 

#define IShellUIHelper7_SetNeedIEAutoLaunchFlag(This,bstrUrl,flag)	\
    ( (This)->lpVtbl -> SetNeedIEAutoLaunchFlag(This,bstrUrl,flag) ) 

#define IShellUIHelper7_HasNeedIEAutoLaunchFlag(This,bstrUrl,exists)	\
    ( (This)->lpVtbl -> HasNeedIEAutoLaunchFlag(This,bstrUrl,exists) ) 

#define IShellUIHelper7_LaunchIE(This,bstrUrl,automated)	\
    ( (This)->lpVtbl -> LaunchIE(This,bstrUrl,automated) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellUIHelper7_INTERFACE_DEFINED__ */


#ifndef __IShellUIHelper8_INTERFACE_DEFINED__
#define __IShellUIHelper8_INTERFACE_DEFINED__

/* interface IShellUIHelper8 */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellUIHelper8;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("66DEBCF2-05B0-4F07-B49B-B96241A65DB2")
    IShellUIHelper8 : public IShellUIHelper7
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetCVListData( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrResult) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetCVListLocalData( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrResult) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetEMIEListData( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrResult) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetEMIEListLocalData( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrResult) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE OpenFavoritesPane( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE OpenFavoritesSettings( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE LaunchInHVSI( 
            /* [in] */ __RPC__in BSTR bstrUrl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellUIHelper8Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellUIHelper8 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellUIHelper8 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetFirstBootMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetFirstBootMode )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetSafeMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetSafeMode )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, RefreshOfflineDesktop)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *RefreshOfflineDesktop )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddFavorite)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddFavorite )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Title);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddChannel)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddChannel )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddDesktopComponent)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddDesktopComponent )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Type,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Left,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Top,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Width,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Height);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, IsSubscribed)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSubscribed )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, NavigateAndFind)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NavigateAndFind )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR strQuery,
            /* [in] */ __RPC__in VARIANT *varTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ImportExportFavorites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ImportExportFavorites )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT_BOOL fImport,
            /* [in] */ __RPC__in BSTR strImpExpPath);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteSaveForm)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteSaveForm )( 
            __RPC__in IShellUIHelper8 * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Form);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoScan)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoScan )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR strSearch,
            /* [in] */ __RPC__in BSTR strFailureUrl,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *pvarTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteAttach)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteAttach )( 
            __RPC__in IShellUIHelper8 * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Reserved);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ShowBrowserUI)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowBrowserUI )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in VARIANT *pvarIn,
            /* [retval][out] */ __RPC__out VARIANT *pvarOut);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, AddSearchProvider)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddSearchProvider )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceShown)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceShown )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SkipRunOnce)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SkipRunOnce )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, CustomizeSettings)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CustomizeSettings )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT_BOOL fSQM,
            /* [in] */ VARIANT_BOOL fPhishing,
            /* [in] */ __RPC__in BSTR bstrLocale);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SqmEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SqmEnabled )( 
            __RPC__in IShellUIHelper8 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, PhishingEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PhishingEnabled )( 
            __RPC__in IShellUIHelper8 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, BrandImageUri)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BrandImageUri )( 
            __RPC__in IShellUIHelper8 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUri);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SkipTabsWelcome)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SkipTabsWelcome )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, DiagnoseConnection)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DiagnoseConnection )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, CustomizeClearType)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CustomizeClearType )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT_BOOL fSet);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, IsSearchProviderInstalled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSearchProviderInstalled )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, IsSearchMigrated)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSearchMigrated )( 
            __RPC__in IShellUIHelper8 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfMigrated);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, DefaultSearchProvider)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DefaultSearchProvider )( 
            __RPC__in IShellUIHelper8 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceRequiredSettingsComplete)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceRequiredSettingsComplete )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT_BOOL fComplete);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceHasShown)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceHasShown )( 
            __RPC__in IShellUIHelper8 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfShown);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SearchGuideUrl)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SearchGuideUrl )( 
            __RPC__in IShellUIHelper8 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUrl);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, AddService)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddService )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, IsServiceInstalled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsServiceInstalled )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Verb,
            /* [retval][out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, InPrivateFilteringEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InPrivateFilteringEnabled )( 
            __RPC__in IShellUIHelper8 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, AddToFavoritesBar)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddToFavoritesBar )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Title,
            /* [in][optional] */ __RPC__in VARIANT *Type);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, BuildNewTabPage)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BuildNewTabPage )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, SetRecentlyClosedVisible)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetRecentlyClosedVisible )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, SetActivitiesVisible)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetActivitiesVisible )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ContentDiscoveryReset)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ContentDiscoveryReset )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, IsSuggestedSitesEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSuggestedSitesEnabled )( 
            __RPC__in IShellUIHelper8 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, EnableSuggestedSites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnableSuggestedSites )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT_BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, NavigateToSuggestedSites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NavigateToSuggestedSites )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR bstrRelativeUrl);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ShowTabsHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowTabsHelp )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ShowInPrivateHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowInPrivateHelp )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msIsSiteMode)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msIsSiteMode )( 
            __RPC__in IShellUIHelper8 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfSiteMode);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowThumbBar)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowThumbBar )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddThumbBarButton)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddThumbBarButton )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR bstrIconURL,
            /* [in] */ __RPC__in BSTR bstrTooltip,
            /* [retval][out] */ __RPC__out VARIANT *pvarButtonID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeUpdateThumbBarButton)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeUpdateThumbBarButton )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT ButtonID,
            /* [in] */ VARIANT_BOOL fEnabled,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeSetIconOverlay)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeSetIconOverlay )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR IconUrl,
            /* [in][optional] */ __RPC__in VARIANT *pvarDescription);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeClearIconOverlay)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeClearIconOverlay )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msAddSiteMode)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msAddSiteMode )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeCreateJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeCreateJumpList )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR bstrHeader);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddJumpListItem)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddJumpListItem )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in BSTR bstrActionUri,
            /* [in] */ __RPC__in BSTR bstrIconUri,
            /* [in][optional] */ __RPC__in VARIANT *pvarWindowType);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeClearJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeClearJumpList )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowJumpList )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddButtonStyle)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddButtonStyle )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT uiButtonID,
            /* [in] */ __RPC__in BSTR bstrIconUrl,
            /* [in] */ __RPC__in BSTR bstrTooltip,
            /* [retval][out] */ __RPC__out VARIANT *pvarStyleID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowButtonStyle)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowButtonStyle )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT uiButtonID,
            /* [in] */ VARIANT uiStyleID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeActivate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeActivate )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msIsSiteModeFirstRun)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msIsSiteModeFirstRun )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT_BOOL fPreserveState,
            /* [retval][out] */ __RPC__out VARIANT *puiFirstRun);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msAddTrackingProtectionList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msAddTrackingProtectionList )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR bstrFilterName);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msTrackingProtectionEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msTrackingProtectionEnabled )( 
            __RPC__in IShellUIHelper8 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msActiveXFilteringEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msActiveXFilteringEnabled )( 
            __RPC__in IShellUIHelper8 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msProvisionNetworks)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msProvisionNetworks )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR bstrProvisioningXml,
            /* [retval][out] */ __RPC__out VARIANT *puiResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msReportSafeUrl)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msReportSafeUrl )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msSiteModeRefreshBadge)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeRefreshBadge )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msSiteModeClearBadge)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeClearBadge )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msDiagnoseConnectionUILess)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msDiagnoseConnectionUILess )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msLaunchNetworkClientHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msLaunchNetworkClientHelp )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msChangeDefaultBrowser)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msChangeDefaultBrowser )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStopPeriodicTileUpdate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStopPeriodicTileUpdate )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStartPeriodicTileUpdate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStartPeriodicTileUpdate )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT pollingUris,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT uiUpdateRecurrence);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStartPeriodicTileUpdateBatch)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStartPeriodicTileUpdateBatch )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT pollingUris,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT uiUpdateRecurrence);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msClearTile)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msClearTile )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msEnableTileNotificationQueue)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msEnableTileNotificationQueue )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msPinnedSiteState)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msPinnedSiteState )( 
            __RPC__in IShellUIHelper8 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSiteState);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msEnableTileNotificationQueueForSquare150x150)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msEnableTileNotificationQueueForSquare150x150 )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msEnableTileNotificationQueueForWide310x150)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msEnableTileNotificationQueueForWide310x150 )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msEnableTileNotificationQueueForSquare310x310)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msEnableTileNotificationQueueForSquare310x310 )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msScheduledTileNotification)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msScheduledTileNotification )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR bstrNotificationXml,
            /* [in] */ __RPC__in BSTR bstrNotificationId,
            /* [in] */ __RPC__in BSTR bstrNotificationTag,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT expirationTime);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msRemoveScheduledTileNotification)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msRemoveScheduledTileNotification )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR bstrNotificationId);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStartPeriodicBadgeUpdate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStartPeriodicBadgeUpdate )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR pollingUri,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT uiUpdateRecurrence);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStopPeriodicBadgeUpdate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStopPeriodicBadgeUpdate )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msLaunchInternetOptions)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msLaunchInternetOptions )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, SetExperimentalFlag)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetExperimentalFlag )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR bstrFlagString,
            /* [in] */ VARIANT_BOOL vfFlag);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, GetExperimentalFlag)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetExperimentalFlag )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR bstrFlagString,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *vfFlag);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, SetExperimentalValue)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetExperimentalValue )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR bstrValueString,
            /* [in] */ DWORD dwValue);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, GetExperimentalValue)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetExperimentalValue )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR bstrValueString,
            /* [retval][out] */ __RPC__out DWORD *pdwValue);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, ResetAllExperimentalFlagsAndValues)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ResetAllExperimentalFlagsAndValues )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, GetNeedIEAutoLaunchFlag)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetNeedIEAutoLaunchFlag )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *flag);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, SetNeedIEAutoLaunchFlag)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetNeedIEAutoLaunchFlag )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [in] */ VARIANT_BOOL flag);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, HasNeedIEAutoLaunchFlag)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *HasNeedIEAutoLaunchFlag )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *exists);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, LaunchIE)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *LaunchIE )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [in] */ VARIANT_BOOL automated);
        
        DECLSPEC_XFGVIRT(IShellUIHelper8, GetCVListData)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetCVListData )( 
            __RPC__in IShellUIHelper8 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper8, GetCVListLocalData)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetCVListLocalData )( 
            __RPC__in IShellUIHelper8 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper8, GetEMIEListData)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEMIEListData )( 
            __RPC__in IShellUIHelper8 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper8, GetEMIEListLocalData)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEMIEListLocalData )( 
            __RPC__in IShellUIHelper8 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper8, OpenFavoritesPane)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *OpenFavoritesPane )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper8, OpenFavoritesSettings)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *OpenFavoritesSettings )( 
            __RPC__in IShellUIHelper8 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper8, LaunchInHVSI)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *LaunchInHVSI )( 
            __RPC__in IShellUIHelper8 * This,
            /* [in] */ __RPC__in BSTR bstrUrl);
        
        END_INTERFACE
    } IShellUIHelper8Vtbl;

    interface IShellUIHelper8
    {
        CONST_VTBL struct IShellUIHelper8Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellUIHelper8_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellUIHelper8_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellUIHelper8_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellUIHelper8_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellUIHelper8_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellUIHelper8_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellUIHelper8_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellUIHelper8_ResetFirstBootMode(This)	\
    ( (This)->lpVtbl -> ResetFirstBootMode(This) ) 

#define IShellUIHelper8_ResetSafeMode(This)	\
    ( (This)->lpVtbl -> ResetSafeMode(This) ) 

#define IShellUIHelper8_RefreshOfflineDesktop(This)	\
    ( (This)->lpVtbl -> RefreshOfflineDesktop(This) ) 

#define IShellUIHelper8_AddFavorite(This,URL,Title)	\
    ( (This)->lpVtbl -> AddFavorite(This,URL,Title) ) 

#define IShellUIHelper8_AddChannel(This,URL)	\
    ( (This)->lpVtbl -> AddChannel(This,URL) ) 

#define IShellUIHelper8_AddDesktopComponent(This,URL,Type,Left,Top,Width,Height)	\
    ( (This)->lpVtbl -> AddDesktopComponent(This,URL,Type,Left,Top,Width,Height) ) 

#define IShellUIHelper8_IsSubscribed(This,URL,pBool)	\
    ( (This)->lpVtbl -> IsSubscribed(This,URL,pBool) ) 

#define IShellUIHelper8_NavigateAndFind(This,URL,strQuery,varTargetFrame)	\
    ( (This)->lpVtbl -> NavigateAndFind(This,URL,strQuery,varTargetFrame) ) 

#define IShellUIHelper8_ImportExportFavorites(This,fImport,strImpExpPath)	\
    ( (This)->lpVtbl -> ImportExportFavorites(This,fImport,strImpExpPath) ) 

#define IShellUIHelper8_AutoCompleteSaveForm(This,Form)	\
    ( (This)->lpVtbl -> AutoCompleteSaveForm(This,Form) ) 

#define IShellUIHelper8_AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame)	\
    ( (This)->lpVtbl -> AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame) ) 

#define IShellUIHelper8_AutoCompleteAttach(This,Reserved)	\
    ( (This)->lpVtbl -> AutoCompleteAttach(This,Reserved) ) 

#define IShellUIHelper8_ShowBrowserUI(This,bstrName,pvarIn,pvarOut)	\
    ( (This)->lpVtbl -> ShowBrowserUI(This,bstrName,pvarIn,pvarOut) ) 


#define IShellUIHelper8_AddSearchProvider(This,URL)	\
    ( (This)->lpVtbl -> AddSearchProvider(This,URL) ) 

#define IShellUIHelper8_RunOnceShown(This)	\
    ( (This)->lpVtbl -> RunOnceShown(This) ) 

#define IShellUIHelper8_SkipRunOnce(This)	\
    ( (This)->lpVtbl -> SkipRunOnce(This) ) 

#define IShellUIHelper8_CustomizeSettings(This,fSQM,fPhishing,bstrLocale)	\
    ( (This)->lpVtbl -> CustomizeSettings(This,fSQM,fPhishing,bstrLocale) ) 

#define IShellUIHelper8_SqmEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> SqmEnabled(This,pfEnabled) ) 

#define IShellUIHelper8_PhishingEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> PhishingEnabled(This,pfEnabled) ) 

#define IShellUIHelper8_BrandImageUri(This,pbstrUri)	\
    ( (This)->lpVtbl -> BrandImageUri(This,pbstrUri) ) 

#define IShellUIHelper8_SkipTabsWelcome(This)	\
    ( (This)->lpVtbl -> SkipTabsWelcome(This) ) 

#define IShellUIHelper8_DiagnoseConnection(This)	\
    ( (This)->lpVtbl -> DiagnoseConnection(This) ) 

#define IShellUIHelper8_CustomizeClearType(This,fSet)	\
    ( (This)->lpVtbl -> CustomizeClearType(This,fSet) ) 

#define IShellUIHelper8_IsSearchProviderInstalled(This,URL,pdwResult)	\
    ( (This)->lpVtbl -> IsSearchProviderInstalled(This,URL,pdwResult) ) 

#define IShellUIHelper8_IsSearchMigrated(This,pfMigrated)	\
    ( (This)->lpVtbl -> IsSearchMigrated(This,pfMigrated) ) 

#define IShellUIHelper8_DefaultSearchProvider(This,pbstrName)	\
    ( (This)->lpVtbl -> DefaultSearchProvider(This,pbstrName) ) 

#define IShellUIHelper8_RunOnceRequiredSettingsComplete(This,fComplete)	\
    ( (This)->lpVtbl -> RunOnceRequiredSettingsComplete(This,fComplete) ) 

#define IShellUIHelper8_RunOnceHasShown(This,pfShown)	\
    ( (This)->lpVtbl -> RunOnceHasShown(This,pfShown) ) 

#define IShellUIHelper8_SearchGuideUrl(This,pbstrUrl)	\
    ( (This)->lpVtbl -> SearchGuideUrl(This,pbstrUrl) ) 


#define IShellUIHelper8_AddService(This,URL)	\
    ( (This)->lpVtbl -> AddService(This,URL) ) 

#define IShellUIHelper8_IsServiceInstalled(This,URL,Verb,pdwResult)	\
    ( (This)->lpVtbl -> IsServiceInstalled(This,URL,Verb,pdwResult) ) 

#define IShellUIHelper8_InPrivateFilteringEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> InPrivateFilteringEnabled(This,pfEnabled) ) 

#define IShellUIHelper8_AddToFavoritesBar(This,URL,Title,Type)	\
    ( (This)->lpVtbl -> AddToFavoritesBar(This,URL,Title,Type) ) 

#define IShellUIHelper8_BuildNewTabPage(This)	\
    ( (This)->lpVtbl -> BuildNewTabPage(This) ) 

#define IShellUIHelper8_SetRecentlyClosedVisible(This,fVisible)	\
    ( (This)->lpVtbl -> SetRecentlyClosedVisible(This,fVisible) ) 

#define IShellUIHelper8_SetActivitiesVisible(This,fVisible)	\
    ( (This)->lpVtbl -> SetActivitiesVisible(This,fVisible) ) 

#define IShellUIHelper8_ContentDiscoveryReset(This)	\
    ( (This)->lpVtbl -> ContentDiscoveryReset(This) ) 

#define IShellUIHelper8_IsSuggestedSitesEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> IsSuggestedSitesEnabled(This,pfEnabled) ) 

#define IShellUIHelper8_EnableSuggestedSites(This,fEnable)	\
    ( (This)->lpVtbl -> EnableSuggestedSites(This,fEnable) ) 

#define IShellUIHelper8_NavigateToSuggestedSites(This,bstrRelativeUrl)	\
    ( (This)->lpVtbl -> NavigateToSuggestedSites(This,bstrRelativeUrl) ) 

#define IShellUIHelper8_ShowTabsHelp(This)	\
    ( (This)->lpVtbl -> ShowTabsHelp(This) ) 

#define IShellUIHelper8_ShowInPrivateHelp(This)	\
    ( (This)->lpVtbl -> ShowInPrivateHelp(This) ) 


#define IShellUIHelper8_msIsSiteMode(This,pfSiteMode)	\
    ( (This)->lpVtbl -> msIsSiteMode(This,pfSiteMode) ) 

#define IShellUIHelper8_msSiteModeShowThumbBar(This)	\
    ( (This)->lpVtbl -> msSiteModeShowThumbBar(This) ) 

#define IShellUIHelper8_msSiteModeAddThumbBarButton(This,bstrIconURL,bstrTooltip,pvarButtonID)	\
    ( (This)->lpVtbl -> msSiteModeAddThumbBarButton(This,bstrIconURL,bstrTooltip,pvarButtonID) ) 

#define IShellUIHelper8_msSiteModeUpdateThumbBarButton(This,ButtonID,fEnabled,fVisible)	\
    ( (This)->lpVtbl -> msSiteModeUpdateThumbBarButton(This,ButtonID,fEnabled,fVisible) ) 

#define IShellUIHelper8_msSiteModeSetIconOverlay(This,IconUrl,pvarDescription)	\
    ( (This)->lpVtbl -> msSiteModeSetIconOverlay(This,IconUrl,pvarDescription) ) 

#define IShellUIHelper8_msSiteModeClearIconOverlay(This)	\
    ( (This)->lpVtbl -> msSiteModeClearIconOverlay(This) ) 

#define IShellUIHelper8_msAddSiteMode(This)	\
    ( (This)->lpVtbl -> msAddSiteMode(This) ) 

#define IShellUIHelper8_msSiteModeCreateJumpList(This,bstrHeader)	\
    ( (This)->lpVtbl -> msSiteModeCreateJumpList(This,bstrHeader) ) 

#define IShellUIHelper8_msSiteModeAddJumpListItem(This,bstrName,bstrActionUri,bstrIconUri,pvarWindowType)	\
    ( (This)->lpVtbl -> msSiteModeAddJumpListItem(This,bstrName,bstrActionUri,bstrIconUri,pvarWindowType) ) 

#define IShellUIHelper8_msSiteModeClearJumpList(This)	\
    ( (This)->lpVtbl -> msSiteModeClearJumpList(This) ) 

#define IShellUIHelper8_msSiteModeShowJumpList(This)	\
    ( (This)->lpVtbl -> msSiteModeShowJumpList(This) ) 

#define IShellUIHelper8_msSiteModeAddButtonStyle(This,uiButtonID,bstrIconUrl,bstrTooltip,pvarStyleID)	\
    ( (This)->lpVtbl -> msSiteModeAddButtonStyle(This,uiButtonID,bstrIconUrl,bstrTooltip,pvarStyleID) ) 

#define IShellUIHelper8_msSiteModeShowButtonStyle(This,uiButtonID,uiStyleID)	\
    ( (This)->lpVtbl -> msSiteModeShowButtonStyle(This,uiButtonID,uiStyleID) ) 

#define IShellUIHelper8_msSiteModeActivate(This)	\
    ( (This)->lpVtbl -> msSiteModeActivate(This) ) 

#define IShellUIHelper8_msIsSiteModeFirstRun(This,fPreserveState,puiFirstRun)	\
    ( (This)->lpVtbl -> msIsSiteModeFirstRun(This,fPreserveState,puiFirstRun) ) 

#define IShellUIHelper8_msAddTrackingProtectionList(This,URL,bstrFilterName)	\
    ( (This)->lpVtbl -> msAddTrackingProtectionList(This,URL,bstrFilterName) ) 

#define IShellUIHelper8_msTrackingProtectionEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> msTrackingProtectionEnabled(This,pfEnabled) ) 

#define IShellUIHelper8_msActiveXFilteringEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> msActiveXFilteringEnabled(This,pfEnabled) ) 


#define IShellUIHelper8_msProvisionNetworks(This,bstrProvisioningXml,puiResult)	\
    ( (This)->lpVtbl -> msProvisionNetworks(This,bstrProvisioningXml,puiResult) ) 

#define IShellUIHelper8_msReportSafeUrl(This)	\
    ( (This)->lpVtbl -> msReportSafeUrl(This) ) 

#define IShellUIHelper8_msSiteModeRefreshBadge(This)	\
    ( (This)->lpVtbl -> msSiteModeRefreshBadge(This) ) 

#define IShellUIHelper8_msSiteModeClearBadge(This)	\
    ( (This)->lpVtbl -> msSiteModeClearBadge(This) ) 

#define IShellUIHelper8_msDiagnoseConnectionUILess(This)	\
    ( (This)->lpVtbl -> msDiagnoseConnectionUILess(This) ) 

#define IShellUIHelper8_msLaunchNetworkClientHelp(This)	\
    ( (This)->lpVtbl -> msLaunchNetworkClientHelp(This) ) 

#define IShellUIHelper8_msChangeDefaultBrowser(This,fChange)	\
    ( (This)->lpVtbl -> msChangeDefaultBrowser(This,fChange) ) 


#define IShellUIHelper8_msStopPeriodicTileUpdate(This)	\
    ( (This)->lpVtbl -> msStopPeriodicTileUpdate(This) ) 

#define IShellUIHelper8_msStartPeriodicTileUpdate(This,pollingUris,startTime,uiUpdateRecurrence)	\
    ( (This)->lpVtbl -> msStartPeriodicTileUpdate(This,pollingUris,startTime,uiUpdateRecurrence) ) 

#define IShellUIHelper8_msStartPeriodicTileUpdateBatch(This,pollingUris,startTime,uiUpdateRecurrence)	\
    ( (This)->lpVtbl -> msStartPeriodicTileUpdateBatch(This,pollingUris,startTime,uiUpdateRecurrence) ) 

#define IShellUIHelper8_msClearTile(This)	\
    ( (This)->lpVtbl -> msClearTile(This) ) 

#define IShellUIHelper8_msEnableTileNotificationQueue(This,fChange)	\
    ( (This)->lpVtbl -> msEnableTileNotificationQueue(This,fChange) ) 

#define IShellUIHelper8_msPinnedSiteState(This,pvarSiteState)	\
    ( (This)->lpVtbl -> msPinnedSiteState(This,pvarSiteState) ) 

#define IShellUIHelper8_msEnableTileNotificationQueueForSquare150x150(This,fChange)	\
    ( (This)->lpVtbl -> msEnableTileNotificationQueueForSquare150x150(This,fChange) ) 

#define IShellUIHelper8_msEnableTileNotificationQueueForWide310x150(This,fChange)	\
    ( (This)->lpVtbl -> msEnableTileNotificationQueueForWide310x150(This,fChange) ) 

#define IShellUIHelper8_msEnableTileNotificationQueueForSquare310x310(This,fChange)	\
    ( (This)->lpVtbl -> msEnableTileNotificationQueueForSquare310x310(This,fChange) ) 

#define IShellUIHelper8_msScheduledTileNotification(This,bstrNotificationXml,bstrNotificationId,bstrNotificationTag,startTime,expirationTime)	\
    ( (This)->lpVtbl -> msScheduledTileNotification(This,bstrNotificationXml,bstrNotificationId,bstrNotificationTag,startTime,expirationTime) ) 

#define IShellUIHelper8_msRemoveScheduledTileNotification(This,bstrNotificationId)	\
    ( (This)->lpVtbl -> msRemoveScheduledTileNotification(This,bstrNotificationId) ) 

#define IShellUIHelper8_msStartPeriodicBadgeUpdate(This,pollingUri,startTime,uiUpdateRecurrence)	\
    ( (This)->lpVtbl -> msStartPeriodicBadgeUpdate(This,pollingUri,startTime,uiUpdateRecurrence) ) 

#define IShellUIHelper8_msStopPeriodicBadgeUpdate(This)	\
    ( (This)->lpVtbl -> msStopPeriodicBadgeUpdate(This) ) 

#define IShellUIHelper8_msLaunchInternetOptions(This)	\
    ( (This)->lpVtbl -> msLaunchInternetOptions(This) ) 


#define IShellUIHelper8_SetExperimentalFlag(This,bstrFlagString,vfFlag)	\
    ( (This)->lpVtbl -> SetExperimentalFlag(This,bstrFlagString,vfFlag) ) 

#define IShellUIHelper8_GetExperimentalFlag(This,bstrFlagString,vfFlag)	\
    ( (This)->lpVtbl -> GetExperimentalFlag(This,bstrFlagString,vfFlag) ) 

#define IShellUIHelper8_SetExperimentalValue(This,bstrValueString,dwValue)	\
    ( (This)->lpVtbl -> SetExperimentalValue(This,bstrValueString,dwValue) ) 

#define IShellUIHelper8_GetExperimentalValue(This,bstrValueString,pdwValue)	\
    ( (This)->lpVtbl -> GetExperimentalValue(This,bstrValueString,pdwValue) ) 

#define IShellUIHelper8_ResetAllExperimentalFlagsAndValues(This)	\
    ( (This)->lpVtbl -> ResetAllExperimentalFlagsAndValues(This) ) 

#define IShellUIHelper8_GetNeedIEAutoLaunchFlag(This,bstrUrl,flag)	\
    ( (This)->lpVtbl -> GetNeedIEAutoLaunchFlag(This,bstrUrl,flag) ) 

#define IShellUIHelper8_SetNeedIEAutoLaunchFlag(This,bstrUrl,flag)	\
    ( (This)->lpVtbl -> SetNeedIEAutoLaunchFlag(This,bstrUrl,flag) ) 

#define IShellUIHelper8_HasNeedIEAutoLaunchFlag(This,bstrUrl,exists)	\
    ( (This)->lpVtbl -> HasNeedIEAutoLaunchFlag(This,bstrUrl,exists) ) 

#define IShellUIHelper8_LaunchIE(This,bstrUrl,automated)	\
    ( (This)->lpVtbl -> LaunchIE(This,bstrUrl,automated) ) 


#define IShellUIHelper8_GetCVListData(This,pbstrResult)	\
    ( (This)->lpVtbl -> GetCVListData(This,pbstrResult) ) 

#define IShellUIHelper8_GetCVListLocalData(This,pbstrResult)	\
    ( (This)->lpVtbl -> GetCVListLocalData(This,pbstrResult) ) 

#define IShellUIHelper8_GetEMIEListData(This,pbstrResult)	\
    ( (This)->lpVtbl -> GetEMIEListData(This,pbstrResult) ) 

#define IShellUIHelper8_GetEMIEListLocalData(This,pbstrResult)	\
    ( (This)->lpVtbl -> GetEMIEListLocalData(This,pbstrResult) ) 

#define IShellUIHelper8_OpenFavoritesPane(This)	\
    ( (This)->lpVtbl -> OpenFavoritesPane(This) ) 

#define IShellUIHelper8_OpenFavoritesSettings(This)	\
    ( (This)->lpVtbl -> OpenFavoritesSettings(This) ) 

#define IShellUIHelper8_LaunchInHVSI(This,bstrUrl)	\
    ( (This)->lpVtbl -> LaunchInHVSI(This,bstrUrl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellUIHelper8_INTERFACE_DEFINED__ */


#ifndef __IShellUIHelper9_INTERFACE_DEFINED__
#define __IShellUIHelper9_INTERFACE_DEFINED__

/* interface IShellUIHelper9 */
/* [object][dual][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IShellUIHelper9;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6cdf73b0-7f2f-451f-bc0f-63e0f3284e54")
    IShellUIHelper9 : public IShellUIHelper8
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetOSSku( 
            /* [retval][out] */ __RPC__out DWORD *pdwResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellUIHelper9Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellUIHelper9 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellUIHelper9 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetFirstBootMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetFirstBootMode )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ResetSafeMode)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *ResetSafeMode )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, RefreshOfflineDesktop)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *RefreshOfflineDesktop )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddFavorite)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddFavorite )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Title);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddChannel)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddChannel )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AddDesktopComponent)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddDesktopComponent )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Type,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Left,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Top,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Width,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Height);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, IsSubscribed)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSubscribed )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, NavigateAndFind)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NavigateAndFind )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR strQuery,
            /* [in] */ __RPC__in VARIANT *varTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ImportExportFavorites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ImportExportFavorites )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT_BOOL fImport,
            /* [in] */ __RPC__in BSTR strImpExpPath);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteSaveForm)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteSaveForm )( 
            __RPC__in IShellUIHelper9 * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Form);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoScan)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AutoScan )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR strSearch,
            /* [in] */ __RPC__in BSTR strFailureUrl,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *pvarTargetFrame);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, AutoCompleteAttach)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *AutoCompleteAttach )( 
            __RPC__in IShellUIHelper9 * This,
            /* [unique][optional][in] */ __RPC__in_opt VARIANT *Reserved);
        
        DECLSPEC_XFGVIRT(IShellUIHelper, ShowBrowserUI)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowBrowserUI )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in VARIANT *pvarIn,
            /* [retval][out] */ __RPC__out VARIANT *pvarOut);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, AddSearchProvider)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddSearchProvider )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceShown)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceShown )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SkipRunOnce)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SkipRunOnce )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, CustomizeSettings)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CustomizeSettings )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT_BOOL fSQM,
            /* [in] */ VARIANT_BOOL fPhishing,
            /* [in] */ __RPC__in BSTR bstrLocale);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SqmEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SqmEnabled )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, PhishingEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PhishingEnabled )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, BrandImageUri)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BrandImageUri )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUri);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SkipTabsWelcome)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SkipTabsWelcome )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, DiagnoseConnection)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DiagnoseConnection )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, CustomizeClearType)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CustomizeClearType )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT_BOOL fSet);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, IsSearchProviderInstalled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSearchProviderInstalled )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [retval][out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, IsSearchMigrated)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSearchMigrated )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfMigrated);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, DefaultSearchProvider)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DefaultSearchProvider )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceRequiredSettingsComplete)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceRequiredSettingsComplete )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT_BOOL fComplete);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, RunOnceHasShown)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RunOnceHasShown )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfShown);
        
        DECLSPEC_XFGVIRT(IShellUIHelper2, SearchGuideUrl)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SearchGuideUrl )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUrl);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, AddService)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddService )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR URL);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, IsServiceInstalled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsServiceInstalled )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Verb,
            /* [retval][out] */ __RPC__out DWORD *pdwResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, InPrivateFilteringEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InPrivateFilteringEnabled )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, AddToFavoritesBar)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddToFavoritesBar )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR Title,
            /* [in][optional] */ __RPC__in VARIANT *Type);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, BuildNewTabPage)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BuildNewTabPage )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, SetRecentlyClosedVisible)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetRecentlyClosedVisible )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, SetActivitiesVisible)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetActivitiesVisible )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ContentDiscoveryReset)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ContentDiscoveryReset )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, IsSuggestedSitesEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsSuggestedSitesEnabled )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, EnableSuggestedSites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EnableSuggestedSites )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT_BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, NavigateToSuggestedSites)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NavigateToSuggestedSites )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR bstrRelativeUrl);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ShowTabsHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowTabsHelp )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper3, ShowInPrivateHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ShowInPrivateHelp )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msIsSiteMode)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msIsSiteMode )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfSiteMode);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowThumbBar)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowThumbBar )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddThumbBarButton)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddThumbBarButton )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR bstrIconURL,
            /* [in] */ __RPC__in BSTR bstrTooltip,
            /* [retval][out] */ __RPC__out VARIANT *pvarButtonID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeUpdateThumbBarButton)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeUpdateThumbBarButton )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT ButtonID,
            /* [in] */ VARIANT_BOOL fEnabled,
            /* [in] */ VARIANT_BOOL fVisible);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeSetIconOverlay)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeSetIconOverlay )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR IconUrl,
            /* [in][optional] */ __RPC__in VARIANT *pvarDescription);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeClearIconOverlay)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeClearIconOverlay )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msAddSiteMode)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msAddSiteMode )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeCreateJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeCreateJumpList )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR bstrHeader);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddJumpListItem)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddJumpListItem )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in BSTR bstrActionUri,
            /* [in] */ __RPC__in BSTR bstrIconUri,
            /* [in][optional] */ __RPC__in VARIANT *pvarWindowType);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeClearJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeClearJumpList )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowJumpList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowJumpList )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeAddButtonStyle)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeAddButtonStyle )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT uiButtonID,
            /* [in] */ __RPC__in BSTR bstrIconUrl,
            /* [in] */ __RPC__in BSTR bstrTooltip,
            /* [retval][out] */ __RPC__out VARIANT *pvarStyleID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeShowButtonStyle)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeShowButtonStyle )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT uiButtonID,
            /* [in] */ VARIANT uiStyleID);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msSiteModeActivate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeActivate )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msIsSiteModeFirstRun)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msIsSiteModeFirstRun )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT_BOOL fPreserveState,
            /* [retval][out] */ __RPC__out VARIANT *puiFirstRun);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msAddTrackingProtectionList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msAddTrackingProtectionList )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR URL,
            /* [in] */ __RPC__in BSTR bstrFilterName);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msTrackingProtectionEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msTrackingProtectionEnabled )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper4, msActiveXFilteringEnabled)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msActiveXFilteringEnabled )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msProvisionNetworks)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msProvisionNetworks )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR bstrProvisioningXml,
            /* [retval][out] */ __RPC__out VARIANT *puiResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msReportSafeUrl)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msReportSafeUrl )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msSiteModeRefreshBadge)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeRefreshBadge )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msSiteModeClearBadge)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msSiteModeClearBadge )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msDiagnoseConnectionUILess)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msDiagnoseConnectionUILess )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msLaunchNetworkClientHelp)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msLaunchNetworkClientHelp )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper5, msChangeDefaultBrowser)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msChangeDefaultBrowser )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStopPeriodicTileUpdate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStopPeriodicTileUpdate )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStartPeriodicTileUpdate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStartPeriodicTileUpdate )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT pollingUris,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT uiUpdateRecurrence);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStartPeriodicTileUpdateBatch)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStartPeriodicTileUpdateBatch )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT pollingUris,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT uiUpdateRecurrence);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msClearTile)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msClearTile )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msEnableTileNotificationQueue)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msEnableTileNotificationQueue )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msPinnedSiteState)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msPinnedSiteState )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSiteState);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msEnableTileNotificationQueueForSquare150x150)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msEnableTileNotificationQueueForSquare150x150 )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msEnableTileNotificationQueueForWide310x150)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msEnableTileNotificationQueueForWide310x150 )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msEnableTileNotificationQueueForSquare310x310)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msEnableTileNotificationQueueForSquare310x310 )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ VARIANT_BOOL fChange);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msScheduledTileNotification)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msScheduledTileNotification )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR bstrNotificationXml,
            /* [in] */ __RPC__in BSTR bstrNotificationId,
            /* [in] */ __RPC__in BSTR bstrNotificationTag,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT expirationTime);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msRemoveScheduledTileNotification)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msRemoveScheduledTileNotification )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR bstrNotificationId);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStartPeriodicBadgeUpdate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStartPeriodicBadgeUpdate )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR pollingUri,
            /* [in][optional] */ VARIANT startTime,
            /* [in][optional] */ VARIANT uiUpdateRecurrence);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msStopPeriodicBadgeUpdate)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msStopPeriodicBadgeUpdate )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper6, msLaunchInternetOptions)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *msLaunchInternetOptions )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, SetExperimentalFlag)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetExperimentalFlag )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR bstrFlagString,
            /* [in] */ VARIANT_BOOL vfFlag);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, GetExperimentalFlag)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetExperimentalFlag )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR bstrFlagString,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *vfFlag);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, SetExperimentalValue)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetExperimentalValue )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR bstrValueString,
            /* [in] */ DWORD dwValue);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, GetExperimentalValue)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetExperimentalValue )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR bstrValueString,
            /* [retval][out] */ __RPC__out DWORD *pdwValue);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, ResetAllExperimentalFlagsAndValues)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ResetAllExperimentalFlagsAndValues )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, GetNeedIEAutoLaunchFlag)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetNeedIEAutoLaunchFlag )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *flag);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, SetNeedIEAutoLaunchFlag)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetNeedIEAutoLaunchFlag )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [in] */ VARIANT_BOOL flag);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, HasNeedIEAutoLaunchFlag)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *HasNeedIEAutoLaunchFlag )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *exists);
        
        DECLSPEC_XFGVIRT(IShellUIHelper7, LaunchIE)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *LaunchIE )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [in] */ VARIANT_BOOL automated);
        
        DECLSPEC_XFGVIRT(IShellUIHelper8, GetCVListData)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetCVListData )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper8, GetCVListLocalData)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetCVListLocalData )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper8, GetEMIEListData)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEMIEListData )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper8, GetEMIEListLocalData)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEMIEListLocalData )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrResult);
        
        DECLSPEC_XFGVIRT(IShellUIHelper8, OpenFavoritesPane)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *OpenFavoritesPane )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper8, OpenFavoritesSettings)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *OpenFavoritesSettings )( 
            __RPC__in IShellUIHelper9 * This);
        
        DECLSPEC_XFGVIRT(IShellUIHelper8, LaunchInHVSI)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *LaunchInHVSI )( 
            __RPC__in IShellUIHelper9 * This,
            /* [in] */ __RPC__in BSTR bstrUrl);
        
        DECLSPEC_XFGVIRT(IShellUIHelper9, GetOSSku)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetOSSku )( 
            __RPC__in IShellUIHelper9 * This,
            /* [retval][out] */ __RPC__out DWORD *pdwResult);
        
        END_INTERFACE
    } IShellUIHelper9Vtbl;

    interface IShellUIHelper9
    {
        CONST_VTBL struct IShellUIHelper9Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellUIHelper9_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellUIHelper9_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellUIHelper9_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellUIHelper9_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellUIHelper9_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellUIHelper9_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellUIHelper9_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellUIHelper9_ResetFirstBootMode(This)	\
    ( (This)->lpVtbl -> ResetFirstBootMode(This) ) 

#define IShellUIHelper9_ResetSafeMode(This)	\
    ( (This)->lpVtbl -> ResetSafeMode(This) ) 

#define IShellUIHelper9_RefreshOfflineDesktop(This)	\
    ( (This)->lpVtbl -> RefreshOfflineDesktop(This) ) 

#define IShellUIHelper9_AddFavorite(This,URL,Title)	\
    ( (This)->lpVtbl -> AddFavorite(This,URL,Title) ) 

#define IShellUIHelper9_AddChannel(This,URL)	\
    ( (This)->lpVtbl -> AddChannel(This,URL) ) 

#define IShellUIHelper9_AddDesktopComponent(This,URL,Type,Left,Top,Width,Height)	\
    ( (This)->lpVtbl -> AddDesktopComponent(This,URL,Type,Left,Top,Width,Height) ) 

#define IShellUIHelper9_IsSubscribed(This,URL,pBool)	\
    ( (This)->lpVtbl -> IsSubscribed(This,URL,pBool) ) 

#define IShellUIHelper9_NavigateAndFind(This,URL,strQuery,varTargetFrame)	\
    ( (This)->lpVtbl -> NavigateAndFind(This,URL,strQuery,varTargetFrame) ) 

#define IShellUIHelper9_ImportExportFavorites(This,fImport,strImpExpPath)	\
    ( (This)->lpVtbl -> ImportExportFavorites(This,fImport,strImpExpPath) ) 

#define IShellUIHelper9_AutoCompleteSaveForm(This,Form)	\
    ( (This)->lpVtbl -> AutoCompleteSaveForm(This,Form) ) 

#define IShellUIHelper9_AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame)	\
    ( (This)->lpVtbl -> AutoScan(This,strSearch,strFailureUrl,pvarTargetFrame) ) 

#define IShellUIHelper9_AutoCompleteAttach(This,Reserved)	\
    ( (This)->lpVtbl -> AutoCompleteAttach(This,Reserved) ) 

#define IShellUIHelper9_ShowBrowserUI(This,bstrName,pvarIn,pvarOut)	\
    ( (This)->lpVtbl -> ShowBrowserUI(This,bstrName,pvarIn,pvarOut) ) 


#define IShellUIHelper9_AddSearchProvider(This,URL)	\
    ( (This)->lpVtbl -> AddSearchProvider(This,URL) ) 

#define IShellUIHelper9_RunOnceShown(This)	\
    ( (This)->lpVtbl -> RunOnceShown(This) ) 

#define IShellUIHelper9_SkipRunOnce(This)	\
    ( (This)->lpVtbl -> SkipRunOnce(This) ) 

#define IShellUIHelper9_CustomizeSettings(This,fSQM,fPhishing,bstrLocale)	\
    ( (This)->lpVtbl -> CustomizeSettings(This,fSQM,fPhishing,bstrLocale) ) 

#define IShellUIHelper9_SqmEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> SqmEnabled(This,pfEnabled) ) 

#define IShellUIHelper9_PhishingEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> PhishingEnabled(This,pfEnabled) ) 

#define IShellUIHelper9_BrandImageUri(This,pbstrUri)	\
    ( (This)->lpVtbl -> BrandImageUri(This,pbstrUri) ) 

#define IShellUIHelper9_SkipTabsWelcome(This)	\
    ( (This)->lpVtbl -> SkipTabsWelcome(This) ) 

#define IShellUIHelper9_DiagnoseConnection(This)	\
    ( (This)->lpVtbl -> DiagnoseConnection(This) ) 

#define IShellUIHelper9_CustomizeClearType(This,fSet)	\
    ( (This)->lpVtbl -> CustomizeClearType(This,fSet) ) 

#define IShellUIHelper9_IsSearchProviderInstalled(This,URL,pdwResult)	\
    ( (This)->lpVtbl -> IsSearchProviderInstalled(This,URL,pdwResult) ) 

#define IShellUIHelper9_IsSearchMigrated(This,pfMigrated)	\
    ( (This)->lpVtbl -> IsSearchMigrated(This,pfMigrated) ) 

#define IShellUIHelper9_DefaultSearchProvider(This,pbstrName)	\
    ( (This)->lpVtbl -> DefaultSearchProvider(This,pbstrName) ) 

#define IShellUIHelper9_RunOnceRequiredSettingsComplete(This,fComplete)	\
    ( (This)->lpVtbl -> RunOnceRequiredSettingsComplete(This,fComplete) ) 

#define IShellUIHelper9_RunOnceHasShown(This,pfShown)	\
    ( (This)->lpVtbl -> RunOnceHasShown(This,pfShown) ) 

#define IShellUIHelper9_SearchGuideUrl(This,pbstrUrl)	\
    ( (This)->lpVtbl -> SearchGuideUrl(This,pbstrUrl) ) 


#define IShellUIHelper9_AddService(This,URL)	\
    ( (This)->lpVtbl -> AddService(This,URL) ) 

#define IShellUIHelper9_IsServiceInstalled(This,URL,Verb,pdwResult)	\
    ( (This)->lpVtbl -> IsServiceInstalled(This,URL,Verb,pdwResult) ) 

#define IShellUIHelper9_InPrivateFilteringEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> InPrivateFilteringEnabled(This,pfEnabled) ) 

#define IShellUIHelper9_AddToFavoritesBar(This,URL,Title,Type)	\
    ( (This)->lpVtbl -> AddToFavoritesBar(This,URL,Title,Type) ) 

#define IShellUIHelper9_BuildNewTabPage(This)	\
    ( (This)->lpVtbl -> BuildNewTabPage(This) ) 

#define IShellUIHelper9_SetRecentlyClosedVisible(This,fVisible)	\
    ( (This)->lpVtbl -> SetRecentlyClosedVisible(This,fVisible) ) 

#define IShellUIHelper9_SetActivitiesVisible(This,fVisible)	\
    ( (This)->lpVtbl -> SetActivitiesVisible(This,fVisible) ) 

#define IShellUIHelper9_ContentDiscoveryReset(This)	\
    ( (This)->lpVtbl -> ContentDiscoveryReset(This) ) 

#define IShellUIHelper9_IsSuggestedSitesEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> IsSuggestedSitesEnabled(This,pfEnabled) ) 

#define IShellUIHelper9_EnableSuggestedSites(This,fEnable)	\
    ( (This)->lpVtbl -> EnableSuggestedSites(This,fEnable) ) 

#define IShellUIHelper9_NavigateToSuggestedSites(This,bstrRelativeUrl)	\
    ( (This)->lpVtbl -> NavigateToSuggestedSites(This,bstrRelativeUrl) ) 

#define IShellUIHelper9_ShowTabsHelp(This)	\
    ( (This)->lpVtbl -> ShowTabsHelp(This) ) 

#define IShellUIHelper9_ShowInPrivateHelp(This)	\
    ( (This)->lpVtbl -> ShowInPrivateHelp(This) ) 


#define IShellUIHelper9_msIsSiteMode(This,pfSiteMode)	\
    ( (This)->lpVtbl -> msIsSiteMode(This,pfSiteMode) ) 

#define IShellUIHelper9_msSiteModeShowThumbBar(This)	\
    ( (This)->lpVtbl -> msSiteModeShowThumbBar(This) ) 

#define IShellUIHelper9_msSiteModeAddThumbBarButton(This,bstrIconURL,bstrTooltip,pvarButtonID)	\
    ( (This)->lpVtbl -> msSiteModeAddThumbBarButton(This,bstrIconURL,bstrTooltip,pvarButtonID) ) 

#define IShellUIHelper9_msSiteModeUpdateThumbBarButton(This,ButtonID,fEnabled,fVisible)	\
    ( (This)->lpVtbl -> msSiteModeUpdateThumbBarButton(This,ButtonID,fEnabled,fVisible) ) 

#define IShellUIHelper9_msSiteModeSetIconOverlay(This,IconUrl,pvarDescription)	\
    ( (This)->lpVtbl -> msSiteModeSetIconOverlay(This,IconUrl,pvarDescription) ) 

#define IShellUIHelper9_msSiteModeClearIconOverlay(This)	\
    ( (This)->lpVtbl -> msSiteModeClearIconOverlay(This) ) 

#define IShellUIHelper9_msAddSiteMode(This)	\
    ( (This)->lpVtbl -> msAddSiteMode(This) ) 

#define IShellUIHelper9_msSiteModeCreateJumpList(This,bstrHeader)	\
    ( (This)->lpVtbl -> msSiteModeCreateJumpList(This,bstrHeader) ) 

#define IShellUIHelper9_msSiteModeAddJumpListItem(This,bstrName,bstrActionUri,bstrIconUri,pvarWindowType)	\
    ( (This)->lpVtbl -> msSiteModeAddJumpListItem(This,bstrName,bstrActionUri,bstrIconUri,pvarWindowType) ) 

#define IShellUIHelper9_msSiteModeClearJumpList(This)	\
    ( (This)->lpVtbl -> msSiteModeClearJumpList(This) ) 

#define IShellUIHelper9_msSiteModeShowJumpList(This)	\
    ( (This)->lpVtbl -> msSiteModeShowJumpList(This) ) 

#define IShellUIHelper9_msSiteModeAddButtonStyle(This,uiButtonID,bstrIconUrl,bstrTooltip,pvarStyleID)	\
    ( (This)->lpVtbl -> msSiteModeAddButtonStyle(This,uiButtonID,bstrIconUrl,bstrTooltip,pvarStyleID) ) 

#define IShellUIHelper9_msSiteModeShowButtonStyle(This,uiButtonID,uiStyleID)	\
    ( (This)->lpVtbl -> msSiteModeShowButtonStyle(This,uiButtonID,uiStyleID) ) 

#define IShellUIHelper9_msSiteModeActivate(This)	\
    ( (This)->lpVtbl -> msSiteModeActivate(This) ) 

#define IShellUIHelper9_msIsSiteModeFirstRun(This,fPreserveState,puiFirstRun)	\
    ( (This)->lpVtbl -> msIsSiteModeFirstRun(This,fPreserveState,puiFirstRun) ) 

#define IShellUIHelper9_msAddTrackingProtectionList(This,URL,bstrFilterName)	\
    ( (This)->lpVtbl -> msAddTrackingProtectionList(This,URL,bstrFilterName) ) 

#define IShellUIHelper9_msTrackingProtectionEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> msTrackingProtectionEnabled(This,pfEnabled) ) 

#define IShellUIHelper9_msActiveXFilteringEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> msActiveXFilteringEnabled(This,pfEnabled) ) 


#define IShellUIHelper9_msProvisionNetworks(This,bstrProvisioningXml,puiResult)	\
    ( (This)->lpVtbl -> msProvisionNetworks(This,bstrProvisioningXml,puiResult) ) 

#define IShellUIHelper9_msReportSafeUrl(This)	\
    ( (This)->lpVtbl -> msReportSafeUrl(This) ) 

#define IShellUIHelper9_msSiteModeRefreshBadge(This)	\
    ( (This)->lpVtbl -> msSiteModeRefreshBadge(This) ) 

#define IShellUIHelper9_msSiteModeClearBadge(This)	\
    ( (This)->lpVtbl -> msSiteModeClearBadge(This) ) 

#define IShellUIHelper9_msDiagnoseConnectionUILess(This)	\
    ( (This)->lpVtbl -> msDiagnoseConnectionUILess(This) ) 

#define IShellUIHelper9_msLaunchNetworkClientHelp(This)	\
    ( (This)->lpVtbl -> msLaunchNetworkClientHelp(This) ) 

#define IShellUIHelper9_msChangeDefaultBrowser(This,fChange)	\
    ( (This)->lpVtbl -> msChangeDefaultBrowser(This,fChange) ) 


#define IShellUIHelper9_msStopPeriodicTileUpdate(This)	\
    ( (This)->lpVtbl -> msStopPeriodicTileUpdate(This) ) 

#define IShellUIHelper9_msStartPeriodicTileUpdate(This,pollingUris,startTime,uiUpdateRecurrence)	\
    ( (This)->lpVtbl -> msStartPeriodicTileUpdate(This,pollingUris,startTime,uiUpdateRecurrence) ) 

#define IShellUIHelper9_msStartPeriodicTileUpdateBatch(This,pollingUris,startTime,uiUpdateRecurrence)	\
    ( (This)->lpVtbl -> msStartPeriodicTileUpdateBatch(This,pollingUris,startTime,uiUpdateRecurrence) ) 

#define IShellUIHelper9_msClearTile(This)	\
    ( (This)->lpVtbl -> msClearTile(This) ) 

#define IShellUIHelper9_msEnableTileNotificationQueue(This,fChange)	\
    ( (This)->lpVtbl -> msEnableTileNotificationQueue(This,fChange) ) 

#define IShellUIHelper9_msPinnedSiteState(This,pvarSiteState)	\
    ( (This)->lpVtbl -> msPinnedSiteState(This,pvarSiteState) ) 

#define IShellUIHelper9_msEnableTileNotificationQueueForSquare150x150(This,fChange)	\
    ( (This)->lpVtbl -> msEnableTileNotificationQueueForSquare150x150(This,fChange) ) 

#define IShellUIHelper9_msEnableTileNotificationQueueForWide310x150(This,fChange)	\
    ( (This)->lpVtbl -> msEnableTileNotificationQueueForWide310x150(This,fChange) ) 

#define IShellUIHelper9_msEnableTileNotificationQueueForSquare310x310(This,fChange)	\
    ( (This)->lpVtbl -> msEnableTileNotificationQueueForSquare310x310(This,fChange) ) 

#define IShellUIHelper9_msScheduledTileNotification(This,bstrNotificationXml,bstrNotificationId,bstrNotificationTag,startTime,expirationTime)	\
    ( (This)->lpVtbl -> msScheduledTileNotification(This,bstrNotificationXml,bstrNotificationId,bstrNotificationTag,startTime,expirationTime) ) 

#define IShellUIHelper9_msRemoveScheduledTileNotification(This,bstrNotificationId)	\
    ( (This)->lpVtbl -> msRemoveScheduledTileNotification(This,bstrNotificationId) ) 

#define IShellUIHelper9_msStartPeriodicBadgeUpdate(This,pollingUri,startTime,uiUpdateRecurrence)	\
    ( (This)->lpVtbl -> msStartPeriodicBadgeUpdate(This,pollingUri,startTime,uiUpdateRecurrence) ) 

#define IShellUIHelper9_msStopPeriodicBadgeUpdate(This)	\
    ( (This)->lpVtbl -> msStopPeriodicBadgeUpdate(This) ) 

#define IShellUIHelper9_msLaunchInternetOptions(This)	\
    ( (This)->lpVtbl -> msLaunchInternetOptions(This) ) 


#define IShellUIHelper9_SetExperimentalFlag(This,bstrFlagString,vfFlag)	\
    ( (This)->lpVtbl -> SetExperimentalFlag(This,bstrFlagString,vfFlag) ) 

#define IShellUIHelper9_GetExperimentalFlag(This,bstrFlagString,vfFlag)	\
    ( (This)->lpVtbl -> GetExperimentalFlag(This,bstrFlagString,vfFlag) ) 

#define IShellUIHelper9_SetExperimentalValue(This,bstrValueString,dwValue)	\
    ( (This)->lpVtbl -> SetExperimentalValue(This,bstrValueString,dwValue) ) 

#define IShellUIHelper9_GetExperimentalValue(This,bstrValueString,pdwValue)	\
    ( (This)->lpVtbl -> GetExperimentalValue(This,bstrValueString,pdwValue) ) 

#define IShellUIHelper9_ResetAllExperimentalFlagsAndValues(This)	\
    ( (This)->lpVtbl -> ResetAllExperimentalFlagsAndValues(This) ) 

#define IShellUIHelper9_GetNeedIEAutoLaunchFlag(This,bstrUrl,flag)	\
    ( (This)->lpVtbl -> GetNeedIEAutoLaunchFlag(This,bstrUrl,flag) ) 

#define IShellUIHelper9_SetNeedIEAutoLaunchFlag(This,bstrUrl,flag)	\
    ( (This)->lpVtbl -> SetNeedIEAutoLaunchFlag(This,bstrUrl,flag) ) 

#define IShellUIHelper9_HasNeedIEAutoLaunchFlag(This,bstrUrl,exists)	\
    ( (This)->lpVtbl -> HasNeedIEAutoLaunchFlag(This,bstrUrl,exists) ) 

#define IShellUIHelper9_LaunchIE(This,bstrUrl,automated)	\
    ( (This)->lpVtbl -> LaunchIE(This,bstrUrl,automated) ) 


#define IShellUIHelper9_GetCVListData(This,pbstrResult)	\
    ( (This)->lpVtbl -> GetCVListData(This,pbstrResult) ) 

#define IShellUIHelper9_GetCVListLocalData(This,pbstrResult)	\
    ( (This)->lpVtbl -> GetCVListLocalData(This,pbstrResult) ) 

#define IShellUIHelper9_GetEMIEListData(This,pbstrResult)	\
    ( (This)->lpVtbl -> GetEMIEListData(This,pbstrResult) ) 

#define IShellUIHelper9_GetEMIEListLocalData(This,pbstrResult)	\
    ( (This)->lpVtbl -> GetEMIEListLocalData(This,pbstrResult) ) 

#define IShellUIHelper9_OpenFavoritesPane(This)	\
    ( (This)->lpVtbl -> OpenFavoritesPane(This) ) 

#define IShellUIHelper9_OpenFavoritesSettings(This)	\
    ( (This)->lpVtbl -> OpenFavoritesSettings(This) ) 

#define IShellUIHelper9_LaunchInHVSI(This,bstrUrl)	\
    ( (This)->lpVtbl -> LaunchInHVSI(This,bstrUrl) ) 


#define IShellUIHelper9_GetOSSku(This,pdwResult)	\
    ( (This)->lpVtbl -> GetOSSku(This,pdwResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellUIHelper9_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_ShellUIHelper;

#ifdef __cplusplus

class DECLSPEC_UUID("64AB4BB7-111E-11d1-8F79-00C04FC2FBE1")
ShellUIHelper;
#endif

#ifndef __DShellNameSpaceEvents_DISPINTERFACE_DEFINED__
#define __DShellNameSpaceEvents_DISPINTERFACE_DEFINED__

/* dispinterface DShellNameSpaceEvents */
/* [uuid] */ 


EXTERN_C const IID DIID_DShellNameSpaceEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("55136806-B2DE-11D1-B9F2-00A0C98BC547")
    DShellNameSpaceEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct DShellNameSpaceEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DShellNameSpaceEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DShellNameSpaceEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DShellNameSpaceEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DShellNameSpaceEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DShellNameSpaceEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DShellNameSpaceEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DShellNameSpaceEvents * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } DShellNameSpaceEventsVtbl;

    interface DShellNameSpaceEvents
    {
        CONST_VTBL struct DShellNameSpaceEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DShellNameSpaceEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DShellNameSpaceEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DShellNameSpaceEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DShellNameSpaceEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DShellNameSpaceEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DShellNameSpaceEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DShellNameSpaceEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __DShellNameSpaceEvents_DISPINTERFACE_DEFINED__ */


#ifndef __IShellFavoritesNameSpace_INTERFACE_DEFINED__
#define __IShellFavoritesNameSpace_INTERFACE_DEFINED__

/* interface IShellFavoritesNameSpace */
/* [hidden][unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IShellFavoritesNameSpace;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("55136804-B2DE-11D1-B9F2-00A0C98BC547")
    IShellFavoritesNameSpace : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE MoveSelectionUp( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE MoveSelectionDown( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ResetSort( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NewFolder( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Synchronize( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Import( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Export( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InvokeContextMenuCommand( 
            /* [in] */ __RPC__in BSTR strCommand) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE MoveSelectionTo( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SubscriptionsEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateSubscriptionForSelection( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteSubscriptionForSelection( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetRoot( 
            /* [in] */ __RPC__in BSTR bstrFullPath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellFavoritesNameSpaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellFavoritesNameSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellFavoritesNameSpace * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellFavoritesNameSpace * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellFavoritesNameSpace * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellFavoritesNameSpace * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellFavoritesNameSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellFavoritesNameSpace * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, MoveSelectionUp)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MoveSelectionUp )( 
            __RPC__in IShellFavoritesNameSpace * This);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, MoveSelectionDown)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MoveSelectionDown )( 
            __RPC__in IShellFavoritesNameSpace * This);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, ResetSort)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ResetSort )( 
            __RPC__in IShellFavoritesNameSpace * This);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, NewFolder)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NewFolder )( 
            __RPC__in IShellFavoritesNameSpace * This);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, Synchronize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Synchronize )( 
            __RPC__in IShellFavoritesNameSpace * This);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, Import)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Import )( 
            __RPC__in IShellFavoritesNameSpace * This);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, Export)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Export )( 
            __RPC__in IShellFavoritesNameSpace * This);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, InvokeContextMenuCommand)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InvokeContextMenuCommand )( 
            __RPC__in IShellFavoritesNameSpace * This,
            /* [in] */ __RPC__in BSTR strCommand);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, MoveSelectionTo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MoveSelectionTo )( 
            __RPC__in IShellFavoritesNameSpace * This);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, get_SubscriptionsEnabled)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubscriptionsEnabled )( 
            __RPC__in IShellFavoritesNameSpace * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, CreateSubscriptionForSelection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateSubscriptionForSelection )( 
            __RPC__in IShellFavoritesNameSpace * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, DeleteSubscriptionForSelection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteSubscriptionForSelection )( 
            __RPC__in IShellFavoritesNameSpace * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, SetRoot)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetRoot )( 
            __RPC__in IShellFavoritesNameSpace * This,
            /* [in] */ __RPC__in BSTR bstrFullPath);
        
        END_INTERFACE
    } IShellFavoritesNameSpaceVtbl;

    interface IShellFavoritesNameSpace
    {
        CONST_VTBL struct IShellFavoritesNameSpaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellFavoritesNameSpace_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellFavoritesNameSpace_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellFavoritesNameSpace_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellFavoritesNameSpace_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellFavoritesNameSpace_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellFavoritesNameSpace_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellFavoritesNameSpace_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellFavoritesNameSpace_MoveSelectionUp(This)	\
    ( (This)->lpVtbl -> MoveSelectionUp(This) ) 

#define IShellFavoritesNameSpace_MoveSelectionDown(This)	\
    ( (This)->lpVtbl -> MoveSelectionDown(This) ) 

#define IShellFavoritesNameSpace_ResetSort(This)	\
    ( (This)->lpVtbl -> ResetSort(This) ) 

#define IShellFavoritesNameSpace_NewFolder(This)	\
    ( (This)->lpVtbl -> NewFolder(This) ) 

#define IShellFavoritesNameSpace_Synchronize(This)	\
    ( (This)->lpVtbl -> Synchronize(This) ) 

#define IShellFavoritesNameSpace_Import(This)	\
    ( (This)->lpVtbl -> Import(This) ) 

#define IShellFavoritesNameSpace_Export(This)	\
    ( (This)->lpVtbl -> Export(This) ) 

#define IShellFavoritesNameSpace_InvokeContextMenuCommand(This,strCommand)	\
    ( (This)->lpVtbl -> InvokeContextMenuCommand(This,strCommand) ) 

#define IShellFavoritesNameSpace_MoveSelectionTo(This)	\
    ( (This)->lpVtbl -> MoveSelectionTo(This) ) 

#define IShellFavoritesNameSpace_get_SubscriptionsEnabled(This,pBool)	\
    ( (This)->lpVtbl -> get_SubscriptionsEnabled(This,pBool) ) 

#define IShellFavoritesNameSpace_CreateSubscriptionForSelection(This,pBool)	\
    ( (This)->lpVtbl -> CreateSubscriptionForSelection(This,pBool) ) 

#define IShellFavoritesNameSpace_DeleteSubscriptionForSelection(This,pBool)	\
    ( (This)->lpVtbl -> DeleteSubscriptionForSelection(This,pBool) ) 

#define IShellFavoritesNameSpace_SetRoot(This,bstrFullPath)	\
    ( (This)->lpVtbl -> SetRoot(This,bstrFullPath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellFavoritesNameSpace_INTERFACE_DEFINED__ */


#ifndef __IShellNameSpace_INTERFACE_DEFINED__
#define __IShellNameSpace_INTERFACE_DEFINED__

/* interface IShellNameSpace */
/* [hidden][unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IShellNameSpace;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e572d3c9-37be-4ae2-825d-d521763e3108")
    IShellNameSpace : public IShellFavoritesNameSpace
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EnumOptions( 
            /* [retval][out] */ __RPC__out LONG *pgrfEnumFlags) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EnumOptions( 
            /* [in] */ LONG lVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SelectedItem( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **pItem) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SelectedItem( 
            /* [in] */ __RPC__in_opt IDispatch *pItem) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Root( 
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Root( 
            /* [in] */ VARIANT var) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Depth( 
            /* [retval][out] */ __RPC__out int *piDepth) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Depth( 
            /* [in] */ int iDepth) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Mode( 
            /* [retval][out] */ __RPC__out UINT *puMode) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Mode( 
            /* [in] */ UINT uMode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Flags( 
            /* [retval][out] */ __RPC__out DWORD *pdwFlags) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Flags( 
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_TVFlags( 
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TVFlags( 
            /* [retval][out] */ __RPC__out DWORD *dwFlags) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Columns( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrColumns) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Columns( 
            /* [in] */ __RPC__in BSTR bstrColumns) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CountViewTypes( 
            /* [retval][out] */ __RPC__out int *piTypes) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetViewType( 
            /* [in] */ int iType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SelectedItems( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Expand( 
            /* [in] */ VARIANT var,
            int iDepth) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UnselectAll( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellNameSpaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellNameSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellNameSpace * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellNameSpace * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IShellNameSpace * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IShellNameSpace * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IShellNameSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IShellNameSpace * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, MoveSelectionUp)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MoveSelectionUp )( 
            __RPC__in IShellNameSpace * This);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, MoveSelectionDown)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MoveSelectionDown )( 
            __RPC__in IShellNameSpace * This);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, ResetSort)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ResetSort )( 
            __RPC__in IShellNameSpace * This);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, NewFolder)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NewFolder )( 
            __RPC__in IShellNameSpace * This);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, Synchronize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Synchronize )( 
            __RPC__in IShellNameSpace * This);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, Import)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Import )( 
            __RPC__in IShellNameSpace * This);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, Export)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Export )( 
            __RPC__in IShellNameSpace * This);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, InvokeContextMenuCommand)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InvokeContextMenuCommand )( 
            __RPC__in IShellNameSpace * This,
            /* [in] */ __RPC__in BSTR strCommand);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, MoveSelectionTo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MoveSelectionTo )( 
            __RPC__in IShellNameSpace * This);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, get_SubscriptionsEnabled)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubscriptionsEnabled )( 
            __RPC__in IShellNameSpace * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, CreateSubscriptionForSelection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateSubscriptionForSelection )( 
            __RPC__in IShellNameSpace * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, DeleteSubscriptionForSelection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteSubscriptionForSelection )( 
            __RPC__in IShellNameSpace * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pBool);
        
        DECLSPEC_XFGVIRT(IShellFavoritesNameSpace, SetRoot)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetRoot )( 
            __RPC__in IShellNameSpace * This,
            /* [in] */ __RPC__in BSTR bstrFullPath);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, get_EnumOptions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnumOptions )( 
            __RPC__in IShellNameSpace * This,
            /* [retval][out] */ __RPC__out LONG *pgrfEnumFlags);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, put_EnumOptions)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EnumOptions )( 
            __RPC__in IShellNameSpace * This,
            /* [in] */ LONG lVal);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, get_SelectedItem)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SelectedItem )( 
            __RPC__in IShellNameSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **pItem);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, put_SelectedItem)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SelectedItem )( 
            __RPC__in IShellNameSpace * This,
            /* [in] */ __RPC__in_opt IDispatch *pItem);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, get_Root)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Root )( 
            __RPC__in IShellNameSpace * This,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, put_Root)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Root )( 
            __RPC__in IShellNameSpace * This,
            /* [in] */ VARIANT var);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, get_Depth)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Depth )( 
            __RPC__in IShellNameSpace * This,
            /* [retval][out] */ __RPC__out int *piDepth);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, put_Depth)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Depth )( 
            __RPC__in IShellNameSpace * This,
            /* [in] */ int iDepth);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, get_Mode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Mode )( 
            __RPC__in IShellNameSpace * This,
            /* [retval][out] */ __RPC__out UINT *puMode);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, put_Mode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Mode )( 
            __RPC__in IShellNameSpace * This,
            /* [in] */ UINT uMode);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, get_Flags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Flags )( 
            __RPC__in IShellNameSpace * This,
            /* [retval][out] */ __RPC__out DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, put_Flags)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Flags )( 
            __RPC__in IShellNameSpace * This,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, put_TVFlags)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TVFlags )( 
            __RPC__in IShellNameSpace * This,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, get_TVFlags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TVFlags )( 
            __RPC__in IShellNameSpace * This,
            /* [retval][out] */ __RPC__out DWORD *dwFlags);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, get_Columns)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Columns )( 
            __RPC__in IShellNameSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrColumns);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, put_Columns)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Columns )( 
            __RPC__in IShellNameSpace * This,
            /* [in] */ __RPC__in BSTR bstrColumns);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, get_CountViewTypes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CountViewTypes )( 
            __RPC__in IShellNameSpace * This,
            /* [retval][out] */ __RPC__out int *piTypes);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, SetViewType)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetViewType )( 
            __RPC__in IShellNameSpace * This,
            /* [in] */ int iType);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, SelectedItems)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SelectedItems )( 
            __RPC__in IShellNameSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppid);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, Expand)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Expand )( 
            __RPC__in IShellNameSpace * This,
            /* [in] */ VARIANT var,
            int iDepth);
        
        DECLSPEC_XFGVIRT(IShellNameSpace, UnselectAll)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UnselectAll )( 
            __RPC__in IShellNameSpace * This);
        
        END_INTERFACE
    } IShellNameSpaceVtbl;

    interface IShellNameSpace
    {
        CONST_VTBL struct IShellNameSpaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellNameSpace_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellNameSpace_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellNameSpace_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellNameSpace_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IShellNameSpace_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IShellNameSpace_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IShellNameSpace_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IShellNameSpace_MoveSelectionUp(This)	\
    ( (This)->lpVtbl -> MoveSelectionUp(This) ) 

#define IShellNameSpace_MoveSelectionDown(This)	\
    ( (This)->lpVtbl -> MoveSelectionDown(This) ) 

#define IShellNameSpace_ResetSort(This)	\
    ( (This)->lpVtbl -> ResetSort(This) ) 

#define IShellNameSpace_NewFolder(This)	\
    ( (This)->lpVtbl -> NewFolder(This) ) 

#define IShellNameSpace_Synchronize(This)	\
    ( (This)->lpVtbl -> Synchronize(This) ) 

#define IShellNameSpace_Import(This)	\
    ( (This)->lpVtbl -> Import(This) ) 

#define IShellNameSpace_Export(This)	\
    ( (This)->lpVtbl -> Export(This) ) 

#define IShellNameSpace_InvokeContextMenuCommand(This,strCommand)	\
    ( (This)->lpVtbl -> InvokeContextMenuCommand(This,strCommand) ) 

#define IShellNameSpace_MoveSelectionTo(This)	\
    ( (This)->lpVtbl -> MoveSelectionTo(This) ) 

#define IShellNameSpace_get_SubscriptionsEnabled(This,pBool)	\
    ( (This)->lpVtbl -> get_SubscriptionsEnabled(This,pBool) ) 

#define IShellNameSpace_CreateSubscriptionForSelection(This,pBool)	\
    ( (This)->lpVtbl -> CreateSubscriptionForSelection(This,pBool) ) 

#define IShellNameSpace_DeleteSubscriptionForSelection(This,pBool)	\
    ( (This)->lpVtbl -> DeleteSubscriptionForSelection(This,pBool) ) 

#define IShellNameSpace_SetRoot(This,bstrFullPath)	\
    ( (This)->lpVtbl -> SetRoot(This,bstrFullPath) ) 


#define IShellNameSpace_get_EnumOptions(This,pgrfEnumFlags)	\
    ( (This)->lpVtbl -> get_EnumOptions(This,pgrfEnumFlags) ) 

#define IShellNameSpace_put_EnumOptions(This,lVal)	\
    ( (This)->lpVtbl -> put_EnumOptions(This,lVal) ) 

#define IShellNameSpace_get_SelectedItem(This,pItem)	\
    ( (This)->lpVtbl -> get_SelectedItem(This,pItem) ) 

#define IShellNameSpace_put_SelectedItem(This,pItem)	\
    ( (This)->lpVtbl -> put_SelectedItem(This,pItem) ) 

#define IShellNameSpace_get_Root(This,pvar)	\
    ( (This)->lpVtbl -> get_Root(This,pvar) ) 

#define IShellNameSpace_put_Root(This,var)	\
    ( (This)->lpVtbl -> put_Root(This,var) ) 

#define IShellNameSpace_get_Depth(This,piDepth)	\
    ( (This)->lpVtbl -> get_Depth(This,piDepth) ) 

#define IShellNameSpace_put_Depth(This,iDepth)	\
    ( (This)->lpVtbl -> put_Depth(This,iDepth) ) 

#define IShellNameSpace_get_Mode(This,puMode)	\
    ( (This)->lpVtbl -> get_Mode(This,puMode) ) 

#define IShellNameSpace_put_Mode(This,uMode)	\
    ( (This)->lpVtbl -> put_Mode(This,uMode) ) 

#define IShellNameSpace_get_Flags(This,pdwFlags)	\
    ( (This)->lpVtbl -> get_Flags(This,pdwFlags) ) 

#define IShellNameSpace_put_Flags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_Flags(This,dwFlags) ) 

#define IShellNameSpace_put_TVFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_TVFlags(This,dwFlags) ) 

#define IShellNameSpace_get_TVFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> get_TVFlags(This,dwFlags) ) 

#define IShellNameSpace_get_Columns(This,bstrColumns)	\
    ( (This)->lpVtbl -> get_Columns(This,bstrColumns) ) 

#define IShellNameSpace_put_Columns(This,bstrColumns)	\
    ( (This)->lpVtbl -> put_Columns(This,bstrColumns) ) 

#define IShellNameSpace_get_CountViewTypes(This,piTypes)	\
    ( (This)->lpVtbl -> get_CountViewTypes(This,piTypes) ) 

#define IShellNameSpace_SetViewType(This,iType)	\
    ( (This)->lpVtbl -> SetViewType(This,iType) ) 

#define IShellNameSpace_SelectedItems(This,ppid)	\
    ( (This)->lpVtbl -> SelectedItems(This,ppid) ) 

#define IShellNameSpace_Expand(This,var,iDepth)	\
    ( (This)->lpVtbl -> Expand(This,var,iDepth) ) 

#define IShellNameSpace_UnselectAll(This)	\
    ( (This)->lpVtbl -> UnselectAll(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellNameSpace_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_ShellNameSpace;

#ifdef __cplusplus

class DECLSPEC_UUID("55136805-B2DE-11D1-B9F2-00A0C98BC547")
ShellNameSpace;
#endif

#ifndef __IScriptErrorList_INTERFACE_DEFINED__
#define __IScriptErrorList_INTERFACE_DEFINED__

/* interface IScriptErrorList */
/* [object][dual][hidden][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_IScriptErrorList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F3470F24-15FD-11d2-BB2E-00805FF7EFCA")
    IScriptErrorList : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE advanceError( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE retreatError( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE canAdvanceError( 
            /* [retval][out] */ __RPC__out BOOL *pfCanAdvance) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE canRetreatError( 
            /* [retval][out] */ __RPC__out BOOL *pfCanRetreat) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getErrorLine( 
            /* [retval][out] */ __RPC__out LONG *plLine) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getErrorChar( 
            /* [retval][out] */ __RPC__out LONG *plChar) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getErrorCode( 
            /* [retval][out] */ __RPC__out LONG *plCode) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getErrorMsg( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstr) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getErrorUrl( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstr) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getAlwaysShowLockState( 
            /* [retval][out] */ __RPC__out BOOL *pfAlwaysShowLocked) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getDetailsPaneOpen( 
            /* [retval][out] */ __RPC__out BOOL *pfDetailsPaneOpen) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE setDetailsPaneOpen( 
            BOOL fDetailsPaneOpen) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getPerErrorDisplay( 
            /* [retval][out] */ __RPC__out BOOL *pfPerErrorDisplay) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE setPerErrorDisplay( 
            BOOL fPerErrorDisplay) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IScriptErrorListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IScriptErrorList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IScriptErrorList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IScriptErrorList * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IScriptErrorList * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IScriptErrorList * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IScriptErrorList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IScriptErrorList * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IScriptErrorList, advanceError)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *advanceError )( 
            __RPC__in IScriptErrorList * This);
        
        DECLSPEC_XFGVIRT(IScriptErrorList, retreatError)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *retreatError )( 
            __RPC__in IScriptErrorList * This);
        
        DECLSPEC_XFGVIRT(IScriptErrorList, canAdvanceError)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *canAdvanceError )( 
            __RPC__in IScriptErrorList * This,
            /* [retval][out] */ __RPC__out BOOL *pfCanAdvance);
        
        DECLSPEC_XFGVIRT(IScriptErrorList, canRetreatError)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *canRetreatError )( 
            __RPC__in IScriptErrorList * This,
            /* [retval][out] */ __RPC__out BOOL *pfCanRetreat);
        
        DECLSPEC_XFGVIRT(IScriptErrorList, getErrorLine)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getErrorLine )( 
            __RPC__in IScriptErrorList * This,
            /* [retval][out] */ __RPC__out LONG *plLine);
        
        DECLSPEC_XFGVIRT(IScriptErrorList, getErrorChar)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getErrorChar )( 
            __RPC__in IScriptErrorList * This,
            /* [retval][out] */ __RPC__out LONG *plChar);
        
        DECLSPEC_XFGVIRT(IScriptErrorList, getErrorCode)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getErrorCode )( 
            __RPC__in IScriptErrorList * This,
            /* [retval][out] */ __RPC__out LONG *plCode);
        
        DECLSPEC_XFGVIRT(IScriptErrorList, getErrorMsg)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getErrorMsg )( 
            __RPC__in IScriptErrorList * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstr);
        
        DECLSPEC_XFGVIRT(IScriptErrorList, getErrorUrl)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getErrorUrl )( 
            __RPC__in IScriptErrorList * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstr);
        
        DECLSPEC_XFGVIRT(IScriptErrorList, getAlwaysShowLockState)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getAlwaysShowLockState )( 
            __RPC__in IScriptErrorList * This,
            /* [retval][out] */ __RPC__out BOOL *pfAlwaysShowLocked);
        
        DECLSPEC_XFGVIRT(IScriptErrorList, getDetailsPaneOpen)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getDetailsPaneOpen )( 
            __RPC__in IScriptErrorList * This,
            /* [retval][out] */ __RPC__out BOOL *pfDetailsPaneOpen);
        
        DECLSPEC_XFGVIRT(IScriptErrorList, setDetailsPaneOpen)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *setDetailsPaneOpen )( 
            __RPC__in IScriptErrorList * This,
            BOOL fDetailsPaneOpen);
        
        DECLSPEC_XFGVIRT(IScriptErrorList, getPerErrorDisplay)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getPerErrorDisplay )( 
            __RPC__in IScriptErrorList * This,
            /* [retval][out] */ __RPC__out BOOL *pfPerErrorDisplay);
        
        DECLSPEC_XFGVIRT(IScriptErrorList, setPerErrorDisplay)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *setPerErrorDisplay )( 
            __RPC__in IScriptErrorList * This,
            BOOL fPerErrorDisplay);
        
        END_INTERFACE
    } IScriptErrorListVtbl;

    interface IScriptErrorList
    {
        CONST_VTBL struct IScriptErrorListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IScriptErrorList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IScriptErrorList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IScriptErrorList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IScriptErrorList_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IScriptErrorList_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IScriptErrorList_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IScriptErrorList_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IScriptErrorList_advanceError(This)	\
    ( (This)->lpVtbl -> advanceError(This) ) 

#define IScriptErrorList_retreatError(This)	\
    ( (This)->lpVtbl -> retreatError(This) ) 

#define IScriptErrorList_canAdvanceError(This,pfCanAdvance)	\
    ( (This)->lpVtbl -> canAdvanceError(This,pfCanAdvance) ) 

#define IScriptErrorList_canRetreatError(This,pfCanRetreat)	\
    ( (This)->lpVtbl -> canRetreatError(This,pfCanRetreat) ) 

#define IScriptErrorList_getErrorLine(This,plLine)	\
    ( (This)->lpVtbl -> getErrorLine(This,plLine) ) 

#define IScriptErrorList_getErrorChar(This,plChar)	\
    ( (This)->lpVtbl -> getErrorChar(This,plChar) ) 

#define IScriptErrorList_getErrorCode(This,plCode)	\
    ( (This)->lpVtbl -> getErrorCode(This,plCode) ) 

#define IScriptErrorList_getErrorMsg(This,pstr)	\
    ( (This)->lpVtbl -> getErrorMsg(This,pstr) ) 

#define IScriptErrorList_getErrorUrl(This,pstr)	\
    ( (This)->lpVtbl -> getErrorUrl(This,pstr) ) 

#define IScriptErrorList_getAlwaysShowLockState(This,pfAlwaysShowLocked)	\
    ( (This)->lpVtbl -> getAlwaysShowLockState(This,pfAlwaysShowLocked) ) 

#define IScriptErrorList_getDetailsPaneOpen(This,pfDetailsPaneOpen)	\
    ( (This)->lpVtbl -> getDetailsPaneOpen(This,pfDetailsPaneOpen) ) 

#define IScriptErrorList_setDetailsPaneOpen(This,fDetailsPaneOpen)	\
    ( (This)->lpVtbl -> setDetailsPaneOpen(This,fDetailsPaneOpen) ) 

#define IScriptErrorList_getPerErrorDisplay(This,pfPerErrorDisplay)	\
    ( (This)->lpVtbl -> getPerErrorDisplay(This,pfPerErrorDisplay) ) 

#define IScriptErrorList_setPerErrorDisplay(This,fPerErrorDisplay)	\
    ( (This)->lpVtbl -> setPerErrorDisplay(This,fPerErrorDisplay) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IScriptErrorList_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_CScriptErrorList;

#ifdef __cplusplus

class DECLSPEC_UUID("EFD01300-160F-11d2-BB2E-00805FF7EFCA")
CScriptErrorList;
#endif
#endif /* __SHDocVw_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_exdisp_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_exdisp_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_exdisp_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


