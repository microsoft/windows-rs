

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

#ifndef __shappmgr_h__
#define __shappmgr_h__

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

#ifndef __IShellApp_FWD_DEFINED__
#define __IShellApp_FWD_DEFINED__
typedef interface IShellApp IShellApp;

#endif 	/* __IShellApp_FWD_DEFINED__ */


#ifndef __IPublishedApp_FWD_DEFINED__
#define __IPublishedApp_FWD_DEFINED__
typedef interface IPublishedApp IPublishedApp;

#endif 	/* __IPublishedApp_FWD_DEFINED__ */


#ifndef __IPublishedApp2_FWD_DEFINED__
#define __IPublishedApp2_FWD_DEFINED__
typedef interface IPublishedApp2 IPublishedApp2;

#endif 	/* __IPublishedApp2_FWD_DEFINED__ */


#ifndef __IEnumPublishedApps_FWD_DEFINED__
#define __IEnumPublishedApps_FWD_DEFINED__
typedef interface IEnumPublishedApps IEnumPublishedApps;

#endif 	/* __IEnumPublishedApps_FWD_DEFINED__ */


#ifndef __IAppPublisher_FWD_DEFINED__
#define __IAppPublisher_FWD_DEFINED__
typedef interface IAppPublisher IAppPublisher;

#endif 	/* __IAppPublisher_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "appmgmt.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_shappmgr_0000_0000 */
/* [local] */ 

#ifndef _SHAPPMGR_H_
#define _SHAPPMGR_H_
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_shappmgr_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shappmgr_0000_0000_v0_0_s_ifspec;

#ifndef __IShellApp_INTERFACE_DEFINED__
#define __IShellApp_INTERFACE_DEFINED__

/* interface IShellApp */
/* [object][uuid] */ 

typedef 
enum _tagAppInfoFlags
    {
        AIM_DISPLAYNAME	= 0x1,
        AIM_VERSION	= 0x2,
        AIM_PUBLISHER	= 0x4,
        AIM_PRODUCTID	= 0x8,
        AIM_REGISTEREDOWNER	= 0x10,
        AIM_REGISTEREDCOMPANY	= 0x20,
        AIM_LANGUAGE	= 0x40,
        AIM_SUPPORTURL	= 0x80,
        AIM_SUPPORTTELEPHONE	= 0x100,
        AIM_HELPLINK	= 0x200,
        AIM_INSTALLLOCATION	= 0x400,
        AIM_INSTALLSOURCE	= 0x800,
        AIM_INSTALLDATE	= 0x1000,
        AIM_CONTACT	= 0x4000,
        AIM_COMMENTS	= 0x8000,
        AIM_IMAGE	= 0x20000,
        AIM_READMEURL	= 0x40000,
        AIM_UPDATEINFOURL	= 0x80000
    } 	APPINFODATAFLAGS;

typedef struct _AppInfoData
    {
    DWORD cbSize;
    DWORD dwMask;
    LPWSTR pszDisplayName;
    LPWSTR pszVersion;
    LPWSTR pszPublisher;
    LPWSTR pszProductID;
    LPWSTR pszRegisteredOwner;
    LPWSTR pszRegisteredCompany;
    LPWSTR pszLanguage;
    LPWSTR pszSupportUrl;
    LPWSTR pszSupportTelephone;
    LPWSTR pszHelpLink;
    LPWSTR pszInstallLocation;
    LPWSTR pszInstallSource;
    LPWSTR pszInstallDate;
    LPWSTR pszContact;
    LPWSTR pszComments;
    LPWSTR pszImage;
    LPWSTR pszReadmeUrl;
    LPWSTR pszUpdateInfoUrl;
    } 	APPINFODATA;

typedef struct _AppInfoData *PAPPINFODATA;

typedef 
enum _tagAppActionFlags
    {
        APPACTION_INSTALL	= 0x1,
        APPACTION_UNINSTALL	= 0x2,
        APPACTION_MODIFY	= 0x4,
        APPACTION_REPAIR	= 0x8,
        APPACTION_UPGRADE	= 0x10,
        APPACTION_CANGETSIZE	= 0x20,
        APPACTION_MODIFYREMOVE	= 0x80,
        APPACTION_ADDLATER	= 0x100,
        APPACTION_UNSCHEDULE	= 0x200
    } 	APPACTIONFLAGS;

typedef struct _tagSlowAppInfo
    {
    ULONGLONG ullSize;
    FILETIME ftLastUsed;
    int iTimesUsed;
    LPWSTR pszImage;
    } 	SLOWAPPINFO;

typedef struct _tagSlowAppInfo *PSLOWAPPINFO;


EXTERN_C const IID IID_IShellApp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A3E14960-935F-11D1-B8B8-006008059382")
    IShellApp : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAppInfo( 
            /* [out][in] */ __RPC__inout PAPPINFODATA pai) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPossibleActions( 
            /* [out] */ __RPC__out DWORD *pdwActions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSlowAppInfo( 
            /* [out] */ __RPC__out PSLOWAPPINFO psaid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCachedSlowAppInfo( 
            /* [out] */ __RPC__out PSLOWAPPINFO psaid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsInstalled( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IShellAppVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IShellApp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IShellApp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IShellApp * This);
        
        DECLSPEC_XFGVIRT(IShellApp, GetAppInfo)
        HRESULT ( STDMETHODCALLTYPE *GetAppInfo )( 
            __RPC__in IShellApp * This,
            /* [out][in] */ __RPC__inout PAPPINFODATA pai);
        
        DECLSPEC_XFGVIRT(IShellApp, GetPossibleActions)
        HRESULT ( STDMETHODCALLTYPE *GetPossibleActions )( 
            __RPC__in IShellApp * This,
            /* [out] */ __RPC__out DWORD *pdwActions);
        
        DECLSPEC_XFGVIRT(IShellApp, GetSlowAppInfo)
        HRESULT ( STDMETHODCALLTYPE *GetSlowAppInfo )( 
            __RPC__in IShellApp * This,
            /* [out] */ __RPC__out PSLOWAPPINFO psaid);
        
        DECLSPEC_XFGVIRT(IShellApp, GetCachedSlowAppInfo)
        HRESULT ( STDMETHODCALLTYPE *GetCachedSlowAppInfo )( 
            __RPC__in IShellApp * This,
            /* [out] */ __RPC__out PSLOWAPPINFO psaid);
        
        DECLSPEC_XFGVIRT(IShellApp, IsInstalled)
        HRESULT ( STDMETHODCALLTYPE *IsInstalled )( 
            __RPC__in IShellApp * This);
        
        END_INTERFACE
    } IShellAppVtbl;

    interface IShellApp
    {
        CONST_VTBL struct IShellAppVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IShellApp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IShellApp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IShellApp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IShellApp_GetAppInfo(This,pai)	\
    ( (This)->lpVtbl -> GetAppInfo(This,pai) ) 

#define IShellApp_GetPossibleActions(This,pdwActions)	\
    ( (This)->lpVtbl -> GetPossibleActions(This,pdwActions) ) 

#define IShellApp_GetSlowAppInfo(This,psaid)	\
    ( (This)->lpVtbl -> GetSlowAppInfo(This,psaid) ) 

#define IShellApp_GetCachedSlowAppInfo(This,psaid)	\
    ( (This)->lpVtbl -> GetCachedSlowAppInfo(This,psaid) ) 

#define IShellApp_IsInstalled(This)	\
    ( (This)->lpVtbl -> IsInstalled(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IShellApp_INTERFACE_DEFINED__ */


#ifndef __IPublishedApp_INTERFACE_DEFINED__
#define __IPublishedApp_INTERFACE_DEFINED__

/* interface IPublishedApp */
/* [object][uuid] */ 

typedef 
enum _tagPublishedAppInfoFlags
    {
        PAI_SOURCE	= 0x1,
        PAI_ASSIGNEDTIME	= 0x2,
        PAI_PUBLISHEDTIME	= 0x4,
        PAI_SCHEDULEDTIME	= 0x8,
        PAI_EXPIRETIME	= 0x10
    } 	PUBAPPINFOFLAGS;

typedef struct _PubAppInfo
    {
    DWORD cbSize;
    DWORD dwMask;
    LPWSTR pszSource;
    SYSTEMTIME stAssigned;
    SYSTEMTIME stPublished;
    SYSTEMTIME stScheduled;
    SYSTEMTIME stExpire;
    } 	PUBAPPINFO;

typedef struct _PubAppInfo *PPUBAPPINFO;


EXTERN_C const IID IID_IPublishedApp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1BC752E0-9046-11D1-B8B3-006008059382")
    IPublishedApp : public IShellApp
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Install( 
            /* [unique][in] */ __RPC__in_opt LPSYSTEMTIME pstInstall) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPublishedAppInfo( 
            /* [out][in] */ __RPC__inout PPUBAPPINFO ppai) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unschedule( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPublishedAppVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPublishedApp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPublishedApp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPublishedApp * This);
        
        DECLSPEC_XFGVIRT(IShellApp, GetAppInfo)
        HRESULT ( STDMETHODCALLTYPE *GetAppInfo )( 
            __RPC__in IPublishedApp * This,
            /* [out][in] */ __RPC__inout PAPPINFODATA pai);
        
        DECLSPEC_XFGVIRT(IShellApp, GetPossibleActions)
        HRESULT ( STDMETHODCALLTYPE *GetPossibleActions )( 
            __RPC__in IPublishedApp * This,
            /* [out] */ __RPC__out DWORD *pdwActions);
        
        DECLSPEC_XFGVIRT(IShellApp, GetSlowAppInfo)
        HRESULT ( STDMETHODCALLTYPE *GetSlowAppInfo )( 
            __RPC__in IPublishedApp * This,
            /* [out] */ __RPC__out PSLOWAPPINFO psaid);
        
        DECLSPEC_XFGVIRT(IShellApp, GetCachedSlowAppInfo)
        HRESULT ( STDMETHODCALLTYPE *GetCachedSlowAppInfo )( 
            __RPC__in IPublishedApp * This,
            /* [out] */ __RPC__out PSLOWAPPINFO psaid);
        
        DECLSPEC_XFGVIRT(IShellApp, IsInstalled)
        HRESULT ( STDMETHODCALLTYPE *IsInstalled )( 
            __RPC__in IPublishedApp * This);
        
        DECLSPEC_XFGVIRT(IPublishedApp, Install)
        HRESULT ( STDMETHODCALLTYPE *Install )( 
            __RPC__in IPublishedApp * This,
            /* [unique][in] */ __RPC__in_opt LPSYSTEMTIME pstInstall);
        
        DECLSPEC_XFGVIRT(IPublishedApp, GetPublishedAppInfo)
        HRESULT ( STDMETHODCALLTYPE *GetPublishedAppInfo )( 
            __RPC__in IPublishedApp * This,
            /* [out][in] */ __RPC__inout PPUBAPPINFO ppai);
        
        DECLSPEC_XFGVIRT(IPublishedApp, Unschedule)
        HRESULT ( STDMETHODCALLTYPE *Unschedule )( 
            __RPC__in IPublishedApp * This);
        
        END_INTERFACE
    } IPublishedAppVtbl;

    interface IPublishedApp
    {
        CONST_VTBL struct IPublishedAppVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPublishedApp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPublishedApp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPublishedApp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPublishedApp_GetAppInfo(This,pai)	\
    ( (This)->lpVtbl -> GetAppInfo(This,pai) ) 

#define IPublishedApp_GetPossibleActions(This,pdwActions)	\
    ( (This)->lpVtbl -> GetPossibleActions(This,pdwActions) ) 

#define IPublishedApp_GetSlowAppInfo(This,psaid)	\
    ( (This)->lpVtbl -> GetSlowAppInfo(This,psaid) ) 

#define IPublishedApp_GetCachedSlowAppInfo(This,psaid)	\
    ( (This)->lpVtbl -> GetCachedSlowAppInfo(This,psaid) ) 

#define IPublishedApp_IsInstalled(This)	\
    ( (This)->lpVtbl -> IsInstalled(This) ) 


#define IPublishedApp_Install(This,pstInstall)	\
    ( (This)->lpVtbl -> Install(This,pstInstall) ) 

#define IPublishedApp_GetPublishedAppInfo(This,ppai)	\
    ( (This)->lpVtbl -> GetPublishedAppInfo(This,ppai) ) 

#define IPublishedApp_Unschedule(This)	\
    ( (This)->lpVtbl -> Unschedule(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPublishedApp_INTERFACE_DEFINED__ */


#ifndef __IPublishedApp2_INTERFACE_DEFINED__
#define __IPublishedApp2_INTERFACE_DEFINED__

/* interface IPublishedApp2 */
/* [object][uuid] */ 


EXTERN_C const IID IID_IPublishedApp2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("12B81347-1B3A-4A04-AA61-3F768B67FD7E")
    IPublishedApp2 : public IPublishedApp
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Install2( 
            /* [unique][in] */ __RPC__in_opt LPSYSTEMTIME pstInstall,
            /* [unique][in] */ __RPC__in_opt HWND hwndParent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPublishedApp2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPublishedApp2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPublishedApp2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPublishedApp2 * This);
        
        DECLSPEC_XFGVIRT(IShellApp, GetAppInfo)
        HRESULT ( STDMETHODCALLTYPE *GetAppInfo )( 
            __RPC__in IPublishedApp2 * This,
            /* [out][in] */ __RPC__inout PAPPINFODATA pai);
        
        DECLSPEC_XFGVIRT(IShellApp, GetPossibleActions)
        HRESULT ( STDMETHODCALLTYPE *GetPossibleActions )( 
            __RPC__in IPublishedApp2 * This,
            /* [out] */ __RPC__out DWORD *pdwActions);
        
        DECLSPEC_XFGVIRT(IShellApp, GetSlowAppInfo)
        HRESULT ( STDMETHODCALLTYPE *GetSlowAppInfo )( 
            __RPC__in IPublishedApp2 * This,
            /* [out] */ __RPC__out PSLOWAPPINFO psaid);
        
        DECLSPEC_XFGVIRT(IShellApp, GetCachedSlowAppInfo)
        HRESULT ( STDMETHODCALLTYPE *GetCachedSlowAppInfo )( 
            __RPC__in IPublishedApp2 * This,
            /* [out] */ __RPC__out PSLOWAPPINFO psaid);
        
        DECLSPEC_XFGVIRT(IShellApp, IsInstalled)
        HRESULT ( STDMETHODCALLTYPE *IsInstalled )( 
            __RPC__in IPublishedApp2 * This);
        
        DECLSPEC_XFGVIRT(IPublishedApp, Install)
        HRESULT ( STDMETHODCALLTYPE *Install )( 
            __RPC__in IPublishedApp2 * This,
            /* [unique][in] */ __RPC__in_opt LPSYSTEMTIME pstInstall);
        
        DECLSPEC_XFGVIRT(IPublishedApp, GetPublishedAppInfo)
        HRESULT ( STDMETHODCALLTYPE *GetPublishedAppInfo )( 
            __RPC__in IPublishedApp2 * This,
            /* [out][in] */ __RPC__inout PPUBAPPINFO ppai);
        
        DECLSPEC_XFGVIRT(IPublishedApp, Unschedule)
        HRESULT ( STDMETHODCALLTYPE *Unschedule )( 
            __RPC__in IPublishedApp2 * This);
        
        DECLSPEC_XFGVIRT(IPublishedApp2, Install2)
        HRESULT ( STDMETHODCALLTYPE *Install2 )( 
            __RPC__in IPublishedApp2 * This,
            /* [unique][in] */ __RPC__in_opt LPSYSTEMTIME pstInstall,
            /* [unique][in] */ __RPC__in_opt HWND hwndParent);
        
        END_INTERFACE
    } IPublishedApp2Vtbl;

    interface IPublishedApp2
    {
        CONST_VTBL struct IPublishedApp2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPublishedApp2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPublishedApp2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPublishedApp2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPublishedApp2_GetAppInfo(This,pai)	\
    ( (This)->lpVtbl -> GetAppInfo(This,pai) ) 

#define IPublishedApp2_GetPossibleActions(This,pdwActions)	\
    ( (This)->lpVtbl -> GetPossibleActions(This,pdwActions) ) 

#define IPublishedApp2_GetSlowAppInfo(This,psaid)	\
    ( (This)->lpVtbl -> GetSlowAppInfo(This,psaid) ) 

#define IPublishedApp2_GetCachedSlowAppInfo(This,psaid)	\
    ( (This)->lpVtbl -> GetCachedSlowAppInfo(This,psaid) ) 

#define IPublishedApp2_IsInstalled(This)	\
    ( (This)->lpVtbl -> IsInstalled(This) ) 


#define IPublishedApp2_Install(This,pstInstall)	\
    ( (This)->lpVtbl -> Install(This,pstInstall) ) 

#define IPublishedApp2_GetPublishedAppInfo(This,ppai)	\
    ( (This)->lpVtbl -> GetPublishedAppInfo(This,ppai) ) 

#define IPublishedApp2_Unschedule(This)	\
    ( (This)->lpVtbl -> Unschedule(This) ) 


#define IPublishedApp2_Install2(This,pstInstall,hwndParent)	\
    ( (This)->lpVtbl -> Install2(This,pstInstall,hwndParent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPublishedApp2_INTERFACE_DEFINED__ */


#ifndef __IEnumPublishedApps_INTERFACE_DEFINED__
#define __IEnumPublishedApps_INTERFACE_DEFINED__

/* interface IEnumPublishedApps */
/* [object][uuid] */ 


EXTERN_C const IID IID_IEnumPublishedApps;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0B124F8C-91F0-11D1-B8B5-006008059382")
    IEnumPublishedApps : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [out] */ __RPC__deref_out_opt IPublishedApp **pia) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumPublishedAppsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumPublishedApps * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumPublishedApps * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumPublishedApps * This);
        
        DECLSPEC_XFGVIRT(IEnumPublishedApps, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumPublishedApps * This,
            /* [out] */ __RPC__deref_out_opt IPublishedApp **pia);
        
        DECLSPEC_XFGVIRT(IEnumPublishedApps, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumPublishedApps * This);
        
        END_INTERFACE
    } IEnumPublishedAppsVtbl;

    interface IEnumPublishedApps
    {
        CONST_VTBL struct IEnumPublishedAppsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumPublishedApps_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumPublishedApps_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumPublishedApps_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumPublishedApps_Next(This,pia)	\
    ( (This)->lpVtbl -> Next(This,pia) ) 

#define IEnumPublishedApps_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumPublishedApps_INTERFACE_DEFINED__ */


#ifndef __IAppPublisher_INTERFACE_DEFINED__
#define __IAppPublisher_INTERFACE_DEFINED__

/* interface IAppPublisher */
/* [object][uuid] */ 


EXTERN_C const IID IID_IAppPublisher;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("07250A10-9CF9-11D1-9076-006008059382")
    IAppPublisher : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNumberOfCategories( 
            /* [out] */ __RPC__out DWORD *pdwCat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCategories( 
            /* [out] */ __RPC__out APPCATEGORYINFOLIST *pAppCategoryList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumberOfApps( 
            /* [out] */ __RPC__out DWORD *pdwApps) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumApps( 
            /* [unique][in] */ __RPC__in_opt GUID *pAppCategoryId,
            /* [out] */ __RPC__deref_out_opt IEnumPublishedApps **ppepa) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAppPublisherVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAppPublisher * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAppPublisher * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAppPublisher * This);
        
        DECLSPEC_XFGVIRT(IAppPublisher, GetNumberOfCategories)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfCategories )( 
            __RPC__in IAppPublisher * This,
            /* [out] */ __RPC__out DWORD *pdwCat);
        
        DECLSPEC_XFGVIRT(IAppPublisher, GetCategories)
        HRESULT ( STDMETHODCALLTYPE *GetCategories )( 
            __RPC__in IAppPublisher * This,
            /* [out] */ __RPC__out APPCATEGORYINFOLIST *pAppCategoryList);
        
        DECLSPEC_XFGVIRT(IAppPublisher, GetNumberOfApps)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfApps )( 
            __RPC__in IAppPublisher * This,
            /* [out] */ __RPC__out DWORD *pdwApps);
        
        DECLSPEC_XFGVIRT(IAppPublisher, EnumApps)
        HRESULT ( STDMETHODCALLTYPE *EnumApps )( 
            __RPC__in IAppPublisher * This,
            /* [unique][in] */ __RPC__in_opt GUID *pAppCategoryId,
            /* [out] */ __RPC__deref_out_opt IEnumPublishedApps **ppepa);
        
        END_INTERFACE
    } IAppPublisherVtbl;

    interface IAppPublisher
    {
        CONST_VTBL struct IAppPublisherVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAppPublisher_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAppPublisher_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAppPublisher_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAppPublisher_GetNumberOfCategories(This,pdwCat)	\
    ( (This)->lpVtbl -> GetNumberOfCategories(This,pdwCat) ) 

#define IAppPublisher_GetCategories(This,pAppCategoryList)	\
    ( (This)->lpVtbl -> GetCategories(This,pAppCategoryList) ) 

#define IAppPublisher_GetNumberOfApps(This,pdwApps)	\
    ( (This)->lpVtbl -> GetNumberOfApps(This,pdwApps) ) 

#define IAppPublisher_EnumApps(This,pAppCategoryId,ppepa)	\
    ( (This)->lpVtbl -> EnumApps(This,pAppCategoryId,ppepa) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAppPublisher_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_shappmgr_0000_0005 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif // _SHAPPMGR_H_


extern RPC_IF_HANDLE __MIDL_itf_shappmgr_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_shappmgr_0000_0005_v0_0_s_ifspec;

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


