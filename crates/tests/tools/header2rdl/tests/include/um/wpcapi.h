

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

#ifndef __wpcapi_h__
#define __wpcapi_h__

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

#ifndef __IWPCProviderState_FWD_DEFINED__
#define __IWPCProviderState_FWD_DEFINED__
typedef interface IWPCProviderState IWPCProviderState;

#endif 	/* __IWPCProviderState_FWD_DEFINED__ */


#ifndef __IWPCProviderConfig_FWD_DEFINED__
#define __IWPCProviderConfig_FWD_DEFINED__
typedef interface IWPCProviderConfig IWPCProviderConfig;

#endif 	/* __IWPCProviderConfig_FWD_DEFINED__ */


#ifndef __IWPCSettings_FWD_DEFINED__
#define __IWPCSettings_FWD_DEFINED__
typedef interface IWPCSettings IWPCSettings;

#endif 	/* __IWPCSettings_FWD_DEFINED__ */


#ifndef __IWPCGamesSettings_FWD_DEFINED__
#define __IWPCGamesSettings_FWD_DEFINED__
typedef interface IWPCGamesSettings IWPCGamesSettings;

#endif 	/* __IWPCGamesSettings_FWD_DEFINED__ */


#ifndef __IWPCWebSettings_FWD_DEFINED__
#define __IWPCWebSettings_FWD_DEFINED__
typedef interface IWPCWebSettings IWPCWebSettings;

#endif 	/* __IWPCWebSettings_FWD_DEFINED__ */


#ifndef __IWindowsParentalControlsCore_FWD_DEFINED__
#define __IWindowsParentalControlsCore_FWD_DEFINED__
typedef interface IWindowsParentalControlsCore IWindowsParentalControlsCore;

#endif 	/* __IWindowsParentalControlsCore_FWD_DEFINED__ */


#ifndef __IWindowsParentalControls_FWD_DEFINED__
#define __IWindowsParentalControls_FWD_DEFINED__
typedef interface IWindowsParentalControls IWindowsParentalControls;

#endif 	/* __IWindowsParentalControls_FWD_DEFINED__ */


#ifndef __IWPCProviderSupport_FWD_DEFINED__
#define __IWPCProviderSupport_FWD_DEFINED__
typedef interface IWPCProviderSupport IWPCProviderSupport;

#endif 	/* __IWPCProviderSupport_FWD_DEFINED__ */


#ifndef __WpcSettingsProvider_FWD_DEFINED__
#define __WpcSettingsProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class WpcSettingsProvider WpcSettingsProvider;
#else
typedef struct WpcSettingsProvider WpcSettingsProvider;
#endif /* __cplusplus */

#endif 	/* __WpcSettingsProvider_FWD_DEFINED__ */


#ifndef __WpcProviderSupport_FWD_DEFINED__
#define __WpcProviderSupport_FWD_DEFINED__

#ifdef __cplusplus
typedef class WpcProviderSupport WpcProviderSupport;
#else
typedef struct WpcProviderSupport WpcProviderSupport;
#endif /* __cplusplus */

#endif 	/* __WpcProviderSupport_FWD_DEFINED__ */


#ifndef __WindowsParentalControls_FWD_DEFINED__
#define __WindowsParentalControls_FWD_DEFINED__

#ifdef __cplusplus
typedef class WindowsParentalControls WindowsParentalControls;
#else
typedef struct WindowsParentalControls WindowsParentalControls;
#endif /* __cplusplus */

#endif 	/* __WindowsParentalControls_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wpcapi_0000_0000 */
/* [local] */ 

/*******************************************************************************/
/*                                                                             */
/*    Copyright (C) Microsoft Corporation.  All rights reserved.                 */
/*                                                                             */
/*    Interfaces for Windows Parental Controls clients.                        */
/*                                                                             */
/*******************************************************************************/
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_wpcapi_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wpcapi_0000_0000_v0_0_s_ifspec;

#ifndef __IWPCProviderState_INTERFACE_DEFINED__
#define __IWPCProviderState_INTERFACE_DEFINED__

/* interface IWPCProviderState */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWPCProviderState;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50B6A267-C4BD-450b-ADB5-759073837C9E")
    IWPCProviderState : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Enable( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Disable( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWPCProviderStateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWPCProviderState * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWPCProviderState * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWPCProviderState * This);
        
        DECLSPEC_XFGVIRT(IWPCProviderState, Enable)
        HRESULT ( STDMETHODCALLTYPE *Enable )( 
            __RPC__in IWPCProviderState * This);
        
        DECLSPEC_XFGVIRT(IWPCProviderState, Disable)
        HRESULT ( STDMETHODCALLTYPE *Disable )( 
            __RPC__in IWPCProviderState * This);
        
        END_INTERFACE
    } IWPCProviderStateVtbl;

    interface IWPCProviderState
    {
        CONST_VTBL struct IWPCProviderStateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWPCProviderState_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWPCProviderState_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWPCProviderState_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWPCProviderState_Enable(This)	\
    ( (This)->lpVtbl -> Enable(This) ) 

#define IWPCProviderState_Disable(This)	\
    ( (This)->lpVtbl -> Disable(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWPCProviderState_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wpcapi_0000_0001 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum tagWPCFLAG_OVERRIDE
    {
        WPCFLAG_APPLICATION	= 0x1
    } 	WPCFLAG_OVERRIDE;



extern RPC_IF_HANDLE __MIDL_itf_wpcapi_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wpcapi_0000_0001_v0_0_s_ifspec;

#ifndef __IWPCProviderConfig_INTERFACE_DEFINED__
#define __IWPCProviderConfig_INTERFACE_DEFINED__

/* interface IWPCProviderConfig */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWPCProviderConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BEF54196-2D02-4a26-B6E5-D65AF295D0F1")
    IWPCProviderConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetUserSummary( 
            /* [in] */ __RPC__in BSTR bstrSID,
            /* [string][out] */ __RPC__deref_out_opt_string BSTR *pbstrUserSummary) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Configure( 
            /* [unique][in] */ __RPC__in_opt HWND hWnd,
            /* [string][in] */ __RPC__in_string BSTR bstrSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestOverride( 
            /* [unique][in] */ __RPC__in_opt HWND hWnd,
            /* [string][in] */ __RPC__in_string BSTR bstrPath,
            /* [in] */ DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWPCProviderConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWPCProviderConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWPCProviderConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWPCProviderConfig * This);
        
        DECLSPEC_XFGVIRT(IWPCProviderConfig, GetUserSummary)
        HRESULT ( STDMETHODCALLTYPE *GetUserSummary )( 
            __RPC__in IWPCProviderConfig * This,
            /* [in] */ __RPC__in BSTR bstrSID,
            /* [string][out] */ __RPC__deref_out_opt_string BSTR *pbstrUserSummary);
        
        DECLSPEC_XFGVIRT(IWPCProviderConfig, Configure)
        HRESULT ( STDMETHODCALLTYPE *Configure )( 
            __RPC__in IWPCProviderConfig * This,
            /* [unique][in] */ __RPC__in_opt HWND hWnd,
            /* [string][in] */ __RPC__in_string BSTR bstrSID);
        
        DECLSPEC_XFGVIRT(IWPCProviderConfig, RequestOverride)
        HRESULT ( STDMETHODCALLTYPE *RequestOverride )( 
            __RPC__in IWPCProviderConfig * This,
            /* [unique][in] */ __RPC__in_opt HWND hWnd,
            /* [string][in] */ __RPC__in_string BSTR bstrPath,
            /* [in] */ DWORD dwFlags);
        
        END_INTERFACE
    } IWPCProviderConfigVtbl;

    interface IWPCProviderConfig
    {
        CONST_VTBL struct IWPCProviderConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWPCProviderConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWPCProviderConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWPCProviderConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWPCProviderConfig_GetUserSummary(This,bstrSID,pbstrUserSummary)	\
    ( (This)->lpVtbl -> GetUserSummary(This,bstrSID,pbstrUserSummary) ) 

#define IWPCProviderConfig_Configure(This,hWnd,bstrSID)	\
    ( (This)->lpVtbl -> Configure(This,hWnd,bstrSID) ) 

#define IWPCProviderConfig_RequestOverride(This,hWnd,bstrPath,dwFlags)	\
    ( (This)->lpVtbl -> RequestOverride(This,hWnd,bstrPath,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWPCProviderConfig_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wpcapi_0000_0002 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum tagWPCFLAG_RESTRICTION
    {
        WPCFLAG_NO_RESTRICTION	= 0,
        WPCFLAG_LOGGING_REQUIRED	= 0x1,
        WPCFLAG_WEB_FILTERED	= 0x2,
        WPCFLAG_HOURS_RESTRICTED	= 0x4,
        WPCFLAG_GAMES_BLOCKED	= 0x8,
        WPCFLAG_APPS_RESTRICTED	= 0x10,
        WPCFLAG_TIME_ALLOWANCE_RESTRICTED	= 0x20,
        WPCFLAG_GAMES_RESTRICTED	= 0x40
    } 	WPCFLAG_RESTRICTION;



extern RPC_IF_HANDLE __MIDL_itf_wpcapi_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wpcapi_0000_0002_v0_0_s_ifspec;

#ifndef __IWPCSettings_INTERFACE_DEFINED__
#define __IWPCSettings_INTERFACE_DEFINED__

/* interface IWPCSettings */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWPCSettings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8FDF6CA1-0189-47e4-B670-1A8A4636E340")
    IWPCSettings : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsLoggingRequired( 
            /* [out] */ __RPC__out BOOL *pfRequired) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLastSettingsChangeTime( 
            /* [out] */ __RPC__out SYSTEMTIME *pTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRestrictions( 
            /* [out] */ __RPC__out DWORD *pdwRestrictions) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWPCSettingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWPCSettings * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWPCSettings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWPCSettings * This);
        
        DECLSPEC_XFGVIRT(IWPCSettings, IsLoggingRequired)
        HRESULT ( STDMETHODCALLTYPE *IsLoggingRequired )( 
            __RPC__in IWPCSettings * This,
            /* [out] */ __RPC__out BOOL *pfRequired);
        
        DECLSPEC_XFGVIRT(IWPCSettings, GetLastSettingsChangeTime)
        HRESULT ( STDMETHODCALLTYPE *GetLastSettingsChangeTime )( 
            __RPC__in IWPCSettings * This,
            /* [out] */ __RPC__out SYSTEMTIME *pTime);
        
        DECLSPEC_XFGVIRT(IWPCSettings, GetRestrictions)
        HRESULT ( STDMETHODCALLTYPE *GetRestrictions )( 
            __RPC__in IWPCSettings * This,
            /* [out] */ __RPC__out DWORD *pdwRestrictions);
        
        END_INTERFACE
    } IWPCSettingsVtbl;

    interface IWPCSettings
    {
        CONST_VTBL struct IWPCSettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWPCSettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWPCSettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWPCSettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWPCSettings_IsLoggingRequired(This,pfRequired)	\
    ( (This)->lpVtbl -> IsLoggingRequired(This,pfRequired) ) 

#define IWPCSettings_GetLastSettingsChangeTime(This,pTime)	\
    ( (This)->lpVtbl -> GetLastSettingsChangeTime(This,pTime) ) 

#define IWPCSettings_GetRestrictions(This,pdwRestrictions)	\
    ( (This)->lpVtbl -> GetRestrictions(This,pdwRestrictions) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWPCSettings_INTERFACE_DEFINED__ */


#ifndef __IWPCGamesSettings_INTERFACE_DEFINED__
#define __IWPCGamesSettings_INTERFACE_DEFINED__

/* interface IWPCGamesSettings */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWPCGamesSettings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("95E87780-E158-489e-B452-BBB850790715")
    IWPCGamesSettings : public IWPCSettings
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsBlocked( 
            /* [in] */ GUID guidAppID,
            /* [out] */ __RPC__out DWORD *pdwReasons) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWPCGamesSettingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWPCGamesSettings * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWPCGamesSettings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWPCGamesSettings * This);
        
        DECLSPEC_XFGVIRT(IWPCSettings, IsLoggingRequired)
        HRESULT ( STDMETHODCALLTYPE *IsLoggingRequired )( 
            __RPC__in IWPCGamesSettings * This,
            /* [out] */ __RPC__out BOOL *pfRequired);
        
        DECLSPEC_XFGVIRT(IWPCSettings, GetLastSettingsChangeTime)
        HRESULT ( STDMETHODCALLTYPE *GetLastSettingsChangeTime )( 
            __RPC__in IWPCGamesSettings * This,
            /* [out] */ __RPC__out SYSTEMTIME *pTime);
        
        DECLSPEC_XFGVIRT(IWPCSettings, GetRestrictions)
        HRESULT ( STDMETHODCALLTYPE *GetRestrictions )( 
            __RPC__in IWPCGamesSettings * This,
            /* [out] */ __RPC__out DWORD *pdwRestrictions);
        
        DECLSPEC_XFGVIRT(IWPCGamesSettings, IsBlocked)
        HRESULT ( STDMETHODCALLTYPE *IsBlocked )( 
            __RPC__in IWPCGamesSettings * This,
            /* [in] */ GUID guidAppID,
            /* [out] */ __RPC__out DWORD *pdwReasons);
        
        END_INTERFACE
    } IWPCGamesSettingsVtbl;

    interface IWPCGamesSettings
    {
        CONST_VTBL struct IWPCGamesSettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWPCGamesSettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWPCGamesSettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWPCGamesSettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWPCGamesSettings_IsLoggingRequired(This,pfRequired)	\
    ( (This)->lpVtbl -> IsLoggingRequired(This,pfRequired) ) 

#define IWPCGamesSettings_GetLastSettingsChangeTime(This,pTime)	\
    ( (This)->lpVtbl -> GetLastSettingsChangeTime(This,pTime) ) 

#define IWPCGamesSettings_GetRestrictions(This,pdwRestrictions)	\
    ( (This)->lpVtbl -> GetRestrictions(This,pdwRestrictions) ) 


#define IWPCGamesSettings_IsBlocked(This,guidAppID,pdwReasons)	\
    ( (This)->lpVtbl -> IsBlocked(This,guidAppID,pdwReasons) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWPCGamesSettings_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wpcapi_0000_0004 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum tagWPCFLAG_WEB_SETTING
    {
        WPCFLAG_WEB_SETTING_NOTBLOCKED	= 0,
        WPCFLAG_WEB_SETTING_DOWNLOADSBLOCKED	= 0x1
    } 	WPCFLAG_WEB_SETTING;



extern RPC_IF_HANDLE __MIDL_itf_wpcapi_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wpcapi_0000_0004_v0_0_s_ifspec;

#ifndef __IWPCWebSettings_INTERFACE_DEFINED__
#define __IWPCWebSettings_INTERFACE_DEFINED__

/* interface IWPCWebSettings */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWPCWebSettings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FFCCBDB8-0992-4c30-B0F1-1CBB09C240AA")
    IWPCWebSettings : public IWPCSettings
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSettings( 
            /* [out] */ __RPC__out DWORD *pdwSettings) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestURLOverride( 
            /* [unique][in] */ __RPC__in_opt HWND hWnd,
            /* [in] */ __RPC__in LPCWSTR pcszURL,
            /* [in] */ DWORD cURLs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cURLs) LPCWSTR *ppcszSubURLs,
            /* [out] */ __RPC__out BOOL *pfChanged) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWPCWebSettingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWPCWebSettings * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWPCWebSettings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWPCWebSettings * This);
        
        DECLSPEC_XFGVIRT(IWPCSettings, IsLoggingRequired)
        HRESULT ( STDMETHODCALLTYPE *IsLoggingRequired )( 
            __RPC__in IWPCWebSettings * This,
            /* [out] */ __RPC__out BOOL *pfRequired);
        
        DECLSPEC_XFGVIRT(IWPCSettings, GetLastSettingsChangeTime)
        HRESULT ( STDMETHODCALLTYPE *GetLastSettingsChangeTime )( 
            __RPC__in IWPCWebSettings * This,
            /* [out] */ __RPC__out SYSTEMTIME *pTime);
        
        DECLSPEC_XFGVIRT(IWPCSettings, GetRestrictions)
        HRESULT ( STDMETHODCALLTYPE *GetRestrictions )( 
            __RPC__in IWPCWebSettings * This,
            /* [out] */ __RPC__out DWORD *pdwRestrictions);
        
        DECLSPEC_XFGVIRT(IWPCWebSettings, GetSettings)
        HRESULT ( STDMETHODCALLTYPE *GetSettings )( 
            __RPC__in IWPCWebSettings * This,
            /* [out] */ __RPC__out DWORD *pdwSettings);
        
        DECLSPEC_XFGVIRT(IWPCWebSettings, RequestURLOverride)
        HRESULT ( STDMETHODCALLTYPE *RequestURLOverride )( 
            __RPC__in IWPCWebSettings * This,
            /* [unique][in] */ __RPC__in_opt HWND hWnd,
            /* [in] */ __RPC__in LPCWSTR pcszURL,
            /* [in] */ DWORD cURLs,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cURLs) LPCWSTR *ppcszSubURLs,
            /* [out] */ __RPC__out BOOL *pfChanged);
        
        END_INTERFACE
    } IWPCWebSettingsVtbl;

    interface IWPCWebSettings
    {
        CONST_VTBL struct IWPCWebSettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWPCWebSettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWPCWebSettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWPCWebSettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWPCWebSettings_IsLoggingRequired(This,pfRequired)	\
    ( (This)->lpVtbl -> IsLoggingRequired(This,pfRequired) ) 

#define IWPCWebSettings_GetLastSettingsChangeTime(This,pTime)	\
    ( (This)->lpVtbl -> GetLastSettingsChangeTime(This,pTime) ) 

#define IWPCWebSettings_GetRestrictions(This,pdwRestrictions)	\
    ( (This)->lpVtbl -> GetRestrictions(This,pdwRestrictions) ) 


#define IWPCWebSettings_GetSettings(This,pdwSettings)	\
    ( (This)->lpVtbl -> GetSettings(This,pdwSettings) ) 

#define IWPCWebSettings_RequestURLOverride(This,hWnd,pcszURL,cURLs,ppcszSubURLs,pfChanged)	\
    ( (This)->lpVtbl -> RequestURLOverride(This,hWnd,pcszURL,cURLs,ppcszSubURLs,pfChanged) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWPCWebSettings_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wpcapi_0000_0005 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum tagWPCFLAG_VISIBILITY
    {
        WPCFLAG_WPC_VISIBLE	= 0,
        WPCFLAG_WPC_HIDDEN	= 0x1
    } 	WPCFLAG_VISIBILITY;



extern RPC_IF_HANDLE __MIDL_itf_wpcapi_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wpcapi_0000_0005_v0_0_s_ifspec;

#ifndef __IWindowsParentalControlsCore_INTERFACE_DEFINED__
#define __IWindowsParentalControlsCore_INTERFACE_DEFINED__

/* interface IWindowsParentalControlsCore */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWindowsParentalControlsCore;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4FF40A0F-3F3B-4d7c-A41B-4F39D7B44D05")
    IWindowsParentalControlsCore : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetVisibility( 
            /* [out] */ __RPC__out WPCFLAG_VISIBILITY *peVisibility) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUserSettings( 
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pcszSID,
            /* [out] */ __RPC__deref_out_opt IWPCSettings **ppSettings) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWebSettings( 
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pcszSID,
            /* [out] */ __RPC__deref_out_opt IWPCWebSettings **ppSettings) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWebFilterInfo( 
            /* [out] */ __RPC__out GUID *pguidID,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt LPWSTR *ppszName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWindowsParentalControlsCoreVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWindowsParentalControlsCore * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWindowsParentalControlsCore * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWindowsParentalControlsCore * This);
        
        DECLSPEC_XFGVIRT(IWindowsParentalControlsCore, GetVisibility)
        HRESULT ( STDMETHODCALLTYPE *GetVisibility )( 
            __RPC__in IWindowsParentalControlsCore * This,
            /* [out] */ __RPC__out WPCFLAG_VISIBILITY *peVisibility);
        
        DECLSPEC_XFGVIRT(IWindowsParentalControlsCore, GetUserSettings)
        HRESULT ( STDMETHODCALLTYPE *GetUserSettings )( 
            __RPC__in IWindowsParentalControlsCore * This,
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pcszSID,
            /* [out] */ __RPC__deref_out_opt IWPCSettings **ppSettings);
        
        DECLSPEC_XFGVIRT(IWindowsParentalControlsCore, GetWebSettings)
        HRESULT ( STDMETHODCALLTYPE *GetWebSettings )( 
            __RPC__in IWindowsParentalControlsCore * This,
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pcszSID,
            /* [out] */ __RPC__deref_out_opt IWPCWebSettings **ppSettings);
        
        DECLSPEC_XFGVIRT(IWindowsParentalControlsCore, GetWebFilterInfo)
        HRESULT ( STDMETHODCALLTYPE *GetWebFilterInfo )( 
            __RPC__in IWindowsParentalControlsCore * This,
            /* [out] */ __RPC__out GUID *pguidID,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt LPWSTR *ppszName);
        
        END_INTERFACE
    } IWindowsParentalControlsCoreVtbl;

    interface IWindowsParentalControlsCore
    {
        CONST_VTBL struct IWindowsParentalControlsCoreVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWindowsParentalControlsCore_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWindowsParentalControlsCore_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWindowsParentalControlsCore_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWindowsParentalControlsCore_GetVisibility(This,peVisibility)	\
    ( (This)->lpVtbl -> GetVisibility(This,peVisibility) ) 

#define IWindowsParentalControlsCore_GetUserSettings(This,pcszSID,ppSettings)	\
    ( (This)->lpVtbl -> GetUserSettings(This,pcszSID,ppSettings) ) 

#define IWindowsParentalControlsCore_GetWebSettings(This,pcszSID,ppSettings)	\
    ( (This)->lpVtbl -> GetWebSettings(This,pcszSID,ppSettings) ) 

#define IWindowsParentalControlsCore_GetWebFilterInfo(This,pguidID,ppszName)	\
    ( (This)->lpVtbl -> GetWebFilterInfo(This,pguidID,ppszName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWindowsParentalControlsCore_INTERFACE_DEFINED__ */


#ifndef __IWindowsParentalControls_INTERFACE_DEFINED__
#define __IWindowsParentalControls_INTERFACE_DEFINED__

/* interface IWindowsParentalControls */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWindowsParentalControls;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("28B4D88B-E072-49e6-804D-26EDBE21A7B9")
    IWindowsParentalControls : public IWindowsParentalControlsCore
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetGamesSettings( 
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pcszSID,
            /* [out] */ __RPC__deref_out_opt IWPCGamesSettings **ppSettings) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWindowsParentalControlsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWindowsParentalControls * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWindowsParentalControls * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWindowsParentalControls * This);
        
        DECLSPEC_XFGVIRT(IWindowsParentalControlsCore, GetVisibility)
        HRESULT ( STDMETHODCALLTYPE *GetVisibility )( 
            __RPC__in IWindowsParentalControls * This,
            /* [out] */ __RPC__out WPCFLAG_VISIBILITY *peVisibility);
        
        DECLSPEC_XFGVIRT(IWindowsParentalControlsCore, GetUserSettings)
        HRESULT ( STDMETHODCALLTYPE *GetUserSettings )( 
            __RPC__in IWindowsParentalControls * This,
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pcszSID,
            /* [out] */ __RPC__deref_out_opt IWPCSettings **ppSettings);
        
        DECLSPEC_XFGVIRT(IWindowsParentalControlsCore, GetWebSettings)
        HRESULT ( STDMETHODCALLTYPE *GetWebSettings )( 
            __RPC__in IWindowsParentalControls * This,
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pcszSID,
            /* [out] */ __RPC__deref_out_opt IWPCWebSettings **ppSettings);
        
        DECLSPEC_XFGVIRT(IWindowsParentalControlsCore, GetWebFilterInfo)
        HRESULT ( STDMETHODCALLTYPE *GetWebFilterInfo )( 
            __RPC__in IWindowsParentalControls * This,
            /* [out] */ __RPC__out GUID *pguidID,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt LPWSTR *ppszName);
        
        DECLSPEC_XFGVIRT(IWindowsParentalControls, GetGamesSettings)
        HRESULT ( STDMETHODCALLTYPE *GetGamesSettings )( 
            __RPC__in IWindowsParentalControls * This,
            /* [string][unique][in] */ __RPC__in_opt_string LPCWSTR pcszSID,
            /* [out] */ __RPC__deref_out_opt IWPCGamesSettings **ppSettings);
        
        END_INTERFACE
    } IWindowsParentalControlsVtbl;

    interface IWindowsParentalControls
    {
        CONST_VTBL struct IWindowsParentalControlsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWindowsParentalControls_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWindowsParentalControls_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWindowsParentalControls_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWindowsParentalControls_GetVisibility(This,peVisibility)	\
    ( (This)->lpVtbl -> GetVisibility(This,peVisibility) ) 

#define IWindowsParentalControls_GetUserSettings(This,pcszSID,ppSettings)	\
    ( (This)->lpVtbl -> GetUserSettings(This,pcszSID,ppSettings) ) 

#define IWindowsParentalControls_GetWebSettings(This,pcszSID,ppSettings)	\
    ( (This)->lpVtbl -> GetWebSettings(This,pcszSID,ppSettings) ) 

#define IWindowsParentalControls_GetWebFilterInfo(This,pguidID,ppszName)	\
    ( (This)->lpVtbl -> GetWebFilterInfo(This,pguidID,ppszName) ) 


#define IWindowsParentalControls_GetGamesSettings(This,pcszSID,ppSettings)	\
    ( (This)->lpVtbl -> GetGamesSettings(This,pcszSID,ppSettings) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWindowsParentalControls_INTERFACE_DEFINED__ */


#ifndef __IWPCProviderSupport_INTERFACE_DEFINED__
#define __IWPCProviderSupport_INTERFACE_DEFINED__

/* interface IWPCProviderSupport */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWPCProviderSupport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("41EBA572-23ED-4779-BEC1-8DF96206C44C")
    IWPCProviderSupport : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCurrent( 
            /* [out] */ __RPC__out GUID *pguidProvider) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWPCProviderSupportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWPCProviderSupport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWPCProviderSupport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWPCProviderSupport * This);
        
        DECLSPEC_XFGVIRT(IWPCProviderSupport, GetCurrent)
        HRESULT ( STDMETHODCALLTYPE *GetCurrent )( 
            __RPC__in IWPCProviderSupport * This,
            /* [out] */ __RPC__out GUID *pguidProvider);
        
        END_INTERFACE
    } IWPCProviderSupportVtbl;

    interface IWPCProviderSupport
    {
        CONST_VTBL struct IWPCProviderSupportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWPCProviderSupport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWPCProviderSupport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWPCProviderSupport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWPCProviderSupport_GetCurrent(This,pguidProvider)	\
    ( (This)->lpVtbl -> GetCurrent(This,pguidProvider) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWPCProviderSupport_INTERFACE_DEFINED__ */



#ifndef __WPCAPILib_LIBRARY_DEFINED__
#define __WPCAPILib_LIBRARY_DEFINED__

/* library WPCAPILib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_WPCAPILib;

EXTERN_C const CLSID CLSID_WpcSettingsProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("355DFFAA-3B9F-435c-B428-5D44290BC5F2")
WpcSettingsProvider;
#endif

EXTERN_C const CLSID CLSID_WpcProviderSupport;

#ifdef __cplusplus

class DECLSPEC_UUID("BB18C7A0-2186-4be0-97D8-04847B628E02")
WpcProviderSupport;
#endif

EXTERN_C const CLSID CLSID_WindowsParentalControls;

#ifdef __cplusplus

class DECLSPEC_UUID("E77CC89B-7401-4c04-8CED-149DB35ADD04")
WindowsParentalControls;
#endif
#endif /* __WPCAPILib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_wpcapi_0000_0009 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wpcapi_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wpcapi_0000_0009_v0_0_s_ifspec;

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


