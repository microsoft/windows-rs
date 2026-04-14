

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

#ifndef __urlmon_h__
#define __urlmon_h__

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

#ifndef __IPersistMoniker_FWD_DEFINED__
#define __IPersistMoniker_FWD_DEFINED__
typedef interface IPersistMoniker IPersistMoniker;

#endif 	/* __IPersistMoniker_FWD_DEFINED__ */


#ifndef __IMonikerProp_FWD_DEFINED__
#define __IMonikerProp_FWD_DEFINED__
typedef interface IMonikerProp IMonikerProp;

#endif 	/* __IMonikerProp_FWD_DEFINED__ */


#ifndef __IBindProtocol_FWD_DEFINED__
#define __IBindProtocol_FWD_DEFINED__
typedef interface IBindProtocol IBindProtocol;

#endif 	/* __IBindProtocol_FWD_DEFINED__ */


#ifndef __IBinding_FWD_DEFINED__
#define __IBinding_FWD_DEFINED__
typedef interface IBinding IBinding;

#endif 	/* __IBinding_FWD_DEFINED__ */


#ifndef __IBindStatusCallback_FWD_DEFINED__
#define __IBindStatusCallback_FWD_DEFINED__
typedef interface IBindStatusCallback IBindStatusCallback;

#endif 	/* __IBindStatusCallback_FWD_DEFINED__ */


#ifndef __IBindStatusCallbackEx_FWD_DEFINED__
#define __IBindStatusCallbackEx_FWD_DEFINED__
typedef interface IBindStatusCallbackEx IBindStatusCallbackEx;

#endif 	/* __IBindStatusCallbackEx_FWD_DEFINED__ */


#ifndef __IAuthenticate_FWD_DEFINED__
#define __IAuthenticate_FWD_DEFINED__
typedef interface IAuthenticate IAuthenticate;

#endif 	/* __IAuthenticate_FWD_DEFINED__ */


#ifndef __IAuthenticateEx_FWD_DEFINED__
#define __IAuthenticateEx_FWD_DEFINED__
typedef interface IAuthenticateEx IAuthenticateEx;

#endif 	/* __IAuthenticateEx_FWD_DEFINED__ */


#ifndef __IHttpNegotiate_FWD_DEFINED__
#define __IHttpNegotiate_FWD_DEFINED__
typedef interface IHttpNegotiate IHttpNegotiate;

#endif 	/* __IHttpNegotiate_FWD_DEFINED__ */


#ifndef __IHttpNegotiate2_FWD_DEFINED__
#define __IHttpNegotiate2_FWD_DEFINED__
typedef interface IHttpNegotiate2 IHttpNegotiate2;

#endif 	/* __IHttpNegotiate2_FWD_DEFINED__ */


#ifndef __IHttpNegotiate3_FWD_DEFINED__
#define __IHttpNegotiate3_FWD_DEFINED__
typedef interface IHttpNegotiate3 IHttpNegotiate3;

#endif 	/* __IHttpNegotiate3_FWD_DEFINED__ */


#ifndef __IWinInetFileStream_FWD_DEFINED__
#define __IWinInetFileStream_FWD_DEFINED__
typedef interface IWinInetFileStream IWinInetFileStream;

#endif 	/* __IWinInetFileStream_FWD_DEFINED__ */


#ifndef __IWindowForBindingUI_FWD_DEFINED__
#define __IWindowForBindingUI_FWD_DEFINED__
typedef interface IWindowForBindingUI IWindowForBindingUI;

#endif 	/* __IWindowForBindingUI_FWD_DEFINED__ */


#ifndef __ICodeInstall_FWD_DEFINED__
#define __ICodeInstall_FWD_DEFINED__
typedef interface ICodeInstall ICodeInstall;

#endif 	/* __ICodeInstall_FWD_DEFINED__ */


#ifndef __IUri_FWD_DEFINED__
#define __IUri_FWD_DEFINED__
typedef interface IUri IUri;

#endif 	/* __IUri_FWD_DEFINED__ */


#ifndef __IUriContainer_FWD_DEFINED__
#define __IUriContainer_FWD_DEFINED__
typedef interface IUriContainer IUriContainer;

#endif 	/* __IUriContainer_FWD_DEFINED__ */


#ifndef __IUriBuilder_FWD_DEFINED__
#define __IUriBuilder_FWD_DEFINED__
typedef interface IUriBuilder IUriBuilder;

#endif 	/* __IUriBuilder_FWD_DEFINED__ */


#ifndef __IUriBuilderFactory_FWD_DEFINED__
#define __IUriBuilderFactory_FWD_DEFINED__
typedef interface IUriBuilderFactory IUriBuilderFactory;

#endif 	/* __IUriBuilderFactory_FWD_DEFINED__ */


#ifndef __IWinInetInfo_FWD_DEFINED__
#define __IWinInetInfo_FWD_DEFINED__
typedef interface IWinInetInfo IWinInetInfo;

#endif 	/* __IWinInetInfo_FWD_DEFINED__ */


#ifndef __IHttpSecurity_FWD_DEFINED__
#define __IHttpSecurity_FWD_DEFINED__
typedef interface IHttpSecurity IHttpSecurity;

#endif 	/* __IHttpSecurity_FWD_DEFINED__ */


#ifndef __IWinInetHttpInfo_FWD_DEFINED__
#define __IWinInetHttpInfo_FWD_DEFINED__
typedef interface IWinInetHttpInfo IWinInetHttpInfo;

#endif 	/* __IWinInetHttpInfo_FWD_DEFINED__ */


#ifndef __IWinInetHttpTimeouts_FWD_DEFINED__
#define __IWinInetHttpTimeouts_FWD_DEFINED__
typedef interface IWinInetHttpTimeouts IWinInetHttpTimeouts;

#endif 	/* __IWinInetHttpTimeouts_FWD_DEFINED__ */


#ifndef __IWinInetCacheHints_FWD_DEFINED__
#define __IWinInetCacheHints_FWD_DEFINED__
typedef interface IWinInetCacheHints IWinInetCacheHints;

#endif 	/* __IWinInetCacheHints_FWD_DEFINED__ */


#ifndef __IWinInetCacheHints2_FWD_DEFINED__
#define __IWinInetCacheHints2_FWD_DEFINED__
typedef interface IWinInetCacheHints2 IWinInetCacheHints2;

#endif 	/* __IWinInetCacheHints2_FWD_DEFINED__ */


#ifndef __IBindHost_FWD_DEFINED__
#define __IBindHost_FWD_DEFINED__
typedef interface IBindHost IBindHost;

#endif 	/* __IBindHost_FWD_DEFINED__ */


#ifndef __IInternet_FWD_DEFINED__
#define __IInternet_FWD_DEFINED__
typedef interface IInternet IInternet;

#endif 	/* __IInternet_FWD_DEFINED__ */


#ifndef __IInternetBindInfo_FWD_DEFINED__
#define __IInternetBindInfo_FWD_DEFINED__
typedef interface IInternetBindInfo IInternetBindInfo;

#endif 	/* __IInternetBindInfo_FWD_DEFINED__ */


#ifndef __IInternetBindInfoEx_FWD_DEFINED__
#define __IInternetBindInfoEx_FWD_DEFINED__
typedef interface IInternetBindInfoEx IInternetBindInfoEx;

#endif 	/* __IInternetBindInfoEx_FWD_DEFINED__ */


#ifndef __IInternetProtocolRoot_FWD_DEFINED__
#define __IInternetProtocolRoot_FWD_DEFINED__
typedef interface IInternetProtocolRoot IInternetProtocolRoot;

#endif 	/* __IInternetProtocolRoot_FWD_DEFINED__ */


#ifndef __IInternetProtocol_FWD_DEFINED__
#define __IInternetProtocol_FWD_DEFINED__
typedef interface IInternetProtocol IInternetProtocol;

#endif 	/* __IInternetProtocol_FWD_DEFINED__ */


#ifndef __IInternetProtocolEx_FWD_DEFINED__
#define __IInternetProtocolEx_FWD_DEFINED__
typedef interface IInternetProtocolEx IInternetProtocolEx;

#endif 	/* __IInternetProtocolEx_FWD_DEFINED__ */


#ifndef __IInternetProtocolSink_FWD_DEFINED__
#define __IInternetProtocolSink_FWD_DEFINED__
typedef interface IInternetProtocolSink IInternetProtocolSink;

#endif 	/* __IInternetProtocolSink_FWD_DEFINED__ */


#ifndef __IInternetProtocolSinkStackable_FWD_DEFINED__
#define __IInternetProtocolSinkStackable_FWD_DEFINED__
typedef interface IInternetProtocolSinkStackable IInternetProtocolSinkStackable;

#endif 	/* __IInternetProtocolSinkStackable_FWD_DEFINED__ */


#ifndef __IInternetSession_FWD_DEFINED__
#define __IInternetSession_FWD_DEFINED__
typedef interface IInternetSession IInternetSession;

#endif 	/* __IInternetSession_FWD_DEFINED__ */


#ifndef __IInternetThreadSwitch_FWD_DEFINED__
#define __IInternetThreadSwitch_FWD_DEFINED__
typedef interface IInternetThreadSwitch IInternetThreadSwitch;

#endif 	/* __IInternetThreadSwitch_FWD_DEFINED__ */


#ifndef __IInternetPriority_FWD_DEFINED__
#define __IInternetPriority_FWD_DEFINED__
typedef interface IInternetPriority IInternetPriority;

#endif 	/* __IInternetPriority_FWD_DEFINED__ */


#ifndef __IInternetProtocolInfo_FWD_DEFINED__
#define __IInternetProtocolInfo_FWD_DEFINED__
typedef interface IInternetProtocolInfo IInternetProtocolInfo;

#endif 	/* __IInternetProtocolInfo_FWD_DEFINED__ */


#ifndef __IInternetSecurityMgrSite_FWD_DEFINED__
#define __IInternetSecurityMgrSite_FWD_DEFINED__
typedef interface IInternetSecurityMgrSite IInternetSecurityMgrSite;

#endif 	/* __IInternetSecurityMgrSite_FWD_DEFINED__ */


#ifndef __IInternetSecurityManager_FWD_DEFINED__
#define __IInternetSecurityManager_FWD_DEFINED__
typedef interface IInternetSecurityManager IInternetSecurityManager;

#endif 	/* __IInternetSecurityManager_FWD_DEFINED__ */


#ifndef __IInternetSecurityManagerEx_FWD_DEFINED__
#define __IInternetSecurityManagerEx_FWD_DEFINED__
typedef interface IInternetSecurityManagerEx IInternetSecurityManagerEx;

#endif 	/* __IInternetSecurityManagerEx_FWD_DEFINED__ */


#ifndef __IInternetSecurityManagerEx2_FWD_DEFINED__
#define __IInternetSecurityManagerEx2_FWD_DEFINED__
typedef interface IInternetSecurityManagerEx2 IInternetSecurityManagerEx2;

#endif 	/* __IInternetSecurityManagerEx2_FWD_DEFINED__ */


#ifndef __IZoneIdentifier_FWD_DEFINED__
#define __IZoneIdentifier_FWD_DEFINED__
typedef interface IZoneIdentifier IZoneIdentifier;

#endif 	/* __IZoneIdentifier_FWD_DEFINED__ */


#ifndef __IZoneIdentifier2_FWD_DEFINED__
#define __IZoneIdentifier2_FWD_DEFINED__
typedef interface IZoneIdentifier2 IZoneIdentifier2;

#endif 	/* __IZoneIdentifier2_FWD_DEFINED__ */


#ifndef __IInternetHostSecurityManager_FWD_DEFINED__
#define __IInternetHostSecurityManager_FWD_DEFINED__
typedef interface IInternetHostSecurityManager IInternetHostSecurityManager;

#endif 	/* __IInternetHostSecurityManager_FWD_DEFINED__ */


#ifndef __IInternetZoneManager_FWD_DEFINED__
#define __IInternetZoneManager_FWD_DEFINED__
typedef interface IInternetZoneManager IInternetZoneManager;

#endif 	/* __IInternetZoneManager_FWD_DEFINED__ */


#ifndef __IInternetZoneManagerEx_FWD_DEFINED__
#define __IInternetZoneManagerEx_FWD_DEFINED__
typedef interface IInternetZoneManagerEx IInternetZoneManagerEx;

#endif 	/* __IInternetZoneManagerEx_FWD_DEFINED__ */


#ifndef __IInternetZoneManagerEx2_FWD_DEFINED__
#define __IInternetZoneManagerEx2_FWD_DEFINED__
typedef interface IInternetZoneManagerEx2 IInternetZoneManagerEx2;

#endif 	/* __IInternetZoneManagerEx2_FWD_DEFINED__ */


#ifndef __ISoftDistExt_FWD_DEFINED__
#define __ISoftDistExt_FWD_DEFINED__
typedef interface ISoftDistExt ISoftDistExt;

#endif 	/* __ISoftDistExt_FWD_DEFINED__ */


#ifndef __ICatalogFileInfo_FWD_DEFINED__
#define __ICatalogFileInfo_FWD_DEFINED__
typedef interface ICatalogFileInfo ICatalogFileInfo;

#endif 	/* __ICatalogFileInfo_FWD_DEFINED__ */


#ifndef __IDataFilter_FWD_DEFINED__
#define __IDataFilter_FWD_DEFINED__
typedef interface IDataFilter IDataFilter;

#endif 	/* __IDataFilter_FWD_DEFINED__ */


#ifndef __IEncodingFilterFactory_FWD_DEFINED__
#define __IEncodingFilterFactory_FWD_DEFINED__
typedef interface IEncodingFilterFactory IEncodingFilterFactory;

#endif 	/* __IEncodingFilterFactory_FWD_DEFINED__ */


#ifndef __IWrappedProtocol_FWD_DEFINED__
#define __IWrappedProtocol_FWD_DEFINED__
typedef interface IWrappedProtocol IWrappedProtocol;

#endif 	/* __IWrappedProtocol_FWD_DEFINED__ */


#ifndef __IGetBindHandle_FWD_DEFINED__
#define __IGetBindHandle_FWD_DEFINED__
typedef interface IGetBindHandle IGetBindHandle;

#endif 	/* __IGetBindHandle_FWD_DEFINED__ */


#ifndef __IBindCallbackRedirect_FWD_DEFINED__
#define __IBindCallbackRedirect_FWD_DEFINED__
typedef interface IBindCallbackRedirect IBindCallbackRedirect;

#endif 	/* __IBindCallbackRedirect_FWD_DEFINED__ */


#ifndef __IBindHttpSecurity_FWD_DEFINED__
#define __IBindHttpSecurity_FWD_DEFINED__
typedef interface IBindHttpSecurity IBindHttpSecurity;

#endif 	/* __IBindHttpSecurity_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "oleidl.h"
#include "servprov.h"
#include "msxml.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_urlmon_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// UrlMon.h
//=--------------------------------------------------------------------------=
// Copyright (c) Microsoft Corporation.  All rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#pragma comment(lib,"uuid.lib")

//---------------------------------------------------------------------------=
// URL Moniker Interfaces.

#include <winapifamily.h>
#if _MSC_VER >= 1200
#pragma warning(push)
#ifndef _MSC_EXTENSIONS
#pragma warning(disable:4309) /* truncation of constant value */
#endif
#pragma warning(disable:4820) /* padding added after data member */
#endif
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)














// Side-by-Side clsid
EXTERN_C const IID CLSID_SBS_StdURLMoniker;  
EXTERN_C const IID CLSID_SBS_HttpProtocol;   
EXTERN_C const IID CLSID_SBS_FtpProtocol;    
EXTERN_C const IID CLSID_SBS_GopherProtocol; 
EXTERN_C const IID CLSID_SBS_HttpSProtocol;  
EXTERN_C const IID CLSID_SBS_FileProtocol;   
EXTERN_C const IID CLSID_SBS_MkProtocol;     
EXTERN_C const IID CLSID_SBS_UrlMkBindCtx;   
EXTERN_C const IID CLSID_SBS_SoftDistExt;  
EXTERN_C const IID CLSID_SBS_CdlProtocol;          
EXTERN_C const IID CLSID_SBS_ClassInstallFilter;   
EXTERN_C const IID CLSID_SBS_InternetSecurityManager;  
EXTERN_C const IID CLSID_SBS_InternetZoneManager;  
// END Side-by-Side clsid
// These are for backwards compatibility with previous URLMON versions
#define BINDF_DONTUSECACHE BINDF_GETNEWESTVERSION
#define BINDF_DONTPUTINCACHE BINDF_NOWRITECACHE
#define BINDF_NOCOPYDATA BINDF_PULLDATA
#define INVALID_P_ROOT_SECURITY_ID ((BYTE*)-1)
#define PI_DOCFILECLSIDLOOKUP PI_CLSIDLOOKUP
EXTERN_C const IID IID_IAsyncMoniker;    
EXTERN_C const IID CLSID_StdURLMoniker;  
EXTERN_C const IID CLSID_HttpProtocol;   
EXTERN_C const IID CLSID_FtpProtocol;    
EXTERN_C const IID CLSID_GopherProtocol; 
EXTERN_C const IID CLSID_HttpSProtocol;  
EXTERN_C const IID CLSID_FileProtocol;   
EXTERN_C const IID CLSID_ResProtocol;    
EXTERN_C const IID CLSID_AboutProtocol;  
EXTERN_C const IID CLSID_JSProtocol;  
EXTERN_C const IID CLSID_MailtoProtocol;  
EXTERN_C const IID CLSID_IE4_PROTOCOLS;  
EXTERN_C const IID CLSID_MkProtocol;     
EXTERN_C const IID CLSID_StdURLProtocol; 
EXTERN_C const IID CLSID_TBAuthProtocol; 
EXTERN_C const IID CLSID_UrlMkBindCtx;   
EXTERN_C const IID CLSID_CdlProtocol;          
EXTERN_C const IID CLSID_ClassInstallFilter;   
EXTERN_C const IID IID_IAsyncBindCtx;    
 
#define SZ_URLCONTEXT           OLESTR("URL Context")
#define SZ_ASYNC_CALLEE         OLESTR("AsyncCallee")
#define MKSYS_URLMONIKER         6            
#define URL_MK_LEGACY            0            
#define URL_MK_UNIFORM           1            
#define URL_MK_NO_CANONICALIZE   2            
 
STDAPI CreateURLMoniker(_In_opt_ LPMONIKER pMkCtx, _In_ LPCWSTR szURL, _Outptr_ LPMONIKER FAR * ppmk);             
STDAPI CreateURLMonikerEx(_In_opt_ LPMONIKER pMkCtx, _In_ LPCWSTR szURL, _Outptr_ LPMONIKER FAR * ppmk, DWORD dwFlags);             
STDAPI GetClassURL(_In_ LPCWSTR szURL, _Out_ CLSID *pClsID);                                           
STDAPI CreateAsyncBindCtx(DWORD reserved, _In_ IBindStatusCallback *pBSCb,                       
                                _In_opt_ IEnumFORMATETC *pEFetc, _Outptr_ IBindCtx **ppBC);                   
#if (_WIN32_IE >= _WIN32_IE_IE70)
STDAPI CreateURLMonikerEx2(_In_opt_ LPMONIKER pMkCtx, _In_ IUri* pUri, _Outptr_ LPMONIKER FAR * ppmk, DWORD dwFlags);             
#endif
STDAPI CreateAsyncBindCtxEx(_In_ IBindCtx *pbc, DWORD dwOptions, _In_ IBindStatusCallback *pBSCb, _In_opt_ IEnumFORMATETC *pEnum,   
                            _Outptr_ IBindCtx **ppBC, DWORD reserved);                                                     
STDAPI MkParseDisplayNameEx(_In_ IBindCtx *pbc, _In_ LPCWSTR szDisplayName, _Out_ _Out_range_(<=, _String_length_(szDisplayName)) ULONG *pchEaten,          
                                _Outptr_ LPMONIKER *ppmk);                                           
STDAPI RegisterBindStatusCallback(_In_ LPBC pBC, _In_ IBindStatusCallback *pBSCb,                     
                                _Outptr_ IBindStatusCallback**  ppBSCBPrev, DWORD dwReserved);       
STDAPI RevokeBindStatusCallback(_In_ LPBC pBC, _In_ IBindStatusCallback *pBSCb);                      
STDAPI GetClassFileOrMime(_In_opt_ LPBC pBC, _In_opt_ LPCWSTR szFilename, _In_reads_bytes_opt_(cbSize) LPVOID pBuffer, DWORD cbSize, _In_opt_ LPCWSTR szMime, DWORD dwReserved, _Out_ CLSID *pclsid); 
STDAPI IsValidURL(_In_opt_ LPBC pBC, _In_ LPCWSTR szURL, DWORD dwReserved);                               
STDAPI CoGetClassObjectFromURL( _In_ REFCLSID rCLASSID,
            _In_ LPCWSTR szCODE, DWORD dwFileVersionMS, 
            DWORD dwFileVersionLS, _In_ LPCWSTR szTYPE,
            _In_ LPBINDCTX pBindCtx, DWORD dwClsContext,
            _Reserved_ LPVOID pvReserved, REFIID riid, _Outptr_ LPVOID * ppv);
STDAPI IEInstallScope(_Out_ LPDWORD pdwScope);
STDAPI FaultInIEFeature( HWND hWnd,
            _In_ uCLSSPEC *pClassSpec,
            _Inout_opt_ QUERYCONTEXT *pQuery, DWORD dwFlags);                                           
STDAPI GetComponentIDFromCLSSPEC(_In_ uCLSSPEC *pClassspec,
            _Outptr_ LPSTR * ppszComponentID);                                                      
// flags for FaultInIEFeature
#define FIEF_FLAG_FORCE_JITUI               0x1     // force JIT ui even if
                                                 // previoulsy rejected by 
                                                 // user in this session or
                                                 // marked as Never Ask Again
#define FIEF_FLAG_PEEK                      0x2     // just peek, don't faultin
#define FIEF_FLAG_SKIP_INSTALLED_VERSION_CHECK        0x4     // force JIT without checking local version
#define FIEF_FLAG_RESERVED_0                0x8     // reserved
 
//helper apis                                                                               
STDAPI IsAsyncMoniker(_In_ IMoniker* pmk);                                                       
STDAPI CreateURLBinding(LPCWSTR lpszUrl, _In_ IBindCtx *pbc, _Inout_ IBinding **ppBdg);
 
STDAPI RegisterMediaTypes(_In_ _In_range_(>, 0) UINT ctypes, _In_reads_(ctypes) const LPCSTR* rgszTypes, _Out_writes_(ctypes) CLIPFORMAT* rgcfTypes);            
STDAPI FindMediaType(_In_ LPCSTR rgszTypes, _Out_ CLIPFORMAT* rgcfTypes);                                       
STDAPI CreateFormatEnumerator( _In_ _In_range_(>, 0) UINT cfmtetc, _In_reads_(cfmtetc) FORMATETC* rgfmtetc, _Outptr_ IEnumFORMATETC** ppenumfmtetc); 
STDAPI RegisterFormatEnumerator(_In_ LPBC pBC, _In_ IEnumFORMATETC *pEFetc, DWORD reserved);          
STDAPI RevokeFormatEnumerator(_In_ LPBC pBC, _In_ IEnumFORMATETC *pEFetc);                            
STDAPI RegisterMediaTypeClass(_In_ LPBC pBC, _In_ _In_range_(>, 0) UINT ctypes, _In_reads_(ctypes) const LPCSTR* rgszTypes, _In_reads_(ctypes)  CLSID *rgclsID, DWORD reserved);    
STDAPI FindMediaTypeClass(_In_ LPBC pBC, _In_ LPCSTR szType, _Out_ CLSID *pclsID, DWORD reserved);                          
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
STDAPI UrlMkSetSessionOption(DWORD dwOption, _In_reads_bytes_opt_(dwBufferLength) LPVOID pBuffer, DWORD dwBufferLength, _Reserved_ DWORD dwReserved);       
STDAPI UrlMkGetSessionOption(DWORD dwOption, _Out_writes_bytes_to_opt_(dwBufferLength,*pdwBufferLengthOut) LPVOID pBuffer, DWORD dwBufferLength, _Out_ DWORD *pdwBufferLengthOut, _Reserved_ DWORD dwReserved);       
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
STDAPI FindMimeFromData(                                                                                                                  
    _In_opt_                     LPBC    pBC,                   // bind context - can be NULL                                                 
    _In_opt_                     LPCWSTR pwzUrl,                // url - can be null                                                          
    _In_reads_bytes_opt_(cbSize) LPVOID  pBuffer,               // buffer with data to sniff - can be null (pwzUrl must be valid)             
                                 DWORD   cbSize,                // size of buffer                                                             
    _In_opt_                     LPCWSTR pwzMimeProposed,       // proposed mime if - can be null                                             
                                 DWORD   dwMimeFlags,           // will be defined                                                            
    _Outptr_                     LPWSTR  *ppwzMimeOut,          // the suggested mime                                                         
    _Reserved_                   DWORD   dwReserved);           // must be 0                                                                  
#define     FMFD_DEFAULT             0x00000000 
#define     FMFD_URLASFILENAME       0x00000001 
#if (_WIN32_IE >= _WIN32_IE_IE60SP2)
#define     FMFD_ENABLEMIMESNIFFING  0x00000002 
#define     FMFD_IGNOREMIMETEXTPLAIN  0x00000004 
#endif //(_WIN32_IE >= _WIN32_IE_IE60SP2)
#define     FMFD_SERVERMIME   0x00000008 
#define     FMFD_RESPECTTEXTPLAIN   0x00000010 
#define     FMFD_RETURNUPDATEDIMGMIMES   0x00000020
#define     FMFD_RESERVED_1   0x00000040
#define     FMFD_RESERVED_2   0x00000080
#define     UAS_EXACTLEGACY   0x00001000 
STDAPI ObtainUserAgentString(                           
                                         DWORD dwOption, 
    _Out_writes_to_(*cbSize, *cbSize)    LPSTR pszUAOut, 
    _Inout_                              DWORD *cbSize); 
STDAPI CompareSecurityIds(_In_reads_(dwLen1) BYTE* pbSecurityId1, _In_ DWORD dwLen1, _In_reads_(dwLen2) BYTE* pbSecurityId2, _In_ DWORD dwLen2, _In_ DWORD dwReserved);    
STDAPI CompatFlagsFromClsid(_In_ CLSID *pclsid, _Out_ LPDWORD pdwCompatFlags, _Out_ LPDWORD pdwMiscStatusFlags);             
 
#if (NTDDI_VERSION > NTDDI_WINBLUE || (NTDDI_VERSION == NTDDI_WINBLUE && defined(WINBLUE_KBSPRING14)))
// Enhanced Protected Mode (EPM) Convenience Functions
 
typedef enum IEObjectType
{
    IE_EPM_OBJECT_EVENT,
    IE_EPM_OBJECT_MUTEX,
    IE_EPM_OBJECT_SEMAPHORE,
    IE_EPM_OBJECT_SHARED_MEMORY,
    IE_EPM_OBJECT_WAITABLE_TIMER,
    IE_EPM_OBJECT_FILE,
    IE_EPM_OBJECT_NAMED_PIPE,
    IE_EPM_OBJECT_REGISTRY,
} IEObjectType;
 
STDAPI SetAccessForIEAppContainer(
    _In_ HANDLE hObject,
    _In_ IEObjectType ieObjectType,
    _In_ DWORD dwAccessMask
    );
#endif
 
// URLMON-specific defines for UrlMkSetSessionOption() above
#define URLMON_OPTION_USERAGENT           0x10000001
#define URLMON_OPTION_USERAGENT_REFRESH   0x10000002
#define URLMON_OPTION_URL_ENCODING        0x10000004
#if (_WIN32_IE >= _WIN32_IE_IE60SP2)
#define URLMON_OPTION_USE_BINDSTRINGCREDS 0x10000008
#endif //(_WIN32_IE >= _WIN32_IE_IE60SP2)
#if (_WIN32_IE >= _WIN32_IE_IE70)
#define URLMON_OPTION_USE_BROWSERAPPSDOCUMENTS 0x10000010
#endif //(_WIN32_IE >= _WIN32_IE_IE70)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
 
#define CF_NULL                 0                                  
#define CFSTR_MIME_NULL         NULL                               
#define CFSTR_MIME_TEXT         (TEXT("text/plain"))             
#define CFSTR_MIME_RICHTEXT     (TEXT("text/richtext"))          
#define CFSTR_MIME_MANIFEST     (TEXT("text/cache-manifest"))    
#define CFSTR_MIME_WEBVTT       (TEXT("text/vtt"))               
#define CFSTR_MIME_X_BITMAP     (TEXT("image/x-xbitmap"))        
#define CFSTR_MIME_POSTSCRIPT   (TEXT("application/postscript")) 
#define CFSTR_MIME_AIFF         (TEXT("audio/aiff"))             
#define CFSTR_MIME_BASICAUDIO   (TEXT("audio/basic"))            
#define CFSTR_MIME_WAV          (TEXT("audio/wav"))              
#define CFSTR_MIME_X_WAV        (TEXT("audio/x-wav"))            
#define CFSTR_MIME_GIF          (TEXT("image/gif"))              
#define CFSTR_MIME_PJPEG        (TEXT("image/pjpeg"))            
#define CFSTR_MIME_JPEG         (TEXT("image/jpeg"))             
#define CFSTR_MIME_TIFF         (TEXT("image/tiff"))             
#define CFSTR_MIME_JPEG_XR      (TEXT("image/vnd.ms-photo"))     
#define CFSTR_MIME_PNG          (TEXT("image/png"))              
#define CFSTR_MIME_DDS          (TEXT("image/vnd.ms-dds"))       
#define CFSTR_MIME_X_PNG        (TEXT("image/x-png"))            
#define CFSTR_MIME_X_ICON       (TEXT("image/x-icon"))           
#define CFSTR_MIME_SVG_XML      (TEXT("image/svg+xml"))          
#define CFSTR_MIME_BMP          (TEXT("image/bmp"))              
#define CFSTR_MIME_X_EMF        (TEXT("image/x-emf"))            
#define CFSTR_MIME_X_WMF        (TEXT("image/x-wmf"))            
#define CFSTR_MIME_AVI          (TEXT("video/avi"))              
#define CFSTR_MIME_MPEG         (TEXT("video/mpeg"))             
#define CFSTR_MIME_FRACTALS     (TEXT("application/fractals"))   
#define CFSTR_MIME_RAWDATA      (TEXT("application/octet-stream"))
#define CFSTR_MIME_RAWDATASTRM  (TEXT("application/octet-stream"))
#define CFSTR_MIME_PDF          (TEXT("application/pdf"))        
#define CFSTR_MIME_HTA          (TEXT("application/hta"))        
#define CFSTR_MIME_APP_XML      (TEXT("application/xml"))  
#define CFSTR_MIME_XHTML        (TEXT("application/xhtml+xml"))  
#define CFSTR_MIME_X_AIFF       (TEXT("audio/x-aiff"))           
#define CFSTR_MIME_X_REALAUDIO  (TEXT("audio/x-pn-realaudio"))   
#define CFSTR_MIME_XBM          (TEXT("image/xbm"))              
#define CFSTR_MIME_QUICKTIME    (TEXT("video/quicktime"))        
#define CFSTR_MIME_X_MSVIDEO    (TEXT("video/x-msvideo"))        
#define CFSTR_MIME_X_SGI_MOVIE  (TEXT("video/x-sgi-movie"))      
#define CFSTR_MIME_X_MIXED_REPLACE (TEXT("multipart/x-mixed-replace")) 
#define CFSTR_MIME_HTML         (TEXT("text/html"))              
#define CFSTR_MIME_XML          (TEXT("text/xml"))               
#define CFSTR_MIME_TTML         (TEXT("application/ttml+xml"))   
#define CFSTR_MIME_TTAF         (TEXT("application/ttaf+xml"))   
#define CFSTR_MIME_X_JAVASCRIPT (TEXT("application/x-javascript"))
#define CFSTR_MIME_TEXT_JSON    (TEXT("text/json"))              
#define CFSTR_MIME_APPLICATION_JAVASCRIPT (TEXT("application/javascript"))
 
// MessageId: MK_S_ASYNCHRONOUS                                              
// MessageText: Operation is successful, but will complete asynchronously.   
//                                                                           
#define MK_S_ASYNCHRONOUS    _HRESULT_TYPEDEF_(0x000401E8L)                  
#ifndef S_ASYNCHRONOUS                                                       
#define S_ASYNCHRONOUS       MK_S_ASYNCHRONOUS                               
#endif                                                                       
                                                                             
#ifndef E_PENDING                                                            
#define E_PENDING _HRESULT_TYPEDEF_(0x8000000AL)                             
#endif                                                                       
                                                                             
//                                                                           
//                                                                           
// WinINet and protocol specific errors are mapped to one of the following   
// error which are returned in IBSC::OnStopBinding                           
//                                                                           
//                                                                           
// Note: FACILITY C is split into ranges of 1k                               
// C0000 - C03FF  INET_E_ (URLMON's original hresult)                        
// C0400 - C07FF  INET_E_CLIENT_xxx                                          
// C0800 - C0BFF  INET_E_SERVER_xxx                                          
// C0C00 - C0FFF  INET_E_????                                                
// C1000 - C13FF  INET_E_AGENT_xxx (info delivery agents)                    
#define INET_E_INVALID_URL               _HRESULT_TYPEDEF_(0x800C0002L)      
#define INET_E_NO_SESSION                _HRESULT_TYPEDEF_(0x800C0003L)      
#define INET_E_CANNOT_CONNECT            _HRESULT_TYPEDEF_(0x800C0004L)      
#define INET_E_RESOURCE_NOT_FOUND        _HRESULT_TYPEDEF_(0x800C0005L)      
#define INET_E_OBJECT_NOT_FOUND          _HRESULT_TYPEDEF_(0x800C0006L)      
#define INET_E_DATA_NOT_AVAILABLE        _HRESULT_TYPEDEF_(0x800C0007L)      
#define INET_E_DOWNLOAD_FAILURE          _HRESULT_TYPEDEF_(0x800C0008L)      
#define INET_E_AUTHENTICATION_REQUIRED   _HRESULT_TYPEDEF_(0x800C0009L)      
#define INET_E_NO_VALID_MEDIA            _HRESULT_TYPEDEF_(0x800C000AL)      
#define INET_E_CONNECTION_TIMEOUT        _HRESULT_TYPEDEF_(0x800C000BL)      
#define INET_E_INVALID_REQUEST           _HRESULT_TYPEDEF_(0x800C000CL)      
#define INET_E_UNKNOWN_PROTOCOL          _HRESULT_TYPEDEF_(0x800C000DL)      
#define INET_E_SECURITY_PROBLEM          _HRESULT_TYPEDEF_(0x800C000EL)      
#define INET_E_CANNOT_LOAD_DATA          _HRESULT_TYPEDEF_(0x800C000FL)      
#define INET_E_CANNOT_INSTANTIATE_OBJECT _HRESULT_TYPEDEF_(0x800C0010L)      
#define INET_E_INVALID_CERTIFICATE       _HRESULT_TYPEDEF_(0x800C0019L)      
#define INET_E_REDIRECT_FAILED           _HRESULT_TYPEDEF_(0x800C0014L)      
#define INET_E_REDIRECT_TO_DIR           _HRESULT_TYPEDEF_(0x800C0015L)      
#define INET_E_CANNOT_LOCK_REQUEST                   _HRESULT_TYPEDEF_(0x800C0016L)      
#define INET_E_USE_EXTEND_BINDING                    _HRESULT_TYPEDEF_(0x800C0017L)      
#define INET_E_TERMINATED_BIND                       _HRESULT_TYPEDEF_(0x800C0018L)      
#define INET_E_RESERVED_1                            _HRESULT_TYPEDEF_(0x800C001AL)      
#define INET_E_BLOCKED_REDIRECT_XSECURITYID          _HRESULT_TYPEDEF_(0x800C001BL)      
#define INET_E_DOMINJECTIONVALIDATION                _HRESULT_TYPEDEF_(0x800C001CL)      
#define INET_E_VTAB_SWITCH_FORCE_ENGINE              _HRESULT_TYPEDEF_(0x800C001DL)      
#define INET_E_HSTS_CERTIFICATE_ERROR                _HRESULT_TYPEDEF_(0x800C001EL)      
#define INET_E_RESERVED_2                            _HRESULT_TYPEDEF_(0x800C001FL)      
#define INET_E_RESERVED_3                            _HRESULT_TYPEDEF_(0x800C0020L)      
#define INET_E_RESERVED_4                            _HRESULT_TYPEDEF_(0x800C0021L)      
#define INET_E_RESERVED_5                            _HRESULT_TYPEDEF_(0x800C0022L)      
#define INET_E_ERROR_FIRST                           _HRESULT_TYPEDEF_(0x800C0002L)      
#define INET_E_CODE_DOWNLOAD_DECLINED                _HRESULT_TYPEDEF_(0x800C0100L)      
#define INET_E_RESULT_DISPATCHED                     _HRESULT_TYPEDEF_(0x800C0200L)      
#define INET_E_CANNOT_REPLACE_SFP_FILE               _HRESULT_TYPEDEF_(0x800C0300L)      
#if (_WIN32_IE >= _WIN32_IE_IE60SP2)
#define INET_E_CODE_INSTALL_SUPPRESSED               _HRESULT_TYPEDEF_(0x800C0400L)      
#endif //(_WIN32_IE >= _WIN32_IE_IE60SP2)
#define INET_E_CODE_INSTALL_BLOCKED_BY_HASH_POLICY   _HRESULT_TYPEDEF_(0x800C0500L)      
#define INET_E_DOWNLOAD_BLOCKED_BY_INPRIVATE         _HRESULT_TYPEDEF_(0x800C0501L)      
#define INET_E_CODE_INSTALL_BLOCKED_IMMERSIVE        _HRESULT_TYPEDEF_(0x800C0502L)      
#define INET_E_FORBIDFRAMING                         _HRESULT_TYPEDEF_(0x800C0503L)      
#define INET_E_CODE_INSTALL_BLOCKED_ARM              _HRESULT_TYPEDEF_(0x800C0504L)      
#define INET_E_BLOCKED_PLUGGABLE_PROTOCOL            _HRESULT_TYPEDEF_(0x800C0505L)      
#define INET_E_BLOCKED_ENHANCEDPROTECTEDMODE         _HRESULT_TYPEDEF_(0x800C0506L)      
#define INET_E_CODE_INSTALL_BLOCKED_BITNESS          _HRESULT_TYPEDEF_(0x800C0507L)      
#define INET_E_DOWNLOAD_BLOCKED_BY_CSP               _HRESULT_TYPEDEF_(0x800C0508L)      
#define INET_E_ERROR_LAST                INET_E_DOWNLOAD_BLOCKED_BY_CSP
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#ifndef _LPPERSISTMONIKER_DEFINED
#define _LPPERSISTMONIKER_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0000_v0_0_s_ifspec;

#ifndef __IPersistMoniker_INTERFACE_DEFINED__
#define __IPersistMoniker_INTERFACE_DEFINED__

/* interface IPersistMoniker */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IPersistMoniker *LPPERSISTMONIKER;


EXTERN_C const IID IID_IPersistMoniker;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9c9-baf9-11ce-8c82-00aa004ba90b")
    IPersistMoniker : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetClassID( 
            /* [out] */ __RPC__out CLSID *pClassID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsDirty( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Load( 
            /* [in] */ BOOL fFullyAvailable,
            /* [in] */ __RPC__in_opt IMoniker *pimkName,
            /* [in] */ __RPC__in_opt LPBC pibc,
            /* [in] */ DWORD grfMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Save( 
            /* [in] */ __RPC__in_opt IMoniker *pimkName,
            /* [in] */ __RPC__in_opt LPBC pbc,
            /* [in] */ BOOL fRemember) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SaveCompleted( 
            /* [in] */ __RPC__in_opt IMoniker *pimkName,
            /* [in] */ __RPC__in_opt LPBC pibc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurMoniker( 
            /* [out] */ __RPC__deref_out_opt IMoniker **ppimkName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPersistMonikerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPersistMoniker * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPersistMoniker * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPersistMoniker * This);
        
        DECLSPEC_XFGVIRT(IPersistMoniker, GetClassID)
        HRESULT ( STDMETHODCALLTYPE *GetClassID )( 
            __RPC__in IPersistMoniker * This,
            /* [out] */ __RPC__out CLSID *pClassID);
        
        DECLSPEC_XFGVIRT(IPersistMoniker, IsDirty)
        HRESULT ( STDMETHODCALLTYPE *IsDirty )( 
            __RPC__in IPersistMoniker * This);
        
        DECLSPEC_XFGVIRT(IPersistMoniker, Load)
        HRESULT ( STDMETHODCALLTYPE *Load )( 
            __RPC__in IPersistMoniker * This,
            /* [in] */ BOOL fFullyAvailable,
            /* [in] */ __RPC__in_opt IMoniker *pimkName,
            /* [in] */ __RPC__in_opt LPBC pibc,
            /* [in] */ DWORD grfMode);
        
        DECLSPEC_XFGVIRT(IPersistMoniker, Save)
        HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IPersistMoniker * This,
            /* [in] */ __RPC__in_opt IMoniker *pimkName,
            /* [in] */ __RPC__in_opt LPBC pbc,
            /* [in] */ BOOL fRemember);
        
        DECLSPEC_XFGVIRT(IPersistMoniker, SaveCompleted)
        HRESULT ( STDMETHODCALLTYPE *SaveCompleted )( 
            __RPC__in IPersistMoniker * This,
            /* [in] */ __RPC__in_opt IMoniker *pimkName,
            /* [in] */ __RPC__in_opt LPBC pibc);
        
        DECLSPEC_XFGVIRT(IPersistMoniker, GetCurMoniker)
        HRESULT ( STDMETHODCALLTYPE *GetCurMoniker )( 
            __RPC__in IPersistMoniker * This,
            /* [out] */ __RPC__deref_out_opt IMoniker **ppimkName);
        
        END_INTERFACE
    } IPersistMonikerVtbl;

    interface IPersistMoniker
    {
        CONST_VTBL struct IPersistMonikerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPersistMoniker_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPersistMoniker_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPersistMoniker_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPersistMoniker_GetClassID(This,pClassID)	\
    ( (This)->lpVtbl -> GetClassID(This,pClassID) ) 

#define IPersistMoniker_IsDirty(This)	\
    ( (This)->lpVtbl -> IsDirty(This) ) 

#define IPersistMoniker_Load(This,fFullyAvailable,pimkName,pibc,grfMode)	\
    ( (This)->lpVtbl -> Load(This,fFullyAvailable,pimkName,pibc,grfMode) ) 

#define IPersistMoniker_Save(This,pimkName,pbc,fRemember)	\
    ( (This)->lpVtbl -> Save(This,pimkName,pbc,fRemember) ) 

#define IPersistMoniker_SaveCompleted(This,pimkName,pibc)	\
    ( (This)->lpVtbl -> SaveCompleted(This,pimkName,pibc) ) 

#define IPersistMoniker_GetCurMoniker(This,ppimkName)	\
    ( (This)->lpVtbl -> GetCurMoniker(This,ppimkName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPersistMoniker_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0001 */
/* [local] */ 

#endif
#ifndef _LPMONIKERPROP_DEFINED
#define _LPMONIKERPROP_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0001_v0_0_s_ifspec;

#ifndef __IMonikerProp_INTERFACE_DEFINED__
#define __IMonikerProp_INTERFACE_DEFINED__

/* interface IMonikerProp */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IMonikerProp *LPMONIKERPROP;

typedef /* [public][public] */ 
enum __MIDL_IMonikerProp_0001
    {
        MIMETYPEPROP	= 0,
        USE_SRC_URL	= 0x1,
        CLASSIDPROP	= 0x2,
        TRUSTEDDOWNLOADPROP	= 0x3,
        POPUPLEVELPROP	= 0x4
    } 	MONIKERPROPERTY;


EXTERN_C const IID IID_IMonikerProp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a5ca5f7f-1847-4d87-9c5b-918509f7511d")
    IMonikerProp : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PutProperty( 
            /* [in] */ MONIKERPROPERTY mkp,
            /* [in] */ __RPC__in LPCWSTR val) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMonikerPropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMonikerProp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMonikerProp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMonikerProp * This);
        
        DECLSPEC_XFGVIRT(IMonikerProp, PutProperty)
        HRESULT ( STDMETHODCALLTYPE *PutProperty )( 
            __RPC__in IMonikerProp * This,
            /* [in] */ MONIKERPROPERTY mkp,
            /* [in] */ __RPC__in LPCWSTR val);
        
        END_INTERFACE
    } IMonikerPropVtbl;

    interface IMonikerProp
    {
        CONST_VTBL struct IMonikerPropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMonikerProp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMonikerProp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMonikerProp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMonikerProp_PutProperty(This,mkp,val)	\
    ( (This)->lpVtbl -> PutProperty(This,mkp,val) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMonikerProp_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0002 */
/* [local] */ 

#endif
#ifndef _LPBINDPROTOCOL_DEFINED
#define _LPBINDPROTOCOL_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0002_v0_0_s_ifspec;

#ifndef __IBindProtocol_INTERFACE_DEFINED__
#define __IBindProtocol_INTERFACE_DEFINED__

/* interface IBindProtocol */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IBindProtocol *LPBINDPROTOCOL;


EXTERN_C const IID IID_IBindProtocol;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9cd-baf9-11ce-8c82-00aa004ba90b")
    IBindProtocol : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateBinding( 
            /* [in] */ LPCWSTR szUrl,
            /* [in] */ IBindCtx *pbc,
            /* [out] */ IBinding **ppb) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBindProtocolVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IBindProtocol * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IBindProtocol * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IBindProtocol * This);
        
        DECLSPEC_XFGVIRT(IBindProtocol, CreateBinding)
        HRESULT ( STDMETHODCALLTYPE *CreateBinding )( 
            IBindProtocol * This,
            /* [in] */ LPCWSTR szUrl,
            /* [in] */ IBindCtx *pbc,
            /* [out] */ IBinding **ppb);
        
        END_INTERFACE
    } IBindProtocolVtbl;

    interface IBindProtocol
    {
        CONST_VTBL struct IBindProtocolVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBindProtocol_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBindProtocol_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBindProtocol_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBindProtocol_CreateBinding(This,szUrl,pbc,ppb)	\
    ( (This)->lpVtbl -> CreateBinding(This,szUrl,pbc,ppb) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBindProtocol_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0003 */
/* [local] */ 

#endif
#ifndef _LPBINDING_DEFINED
#define _LPBINDING_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0003_v0_0_s_ifspec;

#ifndef __IBinding_INTERFACE_DEFINED__
#define __IBinding_INTERFACE_DEFINED__

/* interface IBinding */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IBinding *LPBINDING;


EXTERN_C const IID IID_IBinding;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9c0-baf9-11ce-8c82-00aa004ba90b")
    IBinding : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Abort( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Suspend( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Resume( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPriority( 
            /* [in] */ LONG nPriority) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPriority( 
            /* [out] */ __RPC__out LONG *pnPriority) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetBindResult( 
            /* [out] */ CLSID *pclsidProtocol,
            /* [out] */ DWORD *pdwResult,
            /* [annotation][out] */ 
            __RPC__out  LPOLESTR *pszResult,
            /* [out][in] */ DWORD *pdwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBindingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBinding * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBinding * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBinding * This);
        
        DECLSPEC_XFGVIRT(IBinding, Abort)
        HRESULT ( STDMETHODCALLTYPE *Abort )( 
            __RPC__in IBinding * This);
        
        DECLSPEC_XFGVIRT(IBinding, Suspend)
        HRESULT ( STDMETHODCALLTYPE *Suspend )( 
            __RPC__in IBinding * This);
        
        DECLSPEC_XFGVIRT(IBinding, Resume)
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            __RPC__in IBinding * This);
        
        DECLSPEC_XFGVIRT(IBinding, SetPriority)
        HRESULT ( STDMETHODCALLTYPE *SetPriority )( 
            __RPC__in IBinding * This,
            /* [in] */ LONG nPriority);
        
        DECLSPEC_XFGVIRT(IBinding, GetPriority)
        HRESULT ( STDMETHODCALLTYPE *GetPriority )( 
            __RPC__in IBinding * This,
            /* [out] */ __RPC__out LONG *pnPriority);
        
        DECLSPEC_XFGVIRT(IBinding, GetBindResult)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetBindResult )( 
            IBinding * This,
            /* [out] */ CLSID *pclsidProtocol,
            /* [out] */ DWORD *pdwResult,
            /* [annotation][out] */ 
            __RPC__out  LPOLESTR *pszResult,
            /* [out][in] */ DWORD *pdwReserved);
        
        END_INTERFACE
    } IBindingVtbl;

    interface IBinding
    {
        CONST_VTBL struct IBindingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBinding_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBinding_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBinding_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBinding_Abort(This)	\
    ( (This)->lpVtbl -> Abort(This) ) 

#define IBinding_Suspend(This)	\
    ( (This)->lpVtbl -> Suspend(This) ) 

#define IBinding_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#define IBinding_SetPriority(This,nPriority)	\
    ( (This)->lpVtbl -> SetPriority(This,nPriority) ) 

#define IBinding_GetPriority(This,pnPriority)	\
    ( (This)->lpVtbl -> GetPriority(This,pnPriority) ) 

#define IBinding_GetBindResult(This,pclsidProtocol,pdwResult,pszResult,pdwReserved)	\
    ( (This)->lpVtbl -> GetBindResult(This,pclsidProtocol,pdwResult,pszResult,pdwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IBinding_RemoteGetBindResult_Proxy( 
    __RPC__in IBinding * This,
    /* [out] */ __RPC__out CLSID *pclsidProtocol,
    /* [out] */ __RPC__out DWORD *pdwResult,
    /* [out] */ __RPC__deref_out_opt LPOLESTR *pszResult,
    /* [in] */ DWORD dwReserved);


void __RPC_STUB IBinding_RemoteGetBindResult_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IBinding_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0004 */
/* [local] */ 

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#ifndef _LPBINDSTATUSCALLBACK_DEFINED
#define _LPBINDSTATUSCALLBACK_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0004_v0_0_s_ifspec;

#ifndef __IBindStatusCallback_INTERFACE_DEFINED__
#define __IBindStatusCallback_INTERFACE_DEFINED__

/* interface IBindStatusCallback */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IBindStatusCallback *LPBINDSTATUSCALLBACK;

typedef /* [public] */ 
enum __MIDL_IBindStatusCallback_0001
    {
        BINDVERB_GET	= 0,
        BINDVERB_POST	= 0x1,
        BINDVERB_PUT	= 0x2,
        BINDVERB_CUSTOM	= 0x3,
        BINDVERB_RESERVED1	= 0x4
    } 	BINDVERB;

typedef /* [public] */ 
enum __MIDL_IBindStatusCallback_0002
    {
        BINDINFOF_URLENCODESTGMEDDATA	= 0x1,
        BINDINFOF_URLENCODEDEXTRAINFO	= 0x2
    } 	BINDINFOF;

typedef /* [public] */ 
enum __MIDL_IBindStatusCallback_0003
    {
        BINDF_ASYNCHRONOUS	= 0x1,
        BINDF_ASYNCSTORAGE	= 0x2,
        BINDF_NOPROGRESSIVERENDERING	= 0x4,
        BINDF_OFFLINEOPERATION	= 0x8,
        BINDF_GETNEWESTVERSION	= 0x10,
        BINDF_NOWRITECACHE	= 0x20,
        BINDF_NEEDFILE	= 0x40,
        BINDF_PULLDATA	= 0x80,
        BINDF_IGNORESECURITYPROBLEM	= 0x100,
        BINDF_RESYNCHRONIZE	= 0x200,
        BINDF_HYPERLINK	= 0x400,
        BINDF_NO_UI	= 0x800,
        BINDF_SILENTOPERATION	= 0x1000,
        BINDF_PRAGMA_NO_CACHE	= 0x2000,
        BINDF_GETCLASSOBJECT	= 0x4000,
        BINDF_RESERVED_1	= 0x8000,
        BINDF_FREE_THREADED	= 0x10000,
        BINDF_DIRECT_READ	= 0x20000,
        BINDF_FORMS_SUBMIT	= 0x40000,
        BINDF_GETFROMCACHE_IF_NET_FAIL	= 0x80000,
        BINDF_FROMURLMON	= 0x100000,
        BINDF_FWD_BACK	= 0x200000,
        BINDF_PREFERDEFAULTHANDLER	= 0x400000,
        BINDF_ENFORCERESTRICTED	= 0x800000,
        BINDF_RESERVED_2	= 0x80000000,
        BINDF_RESERVED_3	= 0x1000000,
        BINDF_RESERVED_4	= 0x2000000,
        BINDF_RESERVED_5	= 0x4000000,
        BINDF_RESERVED_6	= 0x8000000,
        BINDF_RESERVED_7	= 0x40000000,
        BINDF_RESERVED_8	= 0x20000000
    } 	BINDF;

typedef /* [public] */ 
enum __MIDL_IBindStatusCallback_0004
    {
        URL_ENCODING_NONE	= 0,
        URL_ENCODING_ENABLE_UTF8	= 0x10000000,
        URL_ENCODING_DISABLE_UTF8	= 0x20000000
    } 	URL_ENCODING;

typedef struct _tagBINDINFO
    {
    ULONG cbSize;
    LPWSTR szExtraInfo;
    STGMEDIUM stgmedData;
    DWORD grfBindInfoF;
    DWORD dwBindVerb;
    LPWSTR szCustomVerb;
    DWORD cbstgmedData;
    DWORD dwOptions;
    DWORD dwOptionsFlags;
    DWORD dwCodePage;
    SECURITY_ATTRIBUTES securityAttributes;
    IID iid;
    IUnknown *pUnk;
    DWORD dwReserved;
    } 	BINDINFO;

typedef struct _REMSECURITY_ATTRIBUTES
    {
    DWORD nLength;
    DWORD lpSecurityDescriptor;
    BOOL bInheritHandle;
    } 	REMSECURITY_ATTRIBUTES;

typedef struct _REMSECURITY_ATTRIBUTES *PREMSECURITY_ATTRIBUTES;

typedef struct _REMSECURITY_ATTRIBUTES *LPREMSECURITY_ATTRIBUTES;

typedef struct _tagRemBINDINFO
    {
    ULONG cbSize;
    LPWSTR szExtraInfo;
    DWORD grfBindInfoF;
    DWORD dwBindVerb;
    LPWSTR szCustomVerb;
    DWORD cbstgmedData;
    DWORD dwOptions;
    DWORD dwOptionsFlags;
    DWORD dwCodePage;
    REMSECURITY_ATTRIBUTES securityAttributes;
    IID iid;
    IUnknown *pUnk;
    DWORD dwReserved;
    } 	RemBINDINFO;

typedef struct tagRemFORMATETC
    {
    DWORD cfFormat;
    DWORD ptd;
    DWORD dwAspect;
    LONG lindex;
    DWORD tymed;
    } 	RemFORMATETC;

typedef struct tagRemFORMATETC *LPREMFORMATETC;

typedef /* [public] */ 
enum __MIDL_IBindStatusCallback_0005
    {
        BINDINFO_OPTIONS_WININETFLAG	= 0x10000,
        BINDINFO_OPTIONS_ENABLE_UTF8	= 0x20000,
        BINDINFO_OPTIONS_DISABLE_UTF8	= 0x40000,
        BINDINFO_OPTIONS_USE_IE_ENCODING	= 0x80000,
        BINDINFO_OPTIONS_BINDTOOBJECT	= 0x100000,
        BINDINFO_OPTIONS_SECURITYOPTOUT	= 0x200000,
        BINDINFO_OPTIONS_IGNOREMIMETEXTPLAIN	= 0x400000,
        BINDINFO_OPTIONS_USEBINDSTRINGCREDS	= 0x800000,
        BINDINFO_OPTIONS_IGNOREHTTPHTTPSREDIRECTS	= 0x1000000,
        BINDINFO_OPTIONS_IGNORE_SSLERRORS_ONCE	= 0x2000000,
        BINDINFO_WPC_DOWNLOADBLOCKED	= 0x8000000,
        BINDINFO_WPC_LOGGING_ENABLED	= 0x10000000,
        BINDINFO_OPTIONS_ALLOWCONNECTDATA	= 0x20000000,
        BINDINFO_OPTIONS_DISABLEAUTOREDIRECTS	= 0x40000000,
        BINDINFO_OPTIONS_SHDOCVW_NAVIGATE	= ( int  )0x80000000
    } 	BINDINFO_OPTIONS;

typedef /* [public] */ 
enum __MIDL_IBindStatusCallback_0006
    {
        BSCF_FIRSTDATANOTIFICATION	= 0x1,
        BSCF_INTERMEDIATEDATANOTIFICATION	= 0x2,
        BSCF_LASTDATANOTIFICATION	= 0x4,
        BSCF_DATAFULLYAVAILABLE	= 0x8,
        BSCF_AVAILABLEDATASIZEUNKNOWN	= 0x10,
        BSCF_SKIPDRAINDATAFORFILEURLS	= 0x20,
        BSCF_64BITLENGTHDOWNLOAD	= 0x40
    } 	BSCF;

typedef 
enum tagBINDSTATUS
    {
        BINDSTATUS_FINDINGRESOURCE	= 1,
        BINDSTATUS_CONNECTING	= ( BINDSTATUS_FINDINGRESOURCE + 1 ) ,
        BINDSTATUS_REDIRECTING	= ( BINDSTATUS_CONNECTING + 1 ) ,
        BINDSTATUS_BEGINDOWNLOADDATA	= ( BINDSTATUS_REDIRECTING + 1 ) ,
        BINDSTATUS_DOWNLOADINGDATA	= ( BINDSTATUS_BEGINDOWNLOADDATA + 1 ) ,
        BINDSTATUS_ENDDOWNLOADDATA	= ( BINDSTATUS_DOWNLOADINGDATA + 1 ) ,
        BINDSTATUS_BEGINDOWNLOADCOMPONENTS	= ( BINDSTATUS_ENDDOWNLOADDATA + 1 ) ,
        BINDSTATUS_INSTALLINGCOMPONENTS	= ( BINDSTATUS_BEGINDOWNLOADCOMPONENTS + 1 ) ,
        BINDSTATUS_ENDDOWNLOADCOMPONENTS	= ( BINDSTATUS_INSTALLINGCOMPONENTS + 1 ) ,
        BINDSTATUS_USINGCACHEDCOPY	= ( BINDSTATUS_ENDDOWNLOADCOMPONENTS + 1 ) ,
        BINDSTATUS_SENDINGREQUEST	= ( BINDSTATUS_USINGCACHEDCOPY + 1 ) ,
        BINDSTATUS_CLASSIDAVAILABLE	= ( BINDSTATUS_SENDINGREQUEST + 1 ) ,
        BINDSTATUS_MIMETYPEAVAILABLE	= ( BINDSTATUS_CLASSIDAVAILABLE + 1 ) ,
        BINDSTATUS_CACHEFILENAMEAVAILABLE	= ( BINDSTATUS_MIMETYPEAVAILABLE + 1 ) ,
        BINDSTATUS_BEGINSYNCOPERATION	= ( BINDSTATUS_CACHEFILENAMEAVAILABLE + 1 ) ,
        BINDSTATUS_ENDSYNCOPERATION	= ( BINDSTATUS_BEGINSYNCOPERATION + 1 ) ,
        BINDSTATUS_BEGINUPLOADDATA	= ( BINDSTATUS_ENDSYNCOPERATION + 1 ) ,
        BINDSTATUS_UPLOADINGDATA	= ( BINDSTATUS_BEGINUPLOADDATA + 1 ) ,
        BINDSTATUS_ENDUPLOADDATA	= ( BINDSTATUS_UPLOADINGDATA + 1 ) ,
        BINDSTATUS_PROTOCOLCLASSID	= ( BINDSTATUS_ENDUPLOADDATA + 1 ) ,
        BINDSTATUS_ENCODING	= ( BINDSTATUS_PROTOCOLCLASSID + 1 ) ,
        BINDSTATUS_VERIFIEDMIMETYPEAVAILABLE	= ( BINDSTATUS_ENCODING + 1 ) ,
        BINDSTATUS_CLASSINSTALLLOCATION	= ( BINDSTATUS_VERIFIEDMIMETYPEAVAILABLE + 1 ) ,
        BINDSTATUS_DECODING	= ( BINDSTATUS_CLASSINSTALLLOCATION + 1 ) ,
        BINDSTATUS_LOADINGMIMEHANDLER	= ( BINDSTATUS_DECODING + 1 ) ,
        BINDSTATUS_CONTENTDISPOSITIONATTACH	= ( BINDSTATUS_LOADINGMIMEHANDLER + 1 ) ,
        BINDSTATUS_FILTERREPORTMIMETYPE	= ( BINDSTATUS_CONTENTDISPOSITIONATTACH + 1 ) ,
        BINDSTATUS_CLSIDCANINSTANTIATE	= ( BINDSTATUS_FILTERREPORTMIMETYPE + 1 ) ,
        BINDSTATUS_IUNKNOWNAVAILABLE	= ( BINDSTATUS_CLSIDCANINSTANTIATE + 1 ) ,
        BINDSTATUS_DIRECTBIND	= ( BINDSTATUS_IUNKNOWNAVAILABLE + 1 ) ,
        BINDSTATUS_RAWMIMETYPE	= ( BINDSTATUS_DIRECTBIND + 1 ) ,
        BINDSTATUS_PROXYDETECTING	= ( BINDSTATUS_RAWMIMETYPE + 1 ) ,
        BINDSTATUS_ACCEPTRANGES	= ( BINDSTATUS_PROXYDETECTING + 1 ) ,
        BINDSTATUS_COOKIE_SENT	= ( BINDSTATUS_ACCEPTRANGES + 1 ) ,
        BINDSTATUS_COMPACT_POLICY_RECEIVED	= ( BINDSTATUS_COOKIE_SENT + 1 ) ,
        BINDSTATUS_COOKIE_SUPPRESSED	= ( BINDSTATUS_COMPACT_POLICY_RECEIVED + 1 ) ,
        BINDSTATUS_COOKIE_STATE_UNKNOWN	= ( BINDSTATUS_COOKIE_SUPPRESSED + 1 ) ,
        BINDSTATUS_COOKIE_STATE_ACCEPT	= ( BINDSTATUS_COOKIE_STATE_UNKNOWN + 1 ) ,
        BINDSTATUS_COOKIE_STATE_REJECT	= ( BINDSTATUS_COOKIE_STATE_ACCEPT + 1 ) ,
        BINDSTATUS_COOKIE_STATE_PROMPT	= ( BINDSTATUS_COOKIE_STATE_REJECT + 1 ) ,
        BINDSTATUS_COOKIE_STATE_LEASH	= ( BINDSTATUS_COOKIE_STATE_PROMPT + 1 ) ,
        BINDSTATUS_COOKIE_STATE_DOWNGRADE	= ( BINDSTATUS_COOKIE_STATE_LEASH + 1 ) ,
        BINDSTATUS_POLICY_HREF	= ( BINDSTATUS_COOKIE_STATE_DOWNGRADE + 1 ) ,
        BINDSTATUS_P3P_HEADER	= ( BINDSTATUS_POLICY_HREF + 1 ) ,
        BINDSTATUS_SESSION_COOKIE_RECEIVED	= ( BINDSTATUS_P3P_HEADER + 1 ) ,
        BINDSTATUS_PERSISTENT_COOKIE_RECEIVED	= ( BINDSTATUS_SESSION_COOKIE_RECEIVED + 1 ) ,
        BINDSTATUS_SESSION_COOKIES_ALLOWED	= ( BINDSTATUS_PERSISTENT_COOKIE_RECEIVED + 1 ) ,
        BINDSTATUS_CACHECONTROL	= ( BINDSTATUS_SESSION_COOKIES_ALLOWED + 1 ) ,
        BINDSTATUS_CONTENTDISPOSITIONFILENAME	= ( BINDSTATUS_CACHECONTROL + 1 ) ,
        BINDSTATUS_MIMETEXTPLAINMISMATCH	= ( BINDSTATUS_CONTENTDISPOSITIONFILENAME + 1 ) ,
        BINDSTATUS_PUBLISHERAVAILABLE	= ( BINDSTATUS_MIMETEXTPLAINMISMATCH + 1 ) ,
        BINDSTATUS_DISPLAYNAMEAVAILABLE	= ( BINDSTATUS_PUBLISHERAVAILABLE + 1 ) ,
        BINDSTATUS_SSLUX_NAVBLOCKED	= ( BINDSTATUS_DISPLAYNAMEAVAILABLE + 1 ) ,
        BINDSTATUS_SERVER_MIMETYPEAVAILABLE	= ( BINDSTATUS_SSLUX_NAVBLOCKED + 1 ) ,
        BINDSTATUS_SNIFFED_CLASSIDAVAILABLE	= ( BINDSTATUS_SERVER_MIMETYPEAVAILABLE + 1 ) ,
        BINDSTATUS_64BIT_PROGRESS	= ( BINDSTATUS_SNIFFED_CLASSIDAVAILABLE + 1 ) ,
        BINDSTATUS_LAST	= BINDSTATUS_64BIT_PROGRESS,
        BINDSTATUS_RESERVED_0	= ( BINDSTATUS_LAST + 1 ) ,
        BINDSTATUS_RESERVED_1	= ( BINDSTATUS_RESERVED_0 + 1 ) ,
        BINDSTATUS_RESERVED_2	= ( BINDSTATUS_RESERVED_1 + 1 ) ,
        BINDSTATUS_RESERVED_3	= ( BINDSTATUS_RESERVED_2 + 1 ) ,
        BINDSTATUS_RESERVED_4	= ( BINDSTATUS_RESERVED_3 + 1 ) ,
        BINDSTATUS_RESERVED_5	= ( BINDSTATUS_RESERVED_4 + 1 ) ,
        BINDSTATUS_RESERVED_6	= ( BINDSTATUS_RESERVED_5 + 1 ) ,
        BINDSTATUS_RESERVED_7	= ( BINDSTATUS_RESERVED_6 + 1 ) ,
        BINDSTATUS_RESERVED_8	= ( BINDSTATUS_RESERVED_7 + 1 ) ,
        BINDSTATUS_RESERVED_9	= ( BINDSTATUS_RESERVED_8 + 1 ) ,
        BINDSTATUS_RESERVED_A	= ( BINDSTATUS_RESERVED_9 + 1 ) ,
        BINDSTATUS_RESERVED_B	= ( BINDSTATUS_RESERVED_A + 1 ) ,
        BINDSTATUS_RESERVED_C	= ( BINDSTATUS_RESERVED_B + 1 ) ,
        BINDSTATUS_RESERVED_D	= ( BINDSTATUS_RESERVED_C + 1 ) ,
        BINDSTATUS_RESERVED_E	= ( BINDSTATUS_RESERVED_D + 1 ) ,
        BINDSTATUS_RESERVED_F	= ( BINDSTATUS_RESERVED_E + 1 ) ,
        BINDSTATUS_RESERVED_10	= ( BINDSTATUS_RESERVED_F + 1 ) ,
        BINDSTATUS_RESERVED_11	= ( BINDSTATUS_RESERVED_10 + 1 ) ,
        BINDSTATUS_RESERVED_12	= ( BINDSTATUS_RESERVED_11 + 1 ) ,
        BINDSTATUS_RESERVED_13	= ( BINDSTATUS_RESERVED_12 + 1 ) ,
        BINDSTATUS_RESERVED_14	= ( BINDSTATUS_RESERVED_13 + 1 ) ,
        BINDSTATUS_LAST_PRIVATE	= BINDSTATUS_RESERVED_14
    } 	BINDSTATUS;


EXTERN_C const IID IID_IBindStatusCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9c1-baf9-11ce-8c82-00aa004ba90b")
    IBindStatusCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnStartBinding( 
            /* [in] */ DWORD dwReserved,
            /* [in] */ __RPC__in_opt IBinding *pib) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPriority( 
            /* [out] */ __RPC__out LONG *pnPriority) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnLowResource( 
            /* [in] */ DWORD reserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnProgress( 
            /* [in] */ ULONG ulProgress,
            /* [in] */ ULONG ulProgressMax,
            /* [in] */ ULONG ulStatusCode,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szStatusText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnStopBinding( 
            /* [in] */ HRESULT hresult,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szError) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetBindInfo( 
            /* [out] */ DWORD *grfBINDF,
            /* [unique][out][in] */ BINDINFO *pbindinfo) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE OnDataAvailable( 
            /* [in] */ DWORD grfBSCF,
            /* [in] */ DWORD dwSize,
            /* [in] */ FORMATETC *pformatetc,
            /* [in] */ STGMEDIUM *pstgmed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnObjectAvailable( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBindStatusCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBindStatusCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBindStatusCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBindStatusCallback * This);
        
        DECLSPEC_XFGVIRT(IBindStatusCallback, OnStartBinding)
        HRESULT ( STDMETHODCALLTYPE *OnStartBinding )( 
            __RPC__in IBindStatusCallback * This,
            /* [in] */ DWORD dwReserved,
            /* [in] */ __RPC__in_opt IBinding *pib);
        
        DECLSPEC_XFGVIRT(IBindStatusCallback, GetPriority)
        HRESULT ( STDMETHODCALLTYPE *GetPriority )( 
            __RPC__in IBindStatusCallback * This,
            /* [out] */ __RPC__out LONG *pnPriority);
        
        DECLSPEC_XFGVIRT(IBindStatusCallback, OnLowResource)
        HRESULT ( STDMETHODCALLTYPE *OnLowResource )( 
            __RPC__in IBindStatusCallback * This,
            /* [in] */ DWORD reserved);
        
        DECLSPEC_XFGVIRT(IBindStatusCallback, OnProgress)
        HRESULT ( STDMETHODCALLTYPE *OnProgress )( 
            __RPC__in IBindStatusCallback * This,
            /* [in] */ ULONG ulProgress,
            /* [in] */ ULONG ulProgressMax,
            /* [in] */ ULONG ulStatusCode,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szStatusText);
        
        DECLSPEC_XFGVIRT(IBindStatusCallback, OnStopBinding)
        HRESULT ( STDMETHODCALLTYPE *OnStopBinding )( 
            __RPC__in IBindStatusCallback * This,
            /* [in] */ HRESULT hresult,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szError);
        
        DECLSPEC_XFGVIRT(IBindStatusCallback, GetBindInfo)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetBindInfo )( 
            IBindStatusCallback * This,
            /* [out] */ DWORD *grfBINDF,
            /* [unique][out][in] */ BINDINFO *pbindinfo);
        
        DECLSPEC_XFGVIRT(IBindStatusCallback, OnDataAvailable)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *OnDataAvailable )( 
            IBindStatusCallback * This,
            /* [in] */ DWORD grfBSCF,
            /* [in] */ DWORD dwSize,
            /* [in] */ FORMATETC *pformatetc,
            /* [in] */ STGMEDIUM *pstgmed);
        
        DECLSPEC_XFGVIRT(IBindStatusCallback, OnObjectAvailable)
        HRESULT ( STDMETHODCALLTYPE *OnObjectAvailable )( 
            __RPC__in IBindStatusCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk);
        
        END_INTERFACE
    } IBindStatusCallbackVtbl;

    interface IBindStatusCallback
    {
        CONST_VTBL struct IBindStatusCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBindStatusCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBindStatusCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBindStatusCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBindStatusCallback_OnStartBinding(This,dwReserved,pib)	\
    ( (This)->lpVtbl -> OnStartBinding(This,dwReserved,pib) ) 

#define IBindStatusCallback_GetPriority(This,pnPriority)	\
    ( (This)->lpVtbl -> GetPriority(This,pnPriority) ) 

#define IBindStatusCallback_OnLowResource(This,reserved)	\
    ( (This)->lpVtbl -> OnLowResource(This,reserved) ) 

#define IBindStatusCallback_OnProgress(This,ulProgress,ulProgressMax,ulStatusCode,szStatusText)	\
    ( (This)->lpVtbl -> OnProgress(This,ulProgress,ulProgressMax,ulStatusCode,szStatusText) ) 

#define IBindStatusCallback_OnStopBinding(This,hresult,szError)	\
    ( (This)->lpVtbl -> OnStopBinding(This,hresult,szError) ) 

#define IBindStatusCallback_GetBindInfo(This,grfBINDF,pbindinfo)	\
    ( (This)->lpVtbl -> GetBindInfo(This,grfBINDF,pbindinfo) ) 

#define IBindStatusCallback_OnDataAvailable(This,grfBSCF,dwSize,pformatetc,pstgmed)	\
    ( (This)->lpVtbl -> OnDataAvailable(This,grfBSCF,dwSize,pformatetc,pstgmed) ) 

#define IBindStatusCallback_OnObjectAvailable(This,riid,punk)	\
    ( (This)->lpVtbl -> OnObjectAvailable(This,riid,punk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IBindStatusCallback_RemoteGetBindInfo_Proxy( 
    __RPC__in IBindStatusCallback * This,
    /* [out] */ __RPC__out DWORD *grfBINDF,
    /* [unique][out][in] */ __RPC__inout_opt RemBINDINFO *pbindinfo,
    /* [unique][out][in] */ __RPC__inout_opt RemSTGMEDIUM *pstgmed);


void __RPC_STUB IBindStatusCallback_RemoteGetBindInfo_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IBindStatusCallback_RemoteOnDataAvailable_Proxy( 
    __RPC__in IBindStatusCallback * This,
    /* [in] */ DWORD grfBSCF,
    /* [in] */ DWORD dwSize,
    /* [in] */ __RPC__in RemFORMATETC *pformatetc,
    /* [in] */ __RPC__in RemSTGMEDIUM *pstgmed);


void __RPC_STUB IBindStatusCallback_RemoteOnDataAvailable_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IBindStatusCallback_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0005 */
/* [local] */ 

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#ifndef _LPBINDSTATUSCALLBACKEX_DEFINED
#define _LPBINDSTATUSCALLBACKEX_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0005_v0_0_s_ifspec;

#ifndef __IBindStatusCallbackEx_INTERFACE_DEFINED__
#define __IBindStatusCallbackEx_INTERFACE_DEFINED__

/* interface IBindStatusCallbackEx */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IBindStatusCallbackEx *LPBINDSTATUSCALLBACKEX;

typedef /* [public] */ 
enum __MIDL_IBindStatusCallbackEx_0001
    {
        BINDF2_DISABLEBASICOVERHTTP	= 0x1,
        BINDF2_DISABLEAUTOCOOKIEHANDLING	= 0x2,
        BINDF2_READ_DATA_GREATER_THAN_4GB	= 0x4,
        BINDF2_DISABLE_HTTP_REDIRECT_XSECURITYID	= 0x8,
        BINDF2_SETDOWNLOADMODE	= 0x20,
        BINDF2_DISABLE_HTTP_REDIRECT_CACHING	= 0x40,
        BINDF2_KEEP_CALLBACK_MODULE_LOADED	= 0x80,
        BINDF2_ALLOW_PROXY_CRED_PROMPT	= 0x100,
        BINDF2_RESERVED_17	= 0x200,
        BINDF2_RESERVED_16	= 0x400,
        BINDF2_RESERVED_15	= 0x800,
        BINDF2_RESERVED_14	= 0x1000,
        BINDF2_RESERVED_13	= 0x2000,
        BINDF2_RESERVED_12	= 0x4000,
        BINDF2_RESERVED_11	= 0x8000,
        BINDF2_RESERVED_10	= 0x10000,
        BINDF2_RESERVED_F	= 0x20000,
        BINDF2_RESERVED_E	= 0x40000,
        BINDF2_RESERVED_D	= 0x80000,
        BINDF2_RESERVED_C	= 0x100000,
        BINDF2_RESERVED_B	= 0x200000,
        BINDF2_RESERVED_A	= 0x400000,
        BINDF2_RESERVED_9	= 0x800000,
        BINDF2_RESERVED_8	= 0x1000000,
        BINDF2_RESERVED_7	= 0x2000000,
        BINDF2_RESERVED_6	= 0x4000000,
        BINDF2_RESERVED_5	= 0x8000000,
        BINDF2_RESERVED_4	= 0x10000000,
        BINDF2_RESERVED_3	= 0x20000000,
        BINDF2_RESERVED_2	= 0x40000000,
        BINDF2_RESERVED_1	= 0x80000000
    } 	BINDF2;


EXTERN_C const IID IID_IBindStatusCallbackEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aaa74ef9-8ee7-4659-88d9-f8c504da73cc")
    IBindStatusCallbackEx : public IBindStatusCallback
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetBindInfoEx( 
            /* [out] */ DWORD *grfBINDF,
            /* [unique][out][in] */ BINDINFO *pbindinfo,
            /* [out] */ DWORD *grfBINDF2,
            /* [out] */ DWORD *pdwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBindStatusCallbackExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBindStatusCallbackEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBindStatusCallbackEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBindStatusCallbackEx * This);
        
        DECLSPEC_XFGVIRT(IBindStatusCallback, OnStartBinding)
        HRESULT ( STDMETHODCALLTYPE *OnStartBinding )( 
            __RPC__in IBindStatusCallbackEx * This,
            /* [in] */ DWORD dwReserved,
            /* [in] */ __RPC__in_opt IBinding *pib);
        
        DECLSPEC_XFGVIRT(IBindStatusCallback, GetPriority)
        HRESULT ( STDMETHODCALLTYPE *GetPriority )( 
            __RPC__in IBindStatusCallbackEx * This,
            /* [out] */ __RPC__out LONG *pnPriority);
        
        DECLSPEC_XFGVIRT(IBindStatusCallback, OnLowResource)
        HRESULT ( STDMETHODCALLTYPE *OnLowResource )( 
            __RPC__in IBindStatusCallbackEx * This,
            /* [in] */ DWORD reserved);
        
        DECLSPEC_XFGVIRT(IBindStatusCallback, OnProgress)
        HRESULT ( STDMETHODCALLTYPE *OnProgress )( 
            __RPC__in IBindStatusCallbackEx * This,
            /* [in] */ ULONG ulProgress,
            /* [in] */ ULONG ulProgressMax,
            /* [in] */ ULONG ulStatusCode,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szStatusText);
        
        DECLSPEC_XFGVIRT(IBindStatusCallback, OnStopBinding)
        HRESULT ( STDMETHODCALLTYPE *OnStopBinding )( 
            __RPC__in IBindStatusCallbackEx * This,
            /* [in] */ HRESULT hresult,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szError);
        
        DECLSPEC_XFGVIRT(IBindStatusCallback, GetBindInfo)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetBindInfo )( 
            IBindStatusCallbackEx * This,
            /* [out] */ DWORD *grfBINDF,
            /* [unique][out][in] */ BINDINFO *pbindinfo);
        
        DECLSPEC_XFGVIRT(IBindStatusCallback, OnDataAvailable)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *OnDataAvailable )( 
            IBindStatusCallbackEx * This,
            /* [in] */ DWORD grfBSCF,
            /* [in] */ DWORD dwSize,
            /* [in] */ FORMATETC *pformatetc,
            /* [in] */ STGMEDIUM *pstgmed);
        
        DECLSPEC_XFGVIRT(IBindStatusCallback, OnObjectAvailable)
        HRESULT ( STDMETHODCALLTYPE *OnObjectAvailable )( 
            __RPC__in IBindStatusCallbackEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk);
        
        DECLSPEC_XFGVIRT(IBindStatusCallbackEx, GetBindInfoEx)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetBindInfoEx )( 
            IBindStatusCallbackEx * This,
            /* [out] */ DWORD *grfBINDF,
            /* [unique][out][in] */ BINDINFO *pbindinfo,
            /* [out] */ DWORD *grfBINDF2,
            /* [out] */ DWORD *pdwReserved);
        
        END_INTERFACE
    } IBindStatusCallbackExVtbl;

    interface IBindStatusCallbackEx
    {
        CONST_VTBL struct IBindStatusCallbackExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBindStatusCallbackEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBindStatusCallbackEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBindStatusCallbackEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBindStatusCallbackEx_OnStartBinding(This,dwReserved,pib)	\
    ( (This)->lpVtbl -> OnStartBinding(This,dwReserved,pib) ) 

#define IBindStatusCallbackEx_GetPriority(This,pnPriority)	\
    ( (This)->lpVtbl -> GetPriority(This,pnPriority) ) 

#define IBindStatusCallbackEx_OnLowResource(This,reserved)	\
    ( (This)->lpVtbl -> OnLowResource(This,reserved) ) 

#define IBindStatusCallbackEx_OnProgress(This,ulProgress,ulProgressMax,ulStatusCode,szStatusText)	\
    ( (This)->lpVtbl -> OnProgress(This,ulProgress,ulProgressMax,ulStatusCode,szStatusText) ) 

#define IBindStatusCallbackEx_OnStopBinding(This,hresult,szError)	\
    ( (This)->lpVtbl -> OnStopBinding(This,hresult,szError) ) 

#define IBindStatusCallbackEx_GetBindInfo(This,grfBINDF,pbindinfo)	\
    ( (This)->lpVtbl -> GetBindInfo(This,grfBINDF,pbindinfo) ) 

#define IBindStatusCallbackEx_OnDataAvailable(This,grfBSCF,dwSize,pformatetc,pstgmed)	\
    ( (This)->lpVtbl -> OnDataAvailable(This,grfBSCF,dwSize,pformatetc,pstgmed) ) 

#define IBindStatusCallbackEx_OnObjectAvailable(This,riid,punk)	\
    ( (This)->lpVtbl -> OnObjectAvailable(This,riid,punk) ) 


#define IBindStatusCallbackEx_GetBindInfoEx(This,grfBINDF,pbindinfo,grfBINDF2,pdwReserved)	\
    ( (This)->lpVtbl -> GetBindInfoEx(This,grfBINDF,pbindinfo,grfBINDF2,pdwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IBindStatusCallbackEx_RemoteGetBindInfoEx_Proxy( 
    __RPC__in IBindStatusCallbackEx * This,
    /* [out] */ __RPC__out DWORD *grfBINDF,
    /* [unique][out][in] */ __RPC__inout_opt RemBINDINFO *pbindinfo,
    /* [unique][out][in] */ __RPC__inout_opt RemSTGMEDIUM *pstgmed,
    /* [out] */ __RPC__out DWORD *grfBINDF2,
    /* [out] */ __RPC__out DWORD *pdwReserved);


void __RPC_STUB IBindStatusCallbackEx_RemoteGetBindInfoEx_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IBindStatusCallbackEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0006 */
/* [local] */ 

#endif
#ifndef _LPAUTHENTICATION_DEFINED
#define _LPAUTHENTICATION_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0006_v0_0_s_ifspec;

#ifndef __IAuthenticate_INTERFACE_DEFINED__
#define __IAuthenticate_INTERFACE_DEFINED__

/* interface IAuthenticate */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IAuthenticate *LPAUTHENTICATION;


EXTERN_C const IID IID_IAuthenticate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9d0-baf9-11ce-8c82-00aa004ba90b")
    IAuthenticate : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Authenticate( 
            /* [out] */ __RPC__deref_out_opt HWND *phwnd,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszUsername,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszPassword) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAuthenticateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAuthenticate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAuthenticate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAuthenticate * This);
        
        DECLSPEC_XFGVIRT(IAuthenticate, Authenticate)
        HRESULT ( STDMETHODCALLTYPE *Authenticate )( 
            __RPC__in IAuthenticate * This,
            /* [out] */ __RPC__deref_out_opt HWND *phwnd,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszUsername,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszPassword);
        
        END_INTERFACE
    } IAuthenticateVtbl;

    interface IAuthenticate
    {
        CONST_VTBL struct IAuthenticateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAuthenticate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAuthenticate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAuthenticate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAuthenticate_Authenticate(This,phwnd,pszUsername,pszPassword)	\
    ( (This)->lpVtbl -> Authenticate(This,phwnd,pszUsername,pszPassword) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAuthenticate_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0007 */
/* [local] */ 

#endif
#ifndef _LPAUTHENTICATIONEX_DEFINED
#define _LPAUTHENTICATIONEX_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0007_v0_0_s_ifspec;

#ifndef __IAuthenticateEx_INTERFACE_DEFINED__
#define __IAuthenticateEx_INTERFACE_DEFINED__

/* interface IAuthenticateEx */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IAuthenticateEx *LPAUTHENTICATIONEX;

typedef /* [public] */ 
enum __MIDL_IAuthenticateEx_0001
    {
        AUTHENTICATEF_PROXY	= 0x1,
        AUTHENTICATEF_BASIC	= 0x2,
        AUTHENTICATEF_HTTP	= 0x4
    } 	AUTHENTICATEF;

typedef struct _tagAUTHENTICATEINFO
    {
    DWORD dwFlags;
    DWORD dwReserved;
    } 	AUTHENTICATEINFO;


EXTERN_C const IID IID_IAuthenticateEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2ad1edaf-d83d-48b5-9adf-03dbe19f53bd")
    IAuthenticateEx : public IAuthenticate
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AuthenticateEx( 
            /* [out] */ __RPC__deref_out_opt HWND *phwnd,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszUsername,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszPassword,
            /* [in] */ __RPC__in AUTHENTICATEINFO *pauthinfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAuthenticateExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAuthenticateEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAuthenticateEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAuthenticateEx * This);
        
        DECLSPEC_XFGVIRT(IAuthenticate, Authenticate)
        HRESULT ( STDMETHODCALLTYPE *Authenticate )( 
            __RPC__in IAuthenticateEx * This,
            /* [out] */ __RPC__deref_out_opt HWND *phwnd,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszUsername,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszPassword);
        
        DECLSPEC_XFGVIRT(IAuthenticateEx, AuthenticateEx)
        HRESULT ( STDMETHODCALLTYPE *AuthenticateEx )( 
            __RPC__in IAuthenticateEx * This,
            /* [out] */ __RPC__deref_out_opt HWND *phwnd,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszUsername,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszPassword,
            /* [in] */ __RPC__in AUTHENTICATEINFO *pauthinfo);
        
        END_INTERFACE
    } IAuthenticateExVtbl;

    interface IAuthenticateEx
    {
        CONST_VTBL struct IAuthenticateExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAuthenticateEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAuthenticateEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAuthenticateEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAuthenticateEx_Authenticate(This,phwnd,pszUsername,pszPassword)	\
    ( (This)->lpVtbl -> Authenticate(This,phwnd,pszUsername,pszPassword) ) 


#define IAuthenticateEx_AuthenticateEx(This,phwnd,pszUsername,pszPassword,pauthinfo)	\
    ( (This)->lpVtbl -> AuthenticateEx(This,phwnd,pszUsername,pszPassword,pauthinfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAuthenticateEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0008 */
/* [local] */ 

#endif
#ifndef _LPHTTPNEGOTIATE_DEFINED
#define _LPHTTPNEGOTIATE_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0008_v0_0_s_ifspec;

#ifndef __IHttpNegotiate_INTERFACE_DEFINED__
#define __IHttpNegotiate_INTERFACE_DEFINED__

/* interface IHttpNegotiate */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IHttpNegotiate *LPHTTPNEGOTIATE;


EXTERN_C const IID IID_IHttpNegotiate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9d2-baf9-11ce-8c82-00aa004ba90b")
    IHttpNegotiate : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BeginningTransaction( 
            /* [in] */ __RPC__in LPCWSTR szURL,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szHeaders,
            /* [in] */ DWORD dwReserved,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszAdditionalHeaders) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnResponse( 
            /* [in] */ DWORD dwResponseCode,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szResponseHeaders,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szRequestHeaders,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszAdditionalRequestHeaders) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHttpNegotiateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IHttpNegotiate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IHttpNegotiate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IHttpNegotiate * This);
        
        DECLSPEC_XFGVIRT(IHttpNegotiate, BeginningTransaction)
        HRESULT ( STDMETHODCALLTYPE *BeginningTransaction )( 
            __RPC__in IHttpNegotiate * This,
            /* [in] */ __RPC__in LPCWSTR szURL,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szHeaders,
            /* [in] */ DWORD dwReserved,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszAdditionalHeaders);
        
        DECLSPEC_XFGVIRT(IHttpNegotiate, OnResponse)
        HRESULT ( STDMETHODCALLTYPE *OnResponse )( 
            __RPC__in IHttpNegotiate * This,
            /* [in] */ DWORD dwResponseCode,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szResponseHeaders,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szRequestHeaders,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszAdditionalRequestHeaders);
        
        END_INTERFACE
    } IHttpNegotiateVtbl;

    interface IHttpNegotiate
    {
        CONST_VTBL struct IHttpNegotiateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHttpNegotiate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHttpNegotiate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHttpNegotiate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHttpNegotiate_BeginningTransaction(This,szURL,szHeaders,dwReserved,pszAdditionalHeaders)	\
    ( (This)->lpVtbl -> BeginningTransaction(This,szURL,szHeaders,dwReserved,pszAdditionalHeaders) ) 

#define IHttpNegotiate_OnResponse(This,dwResponseCode,szResponseHeaders,szRequestHeaders,pszAdditionalRequestHeaders)	\
    ( (This)->lpVtbl -> OnResponse(This,dwResponseCode,szResponseHeaders,szRequestHeaders,pszAdditionalRequestHeaders) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHttpNegotiate_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0009 */
/* [local] */ 

#endif
#ifndef _LPHTTPNEGOTIATE2_DEFINED
#define _LPHTTPNEGOTIATE2_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0009_v0_0_s_ifspec;

#ifndef __IHttpNegotiate2_INTERFACE_DEFINED__
#define __IHttpNegotiate2_INTERFACE_DEFINED__

/* interface IHttpNegotiate2 */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IHttpNegotiate2 *LPHTTPNEGOTIATE2;


EXTERN_C const IID IID_IHttpNegotiate2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4F9F9FCB-E0F4-48eb-B7AB-FA2EA9365CB4")
    IHttpNegotiate2 : public IHttpNegotiate
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRootSecurityId( 
            /* [size_is][out] */ __RPC__out_ecount_full(*pcbSecurityId) BYTE *pbSecurityId,
            /* [out][in] */ __RPC__inout DWORD *pcbSecurityId,
            /* [in] */ DWORD_PTR dwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHttpNegotiate2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IHttpNegotiate2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IHttpNegotiate2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IHttpNegotiate2 * This);
        
        DECLSPEC_XFGVIRT(IHttpNegotiate, BeginningTransaction)
        HRESULT ( STDMETHODCALLTYPE *BeginningTransaction )( 
            __RPC__in IHttpNegotiate2 * This,
            /* [in] */ __RPC__in LPCWSTR szURL,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szHeaders,
            /* [in] */ DWORD dwReserved,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszAdditionalHeaders);
        
        DECLSPEC_XFGVIRT(IHttpNegotiate, OnResponse)
        HRESULT ( STDMETHODCALLTYPE *OnResponse )( 
            __RPC__in IHttpNegotiate2 * This,
            /* [in] */ DWORD dwResponseCode,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szResponseHeaders,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szRequestHeaders,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszAdditionalRequestHeaders);
        
        DECLSPEC_XFGVIRT(IHttpNegotiate2, GetRootSecurityId)
        HRESULT ( STDMETHODCALLTYPE *GetRootSecurityId )( 
            __RPC__in IHttpNegotiate2 * This,
            /* [size_is][out] */ __RPC__out_ecount_full(*pcbSecurityId) BYTE *pbSecurityId,
            /* [out][in] */ __RPC__inout DWORD *pcbSecurityId,
            /* [in] */ DWORD_PTR dwReserved);
        
        END_INTERFACE
    } IHttpNegotiate2Vtbl;

    interface IHttpNegotiate2
    {
        CONST_VTBL struct IHttpNegotiate2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHttpNegotiate2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHttpNegotiate2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHttpNegotiate2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHttpNegotiate2_BeginningTransaction(This,szURL,szHeaders,dwReserved,pszAdditionalHeaders)	\
    ( (This)->lpVtbl -> BeginningTransaction(This,szURL,szHeaders,dwReserved,pszAdditionalHeaders) ) 

#define IHttpNegotiate2_OnResponse(This,dwResponseCode,szResponseHeaders,szRequestHeaders,pszAdditionalRequestHeaders)	\
    ( (This)->lpVtbl -> OnResponse(This,dwResponseCode,szResponseHeaders,szRequestHeaders,pszAdditionalRequestHeaders) ) 


#define IHttpNegotiate2_GetRootSecurityId(This,pbSecurityId,pcbSecurityId,dwReserved)	\
    ( (This)->lpVtbl -> GetRootSecurityId(This,pbSecurityId,pcbSecurityId,dwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHttpNegotiate2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0010 */
/* [local] */ 

#endif
#ifndef _LPHTTPNEGOTIATE3_DEFINED
#define _LPHTTPNEGOTIATE3_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0010_v0_0_s_ifspec;

#ifndef __IHttpNegotiate3_INTERFACE_DEFINED__
#define __IHttpNegotiate3_INTERFACE_DEFINED__

/* interface IHttpNegotiate3 */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IHttpNegotiate3 *LPHTTPNEGOTIATE3;


EXTERN_C const IID IID_IHttpNegotiate3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("57b6c80a-34c2-4602-bc26-66a02fc57153")
    IHttpNegotiate3 : public IHttpNegotiate2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSerializedClientCertContext( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbCert) BYTE **ppbCert,
            /* [out] */ __RPC__out DWORD *pcbCert) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHttpNegotiate3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IHttpNegotiate3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IHttpNegotiate3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IHttpNegotiate3 * This);
        
        DECLSPEC_XFGVIRT(IHttpNegotiate, BeginningTransaction)
        HRESULT ( STDMETHODCALLTYPE *BeginningTransaction )( 
            __RPC__in IHttpNegotiate3 * This,
            /* [in] */ __RPC__in LPCWSTR szURL,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szHeaders,
            /* [in] */ DWORD dwReserved,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszAdditionalHeaders);
        
        DECLSPEC_XFGVIRT(IHttpNegotiate, OnResponse)
        HRESULT ( STDMETHODCALLTYPE *OnResponse )( 
            __RPC__in IHttpNegotiate3 * This,
            /* [in] */ DWORD dwResponseCode,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szResponseHeaders,
            /* [unique][in] */ __RPC__in_opt LPCWSTR szRequestHeaders,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pszAdditionalRequestHeaders);
        
        DECLSPEC_XFGVIRT(IHttpNegotiate2, GetRootSecurityId)
        HRESULT ( STDMETHODCALLTYPE *GetRootSecurityId )( 
            __RPC__in IHttpNegotiate3 * This,
            /* [size_is][out] */ __RPC__out_ecount_full(*pcbSecurityId) BYTE *pbSecurityId,
            /* [out][in] */ __RPC__inout DWORD *pcbSecurityId,
            /* [in] */ DWORD_PTR dwReserved);
        
        DECLSPEC_XFGVIRT(IHttpNegotiate3, GetSerializedClientCertContext)
        HRESULT ( STDMETHODCALLTYPE *GetSerializedClientCertContext )( 
            __RPC__in IHttpNegotiate3 * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbCert) BYTE **ppbCert,
            /* [out] */ __RPC__out DWORD *pcbCert);
        
        END_INTERFACE
    } IHttpNegotiate3Vtbl;

    interface IHttpNegotiate3
    {
        CONST_VTBL struct IHttpNegotiate3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHttpNegotiate3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHttpNegotiate3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHttpNegotiate3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHttpNegotiate3_BeginningTransaction(This,szURL,szHeaders,dwReserved,pszAdditionalHeaders)	\
    ( (This)->lpVtbl -> BeginningTransaction(This,szURL,szHeaders,dwReserved,pszAdditionalHeaders) ) 

#define IHttpNegotiate3_OnResponse(This,dwResponseCode,szResponseHeaders,szRequestHeaders,pszAdditionalRequestHeaders)	\
    ( (This)->lpVtbl -> OnResponse(This,dwResponseCode,szResponseHeaders,szRequestHeaders,pszAdditionalRequestHeaders) ) 


#define IHttpNegotiate3_GetRootSecurityId(This,pbSecurityId,pcbSecurityId,dwReserved)	\
    ( (This)->lpVtbl -> GetRootSecurityId(This,pbSecurityId,pcbSecurityId,dwReserved) ) 


#define IHttpNegotiate3_GetSerializedClientCertContext(This,ppbCert,pcbCert)	\
    ( (This)->lpVtbl -> GetSerializedClientCertContext(This,ppbCert,pcbCert) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHttpNegotiate3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0011 */
/* [local] */ 

#endif
#ifndef _LPWININETFILESTREAM_DEFINED
#define _LPWININETFILESTREAM_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0011_v0_0_s_ifspec;

#ifndef __IWinInetFileStream_INTERFACE_DEFINED__
#define __IWinInetFileStream_INTERFACE_DEFINED__

/* interface IWinInetFileStream */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IWinInetFileStream *LPWININETFILESTREAM;


EXTERN_C const IID IID_IWinInetFileStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F134C4B7-B1F8-4e75-B886-74B90943BECB")
    IWinInetFileStream : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetHandleForUnlock( 
            /* [in] */ DWORD_PTR hWinInetLockHandle,
            /* [in] */ DWORD_PTR dwReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDeleteFile( 
            /* [in] */ DWORD_PTR dwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWinInetFileStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWinInetFileStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWinInetFileStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWinInetFileStream * This);
        
        DECLSPEC_XFGVIRT(IWinInetFileStream, SetHandleForUnlock)
        HRESULT ( STDMETHODCALLTYPE *SetHandleForUnlock )( 
            __RPC__in IWinInetFileStream * This,
            /* [in] */ DWORD_PTR hWinInetLockHandle,
            /* [in] */ DWORD_PTR dwReserved);
        
        DECLSPEC_XFGVIRT(IWinInetFileStream, SetDeleteFile)
        HRESULT ( STDMETHODCALLTYPE *SetDeleteFile )( 
            __RPC__in IWinInetFileStream * This,
            /* [in] */ DWORD_PTR dwReserved);
        
        END_INTERFACE
    } IWinInetFileStreamVtbl;

    interface IWinInetFileStream
    {
        CONST_VTBL struct IWinInetFileStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWinInetFileStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWinInetFileStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWinInetFileStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWinInetFileStream_SetHandleForUnlock(This,hWinInetLockHandle,dwReserved)	\
    ( (This)->lpVtbl -> SetHandleForUnlock(This,hWinInetLockHandle,dwReserved) ) 

#define IWinInetFileStream_SetDeleteFile(This,dwReserved)	\
    ( (This)->lpVtbl -> SetDeleteFile(This,dwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWinInetFileStream_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0012 */
/* [local] */ 

#endif
#ifndef _LPWINDOWFORBINDINGUI_DEFINED
#define _LPWINDOWFORBINDINGUI_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0012_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0012_v0_0_s_ifspec;

#ifndef __IWindowForBindingUI_INTERFACE_DEFINED__
#define __IWindowForBindingUI_INTERFACE_DEFINED__

/* interface IWindowForBindingUI */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IWindowForBindingUI *LPWINDOWFORBINDINGUI;


EXTERN_C const IID IID_IWindowForBindingUI;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9d5-bafa-11ce-8c82-00aa004ba90b")
    IWindowForBindingUI : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetWindow( 
            /* [in] */ REFGUID rguidReason,
            /* [out] */ HWND *phwnd) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWindowForBindingUIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWindowForBindingUI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWindowForBindingUI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWindowForBindingUI * This);
        
        DECLSPEC_XFGVIRT(IWindowForBindingUI, GetWindow)
        HRESULT ( STDMETHODCALLTYPE *GetWindow )( 
            IWindowForBindingUI * This,
            /* [in] */ REFGUID rguidReason,
            /* [out] */ HWND *phwnd);
        
        END_INTERFACE
    } IWindowForBindingUIVtbl;

    interface IWindowForBindingUI
    {
        CONST_VTBL struct IWindowForBindingUIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWindowForBindingUI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWindowForBindingUI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWindowForBindingUI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWindowForBindingUI_GetWindow(This,rguidReason,phwnd)	\
    ( (This)->lpVtbl -> GetWindow(This,rguidReason,phwnd) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWindowForBindingUI_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0013 */
/* [local] */ 

#endif
#ifndef _LPCODEINSTALL_DEFINED
#define _LPCODEINSTALL_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0013_v0_0_s_ifspec;

#ifndef __ICodeInstall_INTERFACE_DEFINED__
#define __ICodeInstall_INTERFACE_DEFINED__

/* interface ICodeInstall */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ ICodeInstall *LPCODEINSTALL;

typedef /* [public] */ 
enum __MIDL_ICodeInstall_0001
    {
        CIP_DISK_FULL	= 0,
        CIP_ACCESS_DENIED	= ( CIP_DISK_FULL + 1 ) ,
        CIP_NEWER_VERSION_EXISTS	= ( CIP_ACCESS_DENIED + 1 ) ,
        CIP_OLDER_VERSION_EXISTS	= ( CIP_NEWER_VERSION_EXISTS + 1 ) ,
        CIP_NAME_CONFLICT	= ( CIP_OLDER_VERSION_EXISTS + 1 ) ,
        CIP_TRUST_VERIFICATION_COMPONENT_MISSING	= ( CIP_NAME_CONFLICT + 1 ) ,
        CIP_EXE_SELF_REGISTERATION_TIMEOUT	= ( CIP_TRUST_VERIFICATION_COMPONENT_MISSING + 1 ) ,
        CIP_UNSAFE_TO_ABORT	= ( CIP_EXE_SELF_REGISTERATION_TIMEOUT + 1 ) ,
        CIP_NEED_REBOOT	= ( CIP_UNSAFE_TO_ABORT + 1 ) ,
        CIP_NEED_REBOOT_UI_PERMISSION	= ( CIP_NEED_REBOOT + 1 ) 
    } 	CIP_STATUS;


EXTERN_C const IID IID_ICodeInstall;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9d1-baf9-11ce-8c82-00aa004ba90b")
    ICodeInstall : public IWindowForBindingUI
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnCodeInstallProblem( 
            /* [in] */ ULONG ulStatusCode,
            /* [unique][in] */ LPCWSTR szDestination,
            /* [unique][in] */ LPCWSTR szSource,
            /* [in] */ DWORD dwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICodeInstallVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICodeInstall * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICodeInstall * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICodeInstall * This);
        
        DECLSPEC_XFGVIRT(IWindowForBindingUI, GetWindow)
        HRESULT ( STDMETHODCALLTYPE *GetWindow )( 
            ICodeInstall * This,
            /* [in] */ REFGUID rguidReason,
            /* [out] */ HWND *phwnd);
        
        DECLSPEC_XFGVIRT(ICodeInstall, OnCodeInstallProblem)
        HRESULT ( STDMETHODCALLTYPE *OnCodeInstallProblem )( 
            ICodeInstall * This,
            /* [in] */ ULONG ulStatusCode,
            /* [unique][in] */ LPCWSTR szDestination,
            /* [unique][in] */ LPCWSTR szSource,
            /* [in] */ DWORD dwReserved);
        
        END_INTERFACE
    } ICodeInstallVtbl;

    interface ICodeInstall
    {
        CONST_VTBL struct ICodeInstallVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICodeInstall_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICodeInstall_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICodeInstall_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICodeInstall_GetWindow(This,rguidReason,phwnd)	\
    ( (This)->lpVtbl -> GetWindow(This,rguidReason,phwnd) ) 


#define ICodeInstall_OnCodeInstallProblem(This,ulStatusCode,szDestination,szSource,dwReserved)	\
    ( (This)->lpVtbl -> OnCodeInstallProblem(This,ulStatusCode,szDestination,szSource,dwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICodeInstall_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0014 */
/* [local] */ 

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#if (_WIN32_IE >= _WIN32_IE_IE70)
#ifndef _LPUri_DEFINED
#define _LPUri_DEFINED
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0014_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0014_v0_0_s_ifspec;

#ifndef __IUri_INTERFACE_DEFINED__
#define __IUri_INTERFACE_DEFINED__

/* interface IUri */
/* [unique][uuid][object] */ 

typedef /* [public][public][public][public][public][helpstring] */ 
enum __MIDL_IUri_0001
    {
        Uri_PROPERTY_ABSOLUTE_URI	= 0,
        Uri_PROPERTY_STRING_START	= Uri_PROPERTY_ABSOLUTE_URI,
        Uri_PROPERTY_AUTHORITY	= 1,
        Uri_PROPERTY_DISPLAY_URI	= 2,
        Uri_PROPERTY_DOMAIN	= 3,
        Uri_PROPERTY_EXTENSION	= 4,
        Uri_PROPERTY_FRAGMENT	= 5,
        Uri_PROPERTY_HOST	= 6,
        Uri_PROPERTY_PASSWORD	= 7,
        Uri_PROPERTY_PATH	= 8,
        Uri_PROPERTY_PATH_AND_QUERY	= 9,
        Uri_PROPERTY_QUERY	= 10,
        Uri_PROPERTY_RAW_URI	= 11,
        Uri_PROPERTY_SCHEME_NAME	= 12,
        Uri_PROPERTY_USER_INFO	= 13,
        Uri_PROPERTY_USER_NAME	= 14,
        Uri_PROPERTY_STRING_LAST	= Uri_PROPERTY_USER_NAME,
        Uri_PROPERTY_HOST_TYPE	= 15,
        Uri_PROPERTY_DWORD_START	= Uri_PROPERTY_HOST_TYPE,
        Uri_PROPERTY_PORT	= 16,
        Uri_PROPERTY_SCHEME	= 17,
        Uri_PROPERTY_ZONE	= 18,
        Uri_PROPERTY_DWORD_LAST	= Uri_PROPERTY_ZONE
    } 	Uri_PROPERTY;

typedef /* [public][helpstring] */ 
enum __MIDL_IUri_0002
    {
        Uri_HOST_UNKNOWN	= 0,
        Uri_HOST_DNS	= ( Uri_HOST_UNKNOWN + 1 ) ,
        Uri_HOST_IPV4	= ( Uri_HOST_DNS + 1 ) ,
        Uri_HOST_IPV6	= ( Uri_HOST_IPV4 + 1 ) ,
        Uri_HOST_IDN	= ( Uri_HOST_IPV6 + 1 ) 
    } 	Uri_HOST_TYPE;


EXTERN_C const IID IID_IUri;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A39EE748-6A27-4817-A6F2-13914BEF5890")
    IUri : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPropertyBSTR( 
            /* [range][in] */ Uri_PROPERTY uriProp,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrProperty,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPropertyLength( 
            /* [range][in] */ Uri_PROPERTY uriProp,
            /* [out] */ __RPC__out DWORD *pcchProperty,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPropertyDWORD( 
            /* [range][in] */ Uri_PROPERTY uriProp,
            /* [out] */ __RPC__out DWORD *pdwProperty,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE HasProperty( 
            /* [range][in] */ Uri_PROPERTY uriProp,
            /* [out] */ __RPC__out BOOL *pfHasProperty) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetAbsoluteUri( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrAbsoluteUri) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetAuthority( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrAuthority) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetDisplayUri( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDisplayString) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetDomain( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDomain) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetExtension( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrExtension) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetFragment( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrFragment) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetHost( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrHost) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPassword( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPassword) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPath( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPath) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPathAndQuery( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPathAndQuery) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetQuery( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrQuery) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetRawUri( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrRawUri) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSchemeName( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSchemeName) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetUserInfo( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrUserInfo) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetUserName( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrUserName) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetHostType( 
            /* [out] */ __RPC__out DWORD *pdwHostType) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPort( 
            /* [out] */ __RPC__out DWORD *pdwPort) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetScheme( 
            /* [out] */ __RPC__out DWORD *pdwScheme) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetZone( 
            /* [out] */ __RPC__out DWORD *pdwZone) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [out] */ __RPC__out LPDWORD pdwFlags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE IsEqual( 
            /* [in] */ __RPC__in_opt IUri *pUri,
            /* [out] */ __RPC__out BOOL *pfEqual) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUriVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUri * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUri * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUri * This);
        
        DECLSPEC_XFGVIRT(IUri, GetPropertyBSTR)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyBSTR )( 
            __RPC__in IUri * This,
            /* [range][in] */ Uri_PROPERTY uriProp,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrProperty,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IUri, GetPropertyLength)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyLength )( 
            __RPC__in IUri * This,
            /* [range][in] */ Uri_PROPERTY uriProp,
            /* [out] */ __RPC__out DWORD *pcchProperty,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IUri, GetPropertyDWORD)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyDWORD )( 
            __RPC__in IUri * This,
            /* [range][in] */ Uri_PROPERTY uriProp,
            /* [out] */ __RPC__out DWORD *pdwProperty,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IUri, HasProperty)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *HasProperty )( 
            __RPC__in IUri * This,
            /* [range][in] */ Uri_PROPERTY uriProp,
            /* [out] */ __RPC__out BOOL *pfHasProperty);
        
        DECLSPEC_XFGVIRT(IUri, GetAbsoluteUri)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetAbsoluteUri )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrAbsoluteUri);
        
        DECLSPEC_XFGVIRT(IUri, GetAuthority)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetAuthority )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrAuthority);
        
        DECLSPEC_XFGVIRT(IUri, GetDisplayUri)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDisplayUri )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDisplayString);
        
        DECLSPEC_XFGVIRT(IUri, GetDomain)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDomain )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDomain);
        
        DECLSPEC_XFGVIRT(IUri, GetExtension)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetExtension )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrExtension);
        
        DECLSPEC_XFGVIRT(IUri, GetFragment)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetFragment )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrFragment);
        
        DECLSPEC_XFGVIRT(IUri, GetHost)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetHost )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrHost);
        
        DECLSPEC_XFGVIRT(IUri, GetPassword)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPassword )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPassword);
        
        DECLSPEC_XFGVIRT(IUri, GetPath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPath )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPath);
        
        DECLSPEC_XFGVIRT(IUri, GetPathAndQuery)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPathAndQuery )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPathAndQuery);
        
        DECLSPEC_XFGVIRT(IUri, GetQuery)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetQuery )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrQuery);
        
        DECLSPEC_XFGVIRT(IUri, GetRawUri)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetRawUri )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrRawUri);
        
        DECLSPEC_XFGVIRT(IUri, GetSchemeName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSchemeName )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSchemeName);
        
        DECLSPEC_XFGVIRT(IUri, GetUserInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetUserInfo )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrUserInfo);
        
        DECLSPEC_XFGVIRT(IUri, GetUserName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetUserName )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrUserName);
        
        DECLSPEC_XFGVIRT(IUri, GetHostType)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetHostType )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__out DWORD *pdwHostType);
        
        DECLSPEC_XFGVIRT(IUri, GetPort)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPort )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__out DWORD *pdwPort);
        
        DECLSPEC_XFGVIRT(IUri, GetScheme)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetScheme )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__out DWORD *pdwScheme);
        
        DECLSPEC_XFGVIRT(IUri, GetZone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetZone )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__out DWORD *pdwZone);
        
        DECLSPEC_XFGVIRT(IUri, GetProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in IUri * This,
            /* [out] */ __RPC__out LPDWORD pdwFlags);
        
        DECLSPEC_XFGVIRT(IUri, IsEqual)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            __RPC__in IUri * This,
            /* [in] */ __RPC__in_opt IUri *pUri,
            /* [out] */ __RPC__out BOOL *pfEqual);
        
        END_INTERFACE
    } IUriVtbl;

    interface IUri
    {
        CONST_VTBL struct IUriVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUri_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUri_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUri_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUri_GetPropertyBSTR(This,uriProp,pbstrProperty,dwFlags)	\
    ( (This)->lpVtbl -> GetPropertyBSTR(This,uriProp,pbstrProperty,dwFlags) ) 

#define IUri_GetPropertyLength(This,uriProp,pcchProperty,dwFlags)	\
    ( (This)->lpVtbl -> GetPropertyLength(This,uriProp,pcchProperty,dwFlags) ) 

#define IUri_GetPropertyDWORD(This,uriProp,pdwProperty,dwFlags)	\
    ( (This)->lpVtbl -> GetPropertyDWORD(This,uriProp,pdwProperty,dwFlags) ) 

#define IUri_HasProperty(This,uriProp,pfHasProperty)	\
    ( (This)->lpVtbl -> HasProperty(This,uriProp,pfHasProperty) ) 

#define IUri_GetAbsoluteUri(This,pbstrAbsoluteUri)	\
    ( (This)->lpVtbl -> GetAbsoluteUri(This,pbstrAbsoluteUri) ) 

#define IUri_GetAuthority(This,pbstrAuthority)	\
    ( (This)->lpVtbl -> GetAuthority(This,pbstrAuthority) ) 

#define IUri_GetDisplayUri(This,pbstrDisplayString)	\
    ( (This)->lpVtbl -> GetDisplayUri(This,pbstrDisplayString) ) 

#define IUri_GetDomain(This,pbstrDomain)	\
    ( (This)->lpVtbl -> GetDomain(This,pbstrDomain) ) 

#define IUri_GetExtension(This,pbstrExtension)	\
    ( (This)->lpVtbl -> GetExtension(This,pbstrExtension) ) 

#define IUri_GetFragment(This,pbstrFragment)	\
    ( (This)->lpVtbl -> GetFragment(This,pbstrFragment) ) 

#define IUri_GetHost(This,pbstrHost)	\
    ( (This)->lpVtbl -> GetHost(This,pbstrHost) ) 

#define IUri_GetPassword(This,pbstrPassword)	\
    ( (This)->lpVtbl -> GetPassword(This,pbstrPassword) ) 

#define IUri_GetPath(This,pbstrPath)	\
    ( (This)->lpVtbl -> GetPath(This,pbstrPath) ) 

#define IUri_GetPathAndQuery(This,pbstrPathAndQuery)	\
    ( (This)->lpVtbl -> GetPathAndQuery(This,pbstrPathAndQuery) ) 

#define IUri_GetQuery(This,pbstrQuery)	\
    ( (This)->lpVtbl -> GetQuery(This,pbstrQuery) ) 

#define IUri_GetRawUri(This,pbstrRawUri)	\
    ( (This)->lpVtbl -> GetRawUri(This,pbstrRawUri) ) 

#define IUri_GetSchemeName(This,pbstrSchemeName)	\
    ( (This)->lpVtbl -> GetSchemeName(This,pbstrSchemeName) ) 

#define IUri_GetUserInfo(This,pbstrUserInfo)	\
    ( (This)->lpVtbl -> GetUserInfo(This,pbstrUserInfo) ) 

#define IUri_GetUserName(This,pbstrUserName)	\
    ( (This)->lpVtbl -> GetUserName(This,pbstrUserName) ) 

#define IUri_GetHostType(This,pdwHostType)	\
    ( (This)->lpVtbl -> GetHostType(This,pdwHostType) ) 

#define IUri_GetPort(This,pdwPort)	\
    ( (This)->lpVtbl -> GetPort(This,pdwPort) ) 

#define IUri_GetScheme(This,pdwScheme)	\
    ( (This)->lpVtbl -> GetScheme(This,pdwScheme) ) 

#define IUri_GetZone(This,pdwZone)	\
    ( (This)->lpVtbl -> GetZone(This,pdwZone) ) 

#define IUri_GetProperties(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetProperties(This,pdwFlags) ) 

#define IUri_IsEqual(This,pUri,pfEqual)	\
    ( (This)->lpVtbl -> IsEqual(This,pUri,pfEqual) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUri_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0015 */
/* [local] */ 

STDAPI CreateUri(_In_ LPCWSTR pwzURI,
                 _In_ DWORD dwFlags,
                 _Reserved_ DWORD_PTR dwReserved,   // must be 0
                 _Outptr_ IUri** ppURI);

STDAPI CreateUriWithFragment(
                 _In_ LPCWSTR pwzURI,
                 _In_opt_ LPCWSTR pwzFragment,        // can be NULL
                 _In_ DWORD dwFlags,
                 _Reserved_ DWORD_PTR dwReserved,   // must be 0
                 _Outptr_ IUri** ppURI);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

STDAPI CreateUriFromMultiByteString(
    _In_       LPCSTR    pszANSIInputUri,
               DWORD     dwEncodingFlags, // ORed set of Uri_ENCODING_ flags
               DWORD     dwCodePage,
               DWORD     dwCreateFlags,
    _Reserved_ DWORD_PTR dwReserved,   // must be 0
    _Outptr_   IUri**    ppUri);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

//The following flags are used with IUri::GetProperties.
#define Uri_HAS_ABSOLUTE_URI    (1 << Uri_PROPERTY_ABSOLUTE_URI)
#define Uri_HAS_AUTHORITY       (1 << Uri_PROPERTY_AUTHORITY)
#define Uri_HAS_DISPLAY_URI     (1 << Uri_PROPERTY_DISPLAY_URI)
#define Uri_HAS_DOMAIN          (1 << Uri_PROPERTY_DOMAIN)
#define Uri_HAS_EXTENSION       (1 << Uri_PROPERTY_EXTENSION)
#define Uri_HAS_FRAGMENT        (1 << Uri_PROPERTY_FRAGMENT)
#define Uri_HAS_HOST            (1 << Uri_PROPERTY_HOST)
#define Uri_HAS_PASSWORD        (1 << Uri_PROPERTY_PASSWORD)
#define Uri_HAS_PATH            (1 << Uri_PROPERTY_PATH)
#define Uri_HAS_QUERY           (1 << Uri_PROPERTY_QUERY)
#define Uri_HAS_RAW_URI         (1 << Uri_PROPERTY_RAW_URI)
#define Uri_HAS_SCHEME_NAME     (1 << Uri_PROPERTY_SCHEME_NAME)
#define Uri_HAS_USER_NAME       (1 << Uri_PROPERTY_USER_NAME)
#define Uri_HAS_PATH_AND_QUERY  (1 << Uri_PROPERTY_PATH_AND_QUERY)
#define Uri_HAS_USER_INFO       (1 << Uri_PROPERTY_USER_INFO)
#define Uri_HAS_HOST_TYPE       (1 << Uri_PROPERTY_HOST_TYPE)
#define Uri_HAS_PORT            (1 << Uri_PROPERTY_PORT)
#define Uri_HAS_SCHEME          (1 << Uri_PROPERTY_SCHEME)
#define Uri_HAS_ZONE            (1 << Uri_PROPERTY_ZONE)

//The following public Uri_CREATE flags may be passed in 
//through the dwFlags parameter of the CreateUri functions.
//Note that ALLOW_RELATIVE and ALLOW_IMPLICIT_WILDCARD_SCHEME are mutually exclusive and may not be passed together.
#define Uri_CREATE_ALLOW_RELATIVE                 0x00000001    // When the scheme is unspecified and not implicit file, assume relative.
#define Uri_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME 0x00000002    // When the scheme is unspecified and not implicit file, assume wildcard.
#define Uri_CREATE_ALLOW_IMPLICIT_FILE_SCHEME     0x00000004    // When the scheme is unspecified and it starts with X: or \\ assume its a file scheme.
#define Uri_CREATE_NOFRAG                         0x00000008    // If there's a query string don't look for a fragment
#define Uri_CREATE_NO_CANONICALIZE                0x00000010    // Do not canonicalize the scheme, host, authority, or path
#define Uri_CREATE_CANONICALIZE                   0x00000100    // DEFAULT: Canonicalize the scheme, host, authority, and path
#define Uri_CREATE_FILE_USE_DOS_PATH              0x00000020    // Use DOS path compat mode for file URI creation
#define Uri_CREATE_DECODE_EXTRA_INFO              0x00000040    // Beta2 DEFAULT: Decode the contents of query and fragment, then re-encode reserved characters
#define Uri_CREATE_NO_DECODE_EXTRA_INFO           0x00000080    // Beta1 DEFAULT: Neither decode nor re-encode any part of the query or fragment
#define Uri_CREATE_CRACK_UNKNOWN_SCHEMES          0x00000200    // Beta2 DEFAULT: Heirarchical URIs with present and unknown schemes will be treated like heirarchical URIs
#define Uri_CREATE_NO_CRACK_UNKNOWN_SCHEMES       0x00000400    // Beta1 DEFAULT: Heirarchical URIs with present and unknown schemes will be treated like opaque URIs
#define Uri_CREATE_PRE_PROCESS_HTML_URI           0x00000800    // DEFAULT:  Perform pre-processing on the URI to remove control characters and whitespace as if the URI comes from the raw href value of an HTML page.
#define Uri_CREATE_NO_PRE_PROCESS_HTML_URI        0x00001000    // Don't perform pre-processing to remove control characters and whitespace as appropriate.
#define Uri_CREATE_IE_SETTINGS                    0x00002000    // Use IE registry settings for such things as whether or not to use IDN.
#define Uri_CREATE_NO_IE_SETTINGS                 0x00004000    // DEFAULT: Don't use IE registry settings.
#define Uri_CREATE_NO_ENCODE_FORBIDDEN_CHARACTERS 0x00008000    // Don't percent-encode characters that are forbidden by the RFC.
#define Uri_CREATE_NORMALIZE_INTL_CHARACTERS      0x00010000    // Percent encode all extended unicode charcters, then decode all percent encoded extended unicode characters (except those identified as dangerous)
#define Uri_CREATE_CANONICALIZE_ABSOLUTE          0x00020000    // Generate a fully UTF-8 encoded value for absoluteUri
//The following flags may be passed in through the dwFlags 
//parameter of the IUri::GetPropertyBSTR or IUri::GetPropertyLength functions.
//The following flags apply only to the property Uri_PROPERTY_DISPLAY_URI:
#define Uri_DISPLAY_NO_FRAGMENT               0x00000001    // Exclude fragment from the DisplayUri, if one exists
#define Uri_PUNYCODE_IDN_HOST                 0x00000002    // Show the hostname encoded as punycode inside the Host, Domain, or AbsoluteUri properties, if the Uri is IDN
#define Uri_DISPLAY_IDN_HOST                  0x00000004    // Show the hostname encoded as punycode or Unicode as it would appear in the DisplayUri property. This applies to the Host, Domain, and AbsoluteUri properties.
#define Uri_DISPLAY_NO_PUNYCODE               0x00000008    // Show the hostname encoded as Unicode from the DisplayUri, if the Uri is IDN
// The following are Uri_ENCODING_ flags for use with the function CreateUriFromMultiByteString
#define Uri_ENCODING_USER_INFO_AND_PATH_IS_PERCENT_ENCODED_UTF8  0x00000001
#define Uri_ENCODING_USER_INFO_AND_PATH_IS_CP                    0x00000002
#define Uri_ENCODING_HOST_IS_IDN                                 0x00000004
#define Uri_ENCODING_HOST_IS_PERCENT_ENCODED_UTF8                0x00000008
#define Uri_ENCODING_HOST_IS_PERCENT_ENCODED_CP                  0x00000010
#define Uri_ENCODING_QUERY_AND_FRAGMENT_IS_PERCENT_ENCODED_UTF8  0x00000020
#define Uri_ENCODING_QUERY_AND_FRAGMENT_IS_CP                    0x00000040
#define Uri_ENCODING_RFC (Uri_ENCODING_USER_INFO_AND_PATH_IS_PERCENT_ENCODED_UTF8 | Uri_ENCODING_HOST_IS_PERCENT_ENCODED_UTF8 | Uri_ENCODING_QUERY_AND_FRAGMENT_IS_PERCENT_ENCODED_UTF8)

//The following flags may be passed in through the dwUriBuilderFlags 
//parameter of the IUriBuilder::CreateUri
#define UriBuilder_USE_ORIGINAL_FLAGS             0x00000001    // Use the create flags from the underlying IUri if they are available.

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#endif
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0015_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0015_v0_0_s_ifspec;

#ifndef __IUriContainer_INTERFACE_DEFINED__
#define __IUriContainer_INTERFACE_DEFINED__

/* interface IUriContainer */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IUriContainer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a158a630-ed6f-45fb-b987-f68676f57752")
    IUriContainer : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetIUri( 
            /* [out] */ IUri **ppIUri) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUriContainerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUriContainer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUriContainer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUriContainer * This);
        
        DECLSPEC_XFGVIRT(IUriContainer, GetIUri)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetIUri )( 
            IUriContainer * This,
            /* [out] */ IUri **ppIUri);
        
        END_INTERFACE
    } IUriContainerVtbl;

    interface IUriContainer
    {
        CONST_VTBL struct IUriContainerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUriContainer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUriContainer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUriContainer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUriContainer_GetIUri(This,ppIUri)	\
    ( (This)->lpVtbl -> GetIUri(This,ppIUri) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUriContainer_INTERFACE_DEFINED__ */


#ifndef __IUriBuilder_INTERFACE_DEFINED__
#define __IUriBuilder_INTERFACE_DEFINED__

/* interface IUriBuilder */
/* [unique][uuid][object][local][helpstring] */ 


EXTERN_C const IID IID_IUriBuilder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4221B2E1-8955-46c0-BD5B-DE9897565DE7")
    IUriBuilder : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateUriSimple( 
            /* [in] */ DWORD dwAllowEncodingPropertyMask,
            /* [in] */ DWORD_PTR dwReserved,
            /* [annotation][out] */ 
            _Outptr_  IUri **ppIUri) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateUri( 
            /* [in] */ DWORD dwCreateFlags,
            /* [in] */ DWORD dwAllowEncodingPropertyMask,
            /* [in] */ DWORD_PTR dwReserved,
            /* [annotation][out] */ 
            _Outptr_  IUri **ppIUri) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateUriWithFlags( 
            /* [in] */ DWORD dwCreateFlags,
            /* [in] */ DWORD dwUriBuilderFlags,
            /* [in] */ DWORD dwAllowEncodingPropertyMask,
            /* [in] */ DWORD_PTR dwReserved,
            /* [annotation][out] */ 
            _Outptr_  IUri **ppIUri) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetIUri( 
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IUri **ppIUri) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetIUri( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUri *pIUri) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetFragment( 
            /* [annotation][out] */ 
            _Out_  DWORD *pcchFragment,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPCWSTR *ppwzFragment) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetHost( 
            /* [annotation][out] */ 
            _Out_  DWORD *pcchHost,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPCWSTR *ppwzHost) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPassword( 
            /* [annotation][out] */ 
            _Out_  DWORD *pcchPassword,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPCWSTR *ppwzPassword) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPath( 
            /* [annotation][out] */ 
            _Out_  DWORD *pcchPath,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPCWSTR *ppwzPath) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPort( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfHasPort,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwPort) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetQuery( 
            /* [annotation][out] */ 
            _Out_  DWORD *pcchQuery,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPCWSTR *ppwzQuery) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSchemeName( 
            /* [annotation][out] */ 
            _Out_  DWORD *pcchSchemeName,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPCWSTR *ppwzSchemeName) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetUserName( 
            /* [annotation][out] */ 
            _Out_  DWORD *pcchUserName,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPCWSTR *ppwzUserName) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetFragment( 
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pwzNewValue) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetHost( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pwzNewValue) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetPassword( 
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pwzNewValue) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetPath( 
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pwzNewValue) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetPort( 
            /* [in] */ BOOL fHasPort,
            /* [in] */ DWORD dwNewValue) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetQuery( 
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pwzNewValue) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetSchemeName( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pwzNewValue) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetUserName( 
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pwzNewValue) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemoveProperties( 
            /* [in] */ DWORD dwPropertyMask) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE HasBeenModified( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfModified) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUriBuilderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUriBuilder * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUriBuilder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUriBuilder * This);
        
        DECLSPEC_XFGVIRT(IUriBuilder, CreateUriSimple)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateUriSimple )( 
            IUriBuilder * This,
            /* [in] */ DWORD dwAllowEncodingPropertyMask,
            /* [in] */ DWORD_PTR dwReserved,
            /* [annotation][out] */ 
            _Outptr_  IUri **ppIUri);
        
        DECLSPEC_XFGVIRT(IUriBuilder, CreateUri)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateUri )( 
            IUriBuilder * This,
            /* [in] */ DWORD dwCreateFlags,
            /* [in] */ DWORD dwAllowEncodingPropertyMask,
            /* [in] */ DWORD_PTR dwReserved,
            /* [annotation][out] */ 
            _Outptr_  IUri **ppIUri);
        
        DECLSPEC_XFGVIRT(IUriBuilder, CreateUriWithFlags)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateUriWithFlags )( 
            IUriBuilder * This,
            /* [in] */ DWORD dwCreateFlags,
            /* [in] */ DWORD dwUriBuilderFlags,
            /* [in] */ DWORD dwAllowEncodingPropertyMask,
            /* [in] */ DWORD_PTR dwReserved,
            /* [annotation][out] */ 
            _Outptr_  IUri **ppIUri);
        
        DECLSPEC_XFGVIRT(IUriBuilder, GetIUri)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetIUri )( 
            IUriBuilder * This,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IUri **ppIUri);
        
        DECLSPEC_XFGVIRT(IUriBuilder, SetIUri)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetIUri )( 
            IUriBuilder * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUri *pIUri);
        
        DECLSPEC_XFGVIRT(IUriBuilder, GetFragment)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetFragment )( 
            IUriBuilder * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcchFragment,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPCWSTR *ppwzFragment);
        
        DECLSPEC_XFGVIRT(IUriBuilder, GetHost)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetHost )( 
            IUriBuilder * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcchHost,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPCWSTR *ppwzHost);
        
        DECLSPEC_XFGVIRT(IUriBuilder, GetPassword)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPassword )( 
            IUriBuilder * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcchPassword,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPCWSTR *ppwzPassword);
        
        DECLSPEC_XFGVIRT(IUriBuilder, GetPath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPath )( 
            IUriBuilder * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcchPath,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPCWSTR *ppwzPath);
        
        DECLSPEC_XFGVIRT(IUriBuilder, GetPort)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPort )( 
            IUriBuilder * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfHasPort,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwPort);
        
        DECLSPEC_XFGVIRT(IUriBuilder, GetQuery)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetQuery )( 
            IUriBuilder * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcchQuery,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPCWSTR *ppwzQuery);
        
        DECLSPEC_XFGVIRT(IUriBuilder, GetSchemeName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSchemeName )( 
            IUriBuilder * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcchSchemeName,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPCWSTR *ppwzSchemeName);
        
        DECLSPEC_XFGVIRT(IUriBuilder, GetUserName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetUserName )( 
            IUriBuilder * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcchUserName,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPCWSTR *ppwzUserName);
        
        DECLSPEC_XFGVIRT(IUriBuilder, SetFragment)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetFragment )( 
            IUriBuilder * This,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pwzNewValue);
        
        DECLSPEC_XFGVIRT(IUriBuilder, SetHost)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetHost )( 
            IUriBuilder * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwzNewValue);
        
        DECLSPEC_XFGVIRT(IUriBuilder, SetPassword)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetPassword )( 
            IUriBuilder * This,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pwzNewValue);
        
        DECLSPEC_XFGVIRT(IUriBuilder, SetPath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetPath )( 
            IUriBuilder * This,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pwzNewValue);
        
        DECLSPEC_XFGVIRT(IUriBuilder, SetPort)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetPort )( 
            IUriBuilder * This,
            /* [in] */ BOOL fHasPort,
            /* [in] */ DWORD dwNewValue);
        
        DECLSPEC_XFGVIRT(IUriBuilder, SetQuery)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetQuery )( 
            IUriBuilder * This,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pwzNewValue);
        
        DECLSPEC_XFGVIRT(IUriBuilder, SetSchemeName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetSchemeName )( 
            IUriBuilder * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwzNewValue);
        
        DECLSPEC_XFGVIRT(IUriBuilder, SetUserName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetUserName )( 
            IUriBuilder * This,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pwzNewValue);
        
        DECLSPEC_XFGVIRT(IUriBuilder, RemoveProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveProperties )( 
            IUriBuilder * This,
            /* [in] */ DWORD dwPropertyMask);
        
        DECLSPEC_XFGVIRT(IUriBuilder, HasBeenModified)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *HasBeenModified )( 
            IUriBuilder * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfModified);
        
        END_INTERFACE
    } IUriBuilderVtbl;

    interface IUriBuilder
    {
        CONST_VTBL struct IUriBuilderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUriBuilder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUriBuilder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUriBuilder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUriBuilder_CreateUriSimple(This,dwAllowEncodingPropertyMask,dwReserved,ppIUri)	\
    ( (This)->lpVtbl -> CreateUriSimple(This,dwAllowEncodingPropertyMask,dwReserved,ppIUri) ) 

#define IUriBuilder_CreateUri(This,dwCreateFlags,dwAllowEncodingPropertyMask,dwReserved,ppIUri)	\
    ( (This)->lpVtbl -> CreateUri(This,dwCreateFlags,dwAllowEncodingPropertyMask,dwReserved,ppIUri) ) 

#define IUriBuilder_CreateUriWithFlags(This,dwCreateFlags,dwUriBuilderFlags,dwAllowEncodingPropertyMask,dwReserved,ppIUri)	\
    ( (This)->lpVtbl -> CreateUriWithFlags(This,dwCreateFlags,dwUriBuilderFlags,dwAllowEncodingPropertyMask,dwReserved,ppIUri) ) 

#define IUriBuilder_GetIUri(This,ppIUri)	\
    ( (This)->lpVtbl -> GetIUri(This,ppIUri) ) 

#define IUriBuilder_SetIUri(This,pIUri)	\
    ( (This)->lpVtbl -> SetIUri(This,pIUri) ) 

#define IUriBuilder_GetFragment(This,pcchFragment,ppwzFragment)	\
    ( (This)->lpVtbl -> GetFragment(This,pcchFragment,ppwzFragment) ) 

#define IUriBuilder_GetHost(This,pcchHost,ppwzHost)	\
    ( (This)->lpVtbl -> GetHost(This,pcchHost,ppwzHost) ) 

#define IUriBuilder_GetPassword(This,pcchPassword,ppwzPassword)	\
    ( (This)->lpVtbl -> GetPassword(This,pcchPassword,ppwzPassword) ) 

#define IUriBuilder_GetPath(This,pcchPath,ppwzPath)	\
    ( (This)->lpVtbl -> GetPath(This,pcchPath,ppwzPath) ) 

#define IUriBuilder_GetPort(This,pfHasPort,pdwPort)	\
    ( (This)->lpVtbl -> GetPort(This,pfHasPort,pdwPort) ) 

#define IUriBuilder_GetQuery(This,pcchQuery,ppwzQuery)	\
    ( (This)->lpVtbl -> GetQuery(This,pcchQuery,ppwzQuery) ) 

#define IUriBuilder_GetSchemeName(This,pcchSchemeName,ppwzSchemeName)	\
    ( (This)->lpVtbl -> GetSchemeName(This,pcchSchemeName,ppwzSchemeName) ) 

#define IUriBuilder_GetUserName(This,pcchUserName,ppwzUserName)	\
    ( (This)->lpVtbl -> GetUserName(This,pcchUserName,ppwzUserName) ) 

#define IUriBuilder_SetFragment(This,pwzNewValue)	\
    ( (This)->lpVtbl -> SetFragment(This,pwzNewValue) ) 

#define IUriBuilder_SetHost(This,pwzNewValue)	\
    ( (This)->lpVtbl -> SetHost(This,pwzNewValue) ) 

#define IUriBuilder_SetPassword(This,pwzNewValue)	\
    ( (This)->lpVtbl -> SetPassword(This,pwzNewValue) ) 

#define IUriBuilder_SetPath(This,pwzNewValue)	\
    ( (This)->lpVtbl -> SetPath(This,pwzNewValue) ) 

#define IUriBuilder_SetPort(This,fHasPort,dwNewValue)	\
    ( (This)->lpVtbl -> SetPort(This,fHasPort,dwNewValue) ) 

#define IUriBuilder_SetQuery(This,pwzNewValue)	\
    ( (This)->lpVtbl -> SetQuery(This,pwzNewValue) ) 

#define IUriBuilder_SetSchemeName(This,pwzNewValue)	\
    ( (This)->lpVtbl -> SetSchemeName(This,pwzNewValue) ) 

#define IUriBuilder_SetUserName(This,pwzNewValue)	\
    ( (This)->lpVtbl -> SetUserName(This,pwzNewValue) ) 

#define IUriBuilder_RemoveProperties(This,dwPropertyMask)	\
    ( (This)->lpVtbl -> RemoveProperties(This,dwPropertyMask) ) 

#define IUriBuilder_HasBeenModified(This,pfModified)	\
    ( (This)->lpVtbl -> HasBeenModified(This,pfModified) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUriBuilder_INTERFACE_DEFINED__ */


#ifndef __IUriBuilderFactory_INTERFACE_DEFINED__
#define __IUriBuilderFactory_INTERFACE_DEFINED__

/* interface IUriBuilderFactory */
/* [unique][uuid][object][local][helpstring] */ 


EXTERN_C const IID IID_IUriBuilderFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E982CE48-0B96-440c-BC37-0C869B27A29E")
    IUriBuilderFactory : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateIUriBuilder( 
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  DWORD_PTR dwReserved,
            /* [annotation][out] */ 
            _Out_  IUriBuilder **ppIUriBuilder) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateInitializedIUriBuilder( 
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  DWORD_PTR dwReserved,
            /* [annotation][out] */ 
            _Out_  IUriBuilder **ppIUriBuilder) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUriBuilderFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUriBuilderFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUriBuilderFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUriBuilderFactory * This);
        
        DECLSPEC_XFGVIRT(IUriBuilderFactory, CreateIUriBuilder)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateIUriBuilder )( 
            IUriBuilderFactory * This,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  DWORD_PTR dwReserved,
            /* [annotation][out] */ 
            _Out_  IUriBuilder **ppIUriBuilder);
        
        DECLSPEC_XFGVIRT(IUriBuilderFactory, CreateInitializedIUriBuilder)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateInitializedIUriBuilder )( 
            IUriBuilderFactory * This,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  DWORD_PTR dwReserved,
            /* [annotation][out] */ 
            _Out_  IUriBuilder **ppIUriBuilder);
        
        END_INTERFACE
    } IUriBuilderFactoryVtbl;

    interface IUriBuilderFactory
    {
        CONST_VTBL struct IUriBuilderFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUriBuilderFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUriBuilderFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUriBuilderFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUriBuilderFactory_CreateIUriBuilder(This,dwFlags,dwReserved,ppIUriBuilder)	\
    ( (This)->lpVtbl -> CreateIUriBuilder(This,dwFlags,dwReserved,ppIUriBuilder) ) 

#define IUriBuilderFactory_CreateInitializedIUriBuilder(This,dwFlags,dwReserved,ppIUriBuilder)	\
    ( (This)->lpVtbl -> CreateInitializedIUriBuilder(This,dwFlags,dwReserved,ppIUriBuilder) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUriBuilderFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0018 */
/* [local] */ 

STDAPI CreateIUriBuilder(
    _In_opt_     IUri         *pIUri,
    _In_         DWORD         dwFlags,
    _In_         DWORD_PTR     dwReserved,
    _Outptr_     IUriBuilder **ppIUriBuilder
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif //(_WIN32_IE >= _WIN32_IE_IE70)
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#ifndef _LPWININETINFO_DEFINED
#define _LPWININETINFO_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0018_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0018_v0_0_s_ifspec;

#ifndef __IWinInetInfo_INTERFACE_DEFINED__
#define __IWinInetInfo_INTERFACE_DEFINED__

/* interface IWinInetInfo */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IWinInetInfo *LPWININETINFO;


EXTERN_C const IID IID_IWinInetInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9d6-bafa-11ce-8c82-00aa004ba90b")
    IWinInetInfo : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE QueryOption( 
            /* [in] */ DWORD dwOption,
            /* [size_is][out][in] */ LPVOID pBuffer,
            /* [out][in] */ DWORD *pcbBuf) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWinInetInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWinInetInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWinInetInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWinInetInfo * This);
        
        DECLSPEC_XFGVIRT(IWinInetInfo, QueryOption)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *QueryOption )( 
            IWinInetInfo * This,
            /* [in] */ DWORD dwOption,
            /* [size_is][out][in] */ LPVOID pBuffer,
            /* [out][in] */ DWORD *pcbBuf);
        
        END_INTERFACE
    } IWinInetInfoVtbl;

    interface IWinInetInfo
    {
        CONST_VTBL struct IWinInetInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWinInetInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWinInetInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWinInetInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWinInetInfo_QueryOption(This,dwOption,pBuffer,pcbBuf)	\
    ( (This)->lpVtbl -> QueryOption(This,dwOption,pBuffer,pcbBuf) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IWinInetInfo_RemoteQueryOption_Proxy( 
    __RPC__in IWinInetInfo * This,
    /* [in] */ DWORD dwOption,
    /* [size_is][out][in] */ __RPC__inout_ecount_full(*pcbBuf) BYTE *pBuffer,
    /* [out][in] */ __RPC__inout DWORD *pcbBuf);


void __RPC_STUB IWinInetInfo_RemoteQueryOption_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IWinInetInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0019 */
/* [local] */ 

#endif
#define WININETINFO_OPTION_LOCK_HANDLE 65534
#ifndef _LPHTTPSECURITY_DEFINED
#define _LPHTTPSECURITY_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0019_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0019_v0_0_s_ifspec;

#ifndef __IHttpSecurity_INTERFACE_DEFINED__
#define __IHttpSecurity_INTERFACE_DEFINED__

/* interface IHttpSecurity */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IHttpSecurity *LPHTTPSECURITY;


EXTERN_C const IID IID_IHttpSecurity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9d7-bafa-11ce-8c82-00aa004ba90b")
    IHttpSecurity : public IWindowForBindingUI
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnSecurityProblem( 
            /* [in] */ DWORD dwProblem) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHttpSecurityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IHttpSecurity * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IHttpSecurity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IHttpSecurity * This);
        
        DECLSPEC_XFGVIRT(IWindowForBindingUI, GetWindow)
        HRESULT ( STDMETHODCALLTYPE *GetWindow )( 
            IHttpSecurity * This,
            /* [in] */ REFGUID rguidReason,
            /* [out] */ HWND *phwnd);
        
        DECLSPEC_XFGVIRT(IHttpSecurity, OnSecurityProblem)
        HRESULT ( STDMETHODCALLTYPE *OnSecurityProblem )( 
            IHttpSecurity * This,
            /* [in] */ DWORD dwProblem);
        
        END_INTERFACE
    } IHttpSecurityVtbl;

    interface IHttpSecurity
    {
        CONST_VTBL struct IHttpSecurityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHttpSecurity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHttpSecurity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHttpSecurity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHttpSecurity_GetWindow(This,rguidReason,phwnd)	\
    ( (This)->lpVtbl -> GetWindow(This,rguidReason,phwnd) ) 


#define IHttpSecurity_OnSecurityProblem(This,dwProblem)	\
    ( (This)->lpVtbl -> OnSecurityProblem(This,dwProblem) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHttpSecurity_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0020 */
/* [local] */ 

#endif
#ifndef _LPWININETHTTPINFO_DEFINED
#define _LPWININETHTTPINFO_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0020_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0020_v0_0_s_ifspec;

#ifndef __IWinInetHttpInfo_INTERFACE_DEFINED__
#define __IWinInetHttpInfo_INTERFACE_DEFINED__

/* interface IWinInetHttpInfo */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IWinInetHttpInfo *LPWININETHTTPINFO;


EXTERN_C const IID IID_IWinInetHttpInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9d8-bafa-11ce-8c82-00aa004ba90b")
    IWinInetHttpInfo : public IWinInetInfo
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE QueryInfo( 
            /* [in] */ DWORD dwOption,
            /* [size_is][out][in] */ LPVOID pBuffer,
            /* [out][in] */ DWORD *pcbBuf,
            /* [out][in] */ DWORD *pdwFlags,
            /* [out][in] */ DWORD *pdwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWinInetHttpInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWinInetHttpInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWinInetHttpInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWinInetHttpInfo * This);
        
        DECLSPEC_XFGVIRT(IWinInetInfo, QueryOption)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *QueryOption )( 
            IWinInetHttpInfo * This,
            /* [in] */ DWORD dwOption,
            /* [size_is][out][in] */ LPVOID pBuffer,
            /* [out][in] */ DWORD *pcbBuf);
        
        DECLSPEC_XFGVIRT(IWinInetHttpInfo, QueryInfo)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *QueryInfo )( 
            IWinInetHttpInfo * This,
            /* [in] */ DWORD dwOption,
            /* [size_is][out][in] */ LPVOID pBuffer,
            /* [out][in] */ DWORD *pcbBuf,
            /* [out][in] */ DWORD *pdwFlags,
            /* [out][in] */ DWORD *pdwReserved);
        
        END_INTERFACE
    } IWinInetHttpInfoVtbl;

    interface IWinInetHttpInfo
    {
        CONST_VTBL struct IWinInetHttpInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWinInetHttpInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWinInetHttpInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWinInetHttpInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWinInetHttpInfo_QueryOption(This,dwOption,pBuffer,pcbBuf)	\
    ( (This)->lpVtbl -> QueryOption(This,dwOption,pBuffer,pcbBuf) ) 


#define IWinInetHttpInfo_QueryInfo(This,dwOption,pBuffer,pcbBuf,pdwFlags,pdwReserved)	\
    ( (This)->lpVtbl -> QueryInfo(This,dwOption,pBuffer,pcbBuf,pdwFlags,pdwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IWinInetHttpInfo_RemoteQueryInfo_Proxy( 
    __RPC__in IWinInetHttpInfo * This,
    /* [in] */ DWORD dwOption,
    /* [size_is][out][in] */ __RPC__inout_ecount_full(*pcbBuf) BYTE *pBuffer,
    /* [out][in] */ __RPC__inout DWORD *pcbBuf,
    /* [out][in] */ __RPC__inout DWORD *pdwFlags,
    /* [out][in] */ __RPC__inout DWORD *pdwReserved);


void __RPC_STUB IWinInetHttpInfo_RemoteQueryInfo_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IWinInetHttpInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0021 */
/* [local] */ 

#endif
#ifndef _LPWININETHTTPTIMEOUTS_DEFINED
#define _LPWININETHTTPTIMEOUTS_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0021_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0021_v0_0_s_ifspec;

#ifndef __IWinInetHttpTimeouts_INTERFACE_DEFINED__
#define __IWinInetHttpTimeouts_INTERFACE_DEFINED__

/* interface IWinInetHttpTimeouts */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IWinInetHttpTimeouts;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F286FA56-C1FD-4270-8E67-B3EB790A81E8")
    IWinInetHttpTimeouts : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetRequestTimeouts( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwConnectTimeout,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwSendTimeout,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwReceiveTimeout) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWinInetHttpTimeoutsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWinInetHttpTimeouts * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWinInetHttpTimeouts * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWinInetHttpTimeouts * This);
        
        DECLSPEC_XFGVIRT(IWinInetHttpTimeouts, GetRequestTimeouts)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetRequestTimeouts )( 
            IWinInetHttpTimeouts * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwConnectTimeout,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwSendTimeout,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwReceiveTimeout);
        
        END_INTERFACE
    } IWinInetHttpTimeoutsVtbl;

    interface IWinInetHttpTimeouts
    {
        CONST_VTBL struct IWinInetHttpTimeoutsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWinInetHttpTimeouts_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWinInetHttpTimeouts_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWinInetHttpTimeouts_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWinInetHttpTimeouts_GetRequestTimeouts(This,pdwConnectTimeout,pdwSendTimeout,pdwReceiveTimeout)	\
    ( (This)->lpVtbl -> GetRequestTimeouts(This,pdwConnectTimeout,pdwSendTimeout,pdwReceiveTimeout) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWinInetHttpTimeouts_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0022 */
/* [local] */ 

#endif
#if (_WIN32_IE >= _WIN32_IE_IE60SP2)
#ifndef _LPWININETCACHEHINTS_DEFINED
#define _LPWININETCACHEHINTS_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0022_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0022_v0_0_s_ifspec;

#ifndef __IWinInetCacheHints_INTERFACE_DEFINED__
#define __IWinInetCacheHints_INTERFACE_DEFINED__

/* interface IWinInetCacheHints */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IWinInetCacheHints *LPWININETCACHEHINTS;


EXTERN_C const IID IID_IWinInetCacheHints;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DD1EC3B3-8391-4fdb-A9E6-347C3CAAA7DD")
    IWinInetCacheHints : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetCacheExtension( 
            /* [in] */ LPCWSTR pwzExt,
            /* [size_is][out][in] */ LPVOID pszCacheFile,
            /* [out][in] */ DWORD *pcbCacheFile,
            /* [out][in] */ DWORD *pdwWinInetError,
            /* [out][in] */ DWORD *pdwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWinInetCacheHintsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWinInetCacheHints * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWinInetCacheHints * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWinInetCacheHints * This);
        
        DECLSPEC_XFGVIRT(IWinInetCacheHints, SetCacheExtension)
        HRESULT ( STDMETHODCALLTYPE *SetCacheExtension )( 
            IWinInetCacheHints * This,
            /* [in] */ LPCWSTR pwzExt,
            /* [size_is][out][in] */ LPVOID pszCacheFile,
            /* [out][in] */ DWORD *pcbCacheFile,
            /* [out][in] */ DWORD *pdwWinInetError,
            /* [out][in] */ DWORD *pdwReserved);
        
        END_INTERFACE
    } IWinInetCacheHintsVtbl;

    interface IWinInetCacheHints
    {
        CONST_VTBL struct IWinInetCacheHintsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWinInetCacheHints_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWinInetCacheHints_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWinInetCacheHints_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWinInetCacheHints_SetCacheExtension(This,pwzExt,pszCacheFile,pcbCacheFile,pdwWinInetError,pdwReserved)	\
    ( (This)->lpVtbl -> SetCacheExtension(This,pwzExt,pszCacheFile,pcbCacheFile,pdwWinInetError,pdwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWinInetCacheHints_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0023 */
/* [local] */ 

#endif
#endif //(_WIN32_IE >= _WIN32_IE_IE60SP2)
#if (_WIN32_IE >= _WIN32_IE_IE70)
#ifndef _LPWININETCACHEHINTS2_DEFINED
#define _LPWININETCACHEHINTS2_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0023_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0023_v0_0_s_ifspec;

#ifndef __IWinInetCacheHints2_INTERFACE_DEFINED__
#define __IWinInetCacheHints2_INTERFACE_DEFINED__

/* interface IWinInetCacheHints2 */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IWinInetCacheHints2 *LPWININETCACHEHINTS2;


EXTERN_C const IID IID_IWinInetCacheHints2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7857AEAC-D31F-49bf-884E-DD46DF36780A")
    IWinInetCacheHints2 : public IWinInetCacheHints
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetCacheExtension2( 
            /* [in] */ LPCWSTR pwzExt,
            /* [annotation][size_is][out] */ 
            __RPC__out_ecount(*pcchCacheFile)  WCHAR *pwzCacheFile,
            /* [out][in] */ DWORD *pcchCacheFile,
            /* [out] */ DWORD *pdwWinInetError,
            /* [out] */ DWORD *pdwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWinInetCacheHints2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWinInetCacheHints2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWinInetCacheHints2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWinInetCacheHints2 * This);
        
        DECLSPEC_XFGVIRT(IWinInetCacheHints, SetCacheExtension)
        HRESULT ( STDMETHODCALLTYPE *SetCacheExtension )( 
            IWinInetCacheHints2 * This,
            /* [in] */ LPCWSTR pwzExt,
            /* [size_is][out][in] */ LPVOID pszCacheFile,
            /* [out][in] */ DWORD *pcbCacheFile,
            /* [out][in] */ DWORD *pdwWinInetError,
            /* [out][in] */ DWORD *pdwReserved);
        
        DECLSPEC_XFGVIRT(IWinInetCacheHints2, SetCacheExtension2)
        HRESULT ( STDMETHODCALLTYPE *SetCacheExtension2 )( 
            IWinInetCacheHints2 * This,
            /* [in] */ LPCWSTR pwzExt,
            /* [annotation][size_is][out] */ 
            __RPC__out_ecount(*pcchCacheFile)  WCHAR *pwzCacheFile,
            /* [out][in] */ DWORD *pcchCacheFile,
            /* [out] */ DWORD *pdwWinInetError,
            /* [out] */ DWORD *pdwReserved);
        
        END_INTERFACE
    } IWinInetCacheHints2Vtbl;

    interface IWinInetCacheHints2
    {
        CONST_VTBL struct IWinInetCacheHints2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWinInetCacheHints2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWinInetCacheHints2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWinInetCacheHints2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWinInetCacheHints2_SetCacheExtension(This,pwzExt,pszCacheFile,pcbCacheFile,pdwWinInetError,pdwReserved)	\
    ( (This)->lpVtbl -> SetCacheExtension(This,pwzExt,pszCacheFile,pcbCacheFile,pdwWinInetError,pdwReserved) ) 


#define IWinInetCacheHints2_SetCacheExtension2(This,pwzExt,pwzCacheFile,pcchCacheFile,pdwWinInetError,pdwReserved)	\
    ( (This)->lpVtbl -> SetCacheExtension2(This,pwzExt,pwzCacheFile,pcchCacheFile,pdwWinInetError,pdwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWinInetCacheHints2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0024 */
/* [local] */ 

#endif
#endif //(_WIN32_IE >= _WIN32_IE_IE70)
#define SID_IBindHost IID_IBindHost
#define SID_SBindHost IID_IBindHost
#ifndef _LPBINDHOST_DEFINED
#define _LPBINDHOST_DEFINED
EXTERN_C const GUID SID_BindHost;


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0024_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0024_v0_0_s_ifspec;

#ifndef __IBindHost_INTERFACE_DEFINED__
#define __IBindHost_INTERFACE_DEFINED__

/* interface IBindHost */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IBindHost *LPBINDHOST;


EXTERN_C const IID IID_IBindHost;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fc4801a1-2ba9-11cf-a229-00aa003d7352")
    IBindHost : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateMoniker( 
            /* [in] */ __RPC__in LPOLESTR szName,
            /* [in] */ __RPC__in_opt IBindCtx *pBC,
            /* [out] */ __RPC__deref_out_opt IMoniker **ppmk,
            /* [in] */ DWORD dwReserved) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE MonikerBindToStorage( 
            /* [in] */ IMoniker *pMk,
            /* [in] */ IBindCtx *pBC,
            /* [in] */ IBindStatusCallback *pBSC,
            /* [in] */ REFIID riid,
            /* [out] */ void **ppvObj) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE MonikerBindToObject( 
            /* [in] */ IMoniker *pMk,
            /* [in] */ IBindCtx *pBC,
            /* [in] */ IBindStatusCallback *pBSC,
            /* [in] */ REFIID riid,
            /* [out] */ void **ppvObj) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBindHostVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBindHost * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBindHost * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBindHost * This);
        
        DECLSPEC_XFGVIRT(IBindHost, CreateMoniker)
        HRESULT ( STDMETHODCALLTYPE *CreateMoniker )( 
            __RPC__in IBindHost * This,
            /* [in] */ __RPC__in LPOLESTR szName,
            /* [in] */ __RPC__in_opt IBindCtx *pBC,
            /* [out] */ __RPC__deref_out_opt IMoniker **ppmk,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IBindHost, MonikerBindToStorage)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *MonikerBindToStorage )( 
            IBindHost * This,
            /* [in] */ IMoniker *pMk,
            /* [in] */ IBindCtx *pBC,
            /* [in] */ IBindStatusCallback *pBSC,
            /* [in] */ REFIID riid,
            /* [out] */ void **ppvObj);
        
        DECLSPEC_XFGVIRT(IBindHost, MonikerBindToObject)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *MonikerBindToObject )( 
            IBindHost * This,
            /* [in] */ IMoniker *pMk,
            /* [in] */ IBindCtx *pBC,
            /* [in] */ IBindStatusCallback *pBSC,
            /* [in] */ REFIID riid,
            /* [out] */ void **ppvObj);
        
        END_INTERFACE
    } IBindHostVtbl;

    interface IBindHost
    {
        CONST_VTBL struct IBindHostVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBindHost_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBindHost_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBindHost_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBindHost_CreateMoniker(This,szName,pBC,ppmk,dwReserved)	\
    ( (This)->lpVtbl -> CreateMoniker(This,szName,pBC,ppmk,dwReserved) ) 

#define IBindHost_MonikerBindToStorage(This,pMk,pBC,pBSC,riid,ppvObj)	\
    ( (This)->lpVtbl -> MonikerBindToStorage(This,pMk,pBC,pBSC,riid,ppvObj) ) 

#define IBindHost_MonikerBindToObject(This,pMk,pBC,pBSC,riid,ppvObj)	\
    ( (This)->lpVtbl -> MonikerBindToObject(This,pMk,pBC,pBSC,riid,ppvObj) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IBindHost_RemoteMonikerBindToStorage_Proxy( 
    __RPC__in IBindHost * This,
    /* [unique][in] */ __RPC__in_opt IMoniker *pMk,
    /* [unique][in] */ __RPC__in_opt IBindCtx *pBC,
    /* [unique][in] */ __RPC__in_opt IBindStatusCallback *pBSC,
    /* [in] */ __RPC__in REFIID riid,
    /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvObj);


void __RPC_STUB IBindHost_RemoteMonikerBindToStorage_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IBindHost_RemoteMonikerBindToObject_Proxy( 
    __RPC__in IBindHost * This,
    /* [unique][in] */ __RPC__in_opt IMoniker *pMk,
    /* [unique][in] */ __RPC__in_opt IBindCtx *pBC,
    /* [unique][in] */ __RPC__in_opt IBindStatusCallback *pBSC,
    /* [in] */ __RPC__in REFIID riid,
    /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvObj);


void __RPC_STUB IBindHost_RemoteMonikerBindToObject_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IBindHost_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0025 */
/* [local] */ 

#endif
                                                                                                           
// These are for backwards compatibility with previous URLMON versions
// Flags for the UrlDownloadToCacheFile                                                                    
#define URLOSTRM_USECACHEDCOPY_ONLY             0x1      // Only get from cache                            
#define URLOSTRM_USECACHEDCOPY                  0x2      // Get from cache if available else download      
#define URLOSTRM_GETNEWESTVERSION               0x3      // Get new version only. But put it in cache too  
                                                                                                           
                                                                                                           
struct IBindStatusCallback;                                                                                
STDAPI HlinkSimpleNavigateToString(                                                                        
    _In_opt_ LPCWSTR szTarget,         // required - target document - null if local jump w/in doc       
    _In_opt_ LPCWSTR szLocation,       // optional, for navigation into middle of a doc                  
    _In_opt_ LPCWSTR szTargetFrameName,// optional, for targeting frame-sets                             
    _In_     IUnknown *pUnk,           // required - we'll search this for other necessary interfaces    
    _In_opt_ IBindCtx *pbc,            // optional. caller may register an IBSC in this                  
    _In_opt_ IBindStatusCallback *,                                                                      
    /* [in] */ DWORD grfHLNF,            // flags                                                          
    /* [in] */ DWORD dwReserved          // for future use, must be NULL                                   
);                                                                                                         
                                                                                                           
STDAPI HlinkSimpleNavigateToMoniker(                                                                       
    _In_opt_ IMoniker *pmkTarget,      // required - target document - (may be null                      
    _In_opt_ LPCWSTR szLocation,       // optional, for navigation into middle of a doc                  
    _In_opt_ LPCWSTR szTargetFrameName,// optional, for targeting frame-sets                             
    _In_opt_ IUnknown *pUnk,           // required - we'll search this for other necessary interfaces    
    _In_opt_ IBindCtx *pbc,            // optional. caller may register an IBSC in this                  
    _In_opt_ IBindStatusCallback *,                                                                      
    /* [in] */ DWORD grfHLNF,            // flags                                                          
    /* [in] */ DWORD dwReserved          // for future use, must be NULL                                   
);                                                                                                         
                                                                                                           
STDAPI URLOpenStreamA(_In_opt_ LPUNKNOWN, _In_ LPCSTR,DWORD, _In_opt_ LPBINDSTATUSCALLBACK);                                        
STDAPI URLOpenStreamW(_In_opt_ LPUNKNOWN, _In_ LPCWSTR,DWORD,_In_opt_ LPBINDSTATUSCALLBACK);                                       
STDAPI URLOpenPullStreamA(_In_opt_ LPUNKNOWN, _In_ LPCSTR,DWORD, _In_opt_ LPBINDSTATUSCALLBACK);                                    
STDAPI URLOpenPullStreamW(_In_opt_ LPUNKNOWN,_In_ LPCWSTR,DWORD, _In_opt_ LPBINDSTATUSCALLBACK);                                   
STDAPI URLDownloadToFileA(_In_opt_ LPUNKNOWN, _In_ LPCSTR, _In_opt_ LPCSTR,DWORD, _In_opt_ LPBINDSTATUSCALLBACK);                             
STDAPI URLDownloadToFileW(_In_opt_ LPUNKNOWN, _In_ LPCWSTR,_In_opt_ LPCWSTR,DWORD, _In_opt_ LPBINDSTATUSCALLBACK);                           
STDAPI URLDownloadToCacheFileA(_In_opt_ LPUNKNOWN, _In_ LPCSTR,  _Out_writes_(cchFileName) LPSTR,  DWORD cchFileName, DWORD, _In_opt_ LPBINDSTATUSCALLBACK);
STDAPI URLDownloadToCacheFileW(_In_opt_ LPUNKNOWN, _In_ LPCWSTR, _Out_writes_(cchFileName) LPWSTR, DWORD cchFileName, DWORD, _In_opt_ LPBINDSTATUSCALLBACK);
STDAPI URLOpenBlockingStreamA(_In_opt_ LPUNKNOWN, _In_ LPCSTR, _Outptr_ LPSTREAM*,DWORD, _In_opt_ LPBINDSTATUSCALLBACK);                      
STDAPI URLOpenBlockingStreamW(_In_opt_ LPUNKNOWN, _In_ LPCWSTR, _Outptr_ LPSTREAM*,DWORD, _In_opt_ LPBINDSTATUSCALLBACK);                     
                                                                                                           
#ifdef UNICODE                                                                                             
#define URLOpenStream            URLOpenStreamW                                                            
#define URLOpenPullStream        URLOpenPullStreamW                                                        
#define URLDownloadToFile        URLDownloadToFileW                                                        
#define URLDownloadToCacheFile   URLDownloadToCacheFileW                                                   
#define URLOpenBlockingStream    URLOpenBlockingStreamW                                                    
#else                                                                                                      
#define URLOpenStream            URLOpenStreamA                                                            
#define URLOpenPullStream        URLOpenPullStreamA                                                        
#define URLDownloadToFile        URLDownloadToFileA                                                        
#define URLDownloadToCacheFile   URLDownloadToCacheFileA                                                   
#define URLOpenBlockingStream    URLOpenBlockingStreamA                                                    
#endif // !UNICODE                                                                                         
                                                                                                           
                                                                                                           
STDAPI HlinkGoBack(_In_ IUnknown *pUnk);                                                                   
STDAPI HlinkGoForward(_In_ IUnknown *pUnk);                                                                
STDAPI HlinkNavigateString(_In_opt_ IUnknown *pUnk, _In_opt_ LPCWSTR szTarget);                            
STDAPI HlinkNavigateMoniker(_In_opt_ IUnknown *pUnk, _In_opt_ IMoniker *pmkTarget);                        
                                                                                                           
#ifndef  _URLMON_NO_ASYNC_PLUGABLE_PROTOCOLS_   










#ifndef _LPIINTERNET
#define _LPIINTERNET


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0025_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0025_v0_0_s_ifspec;

#ifndef __IInternet_INTERFACE_DEFINED__
#define __IInternet_INTERFACE_DEFINED__

/* interface IInternet */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IInternet *LPIINTERNET;


EXTERN_C const IID IID_IInternet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9e0-baf9-11ce-8c82-00aa004ba90b")
    IInternet : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternet * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternet * This);
        
        END_INTERFACE
    } IInternetVtbl;

    interface IInternet
    {
        CONST_VTBL struct IInternetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternet_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0026 */
/* [local] */ 

#endif
#ifndef _LPIINTERNETBINDINFO
#define _LPIINTERNETBINDINFO


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0026_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0026_v0_0_s_ifspec;

#ifndef __IInternetBindInfo_INTERFACE_DEFINED__
#define __IInternetBindInfo_INTERFACE_DEFINED__

/* interface IInternetBindInfo */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IInternetBindInfo *LPIINTERNETBINDINFO;

typedef 
enum tagBINDSTRING
    {
        BINDSTRING_HEADERS	= 1,
        BINDSTRING_ACCEPT_MIMES	= ( BINDSTRING_HEADERS + 1 ) ,
        BINDSTRING_EXTRA_URL	= ( BINDSTRING_ACCEPT_MIMES + 1 ) ,
        BINDSTRING_LANGUAGE	= ( BINDSTRING_EXTRA_URL + 1 ) ,
        BINDSTRING_USERNAME	= ( BINDSTRING_LANGUAGE + 1 ) ,
        BINDSTRING_PASSWORD	= ( BINDSTRING_USERNAME + 1 ) ,
        BINDSTRING_UA_PIXELS	= ( BINDSTRING_PASSWORD + 1 ) ,
        BINDSTRING_UA_COLOR	= ( BINDSTRING_UA_PIXELS + 1 ) ,
        BINDSTRING_OS	= ( BINDSTRING_UA_COLOR + 1 ) ,
        BINDSTRING_USER_AGENT	= ( BINDSTRING_OS + 1 ) ,
        BINDSTRING_ACCEPT_ENCODINGS	= ( BINDSTRING_USER_AGENT + 1 ) ,
        BINDSTRING_POST_COOKIE	= ( BINDSTRING_ACCEPT_ENCODINGS + 1 ) ,
        BINDSTRING_POST_DATA_MIME	= ( BINDSTRING_POST_COOKIE + 1 ) ,
        BINDSTRING_URL	= ( BINDSTRING_POST_DATA_MIME + 1 ) ,
        BINDSTRING_IID	= ( BINDSTRING_URL + 1 ) ,
        BINDSTRING_FLAG_BIND_TO_OBJECT	= ( BINDSTRING_IID + 1 ) ,
        BINDSTRING_PTR_BIND_CONTEXT	= ( BINDSTRING_FLAG_BIND_TO_OBJECT + 1 ) ,
        BINDSTRING_XDR_ORIGIN	= ( BINDSTRING_PTR_BIND_CONTEXT + 1 ) ,
        BINDSTRING_DOWNLOADPATH	= ( BINDSTRING_XDR_ORIGIN + 1 ) ,
        BINDSTRING_ROOTDOC_URL	= ( BINDSTRING_DOWNLOADPATH + 1 ) ,
        BINDSTRING_INITIAL_FILENAME	= ( BINDSTRING_ROOTDOC_URL + 1 ) ,
        BINDSTRING_PROXY_USERNAME	= ( BINDSTRING_INITIAL_FILENAME + 1 ) ,
        BINDSTRING_PROXY_PASSWORD	= ( BINDSTRING_PROXY_USERNAME + 1 ) ,
        BINDSTRING_ENTERPRISE_ID	= ( BINDSTRING_PROXY_PASSWORD + 1 ) ,
        BINDSTRING_DOC_URL	= ( BINDSTRING_ENTERPRISE_ID + 1 ) ,
        BINDSTRING_SAMESITE_COOKIE_LEVEL	= ( BINDSTRING_DOC_URL + 1 ) 
    } 	BINDSTRING;


EXTERN_C const IID IID_IInternetBindInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9e1-baf9-11ce-8c82-00aa004ba90b")
    IInternetBindInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetBindInfo( 
            /* [out] */ DWORD *grfBINDF,
            /* [unique][out][in] */ BINDINFO *pbindinfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBindString( 
            /* [in] */ ULONG ulStringType,
            /* [annotation][out][in] */ 
            __RPC__out  LPOLESTR *ppwzStr,
            /* [in] */ ULONG cEl,
            /* [out][in] */ ULONG *pcElFetched) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetBindInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternetBindInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternetBindInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternetBindInfo * This);
        
        DECLSPEC_XFGVIRT(IInternetBindInfo, GetBindInfo)
        HRESULT ( STDMETHODCALLTYPE *GetBindInfo )( 
            IInternetBindInfo * This,
            /* [out] */ DWORD *grfBINDF,
            /* [unique][out][in] */ BINDINFO *pbindinfo);
        
        DECLSPEC_XFGVIRT(IInternetBindInfo, GetBindString)
        HRESULT ( STDMETHODCALLTYPE *GetBindString )( 
            IInternetBindInfo * This,
            /* [in] */ ULONG ulStringType,
            /* [annotation][out][in] */ 
            __RPC__out  LPOLESTR *ppwzStr,
            /* [in] */ ULONG cEl,
            /* [out][in] */ ULONG *pcElFetched);
        
        END_INTERFACE
    } IInternetBindInfoVtbl;

    interface IInternetBindInfo
    {
        CONST_VTBL struct IInternetBindInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetBindInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetBindInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetBindInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetBindInfo_GetBindInfo(This,grfBINDF,pbindinfo)	\
    ( (This)->lpVtbl -> GetBindInfo(This,grfBINDF,pbindinfo) ) 

#define IInternetBindInfo_GetBindString(This,ulStringType,ppwzStr,cEl,pcElFetched)	\
    ( (This)->lpVtbl -> GetBindString(This,ulStringType,ppwzStr,cEl,pcElFetched) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetBindInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0027 */
/* [local] */ 

#endif
#ifndef _LPIINTERNETBINDINFOEX
#define _LPIINTERNETBINDINFOEX


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0027_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0027_v0_0_s_ifspec;

#ifndef __IInternetBindInfoEx_INTERFACE_DEFINED__
#define __IInternetBindInfoEx_INTERFACE_DEFINED__

/* interface IInternetBindInfoEx */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IInternetBindInfoEx *LPIINTERNETBINDINFOEX;


EXTERN_C const IID IID_IInternetBindInfoEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a3e015b7-a82c-4dcd-a150-569aeeed36ab")
    IInternetBindInfoEx : public IInternetBindInfo
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetBindInfoEx( 
            /* [out] */ DWORD *grfBINDF,
            /* [unique][out][in] */ BINDINFO *pbindinfo,
            /* [out] */ DWORD *grfBINDF2,
            /* [out] */ DWORD *pdwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetBindInfoExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternetBindInfoEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternetBindInfoEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternetBindInfoEx * This);
        
        DECLSPEC_XFGVIRT(IInternetBindInfo, GetBindInfo)
        HRESULT ( STDMETHODCALLTYPE *GetBindInfo )( 
            IInternetBindInfoEx * This,
            /* [out] */ DWORD *grfBINDF,
            /* [unique][out][in] */ BINDINFO *pbindinfo);
        
        DECLSPEC_XFGVIRT(IInternetBindInfo, GetBindString)
        HRESULT ( STDMETHODCALLTYPE *GetBindString )( 
            IInternetBindInfoEx * This,
            /* [in] */ ULONG ulStringType,
            /* [annotation][out][in] */ 
            __RPC__out  LPOLESTR *ppwzStr,
            /* [in] */ ULONG cEl,
            /* [out][in] */ ULONG *pcElFetched);
        
        DECLSPEC_XFGVIRT(IInternetBindInfoEx, GetBindInfoEx)
        HRESULT ( STDMETHODCALLTYPE *GetBindInfoEx )( 
            IInternetBindInfoEx * This,
            /* [out] */ DWORD *grfBINDF,
            /* [unique][out][in] */ BINDINFO *pbindinfo,
            /* [out] */ DWORD *grfBINDF2,
            /* [out] */ DWORD *pdwReserved);
        
        END_INTERFACE
    } IInternetBindInfoExVtbl;

    interface IInternetBindInfoEx
    {
        CONST_VTBL struct IInternetBindInfoExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetBindInfoEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetBindInfoEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetBindInfoEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetBindInfoEx_GetBindInfo(This,grfBINDF,pbindinfo)	\
    ( (This)->lpVtbl -> GetBindInfo(This,grfBINDF,pbindinfo) ) 

#define IInternetBindInfoEx_GetBindString(This,ulStringType,ppwzStr,cEl,pcElFetched)	\
    ( (This)->lpVtbl -> GetBindString(This,ulStringType,ppwzStr,cEl,pcElFetched) ) 


#define IInternetBindInfoEx_GetBindInfoEx(This,grfBINDF,pbindinfo,grfBINDF2,pdwReserved)	\
    ( (This)->lpVtbl -> GetBindInfoEx(This,grfBINDF,pbindinfo,grfBINDF2,pdwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetBindInfoEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0028 */
/* [local] */ 

#endif
#ifndef _LPIINTERNETPROTOCOLROOT_DEFINED
#define _LPIINTERNETPROTOCOLROOT_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0028_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0028_v0_0_s_ifspec;

#ifndef __IInternetProtocolRoot_INTERFACE_DEFINED__
#define __IInternetProtocolRoot_INTERFACE_DEFINED__

/* interface IInternetProtocolRoot */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IInternetProtocolRoot *LPIINTERNETPROTOCOLROOT;

typedef 
enum _tagPI_FLAGS
    {
        PI_PARSE_URL	= 0x1,
        PI_FILTER_MODE	= 0x2,
        PI_FORCE_ASYNC	= 0x4,
        PI_USE_WORKERTHREAD	= 0x8,
        PI_MIMEVERIFICATION	= 0x10,
        PI_CLSIDLOOKUP	= 0x20,
        PI_DATAPROGRESS	= 0x40,
        PI_SYNCHRONOUS	= 0x80,
        PI_APARTMENTTHREADED	= 0x100,
        PI_CLASSINSTALL	= 0x200,
        PI_PASSONBINDCTX	= 0x2000,
        PI_NOMIMEHANDLER	= 0x8000,
        PI_LOADAPPDIRECT	= 0x4000,
        PD_FORCE_SWITCH	= 0x10000,
        PI_PREFERDEFAULTHANDLER	= 0x20000
    } 	PI_FLAGS;

typedef struct _tagPROTOCOLDATA
    {
    DWORD grfFlags;
    DWORD dwState;
    LPVOID pData;
    ULONG cbData;
    } 	PROTOCOLDATA;

typedef struct _tagStartParam
    {
    IID iid;
    IBindCtx *pIBindCtx;
    IUnknown *pItf;
    } 	StartParam;


EXTERN_C const IID IID_IInternetProtocolRoot;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9e3-baf9-11ce-8c82-00aa004ba90b")
    IInternetProtocolRoot : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Start( 
            /* [in] */ LPCWSTR szUrl,
            /* [in] */ IInternetProtocolSink *pOIProtSink,
            /* [in] */ IInternetBindInfo *pOIBindInfo,
            /* [in] */ DWORD grfPI,
            /* [in] */ HANDLE_PTR dwReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Continue( 
            /* [in] */ PROTOCOLDATA *pProtocolData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Abort( 
            /* [in] */ HRESULT hrReason,
            /* [in] */ DWORD dwOptions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Terminate( 
            /* [in] */ DWORD dwOptions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Suspend( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Resume( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetProtocolRootVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternetProtocolRoot * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternetProtocolRoot * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternetProtocolRoot * This);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            IInternetProtocolRoot * This,
            /* [in] */ LPCWSTR szUrl,
            /* [in] */ IInternetProtocolSink *pOIProtSink,
            /* [in] */ IInternetBindInfo *pOIBindInfo,
            /* [in] */ DWORD grfPI,
            /* [in] */ HANDLE_PTR dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Continue)
        HRESULT ( STDMETHODCALLTYPE *Continue )( 
            IInternetProtocolRoot * This,
            /* [in] */ PROTOCOLDATA *pProtocolData);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Abort)
        HRESULT ( STDMETHODCALLTYPE *Abort )( 
            IInternetProtocolRoot * This,
            /* [in] */ HRESULT hrReason,
            /* [in] */ DWORD dwOptions);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Terminate)
        HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            IInternetProtocolRoot * This,
            /* [in] */ DWORD dwOptions);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Suspend)
        HRESULT ( STDMETHODCALLTYPE *Suspend )( 
            IInternetProtocolRoot * This);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Resume)
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            IInternetProtocolRoot * This);
        
        END_INTERFACE
    } IInternetProtocolRootVtbl;

    interface IInternetProtocolRoot
    {
        CONST_VTBL struct IInternetProtocolRootVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetProtocolRoot_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetProtocolRoot_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetProtocolRoot_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetProtocolRoot_Start(This,szUrl,pOIProtSink,pOIBindInfo,grfPI,dwReserved)	\
    ( (This)->lpVtbl -> Start(This,szUrl,pOIProtSink,pOIBindInfo,grfPI,dwReserved) ) 

#define IInternetProtocolRoot_Continue(This,pProtocolData)	\
    ( (This)->lpVtbl -> Continue(This,pProtocolData) ) 

#define IInternetProtocolRoot_Abort(This,hrReason,dwOptions)	\
    ( (This)->lpVtbl -> Abort(This,hrReason,dwOptions) ) 

#define IInternetProtocolRoot_Terminate(This,dwOptions)	\
    ( (This)->lpVtbl -> Terminate(This,dwOptions) ) 

#define IInternetProtocolRoot_Suspend(This)	\
    ( (This)->lpVtbl -> Suspend(This) ) 

#define IInternetProtocolRoot_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetProtocolRoot_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0029 */
/* [local] */ 

#endif
#ifndef _LPIINTERNETPROTOCOL_DEFINED
#define _LPIINTERNETPROTOCOL_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0029_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0029_v0_0_s_ifspec;

#ifndef __IInternetProtocol_INTERFACE_DEFINED__
#define __IInternetProtocol_INTERFACE_DEFINED__

/* interface IInternetProtocol */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IInternetProtocol *LPIINTERNETPROTOCOL;


EXTERN_C const IID IID_IInternetProtocol;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9e4-baf9-11ce-8c82-00aa004ba90b")
    IInternetProtocol : public IInternetProtocolRoot
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Read( 
            /* [length_is][size_is][out][in] */ void *pv,
            /* [in] */ ULONG cb,
            /* [out] */ ULONG *pcbRead) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Seek( 
            /* [in] */ LARGE_INTEGER dlibMove,
            /* [in] */ DWORD dwOrigin,
            /* [out] */ ULARGE_INTEGER *plibNewPosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LockRequest( 
            /* [in] */ DWORD dwOptions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockRequest( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetProtocolVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternetProtocol * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternetProtocol * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternetProtocol * This);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            IInternetProtocol * This,
            /* [in] */ LPCWSTR szUrl,
            /* [in] */ IInternetProtocolSink *pOIProtSink,
            /* [in] */ IInternetBindInfo *pOIBindInfo,
            /* [in] */ DWORD grfPI,
            /* [in] */ HANDLE_PTR dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Continue)
        HRESULT ( STDMETHODCALLTYPE *Continue )( 
            IInternetProtocol * This,
            /* [in] */ PROTOCOLDATA *pProtocolData);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Abort)
        HRESULT ( STDMETHODCALLTYPE *Abort )( 
            IInternetProtocol * This,
            /* [in] */ HRESULT hrReason,
            /* [in] */ DWORD dwOptions);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Terminate)
        HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            IInternetProtocol * This,
            /* [in] */ DWORD dwOptions);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Suspend)
        HRESULT ( STDMETHODCALLTYPE *Suspend )( 
            IInternetProtocol * This);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Resume)
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            IInternetProtocol * This);
        
        DECLSPEC_XFGVIRT(IInternetProtocol, Read)
        HRESULT ( STDMETHODCALLTYPE *Read )( 
            IInternetProtocol * This,
            /* [length_is][size_is][out][in] */ void *pv,
            /* [in] */ ULONG cb,
            /* [out] */ ULONG *pcbRead);
        
        DECLSPEC_XFGVIRT(IInternetProtocol, Seek)
        HRESULT ( STDMETHODCALLTYPE *Seek )( 
            IInternetProtocol * This,
            /* [in] */ LARGE_INTEGER dlibMove,
            /* [in] */ DWORD dwOrigin,
            /* [out] */ ULARGE_INTEGER *plibNewPosition);
        
        DECLSPEC_XFGVIRT(IInternetProtocol, LockRequest)
        HRESULT ( STDMETHODCALLTYPE *LockRequest )( 
            IInternetProtocol * This,
            /* [in] */ DWORD dwOptions);
        
        DECLSPEC_XFGVIRT(IInternetProtocol, UnlockRequest)
        HRESULT ( STDMETHODCALLTYPE *UnlockRequest )( 
            IInternetProtocol * This);
        
        END_INTERFACE
    } IInternetProtocolVtbl;

    interface IInternetProtocol
    {
        CONST_VTBL struct IInternetProtocolVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetProtocol_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetProtocol_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetProtocol_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetProtocol_Start(This,szUrl,pOIProtSink,pOIBindInfo,grfPI,dwReserved)	\
    ( (This)->lpVtbl -> Start(This,szUrl,pOIProtSink,pOIBindInfo,grfPI,dwReserved) ) 

#define IInternetProtocol_Continue(This,pProtocolData)	\
    ( (This)->lpVtbl -> Continue(This,pProtocolData) ) 

#define IInternetProtocol_Abort(This,hrReason,dwOptions)	\
    ( (This)->lpVtbl -> Abort(This,hrReason,dwOptions) ) 

#define IInternetProtocol_Terminate(This,dwOptions)	\
    ( (This)->lpVtbl -> Terminate(This,dwOptions) ) 

#define IInternetProtocol_Suspend(This)	\
    ( (This)->lpVtbl -> Suspend(This) ) 

#define IInternetProtocol_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 


#define IInternetProtocol_Read(This,pv,cb,pcbRead)	\
    ( (This)->lpVtbl -> Read(This,pv,cb,pcbRead) ) 

#define IInternetProtocol_Seek(This,dlibMove,dwOrigin,plibNewPosition)	\
    ( (This)->lpVtbl -> Seek(This,dlibMove,dwOrigin,plibNewPosition) ) 

#define IInternetProtocol_LockRequest(This,dwOptions)	\
    ( (This)->lpVtbl -> LockRequest(This,dwOptions) ) 

#define IInternetProtocol_UnlockRequest(This)	\
    ( (This)->lpVtbl -> UnlockRequest(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetProtocol_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0030 */
/* [local] */ 

#endif
#if (_WIN32_IE >= _WIN32_IE_IE70)
#ifndef _LPIINTERNETPROTOCOLEX_DEFINED
#define _LPIINTERNETPROTOCOLEX_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0030_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0030_v0_0_s_ifspec;

#ifndef __IInternetProtocolEx_INTERFACE_DEFINED__
#define __IInternetProtocolEx_INTERFACE_DEFINED__

/* interface IInternetProtocolEx */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IInternetProtocolEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C7A98E66-1010-492c-A1C8-C809E1F75905")
    IInternetProtocolEx : public IInternetProtocol
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StartEx( 
            /* [in] */ IUri *pUri,
            /* [in] */ IInternetProtocolSink *pOIProtSink,
            /* [in] */ IInternetBindInfo *pOIBindInfo,
            /* [in] */ DWORD grfPI,
            /* [in] */ HANDLE_PTR dwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetProtocolExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternetProtocolEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternetProtocolEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternetProtocolEx * This);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            IInternetProtocolEx * This,
            /* [in] */ LPCWSTR szUrl,
            /* [in] */ IInternetProtocolSink *pOIProtSink,
            /* [in] */ IInternetBindInfo *pOIBindInfo,
            /* [in] */ DWORD grfPI,
            /* [in] */ HANDLE_PTR dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Continue)
        HRESULT ( STDMETHODCALLTYPE *Continue )( 
            IInternetProtocolEx * This,
            /* [in] */ PROTOCOLDATA *pProtocolData);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Abort)
        HRESULT ( STDMETHODCALLTYPE *Abort )( 
            IInternetProtocolEx * This,
            /* [in] */ HRESULT hrReason,
            /* [in] */ DWORD dwOptions);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Terminate)
        HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            IInternetProtocolEx * This,
            /* [in] */ DWORD dwOptions);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Suspend)
        HRESULT ( STDMETHODCALLTYPE *Suspend )( 
            IInternetProtocolEx * This);
        
        DECLSPEC_XFGVIRT(IInternetProtocolRoot, Resume)
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            IInternetProtocolEx * This);
        
        DECLSPEC_XFGVIRT(IInternetProtocol, Read)
        HRESULT ( STDMETHODCALLTYPE *Read )( 
            IInternetProtocolEx * This,
            /* [length_is][size_is][out][in] */ void *pv,
            /* [in] */ ULONG cb,
            /* [out] */ ULONG *pcbRead);
        
        DECLSPEC_XFGVIRT(IInternetProtocol, Seek)
        HRESULT ( STDMETHODCALLTYPE *Seek )( 
            IInternetProtocolEx * This,
            /* [in] */ LARGE_INTEGER dlibMove,
            /* [in] */ DWORD dwOrigin,
            /* [out] */ ULARGE_INTEGER *plibNewPosition);
        
        DECLSPEC_XFGVIRT(IInternetProtocol, LockRequest)
        HRESULT ( STDMETHODCALLTYPE *LockRequest )( 
            IInternetProtocolEx * This,
            /* [in] */ DWORD dwOptions);
        
        DECLSPEC_XFGVIRT(IInternetProtocol, UnlockRequest)
        HRESULT ( STDMETHODCALLTYPE *UnlockRequest )( 
            IInternetProtocolEx * This);
        
        DECLSPEC_XFGVIRT(IInternetProtocolEx, StartEx)
        HRESULT ( STDMETHODCALLTYPE *StartEx )( 
            IInternetProtocolEx * This,
            /* [in] */ IUri *pUri,
            /* [in] */ IInternetProtocolSink *pOIProtSink,
            /* [in] */ IInternetBindInfo *pOIBindInfo,
            /* [in] */ DWORD grfPI,
            /* [in] */ HANDLE_PTR dwReserved);
        
        END_INTERFACE
    } IInternetProtocolExVtbl;

    interface IInternetProtocolEx
    {
        CONST_VTBL struct IInternetProtocolExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetProtocolEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetProtocolEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetProtocolEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetProtocolEx_Start(This,szUrl,pOIProtSink,pOIBindInfo,grfPI,dwReserved)	\
    ( (This)->lpVtbl -> Start(This,szUrl,pOIProtSink,pOIBindInfo,grfPI,dwReserved) ) 

#define IInternetProtocolEx_Continue(This,pProtocolData)	\
    ( (This)->lpVtbl -> Continue(This,pProtocolData) ) 

#define IInternetProtocolEx_Abort(This,hrReason,dwOptions)	\
    ( (This)->lpVtbl -> Abort(This,hrReason,dwOptions) ) 

#define IInternetProtocolEx_Terminate(This,dwOptions)	\
    ( (This)->lpVtbl -> Terminate(This,dwOptions) ) 

#define IInternetProtocolEx_Suspend(This)	\
    ( (This)->lpVtbl -> Suspend(This) ) 

#define IInternetProtocolEx_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 


#define IInternetProtocolEx_Read(This,pv,cb,pcbRead)	\
    ( (This)->lpVtbl -> Read(This,pv,cb,pcbRead) ) 

#define IInternetProtocolEx_Seek(This,dlibMove,dwOrigin,plibNewPosition)	\
    ( (This)->lpVtbl -> Seek(This,dlibMove,dwOrigin,plibNewPosition) ) 

#define IInternetProtocolEx_LockRequest(This,dwOptions)	\
    ( (This)->lpVtbl -> LockRequest(This,dwOptions) ) 

#define IInternetProtocolEx_UnlockRequest(This)	\
    ( (This)->lpVtbl -> UnlockRequest(This) ) 


#define IInternetProtocolEx_StartEx(This,pUri,pOIProtSink,pOIBindInfo,grfPI,dwReserved)	\
    ( (This)->lpVtbl -> StartEx(This,pUri,pOIProtSink,pOIBindInfo,grfPI,dwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetProtocolEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0031 */
/* [local] */ 

#endif
#endif //(_WIN32_IE >= _WIN32_IE_IE70)
#ifndef _LPIINTERNETPROTOCOLSINK_DEFINED
#define _LPIINTERNETPROTOCOLSINK_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0031_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0031_v0_0_s_ifspec;

#ifndef __IInternetProtocolSink_INTERFACE_DEFINED__
#define __IInternetProtocolSink_INTERFACE_DEFINED__

/* interface IInternetProtocolSink */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IInternetProtocolSink *LPIINTERNETPROTOCOLSINK;


EXTERN_C const IID IID_IInternetProtocolSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9e5-baf9-11ce-8c82-00aa004ba90b")
    IInternetProtocolSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Switch( 
            /* [in] */ PROTOCOLDATA *pProtocolData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReportProgress( 
            /* [in] */ ULONG ulStatusCode,
            /* [in] */ LPCWSTR szStatusText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReportData( 
            /* [in] */ DWORD grfBSCF,
            /* [in] */ ULONG ulProgress,
            /* [in] */ ULONG ulProgressMax) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReportResult( 
            /* [in] */ HRESULT hrResult,
            /* [in] */ DWORD dwError,
            /* [in] */ LPCWSTR szResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetProtocolSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternetProtocolSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternetProtocolSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternetProtocolSink * This);
        
        DECLSPEC_XFGVIRT(IInternetProtocolSink, Switch)
        HRESULT ( STDMETHODCALLTYPE *Switch )( 
            IInternetProtocolSink * This,
            /* [in] */ PROTOCOLDATA *pProtocolData);
        
        DECLSPEC_XFGVIRT(IInternetProtocolSink, ReportProgress)
        HRESULT ( STDMETHODCALLTYPE *ReportProgress )( 
            IInternetProtocolSink * This,
            /* [in] */ ULONG ulStatusCode,
            /* [in] */ LPCWSTR szStatusText);
        
        DECLSPEC_XFGVIRT(IInternetProtocolSink, ReportData)
        HRESULT ( STDMETHODCALLTYPE *ReportData )( 
            IInternetProtocolSink * This,
            /* [in] */ DWORD grfBSCF,
            /* [in] */ ULONG ulProgress,
            /* [in] */ ULONG ulProgressMax);
        
        DECLSPEC_XFGVIRT(IInternetProtocolSink, ReportResult)
        HRESULT ( STDMETHODCALLTYPE *ReportResult )( 
            IInternetProtocolSink * This,
            /* [in] */ HRESULT hrResult,
            /* [in] */ DWORD dwError,
            /* [in] */ LPCWSTR szResult);
        
        END_INTERFACE
    } IInternetProtocolSinkVtbl;

    interface IInternetProtocolSink
    {
        CONST_VTBL struct IInternetProtocolSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetProtocolSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetProtocolSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetProtocolSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetProtocolSink_Switch(This,pProtocolData)	\
    ( (This)->lpVtbl -> Switch(This,pProtocolData) ) 

#define IInternetProtocolSink_ReportProgress(This,ulStatusCode,szStatusText)	\
    ( (This)->lpVtbl -> ReportProgress(This,ulStatusCode,szStatusText) ) 

#define IInternetProtocolSink_ReportData(This,grfBSCF,ulProgress,ulProgressMax)	\
    ( (This)->lpVtbl -> ReportData(This,grfBSCF,ulProgress,ulProgressMax) ) 

#define IInternetProtocolSink_ReportResult(This,hrResult,dwError,szResult)	\
    ( (This)->lpVtbl -> ReportResult(This,hrResult,dwError,szResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetProtocolSink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0032 */
/* [local] */ 

#endif
#ifndef _LPIINTERNETPROTOCOLSINKSTACKABLE_DEFINED
#define _LPIINTERNETPROTOCOLSINKSTACKABLE_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0032_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0032_v0_0_s_ifspec;

#ifndef __IInternetProtocolSinkStackable_INTERFACE_DEFINED__
#define __IInternetProtocolSinkStackable_INTERFACE_DEFINED__

/* interface IInternetProtocolSinkStackable */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IInternetProtocolSinkStackable *LPIINTERNETPROTOCOLSINKStackable;


EXTERN_C const IID IID_IInternetProtocolSinkStackable;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9f0-baf9-11ce-8c82-00aa004ba90b")
    IInternetProtocolSinkStackable : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SwitchSink( 
            /* [in] */ IInternetProtocolSink *pOIProtSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CommitSwitch( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RollbackSwitch( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetProtocolSinkStackableVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternetProtocolSinkStackable * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternetProtocolSinkStackable * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternetProtocolSinkStackable * This);
        
        DECLSPEC_XFGVIRT(IInternetProtocolSinkStackable, SwitchSink)
        HRESULT ( STDMETHODCALLTYPE *SwitchSink )( 
            IInternetProtocolSinkStackable * This,
            /* [in] */ IInternetProtocolSink *pOIProtSink);
        
        DECLSPEC_XFGVIRT(IInternetProtocolSinkStackable, CommitSwitch)
        HRESULT ( STDMETHODCALLTYPE *CommitSwitch )( 
            IInternetProtocolSinkStackable * This);
        
        DECLSPEC_XFGVIRT(IInternetProtocolSinkStackable, RollbackSwitch)
        HRESULT ( STDMETHODCALLTYPE *RollbackSwitch )( 
            IInternetProtocolSinkStackable * This);
        
        END_INTERFACE
    } IInternetProtocolSinkStackableVtbl;

    interface IInternetProtocolSinkStackable
    {
        CONST_VTBL struct IInternetProtocolSinkStackableVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetProtocolSinkStackable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetProtocolSinkStackable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetProtocolSinkStackable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetProtocolSinkStackable_SwitchSink(This,pOIProtSink)	\
    ( (This)->lpVtbl -> SwitchSink(This,pOIProtSink) ) 

#define IInternetProtocolSinkStackable_CommitSwitch(This)	\
    ( (This)->lpVtbl -> CommitSwitch(This) ) 

#define IInternetProtocolSinkStackable_RollbackSwitch(This)	\
    ( (This)->lpVtbl -> RollbackSwitch(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetProtocolSinkStackable_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0033 */
/* [local] */ 

#endif
#ifndef _LPIINTERNETSESSION_DEFINED
#define _LPIINTERNETSESSION_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0033_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0033_v0_0_s_ifspec;

#ifndef __IInternetSession_INTERFACE_DEFINED__
#define __IInternetSession_INTERFACE_DEFINED__

/* interface IInternetSession */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IInternetSession *LPIINTERNETSESSION;

typedef 
enum _tagOIBDG_FLAGS
    {
        OIBDG_APARTMENTTHREADED	= 0x100,
        OIBDG_DATAONLY	= 0x1000
    } 	OIBDG_FLAGS;


EXTERN_C const IID IID_IInternetSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9e7-baf9-11ce-8c82-00aa004ba90b")
    IInternetSession : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterNameSpace( 
            /* [in] */ IClassFactory *pCF,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ LPCWSTR pwzProtocol,
            /* [in] */ ULONG cPatterns,
            /* [in] */ const LPCWSTR *ppwzPatterns,
            /* [in] */ DWORD dwReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterNameSpace( 
            /* [in] */ IClassFactory *pCF,
            /* [in] */ LPCWSTR pszProtocol) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterMimeFilter( 
            /* [in] */ IClassFactory *pCF,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ LPCWSTR pwzType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterMimeFilter( 
            /* [in] */ IClassFactory *pCF,
            /* [in] */ LPCWSTR pwzType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateBinding( 
            /* [in] */ LPBC pBC,
            /* [in] */ LPCWSTR szUrl,
            /* [in] */ IUnknown *pUnkOuter,
            /* [unique][out] */ IUnknown **ppUnk,
            /* [unique][out] */ IInternetProtocol **ppOInetProt,
            /* [in] */ DWORD dwOption) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSessionOption( 
            /* [in] */ DWORD dwOption,
            /* [in] */ LPVOID pBuffer,
            /* [in] */ DWORD dwBufferLength,
            /* [in] */ DWORD dwReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSessionOption( 
            /* [in] */ DWORD dwOption,
            /* [out][in] */ LPVOID pBuffer,
            /* [out][in] */ DWORD *pdwBufferLength,
            /* [in] */ DWORD dwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternetSession * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternetSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternetSession * This);
        
        DECLSPEC_XFGVIRT(IInternetSession, RegisterNameSpace)
        HRESULT ( STDMETHODCALLTYPE *RegisterNameSpace )( 
            IInternetSession * This,
            /* [in] */ IClassFactory *pCF,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ LPCWSTR pwzProtocol,
            /* [in] */ ULONG cPatterns,
            /* [in] */ const LPCWSTR *ppwzPatterns,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetSession, UnregisterNameSpace)
        HRESULT ( STDMETHODCALLTYPE *UnregisterNameSpace )( 
            IInternetSession * This,
            /* [in] */ IClassFactory *pCF,
            /* [in] */ LPCWSTR pszProtocol);
        
        DECLSPEC_XFGVIRT(IInternetSession, RegisterMimeFilter)
        HRESULT ( STDMETHODCALLTYPE *RegisterMimeFilter )( 
            IInternetSession * This,
            /* [in] */ IClassFactory *pCF,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ LPCWSTR pwzType);
        
        DECLSPEC_XFGVIRT(IInternetSession, UnregisterMimeFilter)
        HRESULT ( STDMETHODCALLTYPE *UnregisterMimeFilter )( 
            IInternetSession * This,
            /* [in] */ IClassFactory *pCF,
            /* [in] */ LPCWSTR pwzType);
        
        DECLSPEC_XFGVIRT(IInternetSession, CreateBinding)
        HRESULT ( STDMETHODCALLTYPE *CreateBinding )( 
            IInternetSession * This,
            /* [in] */ LPBC pBC,
            /* [in] */ LPCWSTR szUrl,
            /* [in] */ IUnknown *pUnkOuter,
            /* [unique][out] */ IUnknown **ppUnk,
            /* [unique][out] */ IInternetProtocol **ppOInetProt,
            /* [in] */ DWORD dwOption);
        
        DECLSPEC_XFGVIRT(IInternetSession, SetSessionOption)
        HRESULT ( STDMETHODCALLTYPE *SetSessionOption )( 
            IInternetSession * This,
            /* [in] */ DWORD dwOption,
            /* [in] */ LPVOID pBuffer,
            /* [in] */ DWORD dwBufferLength,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetSession, GetSessionOption)
        HRESULT ( STDMETHODCALLTYPE *GetSessionOption )( 
            IInternetSession * This,
            /* [in] */ DWORD dwOption,
            /* [out][in] */ LPVOID pBuffer,
            /* [out][in] */ DWORD *pdwBufferLength,
            /* [in] */ DWORD dwReserved);
        
        END_INTERFACE
    } IInternetSessionVtbl;

    interface IInternetSession
    {
        CONST_VTBL struct IInternetSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetSession_RegisterNameSpace(This,pCF,rclsid,pwzProtocol,cPatterns,ppwzPatterns,dwReserved)	\
    ( (This)->lpVtbl -> RegisterNameSpace(This,pCF,rclsid,pwzProtocol,cPatterns,ppwzPatterns,dwReserved) ) 

#define IInternetSession_UnregisterNameSpace(This,pCF,pszProtocol)	\
    ( (This)->lpVtbl -> UnregisterNameSpace(This,pCF,pszProtocol) ) 

#define IInternetSession_RegisterMimeFilter(This,pCF,rclsid,pwzType)	\
    ( (This)->lpVtbl -> RegisterMimeFilter(This,pCF,rclsid,pwzType) ) 

#define IInternetSession_UnregisterMimeFilter(This,pCF,pwzType)	\
    ( (This)->lpVtbl -> UnregisterMimeFilter(This,pCF,pwzType) ) 

#define IInternetSession_CreateBinding(This,pBC,szUrl,pUnkOuter,ppUnk,ppOInetProt,dwOption)	\
    ( (This)->lpVtbl -> CreateBinding(This,pBC,szUrl,pUnkOuter,ppUnk,ppOInetProt,dwOption) ) 

#define IInternetSession_SetSessionOption(This,dwOption,pBuffer,dwBufferLength,dwReserved)	\
    ( (This)->lpVtbl -> SetSessionOption(This,dwOption,pBuffer,dwBufferLength,dwReserved) ) 

#define IInternetSession_GetSessionOption(This,dwOption,pBuffer,pdwBufferLength,dwReserved)	\
    ( (This)->lpVtbl -> GetSessionOption(This,dwOption,pBuffer,pdwBufferLength,dwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetSession_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0034 */
/* [local] */ 

#endif
#ifndef _LPIINTERNETTHREADSWITCH_DEFINED
#define _LPIINTERNETTHREADSWITCH_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0034_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0034_v0_0_s_ifspec;

#ifndef __IInternetThreadSwitch_INTERFACE_DEFINED__
#define __IInternetThreadSwitch_INTERFACE_DEFINED__

/* interface IInternetThreadSwitch */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IInternetThreadSwitch *LPIINTERNETTHREADSWITCH;


EXTERN_C const IID IID_IInternetThreadSwitch;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9e8-baf9-11ce-8c82-00aa004ba90b")
    IInternetThreadSwitch : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Prepare( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Continue( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetThreadSwitchVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternetThreadSwitch * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternetThreadSwitch * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternetThreadSwitch * This);
        
        DECLSPEC_XFGVIRT(IInternetThreadSwitch, Prepare)
        HRESULT ( STDMETHODCALLTYPE *Prepare )( 
            IInternetThreadSwitch * This);
        
        DECLSPEC_XFGVIRT(IInternetThreadSwitch, Continue)
        HRESULT ( STDMETHODCALLTYPE *Continue )( 
            IInternetThreadSwitch * This);
        
        END_INTERFACE
    } IInternetThreadSwitchVtbl;

    interface IInternetThreadSwitch
    {
        CONST_VTBL struct IInternetThreadSwitchVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetThreadSwitch_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetThreadSwitch_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetThreadSwitch_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetThreadSwitch_Prepare(This)	\
    ( (This)->lpVtbl -> Prepare(This) ) 

#define IInternetThreadSwitch_Continue(This)	\
    ( (This)->lpVtbl -> Continue(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetThreadSwitch_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0035 */
/* [local] */ 

#endif
#ifndef _LPIINTERNETPRIORITY_DEFINED
#define _LPIINTERNETPRIORITY_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0035_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0035_v0_0_s_ifspec;

#ifndef __IInternetPriority_INTERFACE_DEFINED__
#define __IInternetPriority_INTERFACE_DEFINED__

/* interface IInternetPriority */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IInternetPriority *LPIINTERNETPRIORITY;


EXTERN_C const IID IID_IInternetPriority;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9eb-baf9-11ce-8c82-00aa004ba90b")
    IInternetPriority : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetPriority( 
            /* [in] */ LONG nPriority) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPriority( 
            /* [out] */ LONG *pnPriority) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetPriorityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternetPriority * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternetPriority * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternetPriority * This);
        
        DECLSPEC_XFGVIRT(IInternetPriority, SetPriority)
        HRESULT ( STDMETHODCALLTYPE *SetPriority )( 
            IInternetPriority * This,
            /* [in] */ LONG nPriority);
        
        DECLSPEC_XFGVIRT(IInternetPriority, GetPriority)
        HRESULT ( STDMETHODCALLTYPE *GetPriority )( 
            IInternetPriority * This,
            /* [out] */ LONG *pnPriority);
        
        END_INTERFACE
    } IInternetPriorityVtbl;

    interface IInternetPriority
    {
        CONST_VTBL struct IInternetPriorityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetPriority_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetPriority_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetPriority_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetPriority_SetPriority(This,nPriority)	\
    ( (This)->lpVtbl -> SetPriority(This,nPriority) ) 

#define IInternetPriority_GetPriority(This,pnPriority)	\
    ( (This)->lpVtbl -> GetPriority(This,pnPriority) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetPriority_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0036 */
/* [local] */ 

#endif
#ifndef _LPIINTERNETPROTOCOLINFO_DEFINED
#define _LPIINTERNETPROTOCOLINFO_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0036_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0036_v0_0_s_ifspec;

#ifndef __IInternetProtocolInfo_INTERFACE_DEFINED__
#define __IInternetProtocolInfo_INTERFACE_DEFINED__

/* interface IInternetProtocolInfo */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IInternetProtocolInfo *LPIINTERNETPROTOCOLINFO;

typedef 
enum _tagPARSEACTION
    {
        PARSE_CANONICALIZE	= 1,
        PARSE_FRIENDLY	= ( PARSE_CANONICALIZE + 1 ) ,
        PARSE_SECURITY_URL	= ( PARSE_FRIENDLY + 1 ) ,
        PARSE_ROOTDOCUMENT	= ( PARSE_SECURITY_URL + 1 ) ,
        PARSE_DOCUMENT	= ( PARSE_ROOTDOCUMENT + 1 ) ,
        PARSE_ANCHOR	= ( PARSE_DOCUMENT + 1 ) ,
        PARSE_ENCODE_IS_UNESCAPE	= ( PARSE_ANCHOR + 1 ) ,
        PARSE_DECODE_IS_ESCAPE	= ( PARSE_ENCODE_IS_UNESCAPE + 1 ) ,
        PARSE_PATH_FROM_URL	= ( PARSE_DECODE_IS_ESCAPE + 1 ) ,
        PARSE_URL_FROM_PATH	= ( PARSE_PATH_FROM_URL + 1 ) ,
        PARSE_MIME	= ( PARSE_URL_FROM_PATH + 1 ) ,
        PARSE_SERVER	= ( PARSE_MIME + 1 ) ,
        PARSE_SCHEMA	= ( PARSE_SERVER + 1 ) ,
        PARSE_SITE	= ( PARSE_SCHEMA + 1 ) ,
        PARSE_DOMAIN	= ( PARSE_SITE + 1 ) ,
        PARSE_LOCATION	= ( PARSE_DOMAIN + 1 ) ,
        PARSE_SECURITY_DOMAIN	= ( PARSE_LOCATION + 1 ) ,
        PARSE_ESCAPE	= ( PARSE_SECURITY_DOMAIN + 1 ) ,
        PARSE_UNESCAPE	= ( PARSE_ESCAPE + 1 ) 
    } 	PARSEACTION;

typedef 
enum _tagPSUACTION
    {
        PSU_DEFAULT	= 1,
        PSU_SECURITY_URL_ONLY	= ( PSU_DEFAULT + 1 ) 
    } 	PSUACTION;

typedef 
enum _tagQUERYOPTION
    {
        QUERY_EXPIRATION_DATE	= 1,
        QUERY_TIME_OF_LAST_CHANGE	= ( QUERY_EXPIRATION_DATE + 1 ) ,
        QUERY_CONTENT_ENCODING	= ( QUERY_TIME_OF_LAST_CHANGE + 1 ) ,
        QUERY_CONTENT_TYPE	= ( QUERY_CONTENT_ENCODING + 1 ) ,
        QUERY_REFRESH	= ( QUERY_CONTENT_TYPE + 1 ) ,
        QUERY_RECOMBINE	= ( QUERY_REFRESH + 1 ) ,
        QUERY_CAN_NAVIGATE	= ( QUERY_RECOMBINE + 1 ) ,
        QUERY_USES_NETWORK	= ( QUERY_CAN_NAVIGATE + 1 ) ,
        QUERY_IS_CACHED	= ( QUERY_USES_NETWORK + 1 ) ,
        QUERY_IS_INSTALLEDENTRY	= ( QUERY_IS_CACHED + 1 ) ,
        QUERY_IS_CACHED_OR_MAPPED	= ( QUERY_IS_INSTALLEDENTRY + 1 ) ,
        QUERY_USES_CACHE	= ( QUERY_IS_CACHED_OR_MAPPED + 1 ) ,
        QUERY_IS_SECURE	= ( QUERY_USES_CACHE + 1 ) ,
        QUERY_IS_SAFE	= ( QUERY_IS_SECURE + 1 ) ,
        QUERY_USES_HISTORYFOLDER	= ( QUERY_IS_SAFE + 1 ) ,
        QUERY_IS_CACHED_AND_USABLE_OFFLINE	= ( QUERY_USES_HISTORYFOLDER + 1 ) 
    } 	QUERYOPTION;


EXTERN_C const IID IID_IInternetProtocolInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9ec-baf9-11ce-8c82-00aa004ba90b")
    IInternetProtocolInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ParseUrl( 
            /* [in] */ LPCWSTR pwzUrl,
            /* [in] */ PARSEACTION ParseAction,
            /* [in] */ DWORD dwParseFlags,
            /* [annotation][out] */ 
            __RPC__out  LPWSTR pwzResult,
            /* [in] */ DWORD cchResult,
            /* [out] */ DWORD *pcchResult,
            /* [in] */ DWORD dwReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CombineUrl( 
            /* [in] */ LPCWSTR pwzBaseUrl,
            /* [in] */ LPCWSTR pwzRelativeUrl,
            /* [in] */ DWORD dwCombineFlags,
            /* [annotation][out] */ 
            __RPC__in  LPWSTR pwzResult,
            /* [in] */ DWORD cchResult,
            /* [out] */ DWORD *pcchResult,
            /* [in] */ DWORD dwReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CompareUrl( 
            /* [in] */ LPCWSTR pwzUrl1,
            /* [in] */ LPCWSTR pwzUrl2,
            /* [in] */ DWORD dwCompareFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryInfo( 
            /* [in] */ LPCWSTR pwzUrl,
            /* [in] */ QUERYOPTION OueryOption,
            /* [in] */ DWORD dwQueryFlags,
            /* [size_is][out][in] */ LPVOID pBuffer,
            /* [in] */ DWORD cbBuffer,
            /* [out][in] */ DWORD *pcbBuf,
            /* [in] */ DWORD dwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetProtocolInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternetProtocolInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternetProtocolInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternetProtocolInfo * This);
        
        DECLSPEC_XFGVIRT(IInternetProtocolInfo, ParseUrl)
        HRESULT ( STDMETHODCALLTYPE *ParseUrl )( 
            IInternetProtocolInfo * This,
            /* [in] */ LPCWSTR pwzUrl,
            /* [in] */ PARSEACTION ParseAction,
            /* [in] */ DWORD dwParseFlags,
            /* [annotation][out] */ 
            __RPC__out  LPWSTR pwzResult,
            /* [in] */ DWORD cchResult,
            /* [out] */ DWORD *pcchResult,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetProtocolInfo, CombineUrl)
        HRESULT ( STDMETHODCALLTYPE *CombineUrl )( 
            IInternetProtocolInfo * This,
            /* [in] */ LPCWSTR pwzBaseUrl,
            /* [in] */ LPCWSTR pwzRelativeUrl,
            /* [in] */ DWORD dwCombineFlags,
            /* [annotation][out] */ 
            __RPC__in  LPWSTR pwzResult,
            /* [in] */ DWORD cchResult,
            /* [out] */ DWORD *pcchResult,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetProtocolInfo, CompareUrl)
        HRESULT ( STDMETHODCALLTYPE *CompareUrl )( 
            IInternetProtocolInfo * This,
            /* [in] */ LPCWSTR pwzUrl1,
            /* [in] */ LPCWSTR pwzUrl2,
            /* [in] */ DWORD dwCompareFlags);
        
        DECLSPEC_XFGVIRT(IInternetProtocolInfo, QueryInfo)
        HRESULT ( STDMETHODCALLTYPE *QueryInfo )( 
            IInternetProtocolInfo * This,
            /* [in] */ LPCWSTR pwzUrl,
            /* [in] */ QUERYOPTION OueryOption,
            /* [in] */ DWORD dwQueryFlags,
            /* [size_is][out][in] */ LPVOID pBuffer,
            /* [in] */ DWORD cbBuffer,
            /* [out][in] */ DWORD *pcbBuf,
            /* [in] */ DWORD dwReserved);
        
        END_INTERFACE
    } IInternetProtocolInfoVtbl;

    interface IInternetProtocolInfo
    {
        CONST_VTBL struct IInternetProtocolInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetProtocolInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetProtocolInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetProtocolInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetProtocolInfo_ParseUrl(This,pwzUrl,ParseAction,dwParseFlags,pwzResult,cchResult,pcchResult,dwReserved)	\
    ( (This)->lpVtbl -> ParseUrl(This,pwzUrl,ParseAction,dwParseFlags,pwzResult,cchResult,pcchResult,dwReserved) ) 

#define IInternetProtocolInfo_CombineUrl(This,pwzBaseUrl,pwzRelativeUrl,dwCombineFlags,pwzResult,cchResult,pcchResult,dwReserved)	\
    ( (This)->lpVtbl -> CombineUrl(This,pwzBaseUrl,pwzRelativeUrl,dwCombineFlags,pwzResult,cchResult,pcchResult,dwReserved) ) 

#define IInternetProtocolInfo_CompareUrl(This,pwzUrl1,pwzUrl2,dwCompareFlags)	\
    ( (This)->lpVtbl -> CompareUrl(This,pwzUrl1,pwzUrl2,dwCompareFlags) ) 

#define IInternetProtocolInfo_QueryInfo(This,pwzUrl,OueryOption,dwQueryFlags,pBuffer,cbBuffer,pcbBuf,dwReserved)	\
    ( (This)->lpVtbl -> QueryInfo(This,pwzUrl,OueryOption,dwQueryFlags,pBuffer,cbBuffer,pcbBuf,dwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetProtocolInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0037 */
/* [local] */ 

#ifndef URLMON_STRICT
// PARSE_ENCODE was misnamed and is deprecated for PARSE_UNESCAPE
#define PARSE_ENCODE PARSE_ENCODE_IS_UNESCAPE
// PARSE_DECODE was misnamed and is deprecated for PARSE_ESCAPE
#define PARSE_DECODE PARSE_DECODE_IS_ESCAPE
#endif //!URLMON_STRICT
#endif
#define IOInet               IInternet            
#define IOInetBindInfo       IInternetBindInfo    
#define IOInetBindInfoEx     IInternetBindInfoEx  
#define IOInetProtocolRoot   IInternetProtocolRoot
#define IOInetProtocol       IInternetProtocol    
#if (_WIN32_IE >= _WIN32_IE_IE70)
#define IOInetProtocolEx     IInternetProtocolEx  
#endif //(_WIN32_IE >= _WIN32_IE_IE70)
#define IOInetProtocolSink   IInternetProtocolSink
#define IOInetProtocolInfo   IInternetProtocolInfo
#define IOInetSession        IInternetSession     
#define IOInetPriority       IInternetPriority    
#define IOInetThreadSwitch   IInternetThreadSwitch
#define IOInetProtocolSinkStackable   IInternetProtocolSinkStackable
#define LPOINET              LPIINTERNET             
#define LPOINETPROTOCOLINFO  LPIINTERNETPROTOCOLINFO 
#define LPOINETBINDINFO      LPIINTERNETBINDINFO     
#define LPOINETPROTOCOLROOT  LPIINTERNETPROTOCOLROOT 
#define LPOINETPROTOCOL      LPIINTERNETPROTOCOL     
#if (_WIN32_IE >= _WIN32_IE_IE70)
#define LPOINETPROTOCOLEX    LPIINTERNETPROTOCOLEX   
#endif //(_WIN32_IE >= _WIN32_IE_IE70)
#define LPOINETPROTOCOLSINK  LPIINTERNETPROTOCOLSINK 
#define LPOINETSESSION       LPIINTERNETSESSION      
#define LPOINETTHREADSWITCH  LPIINTERNETTHREADSWITCH 
#define LPOINETPRIORITY      LPIINTERNETPRIORITY     
#define LPOINETPROTOCOLINFO  LPIINTERNETPROTOCOLINFO 
#define LPOINETPROTOCOLSINKSTACKABLE  LPIINTERNETPROTOCOLSINKSTACKABLE 
#define IID_IOInet               IID_IInternet            
#define IID_IOInetBindInfo       IID_IInternetBindInfo    
#define IID_IOInetBindInfoEx     IID_IInternetBindInfoEx  
#define IID_IOInetProtocolRoot   IID_IInternetProtocolRoot
#define IID_IOInetProtocol       IID_IInternetProtocol    
#if (_WIN32_IE >= _WIN32_IE_IE70)
#define IID_IOInetProtocolEx     IID_IInternetProtocolEx  
#endif //(_WIN32_IE >= _WIN32_IE_IE70)
#define IID_IOInetProtocolSink   IID_IInternetProtocolSink
#define IID_IOInetProtocolInfo   IID_IInternetProtocolInfo
#define IID_IOInetSession        IID_IInternetSession     
#define IID_IOInetPriority       IID_IInternetPriority    
#define IID_IOInetThreadSwitch   IID_IInternetThreadSwitch
#define IID_IOInetProtocolSinkStackable   IID_IInternetProtocolSinkStackable
STDAPI CoInternetParseUrl(                              
                                             LPCWSTR      pwzUrl,        
                                             PARSEACTION  ParseAction,   
                                             DWORD        dwFlags,       
    _Out_writes_to_(cchResult,*pcchResult+1) LPWSTR       pszResult,     
                                             DWORD        cchResult,     
    _Out_                                    DWORD       *pcchResult,    
                                             DWORD        dwReserved     
    );                                                  
#if (_WIN32_IE >= _WIN32_IE_IE70)
STDAPI CoInternetParseIUri(                             
    _In_                                     IUri        *pIUri,         
                                             PARSEACTION  ParseAction,   
                                             DWORD        dwFlags,       
    _Out_writes_to_(cchResult,*pcchResult+1) LPWSTR       pwzResult,     
                                             DWORD        cchResult,     
    _Out_                                    DWORD       *pcchResult,    
    _Reserved_                               DWORD_PTR    dwReserved     
    );                                                  
#endif //(_WIN32_IE >= _WIN32_IE_IE70)
STDAPI CoInternetCombineUrl(                                           
                                             LPCWSTR   pwzBaseUrl,     
                                             LPCWSTR   pwzRelativeUrl, 
                                             DWORD     dwCombineFlags, 
    _Out_writes_to_(cchResult,*pcchResult+1) LPWSTR    pszResult,      
                                             DWORD     cchResult,      
    _Out_opt_                                DWORD     *pcchResult,    
    _Reserved_                               DWORD     dwReserved      
    );                                                                 
#if (_WIN32_IE >= _WIN32_IE_IE70)
STDAPI CoInternetCombineUrlEx(               
    _In_opt_     IUri       *pBaseUri,       
    _In_opt_     LPCWSTR     pwzRelativeUrl, 
                 DWORD       dwCombineFlags, 
    _Outptr_     IUri      **ppCombinedUri,  
    _In_opt_     DWORD_PTR   dwReserved      
    );                                       
STDAPI CoInternetCombineIUri (               
    _In_         IUri       *pBaseUri,       
    _In_         IUri       *pRelativeUri,   
                 DWORD       dwCombineFlags, 
    _Outptr_     IUri      **ppCombinedUri,  
    _In_opt_     DWORD_PTR   dwReserved      
    );                                       
#endif //(_WIN32_IE >= _WIN32_IE_IE70)
STDAPI CoInternetCompareUrl(             
    LPCWSTR pwzUrl1,                     
    LPCWSTR pwzUrl2,                     
    DWORD dwFlags                        
    );                                   
STDAPI CoInternetGetProtocolFlags(       
             LPCWSTR     pwzUrl,         
    _Out_    DWORD      *pdwFlags,       
             DWORD       dwReserved      
    );                                   
STDAPI CoInternetQueryInfo(                                                
                                                 LPCWSTR     pwzUrl,       
                                                 QUERYOPTION QueryOptions, 
                                                 DWORD       dwQueryFlags, 
    _Out_writes_bytes_to_(cbBuffer, *pcbBuffer)  LPVOID      pvBuffer,     
    _In_range_(>=, sizeof(DWORD))                DWORD       cbBuffer,     
    _Out_opt_                                    DWORD      *pcbBuffer,    
                                                 DWORD       dwReserved    
    );                                                                     
STDAPI CoInternetGetSession(                             
                 DWORD               dwSessionMode,      
    _Outptr_     IInternetSession  **ppIInternetSession, 
                 DWORD               dwReserved          
    );                                                   
STDAPI CoInternetGetSecurityUrl(         
                 LPCWSTR pwszUrl,        
    _Outptr_     LPWSTR *ppwszSecUrl,    
    _In_         PSUACTION   psuAction,  
    _Reserved_   DWORD dwReserved        
    );                                   
STDAPI AsyncInstallDistributionUnit(     
    _In_ LPCWSTR szDistUnit,             
    _In_opt_ LPCWSTR szTYPE,             
    _In_opt_ LPCWSTR szExt,              
    DWORD dwFileVersionMS,               
    DWORD dwFileVersionLS,               
    _In_opt_ LPCWSTR szURL,                  
    _In_ IBindCtx *pbc,                  
    _Reserved_ LPVOID   pvReserved,      
    DWORD   flags                        
    );                                   
#if (_WIN32_IE >= _WIN32_IE_IE70)
STDAPI CoInternetGetSecurityUrlEx(           
    _In_         IUri           *pUri,       
    _Outptr_     IUri          **ppSecUri,   
                 PSUACTION       psuAction,  
    _Reserved_   DWORD_PTR       dwReserved  
    );                                       
#endif //(_WIN32_IE >= _WIN32_IE_IE70)
#if (_WIN32_IE >= _WIN32_IE_IE60SP2)
#ifndef _INTERNETFEATURELIST_DEFINED
#define _INTERNETFEATURELIST_DEFINED
typedef 
enum _tagINTERNETFEATURELIST
    {
        FEATURE_OBJECT_CACHING	= 0,
        FEATURE_ZONE_ELEVATION	= ( FEATURE_OBJECT_CACHING + 1 ) ,
        FEATURE_MIME_HANDLING	= ( FEATURE_ZONE_ELEVATION + 1 ) ,
        FEATURE_MIME_SNIFFING	= ( FEATURE_MIME_HANDLING + 1 ) ,
        FEATURE_WINDOW_RESTRICTIONS	= ( FEATURE_MIME_SNIFFING + 1 ) ,
        FEATURE_WEBOC_POPUPMANAGEMENT	= ( FEATURE_WINDOW_RESTRICTIONS + 1 ) ,
        FEATURE_BEHAVIORS	= ( FEATURE_WEBOC_POPUPMANAGEMENT + 1 ) ,
        FEATURE_DISABLE_MK_PROTOCOL	= ( FEATURE_BEHAVIORS + 1 ) ,
        FEATURE_LOCALMACHINE_LOCKDOWN	= ( FEATURE_DISABLE_MK_PROTOCOL + 1 ) ,
        FEATURE_SECURITYBAND	= ( FEATURE_LOCALMACHINE_LOCKDOWN + 1 ) ,
        FEATURE_RESTRICT_ACTIVEXINSTALL	= ( FEATURE_SECURITYBAND + 1 ) ,
        FEATURE_VALIDATE_NAVIGATE_URL	= ( FEATURE_RESTRICT_ACTIVEXINSTALL + 1 ) ,
        FEATURE_RESTRICT_FILEDOWNLOAD	= ( FEATURE_VALIDATE_NAVIGATE_URL + 1 ) ,
        FEATURE_ADDON_MANAGEMENT	= ( FEATURE_RESTRICT_FILEDOWNLOAD + 1 ) ,
        FEATURE_PROTOCOL_LOCKDOWN	= ( FEATURE_ADDON_MANAGEMENT + 1 ) ,
        FEATURE_HTTP_USERNAME_PASSWORD_DISABLE	= ( FEATURE_PROTOCOL_LOCKDOWN + 1 ) ,
        FEATURE_SAFE_BINDTOOBJECT	= ( FEATURE_HTTP_USERNAME_PASSWORD_DISABLE + 1 ) ,
        FEATURE_UNC_SAVEDFILECHECK	= ( FEATURE_SAFE_BINDTOOBJECT + 1 ) ,
        FEATURE_GET_URL_DOM_FILEPATH_UNENCODED	= ( FEATURE_UNC_SAVEDFILECHECK + 1 ) ,
        FEATURE_TABBED_BROWSING	= ( FEATURE_GET_URL_DOM_FILEPATH_UNENCODED + 1 ) ,
        FEATURE_SSLUX	= ( FEATURE_TABBED_BROWSING + 1 ) ,
        FEATURE_DISABLE_NAVIGATION_SOUNDS	= ( FEATURE_SSLUX + 1 ) ,
        FEATURE_DISABLE_LEGACY_COMPRESSION	= ( FEATURE_DISABLE_NAVIGATION_SOUNDS + 1 ) ,
        FEATURE_FORCE_ADDR_AND_STATUS	= ( FEATURE_DISABLE_LEGACY_COMPRESSION + 1 ) ,
        FEATURE_XMLHTTP	= ( FEATURE_FORCE_ADDR_AND_STATUS + 1 ) ,
        FEATURE_DISABLE_TELNET_PROTOCOL	= ( FEATURE_XMLHTTP + 1 ) ,
        FEATURE_FEEDS	= ( FEATURE_DISABLE_TELNET_PROTOCOL + 1 ) ,
        FEATURE_BLOCK_INPUT_PROMPTS	= ( FEATURE_FEEDS + 1 ) ,
        FEATURE_ENTRY_COUNT	= ( FEATURE_BLOCK_INPUT_PROMPTS + 1 ) 
    } 	INTERNETFEATURELIST;


// CoInternetSetFeatureEnabled can be used to set/reset features. 
// The following flags control where the feature is set

#define SET_FEATURE_ON_THREAD                       0x00000001
#define SET_FEATURE_ON_PROCESS                      0x00000002
#define SET_FEATURE_IN_REGISTRY                     0x00000004
#define SET_FEATURE_ON_THREAD_LOCALMACHINE          0x00000008
#define SET_FEATURE_ON_THREAD_INTRANET              0x00000010
#define SET_FEATURE_ON_THREAD_TRUSTED               0x00000020
#define SET_FEATURE_ON_THREAD_INTERNET              0x00000040
#define SET_FEATURE_ON_THREAD_RESTRICTED            0x00000080

// CoInternetIsFeatureEnabled can be used to get features. 
// The following flags control where the feature is obtained from
// default is from process

#define GET_FEATURE_FROM_THREAD                      0x00000001
#define GET_FEATURE_FROM_PROCESS                     0x00000002
#define GET_FEATURE_FROM_REGISTRY                    0x00000004
#define GET_FEATURE_FROM_THREAD_LOCALMACHINE         0x00000008
#define GET_FEATURE_FROM_THREAD_INTRANET             0x00000010
#define GET_FEATURE_FROM_THREAD_TRUSTED              0x00000020
#define GET_FEATURE_FROM_THREAD_INTERNET             0x00000040
#define GET_FEATURE_FROM_THREAD_RESTRICTED           0x00000080
#endif
STDAPI CoInternetSetFeatureEnabled(      
    INTERNETFEATURELIST FeatureEntry,    
    DWORD dwFlags,                       
    BOOL fEnable                         
    );                                   
STDAPI CoInternetIsFeatureEnabled(       
    INTERNETFEATURELIST FeatureEntry,    
    DWORD dwFlags                        
    );                                   
STDAPI CoInternetIsFeatureEnabledForUrl( 
    INTERNETFEATURELIST FeatureEntry,    
    DWORD dwFlags,                       
    _In_opt_ LPCWSTR szURL,                       
    _In_opt_ IInternetSecurityManager *pSecMgr    
    );                                   
STDAPI CoInternetIsFeatureEnabledForIUri( 
    INTERNETFEATURELIST FeatureEntry,     
    DWORD dwFlags,                        
    _In_opt_ IUri * pIUri,                         
    _In_opt_ IInternetSecurityManagerEx2 *pSecMgr  
    );                                    
STDAPI CoInternetIsFeatureZoneElevationEnabled( 
    _In_opt_ LPCWSTR szFromURL,                 
    _In_ LPCWSTR szToURL,                       
    _In_opt_ IInternetSecurityManager *pSecMgr, 
    DWORD dwFlags                               
    );                                          
#endif //(_WIN32_IE >= _WIN32_IE_IE60SP2)
 
STDAPI CopyStgMedium(_In_ const STGMEDIUM * pcstgmedSrc,  
                     _Out_      STGMEDIUM * pstgmedDest); 
STDAPI CopyBindInfo(_In_ const BINDINFO * pcbiSrc,   
                    _Out_      BINDINFO * pbiDest ); 
STDAPI_(void) ReleaseBindInfo( _Inout_ BINDINFO * pbindinfo );  
 
#define INET_E_USE_DEFAULT_PROTOCOLHANDLER _HRESULT_TYPEDEF_(0x800C0011L)      
#define INET_E_USE_DEFAULT_SETTING         _HRESULT_TYPEDEF_(0x800C0012L)      
#define INET_E_DEFAULT_ACTION              INET_E_USE_DEFAULT_PROTOCOLHANDLER  
#define INET_E_QUERYOPTION_UNKNOWN         _HRESULT_TYPEDEF_(0x800C0013L)      
#define INET_E_REDIRECTING                 _HRESULT_TYPEDEF_(0x800C0014L)      
#define OInetParseUrl               CoInternetParseUrl               
#define OInetCombineUrl             CoInternetCombineUrl             
#if (_WIN32_IE >= _WIN32_IE_IE70)
#define OInetCombineUrlEx           CoInternetCombineUrlEx           
#define OInetCombineIUri            CoInternetCombineIUri            
#endif //(_WIN32_IE >= _WIN32_IE_IE70)
#define OInetCompareUrl             CoInternetCompareUrl             
#define OInetQueryInfo              CoInternetQueryInfo              
#define OInetGetSession             CoInternetGetSession             
#endif // !_URLMON_NO_ASYNC_PLUGABLE_PROTOCOLS_ 
//
// Static Protocol flags
//
#define PROTOCOLFLAG_NO_PICS_CHECK     0x00000001

// Do not take a dependency on the exact value of the private namespace, no guarantee is given that it won't change.
STDAPI_(PWSTR) IEGetUserPrivateNamespaceName(void);
 
// Creates the security manager object. The first argument is the Service provider
// to allow for delegation
STDAPI CoInternetCreateSecurityManager(_In_opt_ IServiceProvider *pSP, _Outptr_ IInternetSecurityManager **ppSM, DWORD dwReserved);

STDAPI CoInternetCreateZoneManager(_In_opt_ IServiceProvider *pSP, _Outptr_ IInternetZoneManager **ppZM, DWORD dwReserved);


// Security manager CLSID's
EXTERN_C const IID CLSID_InternetSecurityManager;  
EXTERN_C const IID CLSID_InternetZoneManager;  
#if (_WIN32_IE >= _WIN32_IE_IE60SP2)
EXTERN_C const IID CLSID_PersistentZoneIdentifier;  
#endif //(_WIN32_IE >= _WIN32_IE_IE60SP2)
// This service is used for delegation support on the Security Manager interface
#define SID_SInternetSecurityManager         IID_IInternetSecurityManager

#if (_WIN32_IE >= _WIN32_IE_IE60SP2)
#define SID_SInternetSecurityManagerEx         IID_IInternetSecurityManagerEx
#endif //(_WIN32_IE >= _WIN32_IE_IE60SP2)

#if (_WIN32_IE >= _WIN32_IE_IE70)
#define SID_SInternetSecurityManagerEx2         IID_IInternetSecurityManagerEx2
#endif //(_WIN32_IE >= _WIN32_IE_IE70)

#define SID_SInternetHostSecurityManager     IID_IInternetHostSecurityManager

#ifndef _LPINTERNETSECURITYMGRSITE_DEFINED
#define _LPINTERNETSECURITYMGRSITE_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0037_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0037_v0_0_s_ifspec;

#ifndef __IInternetSecurityMgrSite_INTERFACE_DEFINED__
#define __IInternetSecurityMgrSite_INTERFACE_DEFINED__

/* interface IInternetSecurityMgrSite */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IInternetSecurityMgrSite;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9ed-baf9-11ce-8c82-00aa004ba90b")
    IInternetSecurityMgrSite : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetWindow( 
            /* [out] */ HWND *phwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnableModeless( 
            /* [in] */ BOOL fEnable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetSecurityMgrSiteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternetSecurityMgrSite * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternetSecurityMgrSite * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternetSecurityMgrSite * This);
        
        DECLSPEC_XFGVIRT(IInternetSecurityMgrSite, GetWindow)
        HRESULT ( STDMETHODCALLTYPE *GetWindow )( 
            IInternetSecurityMgrSite * This,
            /* [out] */ HWND *phwnd);
        
        DECLSPEC_XFGVIRT(IInternetSecurityMgrSite, EnableModeless)
        HRESULT ( STDMETHODCALLTYPE *EnableModeless )( 
            IInternetSecurityMgrSite * This,
            /* [in] */ BOOL fEnable);
        
        END_INTERFACE
    } IInternetSecurityMgrSiteVtbl;

    interface IInternetSecurityMgrSite
    {
        CONST_VTBL struct IInternetSecurityMgrSiteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetSecurityMgrSite_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetSecurityMgrSite_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetSecurityMgrSite_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetSecurityMgrSite_GetWindow(This,phwnd)	\
    ( (This)->lpVtbl -> GetWindow(This,phwnd) ) 

#define IInternetSecurityMgrSite_EnableModeless(This,fEnable)	\
    ( (This)->lpVtbl -> EnableModeless(This,fEnable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetSecurityMgrSite_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0038 */
/* [local] */ 

#endif
#ifndef _LPINTERNETSECURITYMANANGER_DEFINED
#define _LPINTERNETSECURITYMANANGER_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0038_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0038_v0_0_s_ifspec;

#ifndef __IInternetSecurityManager_INTERFACE_DEFINED__
#define __IInternetSecurityManager_INTERFACE_DEFINED__

/* interface IInternetSecurityManager */
/* [object][unique][helpstring][uuid] */ 

#define MUTZ_NOSAVEDFILECHECK        0x00000001 // don't check file: for saved file comment
#define MUTZ_ISFILE                  0x00000002 // Assume URL if File, url does not need file://
#define MUTZ_ACCEPT_WILDCARD_SCHEME  0x00000080 // Accept a wildcard scheme
#define MUTZ_ENFORCERESTRICTED       0x00000100 // enforce restricted zone independent of URL
#define MUTZ_RESERVED                0x00000200 // This is same as PUAF_NOSAVEDFILECHECK. However we already have MUTZ_NOSAVEDFILECHECK for this.
#define MUTZ_REQUIRESAVEDFILECHECK   0x00000400 // always check the file for MOTW (overriding FEATURE_UNC_SAVEDFILECHECK)
#define MUTZ_DONT_UNESCAPE           0x00000800 // Do not unescape the url
#define MUTZ_DONT_USE_CACHE          0x00001000 // Do not check the cache
#define MUTZ_FORCE_INTRANET_FLAGS    0x00002000 // Force the intranet flags to be active
#define MUTZ_IGNORE_ZONE_MAPPINGS    0x00004000 // Don't look up the Zone Mappings
// MapUrlToZone returns the zone index given a URL
#define MAX_SIZE_SECURITY_ID 512 // bytes


typedef /* [public] */ 
enum __MIDL_IInternetSecurityManager_0001
    {
        PUAF_DEFAULT	= 0,
        PUAF_NOUI	= 0x1,
        PUAF_ISFILE	= 0x2,
        PUAF_WARN_IF_DENIED	= 0x4,
        PUAF_FORCEUI_FOREGROUND	= 0x8,
        PUAF_CHECK_TIFS	= 0x10,
        PUAF_DONTCHECKBOXINDIALOG	= 0x20,
        PUAF_TRUSTED	= 0x40,
        PUAF_ACCEPT_WILDCARD_SCHEME	= 0x80,
        PUAF_ENFORCERESTRICTED	= 0x100,
        PUAF_NOSAVEDFILECHECK	= 0x200,
        PUAF_REQUIRESAVEDFILECHECK	= 0x400,
        PUAF_DONT_USE_CACHE	= 0x1000,
        PUAF_RESERVED1	= 0x2000,
        PUAF_RESERVED2	= 0x4000,
        PUAF_LMZ_UNLOCKED	= 0x10000,
        PUAF_LMZ_LOCKED	= 0x20000,
        PUAF_DEFAULTZONEPOL	= 0x40000,
        PUAF_NPL_USE_LOCKED_IF_RESTRICTED	= 0x80000,
        PUAF_NOUIIFLOCKED	= 0x100000,
        PUAF_DRAGPROTOCOLCHECK	= 0x200000
    } 	PUAF;

typedef /* [public] */ 
enum __MIDL_IInternetSecurityManager_0002
    {
        PUAFOUT_DEFAULT	= 0,
        PUAFOUT_ISLOCKZONEPOLICY	= 0x1
    } 	PUAFOUT;

// Note that for the below function, the semantics of the 'pwszUrl',
// 'pContext', and 'cbContext' parameters depend on the specific
// URLACTION_* enum value that is passed for 'dwAction'. For example,
// when 'dwAction' is URLACTION_HTML_MIXED_CONTENT, 'pwszUrl' will be
// the target URL of the resource, 'pContext' will be the containing
// document's IUri* cast to BYTE*, and 'cbContext' will be sizeof(Uri*).
// When 'dwAction' is URLACTION_CROSS_DOMAIN_DATA, 'pwszUrl' will be
// the source URL, 'pContext' will be the target PCWSTR cast to BYTE*,
// and 'cbContext' will be the size of the string including its null
// terminator. Implementers should use the value of 'dwAction' to
// correctly interpret the 'pswzUrl', 'pContext' and 'cbContext' for
// each action type.
// This is the wrapper function that most clients will use.
// It figures out the current Policy for the passed in Action,
// and puts up UI if the current Policy indicates that the user
// should be queried. It returns back the Policy which the caller
// will use to determine if the action should be allowed
// This is the wrapper function to conveniently read a custom policy.
typedef /* [public] */ 
enum __MIDL_IInternetSecurityManager_0003
    {
        SZM_CREATE	= 0,
        SZM_DELETE	= 0x1
    } 	SZM_FLAGS;

// SetZoneMapping
//    lpszPattern: string denoting a URL pattern
//        Examples of valid patterns:   
//            *://*.msn.com             
//            http://*.sony.co.jp       
//            *://et.msn.com            
//            ftp://157.54.23.41/       
//            https://localsvr          
//            file:\localsvr\share     
//            *://157.54.100-200.*      
//        Examples of invalid patterns: 
//            http://*.lcs.mit.edu      
//            ftp://*                   
//    dwFlags: SZM_FLAGS values         

EXTERN_C const IID IID_IInternetSecurityManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9ee-baf9-11ce-8c82-00aa004ba90b")
    IInternetSecurityManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetSecuritySite( 
            /* [unique][in] */ __RPC__in_opt IInternetSecurityMgrSite *pSite) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSecuritySite( 
            /* [out] */ __RPC__deref_out_opt IInternetSecurityMgrSite **ppSite) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MapUrlToZone( 
            /* [in] */ __RPC__in LPCWSTR pwszUrl,
            /* [out] */ __RPC__out DWORD *pdwZone,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSecurityId( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszUrl,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_(MAX_SIZE_SECURITY_ID, *pcbSecurityId)  BYTE *pbSecurityId,
            /* [annotation][out][in] */ 
            _Inout_ _At_(*pcbSecurityId, _In_range_(>= , MAX_SIZE_SECURITY_ID) _Out_range_(0, MAX_SIZE_SECURITY_ID))  DWORD *pcbSecurityId,
            /* [annotation][in] */ 
            _In_  DWORD_PTR dwReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessUrlAction( 
            /* [in] */ __RPC__in LPCWSTR pwszUrl,
            /* [in] */ DWORD dwAction,
            /* [size_is][out] */ __RPC__out_ecount_full(cbPolicy) BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [unique][in] */ __RPC__in_opt BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryCustomPolicy( 
            /* [in] */ __RPC__in LPCWSTR pwszUrl,
            /* [in] */ __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbPolicy) BYTE **ppPolicy,
            /* [out] */ __RPC__out DWORD *pcbPolicy,
            /* [in] */ __RPC__in BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD dwReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetZoneMapping( 
            /* [in] */ DWORD dwZone,
            /* [in] */ __RPC__in LPCWSTR lpszPattern,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetZoneMappings( 
            /* [in] */ DWORD dwZone,
            /* [out] */ __RPC__deref_out_opt IEnumString **ppenumString,
            /* [in] */ DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetSecurityManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInternetSecurityManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInternetSecurityManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInternetSecurityManager * This);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, SetSecuritySite)
        HRESULT ( STDMETHODCALLTYPE *SetSecuritySite )( 
            __RPC__in IInternetSecurityManager * This,
            /* [unique][in] */ __RPC__in_opt IInternetSecurityMgrSite *pSite);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, GetSecuritySite)
        HRESULT ( STDMETHODCALLTYPE *GetSecuritySite )( 
            __RPC__in IInternetSecurityManager * This,
            /* [out] */ __RPC__deref_out_opt IInternetSecurityMgrSite **ppSite);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, MapUrlToZone)
        HRESULT ( STDMETHODCALLTYPE *MapUrlToZone )( 
            __RPC__in IInternetSecurityManager * This,
            /* [in] */ __RPC__in LPCWSTR pwszUrl,
            /* [out] */ __RPC__out DWORD *pdwZone,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, GetSecurityId)
        HRESULT ( STDMETHODCALLTYPE *GetSecurityId )( 
            __RPC__in IInternetSecurityManager * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszUrl,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_(MAX_SIZE_SECURITY_ID, *pcbSecurityId)  BYTE *pbSecurityId,
            /* [annotation][out][in] */ 
            _Inout_ _At_(*pcbSecurityId, _In_range_(>= , MAX_SIZE_SECURITY_ID) _Out_range_(0, MAX_SIZE_SECURITY_ID))  DWORD *pcbSecurityId,
            /* [annotation][in] */ 
            _In_  DWORD_PTR dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, ProcessUrlAction)
        HRESULT ( STDMETHODCALLTYPE *ProcessUrlAction )( 
            __RPC__in IInternetSecurityManager * This,
            /* [in] */ __RPC__in LPCWSTR pwszUrl,
            /* [in] */ DWORD dwAction,
            /* [size_is][out] */ __RPC__out_ecount_full(cbPolicy) BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [unique][in] */ __RPC__in_opt BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, QueryCustomPolicy)
        HRESULT ( STDMETHODCALLTYPE *QueryCustomPolicy )( 
            __RPC__in IInternetSecurityManager * This,
            /* [in] */ __RPC__in LPCWSTR pwszUrl,
            /* [in] */ __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbPolicy) BYTE **ppPolicy,
            /* [out] */ __RPC__out DWORD *pcbPolicy,
            /* [in] */ __RPC__in BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, SetZoneMapping)
        HRESULT ( STDMETHODCALLTYPE *SetZoneMapping )( 
            __RPC__in IInternetSecurityManager * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ __RPC__in LPCWSTR lpszPattern,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, GetZoneMappings)
        HRESULT ( STDMETHODCALLTYPE *GetZoneMappings )( 
            __RPC__in IInternetSecurityManager * This,
            /* [in] */ DWORD dwZone,
            /* [out] */ __RPC__deref_out_opt IEnumString **ppenumString,
            /* [in] */ DWORD dwFlags);
        
        END_INTERFACE
    } IInternetSecurityManagerVtbl;

    interface IInternetSecurityManager
    {
        CONST_VTBL struct IInternetSecurityManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetSecurityManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetSecurityManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetSecurityManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetSecurityManager_SetSecuritySite(This,pSite)	\
    ( (This)->lpVtbl -> SetSecuritySite(This,pSite) ) 

#define IInternetSecurityManager_GetSecuritySite(This,ppSite)	\
    ( (This)->lpVtbl -> GetSecuritySite(This,ppSite) ) 

#define IInternetSecurityManager_MapUrlToZone(This,pwszUrl,pdwZone,dwFlags)	\
    ( (This)->lpVtbl -> MapUrlToZone(This,pwszUrl,pdwZone,dwFlags) ) 

#define IInternetSecurityManager_GetSecurityId(This,pwszUrl,pbSecurityId,pcbSecurityId,dwReserved)	\
    ( (This)->lpVtbl -> GetSecurityId(This,pwszUrl,pbSecurityId,pcbSecurityId,dwReserved) ) 

#define IInternetSecurityManager_ProcessUrlAction(This,pwszUrl,dwAction,pPolicy,cbPolicy,pContext,cbContext,dwFlags,dwReserved)	\
    ( (This)->lpVtbl -> ProcessUrlAction(This,pwszUrl,dwAction,pPolicy,cbPolicy,pContext,cbContext,dwFlags,dwReserved) ) 

#define IInternetSecurityManager_QueryCustomPolicy(This,pwszUrl,guidKey,ppPolicy,pcbPolicy,pContext,cbContext,dwReserved)	\
    ( (This)->lpVtbl -> QueryCustomPolicy(This,pwszUrl,guidKey,ppPolicy,pcbPolicy,pContext,cbContext,dwReserved) ) 

#define IInternetSecurityManager_SetZoneMapping(This,dwZone,lpszPattern,dwFlags)	\
    ( (This)->lpVtbl -> SetZoneMapping(This,dwZone,lpszPattern,dwFlags) ) 

#define IInternetSecurityManager_GetZoneMappings(This,dwZone,ppenumString,dwFlags)	\
    ( (This)->lpVtbl -> GetZoneMappings(This,dwZone,ppenumString,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetSecurityManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0039 */
/* [local] */ 

#endif
#if (_WIN32_IE >= _WIN32_IE_IE60SP2)
#ifndef _LPINTERNETSECURITYMANANGEREX_DEFINED
#define _LPINTERNETSECURITYMANANGEREX_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0039_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0039_v0_0_s_ifspec;

#ifndef __IInternetSecurityManagerEx_INTERFACE_DEFINED__
#define __IInternetSecurityManagerEx_INTERFACE_DEFINED__

/* interface IInternetSecurityManagerEx */
/* [object][unique][helpstring][uuid] */ 

// Please see notes on IInternetSecurityManager::ProcessUrlAction
// This is the wrapper function that most clients will use.
// It figures out the current Policy for the passed in Action,
// and puts up UI if the current Policy indicates that the user
// should be queried. It returns back the Policy which the caller
// will use to determine if the action should be allowed

EXTERN_C const IID IID_IInternetSecurityManagerEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F164EDF1-CC7C-4f0d-9A94-34222625C393")
    IInternetSecurityManagerEx : public IInternetSecurityManager
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ProcessUrlActionEx( 
            /* [in] */ __RPC__in LPCWSTR pwszUrl,
            /* [in] */ DWORD dwAction,
            /* [size_is][out] */ __RPC__out_ecount_full(cbPolicy) BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ __RPC__in BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwReserved,
            /* [out] */ __RPC__out DWORD *pdwOutFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetSecurityManagerExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInternetSecurityManagerEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInternetSecurityManagerEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInternetSecurityManagerEx * This);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, SetSecuritySite)
        HRESULT ( STDMETHODCALLTYPE *SetSecuritySite )( 
            __RPC__in IInternetSecurityManagerEx * This,
            /* [unique][in] */ __RPC__in_opt IInternetSecurityMgrSite *pSite);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, GetSecuritySite)
        HRESULT ( STDMETHODCALLTYPE *GetSecuritySite )( 
            __RPC__in IInternetSecurityManagerEx * This,
            /* [out] */ __RPC__deref_out_opt IInternetSecurityMgrSite **ppSite);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, MapUrlToZone)
        HRESULT ( STDMETHODCALLTYPE *MapUrlToZone )( 
            __RPC__in IInternetSecurityManagerEx * This,
            /* [in] */ __RPC__in LPCWSTR pwszUrl,
            /* [out] */ __RPC__out DWORD *pdwZone,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, GetSecurityId)
        HRESULT ( STDMETHODCALLTYPE *GetSecurityId )( 
            __RPC__in IInternetSecurityManagerEx * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszUrl,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_(MAX_SIZE_SECURITY_ID, *pcbSecurityId)  BYTE *pbSecurityId,
            /* [annotation][out][in] */ 
            _Inout_ _At_(*pcbSecurityId, _In_range_(>= , MAX_SIZE_SECURITY_ID) _Out_range_(0, MAX_SIZE_SECURITY_ID))  DWORD *pcbSecurityId,
            /* [annotation][in] */ 
            _In_  DWORD_PTR dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, ProcessUrlAction)
        HRESULT ( STDMETHODCALLTYPE *ProcessUrlAction )( 
            __RPC__in IInternetSecurityManagerEx * This,
            /* [in] */ __RPC__in LPCWSTR pwszUrl,
            /* [in] */ DWORD dwAction,
            /* [size_is][out] */ __RPC__out_ecount_full(cbPolicy) BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [unique][in] */ __RPC__in_opt BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, QueryCustomPolicy)
        HRESULT ( STDMETHODCALLTYPE *QueryCustomPolicy )( 
            __RPC__in IInternetSecurityManagerEx * This,
            /* [in] */ __RPC__in LPCWSTR pwszUrl,
            /* [in] */ __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbPolicy) BYTE **ppPolicy,
            /* [out] */ __RPC__out DWORD *pcbPolicy,
            /* [in] */ __RPC__in BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, SetZoneMapping)
        HRESULT ( STDMETHODCALLTYPE *SetZoneMapping )( 
            __RPC__in IInternetSecurityManagerEx * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ __RPC__in LPCWSTR lpszPattern,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, GetZoneMappings)
        HRESULT ( STDMETHODCALLTYPE *GetZoneMappings )( 
            __RPC__in IInternetSecurityManagerEx * This,
            /* [in] */ DWORD dwZone,
            /* [out] */ __RPC__deref_out_opt IEnumString **ppenumString,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManagerEx, ProcessUrlActionEx)
        HRESULT ( STDMETHODCALLTYPE *ProcessUrlActionEx )( 
            __RPC__in IInternetSecurityManagerEx * This,
            /* [in] */ __RPC__in LPCWSTR pwszUrl,
            /* [in] */ DWORD dwAction,
            /* [size_is][out] */ __RPC__out_ecount_full(cbPolicy) BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ __RPC__in BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwReserved,
            /* [out] */ __RPC__out DWORD *pdwOutFlags);
        
        END_INTERFACE
    } IInternetSecurityManagerExVtbl;

    interface IInternetSecurityManagerEx
    {
        CONST_VTBL struct IInternetSecurityManagerExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetSecurityManagerEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetSecurityManagerEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetSecurityManagerEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetSecurityManagerEx_SetSecuritySite(This,pSite)	\
    ( (This)->lpVtbl -> SetSecuritySite(This,pSite) ) 

#define IInternetSecurityManagerEx_GetSecuritySite(This,ppSite)	\
    ( (This)->lpVtbl -> GetSecuritySite(This,ppSite) ) 

#define IInternetSecurityManagerEx_MapUrlToZone(This,pwszUrl,pdwZone,dwFlags)	\
    ( (This)->lpVtbl -> MapUrlToZone(This,pwszUrl,pdwZone,dwFlags) ) 

#define IInternetSecurityManagerEx_GetSecurityId(This,pwszUrl,pbSecurityId,pcbSecurityId,dwReserved)	\
    ( (This)->lpVtbl -> GetSecurityId(This,pwszUrl,pbSecurityId,pcbSecurityId,dwReserved) ) 

#define IInternetSecurityManagerEx_ProcessUrlAction(This,pwszUrl,dwAction,pPolicy,cbPolicy,pContext,cbContext,dwFlags,dwReserved)	\
    ( (This)->lpVtbl -> ProcessUrlAction(This,pwszUrl,dwAction,pPolicy,cbPolicy,pContext,cbContext,dwFlags,dwReserved) ) 

#define IInternetSecurityManagerEx_QueryCustomPolicy(This,pwszUrl,guidKey,ppPolicy,pcbPolicy,pContext,cbContext,dwReserved)	\
    ( (This)->lpVtbl -> QueryCustomPolicy(This,pwszUrl,guidKey,ppPolicy,pcbPolicy,pContext,cbContext,dwReserved) ) 

#define IInternetSecurityManagerEx_SetZoneMapping(This,dwZone,lpszPattern,dwFlags)	\
    ( (This)->lpVtbl -> SetZoneMapping(This,dwZone,lpszPattern,dwFlags) ) 

#define IInternetSecurityManagerEx_GetZoneMappings(This,dwZone,ppenumString,dwFlags)	\
    ( (This)->lpVtbl -> GetZoneMappings(This,dwZone,ppenumString,dwFlags) ) 


#define IInternetSecurityManagerEx_ProcessUrlActionEx(This,pwszUrl,dwAction,pPolicy,cbPolicy,pContext,cbContext,dwFlags,dwReserved,pdwOutFlags)	\
    ( (This)->lpVtbl -> ProcessUrlActionEx(This,pwszUrl,dwAction,pPolicy,cbPolicy,pContext,cbContext,dwFlags,dwReserved,pdwOutFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetSecurityManagerEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0040 */
/* [local] */ 

#endif
#endif //(_WIN32_IE >= _WIN32_IE_IE60SP2)
#if (_WIN32_IE >= _WIN32_IE_IE70)
#ifndef _LPINTERNETSECURITYMANANGEREx2_DEFINED
#define _LPINTERNETSECURITYMANANGEREx2_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0040_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0040_v0_0_s_ifspec;

#ifndef __IInternetSecurityManagerEx2_INTERFACE_DEFINED__
#define __IInternetSecurityManagerEx2_INTERFACE_DEFINED__

/* interface IInternetSecurityManagerEx2 */
/* [object][unique][helpstring][uuid] */ 


// Please see notes on IInternetSecurityManager::ProcessUrlAction


EXTERN_C const IID IID_IInternetSecurityManagerEx2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F1E50292-A795-4117-8E09-2B560A72AC60")
    IInternetSecurityManagerEx2 : public IInternetSecurityManagerEx
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MapUrlToZoneEx2( 
            /* [annotation][in] */ 
            _In_  IUri *pUri,
            /* [out] */ __RPC__out DWORD *pdwZone,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppwszMappedUrl,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwOutFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessUrlActionEx2( 
            /* [annotation][in] */ 
            _In_  IUri *pUri,
            /* [in] */ DWORD dwAction,
            /* [size_is][out] */ __RPC__out_ecount_full(cbPolicy) BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [unique][in] */ __RPC__in_opt BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD_PTR dwReserved,
            /* [out] */ __RPC__out DWORD *pdwOutFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSecurityIdEx2( 
            /* [annotation][in] */ 
            _In_  IUri *pUri,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_(MAX_SIZE_SECURITY_ID, *pcbSecurityId)  BYTE *pbSecurityId,
            /* [annotation][out][in] */ 
            _Inout_ _At_(*pcbSecurityId, _In_range_(>= , MAX_SIZE_SECURITY_ID) _Out_range_(0, MAX_SIZE_SECURITY_ID))  DWORD *pcbSecurityId,
            /* [annotation][in] */ 
            _In_  DWORD_PTR dwReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryCustomPolicyEx2( 
            /* [annotation][in] */ 
            _In_  IUri *pUri,
            /* [in] */ __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbPolicy) BYTE **ppPolicy,
            /* [out] */ __RPC__out DWORD *pcbPolicy,
            /* [in] */ __RPC__in BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD_PTR dwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetSecurityManagerEx2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInternetSecurityManagerEx2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInternetSecurityManagerEx2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInternetSecurityManagerEx2 * This);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, SetSecuritySite)
        HRESULT ( STDMETHODCALLTYPE *SetSecuritySite )( 
            __RPC__in IInternetSecurityManagerEx2 * This,
            /* [unique][in] */ __RPC__in_opt IInternetSecurityMgrSite *pSite);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, GetSecuritySite)
        HRESULT ( STDMETHODCALLTYPE *GetSecuritySite )( 
            __RPC__in IInternetSecurityManagerEx2 * This,
            /* [out] */ __RPC__deref_out_opt IInternetSecurityMgrSite **ppSite);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, MapUrlToZone)
        HRESULT ( STDMETHODCALLTYPE *MapUrlToZone )( 
            __RPC__in IInternetSecurityManagerEx2 * This,
            /* [in] */ __RPC__in LPCWSTR pwszUrl,
            /* [out] */ __RPC__out DWORD *pdwZone,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, GetSecurityId)
        HRESULT ( STDMETHODCALLTYPE *GetSecurityId )( 
            __RPC__in IInternetSecurityManagerEx2 * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszUrl,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_(MAX_SIZE_SECURITY_ID, *pcbSecurityId)  BYTE *pbSecurityId,
            /* [annotation][out][in] */ 
            _Inout_ _At_(*pcbSecurityId, _In_range_(>= , MAX_SIZE_SECURITY_ID) _Out_range_(0, MAX_SIZE_SECURITY_ID))  DWORD *pcbSecurityId,
            /* [annotation][in] */ 
            _In_  DWORD_PTR dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, ProcessUrlAction)
        HRESULT ( STDMETHODCALLTYPE *ProcessUrlAction )( 
            __RPC__in IInternetSecurityManagerEx2 * This,
            /* [in] */ __RPC__in LPCWSTR pwszUrl,
            /* [in] */ DWORD dwAction,
            /* [size_is][out] */ __RPC__out_ecount_full(cbPolicy) BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [unique][in] */ __RPC__in_opt BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, QueryCustomPolicy)
        HRESULT ( STDMETHODCALLTYPE *QueryCustomPolicy )( 
            __RPC__in IInternetSecurityManagerEx2 * This,
            /* [in] */ __RPC__in LPCWSTR pwszUrl,
            /* [in] */ __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbPolicy) BYTE **ppPolicy,
            /* [out] */ __RPC__out DWORD *pcbPolicy,
            /* [in] */ __RPC__in BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, SetZoneMapping)
        HRESULT ( STDMETHODCALLTYPE *SetZoneMapping )( 
            __RPC__in IInternetSecurityManagerEx2 * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ __RPC__in LPCWSTR lpszPattern,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManager, GetZoneMappings)
        HRESULT ( STDMETHODCALLTYPE *GetZoneMappings )( 
            __RPC__in IInternetSecurityManagerEx2 * This,
            /* [in] */ DWORD dwZone,
            /* [out] */ __RPC__deref_out_opt IEnumString **ppenumString,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManagerEx, ProcessUrlActionEx)
        HRESULT ( STDMETHODCALLTYPE *ProcessUrlActionEx )( 
            __RPC__in IInternetSecurityManagerEx2 * This,
            /* [in] */ __RPC__in LPCWSTR pwszUrl,
            /* [in] */ DWORD dwAction,
            /* [size_is][out] */ __RPC__out_ecount_full(cbPolicy) BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ __RPC__in BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwReserved,
            /* [out] */ __RPC__out DWORD *pdwOutFlags);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManagerEx2, MapUrlToZoneEx2)
        HRESULT ( STDMETHODCALLTYPE *MapUrlToZoneEx2 )( 
            __RPC__in IInternetSecurityManagerEx2 * This,
            /* [annotation][in] */ 
            _In_  IUri *pUri,
            /* [out] */ __RPC__out DWORD *pdwZone,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Outptr_opt_  LPWSTR *ppwszMappedUrl,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwOutFlags);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManagerEx2, ProcessUrlActionEx2)
        HRESULT ( STDMETHODCALLTYPE *ProcessUrlActionEx2 )( 
            __RPC__in IInternetSecurityManagerEx2 * This,
            /* [annotation][in] */ 
            _In_  IUri *pUri,
            /* [in] */ DWORD dwAction,
            /* [size_is][out] */ __RPC__out_ecount_full(cbPolicy) BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [unique][in] */ __RPC__in_opt BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD_PTR dwReserved,
            /* [out] */ __RPC__out DWORD *pdwOutFlags);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManagerEx2, GetSecurityIdEx2)
        HRESULT ( STDMETHODCALLTYPE *GetSecurityIdEx2 )( 
            __RPC__in IInternetSecurityManagerEx2 * This,
            /* [annotation][in] */ 
            _In_  IUri *pUri,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_(MAX_SIZE_SECURITY_ID, *pcbSecurityId)  BYTE *pbSecurityId,
            /* [annotation][out][in] */ 
            _Inout_ _At_(*pcbSecurityId, _In_range_(>= , MAX_SIZE_SECURITY_ID) _Out_range_(0, MAX_SIZE_SECURITY_ID))  DWORD *pcbSecurityId,
            /* [annotation][in] */ 
            _In_  DWORD_PTR dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetSecurityManagerEx2, QueryCustomPolicyEx2)
        HRESULT ( STDMETHODCALLTYPE *QueryCustomPolicyEx2 )( 
            __RPC__in IInternetSecurityManagerEx2 * This,
            /* [annotation][in] */ 
            _In_  IUri *pUri,
            /* [in] */ __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbPolicy) BYTE **ppPolicy,
            /* [out] */ __RPC__out DWORD *pcbPolicy,
            /* [in] */ __RPC__in BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD_PTR dwReserved);
        
        END_INTERFACE
    } IInternetSecurityManagerEx2Vtbl;

    interface IInternetSecurityManagerEx2
    {
        CONST_VTBL struct IInternetSecurityManagerEx2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetSecurityManagerEx2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetSecurityManagerEx2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetSecurityManagerEx2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetSecurityManagerEx2_SetSecuritySite(This,pSite)	\
    ( (This)->lpVtbl -> SetSecuritySite(This,pSite) ) 

#define IInternetSecurityManagerEx2_GetSecuritySite(This,ppSite)	\
    ( (This)->lpVtbl -> GetSecuritySite(This,ppSite) ) 

#define IInternetSecurityManagerEx2_MapUrlToZone(This,pwszUrl,pdwZone,dwFlags)	\
    ( (This)->lpVtbl -> MapUrlToZone(This,pwszUrl,pdwZone,dwFlags) ) 

#define IInternetSecurityManagerEx2_GetSecurityId(This,pwszUrl,pbSecurityId,pcbSecurityId,dwReserved)	\
    ( (This)->lpVtbl -> GetSecurityId(This,pwszUrl,pbSecurityId,pcbSecurityId,dwReserved) ) 

#define IInternetSecurityManagerEx2_ProcessUrlAction(This,pwszUrl,dwAction,pPolicy,cbPolicy,pContext,cbContext,dwFlags,dwReserved)	\
    ( (This)->lpVtbl -> ProcessUrlAction(This,pwszUrl,dwAction,pPolicy,cbPolicy,pContext,cbContext,dwFlags,dwReserved) ) 

#define IInternetSecurityManagerEx2_QueryCustomPolicy(This,pwszUrl,guidKey,ppPolicy,pcbPolicy,pContext,cbContext,dwReserved)	\
    ( (This)->lpVtbl -> QueryCustomPolicy(This,pwszUrl,guidKey,ppPolicy,pcbPolicy,pContext,cbContext,dwReserved) ) 

#define IInternetSecurityManagerEx2_SetZoneMapping(This,dwZone,lpszPattern,dwFlags)	\
    ( (This)->lpVtbl -> SetZoneMapping(This,dwZone,lpszPattern,dwFlags) ) 

#define IInternetSecurityManagerEx2_GetZoneMappings(This,dwZone,ppenumString,dwFlags)	\
    ( (This)->lpVtbl -> GetZoneMappings(This,dwZone,ppenumString,dwFlags) ) 


#define IInternetSecurityManagerEx2_ProcessUrlActionEx(This,pwszUrl,dwAction,pPolicy,cbPolicy,pContext,cbContext,dwFlags,dwReserved,pdwOutFlags)	\
    ( (This)->lpVtbl -> ProcessUrlActionEx(This,pwszUrl,dwAction,pPolicy,cbPolicy,pContext,cbContext,dwFlags,dwReserved,pdwOutFlags) ) 


#define IInternetSecurityManagerEx2_MapUrlToZoneEx2(This,pUri,pdwZone,dwFlags,ppwszMappedUrl,pdwOutFlags)	\
    ( (This)->lpVtbl -> MapUrlToZoneEx2(This,pUri,pdwZone,dwFlags,ppwszMappedUrl,pdwOutFlags) ) 

#define IInternetSecurityManagerEx2_ProcessUrlActionEx2(This,pUri,dwAction,pPolicy,cbPolicy,pContext,cbContext,dwFlags,dwReserved,pdwOutFlags)	\
    ( (This)->lpVtbl -> ProcessUrlActionEx2(This,pUri,dwAction,pPolicy,cbPolicy,pContext,cbContext,dwFlags,dwReserved,pdwOutFlags) ) 

#define IInternetSecurityManagerEx2_GetSecurityIdEx2(This,pUri,pbSecurityId,pcbSecurityId,dwReserved)	\
    ( (This)->lpVtbl -> GetSecurityIdEx2(This,pUri,pbSecurityId,pcbSecurityId,dwReserved) ) 

#define IInternetSecurityManagerEx2_QueryCustomPolicyEx2(This,pUri,guidKey,ppPolicy,pcbPolicy,pContext,cbContext,dwReserved)	\
    ( (This)->lpVtbl -> QueryCustomPolicyEx2(This,pUri,guidKey,ppPolicy,pcbPolicy,pContext,cbContext,dwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetSecurityManagerEx2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0041 */
/* [local] */ 

#endif
#endif //(_WIN32_IE >= _WIN32_IE_IE70)
#if (_WIN32_IE >= _WIN32_IE_IE60SP2)


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0041_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0041_v0_0_s_ifspec;

#ifndef __IZoneIdentifier_INTERFACE_DEFINED__
#define __IZoneIdentifier_INTERFACE_DEFINED__

/* interface IZoneIdentifier */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IZoneIdentifier;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("cd45f185-1b21-48e2-967b-ead743a8914e")
    IZoneIdentifier : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetId( 
            /* [out] */ __RPC__out DWORD *pdwZone) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetId( 
            /* [in] */ DWORD dwZone) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IZoneIdentifierVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IZoneIdentifier * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IZoneIdentifier * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IZoneIdentifier * This);
        
        DECLSPEC_XFGVIRT(IZoneIdentifier, GetId)
        HRESULT ( STDMETHODCALLTYPE *GetId )( 
            __RPC__in IZoneIdentifier * This,
            /* [out] */ __RPC__out DWORD *pdwZone);
        
        DECLSPEC_XFGVIRT(IZoneIdentifier, SetId)
        HRESULT ( STDMETHODCALLTYPE *SetId )( 
            __RPC__in IZoneIdentifier * This,
            /* [in] */ DWORD dwZone);
        
        DECLSPEC_XFGVIRT(IZoneIdentifier, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IZoneIdentifier * This);
        
        END_INTERFACE
    } IZoneIdentifierVtbl;

    interface IZoneIdentifier
    {
        CONST_VTBL struct IZoneIdentifierVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IZoneIdentifier_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IZoneIdentifier_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IZoneIdentifier_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IZoneIdentifier_GetId(This,pdwZone)	\
    ( (This)->lpVtbl -> GetId(This,pdwZone) ) 

#define IZoneIdentifier_SetId(This,dwZone)	\
    ( (This)->lpVtbl -> SetId(This,dwZone) ) 

#define IZoneIdentifier_Remove(This)	\
    ( (This)->lpVtbl -> Remove(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IZoneIdentifier_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0042 */
/* [local] */ 

#endif //(_WIN32_IE >= _WIN32_IE_IE60SP2)
#if (_WIN32_IE >= _WIN32_IE_WIN10)


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0042_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0042_v0_0_s_ifspec;

#ifndef __IZoneIdentifier2_INTERFACE_DEFINED__
#define __IZoneIdentifier2_INTERFACE_DEFINED__

/* interface IZoneIdentifier2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IZoneIdentifier2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EB5E760C-09EF-45C0-B510-70830CE31E6A")
    IZoneIdentifier2 : public IZoneIdentifier
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetLastWriterPackageFamilyName( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *packageFamilyName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLastWriterPackageFamilyName( 
            /* [in] */ __RPC__in LPCWSTR packageFamilyName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveLastWriterPackageFamilyName( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAppZoneId( 
            /* [out] */ __RPC__out DWORD *zone) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAppZoneId( 
            /* [in] */ DWORD zone) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAppZoneId( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IZoneIdentifier2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IZoneIdentifier2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IZoneIdentifier2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IZoneIdentifier2 * This);
        
        DECLSPEC_XFGVIRT(IZoneIdentifier, GetId)
        HRESULT ( STDMETHODCALLTYPE *GetId )( 
            __RPC__in IZoneIdentifier2 * This,
            /* [out] */ __RPC__out DWORD *pdwZone);
        
        DECLSPEC_XFGVIRT(IZoneIdentifier, SetId)
        HRESULT ( STDMETHODCALLTYPE *SetId )( 
            __RPC__in IZoneIdentifier2 * This,
            /* [in] */ DWORD dwZone);
        
        DECLSPEC_XFGVIRT(IZoneIdentifier, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IZoneIdentifier2 * This);
        
        DECLSPEC_XFGVIRT(IZoneIdentifier2, GetLastWriterPackageFamilyName)
        HRESULT ( STDMETHODCALLTYPE *GetLastWriterPackageFamilyName )( 
            __RPC__in IZoneIdentifier2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *packageFamilyName);
        
        DECLSPEC_XFGVIRT(IZoneIdentifier2, SetLastWriterPackageFamilyName)
        HRESULT ( STDMETHODCALLTYPE *SetLastWriterPackageFamilyName )( 
            __RPC__in IZoneIdentifier2 * This,
            /* [in] */ __RPC__in LPCWSTR packageFamilyName);
        
        DECLSPEC_XFGVIRT(IZoneIdentifier2, RemoveLastWriterPackageFamilyName)
        HRESULT ( STDMETHODCALLTYPE *RemoveLastWriterPackageFamilyName )( 
            __RPC__in IZoneIdentifier2 * This);
        
        DECLSPEC_XFGVIRT(IZoneIdentifier2, GetAppZoneId)
        HRESULT ( STDMETHODCALLTYPE *GetAppZoneId )( 
            __RPC__in IZoneIdentifier2 * This,
            /* [out] */ __RPC__out DWORD *zone);
        
        DECLSPEC_XFGVIRT(IZoneIdentifier2, SetAppZoneId)
        HRESULT ( STDMETHODCALLTYPE *SetAppZoneId )( 
            __RPC__in IZoneIdentifier2 * This,
            /* [in] */ DWORD zone);
        
        DECLSPEC_XFGVIRT(IZoneIdentifier2, RemoveAppZoneId)
        HRESULT ( STDMETHODCALLTYPE *RemoveAppZoneId )( 
            __RPC__in IZoneIdentifier2 * This);
        
        END_INTERFACE
    } IZoneIdentifier2Vtbl;

    interface IZoneIdentifier2
    {
        CONST_VTBL struct IZoneIdentifier2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IZoneIdentifier2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IZoneIdentifier2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IZoneIdentifier2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IZoneIdentifier2_GetId(This,pdwZone)	\
    ( (This)->lpVtbl -> GetId(This,pdwZone) ) 

#define IZoneIdentifier2_SetId(This,dwZone)	\
    ( (This)->lpVtbl -> SetId(This,dwZone) ) 

#define IZoneIdentifier2_Remove(This)	\
    ( (This)->lpVtbl -> Remove(This) ) 


#define IZoneIdentifier2_GetLastWriterPackageFamilyName(This,packageFamilyName)	\
    ( (This)->lpVtbl -> GetLastWriterPackageFamilyName(This,packageFamilyName) ) 

#define IZoneIdentifier2_SetLastWriterPackageFamilyName(This,packageFamilyName)	\
    ( (This)->lpVtbl -> SetLastWriterPackageFamilyName(This,packageFamilyName) ) 

#define IZoneIdentifier2_RemoveLastWriterPackageFamilyName(This)	\
    ( (This)->lpVtbl -> RemoveLastWriterPackageFamilyName(This) ) 

#define IZoneIdentifier2_GetAppZoneId(This,zone)	\
    ( (This)->lpVtbl -> GetAppZoneId(This,zone) ) 

#define IZoneIdentifier2_SetAppZoneId(This,zone)	\
    ( (This)->lpVtbl -> SetAppZoneId(This,zone) ) 

#define IZoneIdentifier2_RemoveAppZoneId(This)	\
    ( (This)->lpVtbl -> RemoveAppZoneId(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IZoneIdentifier2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0043 */
/* [local] */ 

#endif //(_WIN32_IE >= _WIN32_IE_WIN10
#ifndef _LPINTERNETHOSTSECURITYMANANGER_DEFINED
#define _LPINTERNETHOSTSECURITYMANANGER_DEFINED
//This is the interface MSHTML exposes to its clients
//The clients need not pass in a URL to these functions
//since MSHTML maintains the notion of the current URL


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0043_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0043_v0_0_s_ifspec;

#ifndef __IInternetHostSecurityManager_INTERFACE_DEFINED__
#define __IInternetHostSecurityManager_INTERFACE_DEFINED__

/* interface IInternetHostSecurityManager */
/* [unique][helpstring][uuid][object][local] */ 

// Please see notes on IInternetSecurityManager::ProcessUrlAction

EXTERN_C const IID IID_IInternetHostSecurityManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3af280b6-cb3f-11d0-891e-00c04fb6bfc4")
    IInternetHostSecurityManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSecurityId( 
            /* [annotation][size_is][out] */ 
            _Out_writes_(*pcbSecurityId)  BYTE *pbSecurityId,
            /* [annotation][out][in] */ 
            _Inout_ _Deref_in_range_(MAX_SIZE_SECURITY_ID, UINT_MAX) _Deref_out_range_(0, MAX_SIZE_SECURITY_ID)  DWORD *pcbSecurityId,
            /* [in] */ DWORD_PTR dwReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessUrlAction( 
            /* [in] */ DWORD dwAction,
            /* [annotation][size_is][out] */ 
            _Out_writes_all_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [annotation][in] */ 
            _In_reads_opt_(cbContext)  BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryCustomPolicy( 
            /* [in] */ REFGUID guidKey,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_maybenull_(*pcbPolicy)  BYTE **ppPolicy,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbPolicy,
            /* [annotation][in] */ 
            _In_reads_(cbContext)  BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD dwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetHostSecurityManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternetHostSecurityManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternetHostSecurityManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternetHostSecurityManager * This);
        
        DECLSPEC_XFGVIRT(IInternetHostSecurityManager, GetSecurityId)
        HRESULT ( STDMETHODCALLTYPE *GetSecurityId )( 
            IInternetHostSecurityManager * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_(*pcbSecurityId)  BYTE *pbSecurityId,
            /* [annotation][out][in] */ 
            _Inout_ _Deref_in_range_(MAX_SIZE_SECURITY_ID, UINT_MAX) _Deref_out_range_(0, MAX_SIZE_SECURITY_ID)  DWORD *pcbSecurityId,
            /* [in] */ DWORD_PTR dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetHostSecurityManager, ProcessUrlAction)
        HRESULT ( STDMETHODCALLTYPE *ProcessUrlAction )( 
            IInternetHostSecurityManager * This,
            /* [in] */ DWORD dwAction,
            /* [annotation][size_is][out] */ 
            _Out_writes_all_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [annotation][in] */ 
            _In_reads_opt_(cbContext)  BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetHostSecurityManager, QueryCustomPolicy)
        HRESULT ( STDMETHODCALLTYPE *QueryCustomPolicy )( 
            IInternetHostSecurityManager * This,
            /* [in] */ REFGUID guidKey,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_maybenull_(*pcbPolicy)  BYTE **ppPolicy,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbPolicy,
            /* [annotation][in] */ 
            _In_reads_(cbContext)  BYTE *pContext,
            /* [in] */ DWORD cbContext,
            /* [in] */ DWORD dwReserved);
        
        END_INTERFACE
    } IInternetHostSecurityManagerVtbl;

    interface IInternetHostSecurityManager
    {
        CONST_VTBL struct IInternetHostSecurityManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetHostSecurityManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetHostSecurityManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetHostSecurityManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetHostSecurityManager_GetSecurityId(This,pbSecurityId,pcbSecurityId,dwReserved)	\
    ( (This)->lpVtbl -> GetSecurityId(This,pbSecurityId,pcbSecurityId,dwReserved) ) 

#define IInternetHostSecurityManager_ProcessUrlAction(This,dwAction,pPolicy,cbPolicy,pContext,cbContext,dwFlags,dwReserved)	\
    ( (This)->lpVtbl -> ProcessUrlAction(This,dwAction,pPolicy,cbPolicy,pContext,cbContext,dwFlags,dwReserved) ) 

#define IInternetHostSecurityManager_QueryCustomPolicy(This,guidKey,ppPolicy,pcbPolicy,pContext,cbContext,dwReserved)	\
    ( (This)->lpVtbl -> QueryCustomPolicy(This,guidKey,ppPolicy,pcbPolicy,pContext,cbContext,dwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetHostSecurityManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0044 */
/* [local] */ 

#endif

// The zone manager maintains policies for a set of standard actions. 
// These actions are identified by integral values (called action indexes)
// specified below.

// Minimum legal value for an action    
#define URLACTION_MIN                                          0x00001000

#define URLACTION_DOWNLOAD_MIN                                 0x00001000
#define URLACTION_DOWNLOAD_SIGNED_ACTIVEX                      0x00001001
#define URLACTION_DOWNLOAD_UNSIGNED_ACTIVEX                    0x00001004
#define URLACTION_DOWNLOAD_CURR_MAX                            0x00001004
#define URLACTION_DOWNLOAD_MAX                                 0x000011FF

#define URLACTION_ACTIVEX_MIN                                  0x00001200
#define URLACTION_ACTIVEX_RUN                                  0x00001200
#define URLPOLICY_ACTIVEX_CHECK_LIST                           0x00010000
#define URLACTION_ACTIVEX_OVERRIDE_OBJECT_SAFETY               0x00001201 // aggregate next four
#define URLACTION_ACTIVEX_OVERRIDE_DATA_SAFETY                 0x00001202 //
#define URLACTION_ACTIVEX_OVERRIDE_SCRIPT_SAFETY               0x00001203 //
#define URLACTION_SCRIPT_OVERRIDE_SAFETY                       0x00001401 //
#define URLACTION_ACTIVEX_CONFIRM_NOOBJECTSAFETY               0x00001204 //
#define URLACTION_ACTIVEX_TREATASUNTRUSTED                     0x00001205
#define URLACTION_ACTIVEX_NO_WEBOC_SCRIPT                      0x00001206
#define URLACTION_ACTIVEX_OVERRIDE_REPURPOSEDETECTION          0x00001207
#define URLACTION_ACTIVEX_OVERRIDE_OPTIN                       0x00001208
#define URLACTION_ACTIVEX_SCRIPTLET_RUN                        0x00001209
#define URLACTION_ACTIVEX_DYNSRC_VIDEO_AND_ANIMATION           0x0000120A //
#define URLACTION_ACTIVEX_OVERRIDE_DOMAINLIST                  0x0000120B
#define URLACTION_ACTIVEX_ALLOW_TDC                            0x0000120C
#define URLACTION_ACTIVEX_CURR_MAX                             0x0000120C
#define URLACTION_ACTIVEX_MAX                                  0x000013ff

#define URLACTION_SCRIPT_MIN                                   0x00001400
#define URLACTION_SCRIPT_RUN                                   0x00001400
#define URLACTION_SCRIPT_JAVA_USE                              0x00001402
#define URLACTION_SCRIPT_SAFE_ACTIVEX                          0x00001405
#define URLACTION_CROSS_DOMAIN_DATA                            0x00001406
#define URLACTION_SCRIPT_PASTE                                 0x00001407
#define URLACTION_ALLOW_XDOMAIN_SUBFRAME_RESIZE                0x00001408
#define URLACTION_SCRIPT_XSSFILTER                             0x00001409
#define URLACTION_SCRIPT_NAVIGATE                              0x0000140A
#define URLACTION_PLUGGABLE_PROTOCOL_XHR                       0x0000140B
#define URLACTION_ALLOW_VBSCRIPT_IE                            0x0000140C
#define URLACTION_ALLOW_JSCRIPT_IE                             0x0000140D
#define URLACTION_SCRIPT_CURR_MAX                              0x0000140D
#define URLACTION_SCRIPT_MAX                                   0x000015ff

#define URLACTION_HTML_MIN                                     0x00001600
#define URLACTION_HTML_SUBMIT_FORMS                            0x00001601 // aggregate next two
#define URLACTION_HTML_SUBMIT_FORMS_FROM                       0x00001602 //
#define URLACTION_HTML_SUBMIT_FORMS_TO                         0x00001603 //
#define URLACTION_HTML_FONT_DOWNLOAD                           0x00001604
#define URLACTION_HTML_JAVA_RUN                                0x00001605 // derive from Java custom policy
#define URLACTION_HTML_USERDATA_SAVE                           0x00001606
#define URLACTION_HTML_SUBFRAME_NAVIGATE                       0x00001607
#define URLACTION_HTML_META_REFRESH                            0x00001608
#define URLACTION_HTML_MIXED_CONTENT                           0x00001609
#define URLACTION_HTML_INCLUDE_FILE_PATH                       0x0000160A
#define URLACTION_HTML_ALLOW_INJECTED_DYNAMIC_HTML             0x0000160B
#define URLACTION_HTML_REQUIRE_UTF8_DOCUMENT_CODEPAGE          0x0000160C
#define URLACTION_HTML_ALLOW_CROSS_DOMAIN_CANVAS               0x0000160D
#define URLACTION_HTML_ALLOW_WINDOW_CLOSE                      0x0000160E
#define URLACTION_HTML_ALLOW_CROSS_DOMAIN_WEBWORKER            0x0000160F
#define URLACTION_HTML_ALLOW_CROSS_DOMAIN_TEXTTRACK            0x00001610
#define URLACTION_HTML_ALLOW_INDEXEDDB                         0x00001611
#define URLACTION_HTML_MAX                                     0x000017ff

#define URLACTION_SHELL_MIN                                    0x00001800
#define URLACTION_SHELL_INSTALL_DTITEMS                        0x00001800
#define URLACTION_SHELL_MOVE_OR_COPY                           0x00001802
#define URLACTION_SHELL_FILE_DOWNLOAD                          0x00001803
#define URLACTION_SHELL_VERB                                   0x00001804
#define URLACTION_SHELL_WEBVIEW_VERB                           0x00001805
#define URLACTION_SHELL_SHELLEXECUTE                           0x00001806
#if (_WIN32_IE >= _WIN32_IE_IE60SP2)
#define URLACTION_SHELL_EXECUTE_HIGHRISK                       0x00001806
#define URLACTION_SHELL_EXECUTE_MODRISK                        0x00001807
#define URLACTION_SHELL_EXECUTE_LOWRISK                        0x00001808
#define URLACTION_SHELL_POPUPMGR                               0x00001809
#define URLACTION_SHELL_RTF_OBJECTS_LOAD                       0x0000180A
#define URLACTION_SHELL_ENHANCED_DRAGDROP_SECURITY             0x0000180B
#define URLACTION_SHELL_EXTENSIONSECURITY                      0x0000180C
#define URLACTION_SHELL_SECURE_DRAGSOURCE                      0x0000180D
#endif //(_WIN32_IE >= _WIN32_IE_IE60SP2)
#if (_WIN32_IE >= _WIN32_IE_WIN7)
#define URLACTION_SHELL_REMOTEQUERY                            0x0000180E
#define URLACTION_SHELL_PREVIEW                                0x0000180F
#define URLACTION_SHELL_SHARE                                  0x00001810
#define URLACTION_SHELL_ALLOW_CROSS_SITE_SHARE                 0x00001811
#endif //(_WIN32_IE >= _WIN32_IE_WIN7)
#define URLACTION_SHELL_TOCTOU_RISK                            0x00001812
#define URLACTION_SHELL_CURR_MAX                               0x00001812
#define URLACTION_SHELL_MAX                                    0x000019ff

#define URLACTION_NETWORK_MIN                                  0x00001A00

#define URLACTION_CREDENTIALS_USE                              0x00001A00
#define URLPOLICY_CREDENTIALS_SILENT_LOGON_OK        0x00000000
#define URLPOLICY_CREDENTIALS_MUST_PROMPT_USER       0x00010000
#define URLPOLICY_CREDENTIALS_CONDITIONAL_PROMPT     0x00020000
#define URLPOLICY_CREDENTIALS_ANONYMOUS_ONLY         0x00030000

#define URLACTION_AUTHENTICATE_CLIENT                          0x00001A01
#define URLPOLICY_AUTHENTICATE_CLEARTEXT_OK          0x00000000
#define URLPOLICY_AUTHENTICATE_CHALLENGE_RESPONSE    0x00010000
#define URLPOLICY_AUTHENTICATE_MUTUAL_ONLY           0x00030000


#define URLACTION_COOKIES                                      0x00001A02
#define URLACTION_COOKIES_SESSION                              0x00001A03

#define URLACTION_CLIENT_CERT_PROMPT                           0x00001A04

#define URLACTION_COOKIES_THIRD_PARTY                          0x00001A05
#define URLACTION_COOKIES_SESSION_THIRD_PARTY                  0x00001A06

#define URLACTION_COOKIES_ENABLED                              0x00001A10

#define URLACTION_NETWORK_CURR_MAX                             0x00001A10
#define URLACTION_NETWORK_MAX                                  0x00001Bff


#define URLACTION_JAVA_MIN                                     0x00001C00
#define URLACTION_JAVA_PERMISSIONS                             0x00001C00
#define URLPOLICY_JAVA_PROHIBIT                      0x00000000
#define URLPOLICY_JAVA_HIGH                          0x00010000
#define URLPOLICY_JAVA_MEDIUM                        0x00020000
#define URLPOLICY_JAVA_LOW                           0x00030000
#define URLPOLICY_JAVA_CUSTOM                        0x00800000
#define URLACTION_JAVA_CURR_MAX                                0x00001C00
#define URLACTION_JAVA_MAX                                     0x00001Cff


// The following Infodelivery actions should have no default policies
// in the registry.  They assume that no default policy means fall
// back to the global restriction.  If an admin sets a policy per
// zone, then it overrides the global restriction.

#define URLACTION_INFODELIVERY_MIN                           0x00001D00
#define URLACTION_INFODELIVERY_NO_ADDING_CHANNELS            0x00001D00
#define URLACTION_INFODELIVERY_NO_EDITING_CHANNELS           0x00001D01
#define URLACTION_INFODELIVERY_NO_REMOVING_CHANNELS          0x00001D02
#define URLACTION_INFODELIVERY_NO_ADDING_SUBSCRIPTIONS       0x00001D03
#define URLACTION_INFODELIVERY_NO_EDITING_SUBSCRIPTIONS      0x00001D04
#define URLACTION_INFODELIVERY_NO_REMOVING_SUBSCRIPTIONS     0x00001D05
#define URLACTION_INFODELIVERY_NO_CHANNEL_LOGGING            0x00001D06
#define URLACTION_INFODELIVERY_CURR_MAX                      0x00001D06
#define URLACTION_INFODELIVERY_MAX                           0x00001Dff
#define URLACTION_CHANNEL_SOFTDIST_MIN                       0x00001E00
#define URLACTION_CHANNEL_SOFTDIST_PERMISSIONS               0x00001E05
#define URLPOLICY_CHANNEL_SOFTDIST_PROHIBIT          0x00010000
#define URLPOLICY_CHANNEL_SOFTDIST_PRECACHE          0x00020000
#define URLPOLICY_CHANNEL_SOFTDIST_AUTOINSTALL       0x00030000
#define URLACTION_CHANNEL_SOFTDIST_MAX                       0x00001Eff
#if (_WIN32_IE >= _WIN32_IE_IE80)
#define URLACTION_DOTNET_USERCONTROLS                        0x00002005
#endif //(_WIN32_IE >= _WIN32_IE_IE80)
#if (_WIN32_IE >= _WIN32_IE_IE60SP2)
#define URLACTION_BEHAVIOR_MIN                               0x00002000
#define URLACTION_BEHAVIOR_RUN                               0x00002000
#define URLPOLICY_BEHAVIOR_CHECK_LIST                        0x00010000

// The following actions correspond to the Feature options above.
// However, they are NOT in the same order.
#define URLACTION_FEATURE_MIN                                0x00002100
#define URLACTION_FEATURE_MIME_SNIFFING                      0x00002100
#define URLACTION_FEATURE_ZONE_ELEVATION                     0x00002101
#define URLACTION_FEATURE_WINDOW_RESTRICTIONS                0x00002102
#define URLACTION_FEATURE_SCRIPT_STATUS_BAR                  0x00002103
#define URLACTION_FEATURE_FORCE_ADDR_AND_STATUS              0x00002104
#define URLACTION_FEATURE_BLOCK_INPUT_PROMPTS                0x00002105
#define URLACTION_FEATURE_DATA_BINDING                       0x00002106
#define URLACTION_FEATURE_CROSSDOMAIN_FOCUS_CHANGE           0x00002107

#define URLACTION_AUTOMATIC_DOWNLOAD_UI_MIN                  0x00002200
#define URLACTION_AUTOMATIC_DOWNLOAD_UI                      0x00002200
#define URLACTION_AUTOMATIC_ACTIVEX_UI                       0x00002201

#define URLACTION_ALLOW_RESTRICTEDPROTOCOLS                0x00002300

#endif //(_WIN32_IE >= _WIN32_IE_IE60SP2)
#if (_WIN32_IE >= _WIN32_IE_IE70)
// Whether to do the Anti-Phishing check.
#define URLACTION_ALLOW_APEVALUATION                       0x00002301
#define URLACTION_ALLOW_XHR_EVALUATION                     0x00002302

// The following ExpressAPP and XPS actions are trumped by registry in
// case of Internet Explorer upgrade from IE 6.0 which honors registry.
#define URLACTION_WINDOWS_BROWSER_APPLICATIONS             0x00002400
#define URLACTION_XPS_DOCUMENTS                            0x00002401
#define URLACTION_LOOSE_XAML                               0x00002402
#define URLACTION_LOWRIGHTS                                0x00002500
// The following action belong to WinFX Bootstrapper
#define URLACTION_WINFX_SETUP                              0x00002600

#define URLACTION_INPRIVATE_BLOCKING                       0x00002700
#endif //(_WIN32_IE >= _WIN32_IE_IE70)

#define URLACTION_ALLOW_AUDIO_VIDEO                        0x00002701


#define URLACTION_ALLOW_ACTIVEX_FILTERING                  0x00002702


#define URLACTION_ALLOW_STRUCTURED_STORAGE_SNIFFING        0x00002703


#define URLACTION_ALLOW_AUDIO_VIDEO_PLUGINS                0x00002704

// The following two URLACTIONs each have as their context parameter a string that is the destination URI.
#define URLACTION_ALLOW_ZONE_ELEVATION_VIA_OPT_OUT         0x00002705
#define URLACTION_ALLOW_ZONE_ELEVATION_OPT_OUT_ADDITION    0x00002706


#define URLACTION_ALLOW_CROSSDOMAIN_DROP_WITHIN_WINDOW     0x00002708
#define URLACTION_ALLOW_CROSSDOMAIN_DROP_ACROSS_WINDOWS    0x00002709


#define URLACTION_ALLOW_CROSSDOMAIN_APPCACHE_MANIFEST      0x0000270A


#define URLACTION_ALLOW_RENDER_LEGACY_DXTFILTERS           0x0000270B


#define URLACTION_ALLOW_ANTIMALWARE_SCANNING_OF_ACTIVEX    0x0000270C


#define URLACTION_ALLOW_CSS_EXPRESSIONS                    0x0000270D

// For each action specified above the system maintains
// a set of policies for the action. 
// The only policies supported currently are permissions (i.e. is something allowed)
// and logging status. 
// IMPORTANT: If you are defining your own policies don't overload the meaning of the
// loword of the policy. You can use the hiword to store any policy bits which are only
// meaningful to your action.
// For an example of how to do this look at the URLPOLICY_JAVA above

// Permissions 
#define URLPOLICY_ALLOW                0x00
#define URLPOLICY_QUERY                0x01
#define URLPOLICY_DISALLOW             0x03

// Notifications are not done when user already queried.
#define URLPOLICY_NOTIFY_ON_ALLOW      0x10
#define URLPOLICY_NOTIFY_ON_DISALLOW   0x20

// Logging is done regardless of whether user was queried.
#define URLPOLICY_LOG_ON_ALLOW         0x40
#define URLPOLICY_LOG_ON_DISALLOW      0x80

#define URLPOLICY_MASK_PERMISSIONS     0x0f
#define GetUrlPolicyPermissions(dw)        (dw & URLPOLICY_MASK_PERMISSIONS)
#define SetUrlPolicyPermissions(dw,dw2)    ((dw) = ((dw) & ~(URLPOLICY_MASK_PERMISSIONS)) | (dw2))


#define URLPOLICY_DONTCHECKDLGBOX     0x100
// The ordinal #'s that define the predefined zones internet explorer knows about. 
// When we support user-defined zones their zone numbers should be between 
// URLZONE_USER_MIN and URLZONE_USER_MAX
#if (_WIN32_IE >= _WIN32_IE_IE60SP2)
// Custom policy to query whether the local machine zone
// has been unlocked for current document.
EXTERN_C const GUID GUID_CUSTOM_LOCALMACHINEZONEUNLOCKED; 
#endif //(_WIN32_IE >= _WIN32_IE_IE60SP2)
#ifndef _LPINTERNETZONEMANAGER_DEFINED
#define _LPINTERNETZONEMANAGER_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0044_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0044_v0_0_s_ifspec;

#ifndef __IInternetZoneManager_INTERFACE_DEFINED__
#define __IInternetZoneManager_INTERFACE_DEFINED__

/* interface IInternetZoneManager */
/* [unique][helpstring][uuid][object][local] */ 

typedef /* [unique] */ IInternetZoneManager *LPURLZONEMANAGER;

typedef 
enum tagURLZONE
    {
        URLZONE_INVALID	= -1,
        URLZONE_PREDEFINED_MIN	= 0,
        URLZONE_LOCAL_MACHINE	= 0,
        URLZONE_INTRANET	= ( URLZONE_LOCAL_MACHINE + 1 ) ,
        URLZONE_TRUSTED	= ( URLZONE_INTRANET + 1 ) ,
        URLZONE_INTERNET	= ( URLZONE_TRUSTED + 1 ) ,
        URLZONE_UNTRUSTED	= ( URLZONE_INTERNET + 1 ) ,
        URLZONE_PREDEFINED_MAX	= 999,
        URLZONE_USER_MIN	= 1000,
        URLZONE_USER_MAX	= 10000
    } 	URLZONE;

// Enhanced Security Configuration zone mapping flag for IInternetSecurityManager::SetZoneMapping
#define URLZONE_ESC_FLAG     0x100
typedef 
enum tagURLTEMPLATE
    {
        URLTEMPLATE_CUSTOM	= 0,
        URLTEMPLATE_PREDEFINED_MIN	= 0x10000,
        URLTEMPLATE_LOW	= 0x10000,
        URLTEMPLATE_MEDLOW	= 0x10500,
        URLTEMPLATE_MEDIUM	= 0x11000,
        URLTEMPLATE_MEDHIGH	= 0x11500,
        URLTEMPLATE_HIGH	= 0x12000,
        URLTEMPLATE_PREDEFINED_MAX	= 0x20000
    } 	URLTEMPLATE;


enum __MIDL_IInternetZoneManager_0001
    {
        MAX_ZONE_PATH	= 260,
        MAX_ZONE_DESCRIPTION	= 200
    } ;
typedef /* [public] */ 
enum __MIDL_IInternetZoneManager_0002
    {
        ZAFLAGS_CUSTOM_EDIT	= 0x1,
        ZAFLAGS_ADD_SITES	= 0x2,
        ZAFLAGS_REQUIRE_VERIFICATION	= 0x4,
        ZAFLAGS_INCLUDE_PROXY_OVERRIDE	= 0x8,
        ZAFLAGS_INCLUDE_INTRANET_SITES	= 0x10,
        ZAFLAGS_NO_UI	= 0x20,
        ZAFLAGS_SUPPORTS_VERIFICATION	= 0x40,
        ZAFLAGS_UNC_AS_INTRANET	= 0x80,
        ZAFLAGS_DETECT_INTRANET	= 0x100,
        ZAFLAGS_USE_LOCKED_ZONES	= 0x10000,
        ZAFLAGS_VERIFY_TEMPLATE_SETTINGS	= 0x20000,
        ZAFLAGS_NO_CACHE	= 0x40000
    } 	ZAFLAGS;

typedef struct _ZONEATTRIBUTES
    {
    ULONG cbSize;
    WCHAR szDisplayName[ 260 ];
    WCHAR szDescription[ 200 ];
    WCHAR szIconPath[ 260 ];
    DWORD dwTemplateMinLevel;
    DWORD dwTemplateRecommended;
    DWORD dwTemplateCurrentLevel;
    DWORD dwFlags;
    } 	ZONEATTRIBUTES;

typedef struct _ZONEATTRIBUTES *LPZONEATTRIBUTES;

// Gets the zone attributes (information in registry other than actual security
// policies associated with the zone).  Zone attributes are fixed as:
// Sets the zone attributes (information in registry other than actual security
// policies associated with the zone).  Zone attributes as above.
// Returns S_OK or ??? if failed to write the zone attributes.
/* Registry Flags

    When reading, default behavior is:
        If HKLM allows override and HKCU value exists
            Then use HKCU value
            Else use HKLM value
    When writing, default behavior is same as HKCU
        If HKLM allows override
           Then Write to HKCU
           Else Fail
*/
typedef 
enum _URLZONEREG
    {
        URLZONEREG_DEFAULT	= 0,
        URLZONEREG_HKLM	= ( URLZONEREG_DEFAULT + 1 ) ,
        URLZONEREG_HKCU	= ( URLZONEREG_HKLM + 1 ) 
    } 	URLZONEREG;

// Gets a named custom policy associated with a zone;
// e.g. the Java VM settings can be defined with a unique key such as 'Java'.
// Custom policy support is intended to allow extensibility from the predefined
// set of policies that IE4 has built in.
// 
// pwszKey is the string name designating the custom policy.  Components are
//   responsible for having unique names.
// ppPolicy is the callee allocated buffer for the policy byte blob; caller is
//   responsible for freeing this buffer eventually.
// pcbPolicy is the size of the byte blob returned.
// dwRegFlags determines how registry is accessed (see above).
// Returns S_OK if key is found and buffer allocated; ??? if key is not found (no buffer alloced).
// Sets a named custom policy associated with a zone;
// e.g. the Java VM settings can be defined with a unique key such as 'Java'.
// Custom policy support is intended to allow extensibility from the predefined
// set of policies that IE4 has built in.  
// 
// pwszKey is the string name designating the custom policy.  Components are
//   responsible for having unique names.
// ppPolicy is the caller allocated buffer for the policy byte blob.
// pcbPolicy is the size of the byte blob to be set.
// dwRegFlags determines if HTCU or HKLM is set.
// Returns S_OK or ??? if failed to write the zone custom policy.
// Gets action policy associated with a zone, the builtin, fixed-length policies info.

// dwAction is the action code for the action as defined above.
// pPolicy is the caller allocated buffer for the policy data.
// cbPolicy is the size of the caller allocated buffer.
// dwRegFlags determines how registry is accessed (see above).
// Returns S_OK if action is valid; ??? if action is not valid.

EXTERN_C const IID IID_IInternetZoneManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79eac9ef-baf9-11ce-8c82-00aa004ba90b")
    IInternetZoneManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetZoneAttributes( 
            /* [in] */ DWORD dwZone,
            /* [annotation][unique][out][in] */ 
            _Inout_  ZONEATTRIBUTES *pZoneAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetZoneAttributes( 
            /* [in] */ DWORD dwZone,
            /* [annotation][in] */ 
            _In_  ZONEATTRIBUTES *pZoneAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetZoneCustomPolicy( 
            /* [in] */ DWORD dwZone,
            /* [in] */ REFGUID guidKey,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_(*pcbPolicy)  BYTE **ppPolicy,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbPolicy,
            /* [in] */ URLZONEREG urlZoneReg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetZoneCustomPolicy( 
            /* [in] */ DWORD dwZone,
            /* [in] */ REFGUID guidKey,
            /* [annotation][size_is][in] */ 
            _In_reads_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetZoneActionPolicy( 
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwAction,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetZoneActionPolicy( 
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwAction,
            /* [annotation][size_is][in] */ 
            _In_reads_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PromptAction( 
            /* [in] */ DWORD dwAction,
            /* [in] */ HWND hwndParent,
            /* [in] */ LPCWSTR pwszUrl,
            /* [in] */ LPCWSTR pwszText,
            /* [in] */ DWORD dwPromptFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LogAction( 
            /* [in] */ DWORD dwAction,
            /* [in] */ LPCWSTR pwszUrl,
            /* [in] */ LPCWSTR pwszText,
            /* [in] */ DWORD dwLogFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateZoneEnumerator( 
            /* [out] */ DWORD *pdwEnum,
            /* [out] */ DWORD *pdwCount,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetZoneAt( 
            /* [in] */ DWORD dwEnum,
            /* [in] */ DWORD dwIndex,
            /* [out] */ DWORD *pdwZone) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DestroyZoneEnumerator( 
            /* [in] */ DWORD dwEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CopyTemplatePoliciesToZone( 
            /* [in] */ DWORD dwTemplate,
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetZoneManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternetZoneManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternetZoneManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternetZoneManager * This);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, GetZoneAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetZoneAttributes )( 
            IInternetZoneManager * This,
            /* [in] */ DWORD dwZone,
            /* [annotation][unique][out][in] */ 
            _Inout_  ZONEATTRIBUTES *pZoneAttributes);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, SetZoneAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetZoneAttributes )( 
            IInternetZoneManager * This,
            /* [in] */ DWORD dwZone,
            /* [annotation][in] */ 
            _In_  ZONEATTRIBUTES *pZoneAttributes);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, GetZoneCustomPolicy)
        HRESULT ( STDMETHODCALLTYPE *GetZoneCustomPolicy )( 
            IInternetZoneManager * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ REFGUID guidKey,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_(*pcbPolicy)  BYTE **ppPolicy,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbPolicy,
            /* [in] */ URLZONEREG urlZoneReg);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, SetZoneCustomPolicy)
        HRESULT ( STDMETHODCALLTYPE *SetZoneCustomPolicy )( 
            IInternetZoneManager * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ REFGUID guidKey,
            /* [annotation][size_is][in] */ 
            _In_reads_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, GetZoneActionPolicy)
        HRESULT ( STDMETHODCALLTYPE *GetZoneActionPolicy )( 
            IInternetZoneManager * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwAction,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, SetZoneActionPolicy)
        HRESULT ( STDMETHODCALLTYPE *SetZoneActionPolicy )( 
            IInternetZoneManager * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwAction,
            /* [annotation][size_is][in] */ 
            _In_reads_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, PromptAction)
        HRESULT ( STDMETHODCALLTYPE *PromptAction )( 
            IInternetZoneManager * This,
            /* [in] */ DWORD dwAction,
            /* [in] */ HWND hwndParent,
            /* [in] */ LPCWSTR pwszUrl,
            /* [in] */ LPCWSTR pwszText,
            /* [in] */ DWORD dwPromptFlags);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, LogAction)
        HRESULT ( STDMETHODCALLTYPE *LogAction )( 
            IInternetZoneManager * This,
            /* [in] */ DWORD dwAction,
            /* [in] */ LPCWSTR pwszUrl,
            /* [in] */ LPCWSTR pwszText,
            /* [in] */ DWORD dwLogFlags);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, CreateZoneEnumerator)
        HRESULT ( STDMETHODCALLTYPE *CreateZoneEnumerator )( 
            IInternetZoneManager * This,
            /* [out] */ DWORD *pdwEnum,
            /* [out] */ DWORD *pdwCount,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, GetZoneAt)
        HRESULT ( STDMETHODCALLTYPE *GetZoneAt )( 
            IInternetZoneManager * This,
            /* [in] */ DWORD dwEnum,
            /* [in] */ DWORD dwIndex,
            /* [out] */ DWORD *pdwZone);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, DestroyZoneEnumerator)
        HRESULT ( STDMETHODCALLTYPE *DestroyZoneEnumerator )( 
            IInternetZoneManager * This,
            /* [in] */ DWORD dwEnum);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, CopyTemplatePoliciesToZone)
        HRESULT ( STDMETHODCALLTYPE *CopyTemplatePoliciesToZone )( 
            IInternetZoneManager * This,
            /* [in] */ DWORD dwTemplate,
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwReserved);
        
        END_INTERFACE
    } IInternetZoneManagerVtbl;

    interface IInternetZoneManager
    {
        CONST_VTBL struct IInternetZoneManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetZoneManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetZoneManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetZoneManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetZoneManager_GetZoneAttributes(This,dwZone,pZoneAttributes)	\
    ( (This)->lpVtbl -> GetZoneAttributes(This,dwZone,pZoneAttributes) ) 

#define IInternetZoneManager_SetZoneAttributes(This,dwZone,pZoneAttributes)	\
    ( (This)->lpVtbl -> SetZoneAttributes(This,dwZone,pZoneAttributes) ) 

#define IInternetZoneManager_GetZoneCustomPolicy(This,dwZone,guidKey,ppPolicy,pcbPolicy,urlZoneReg)	\
    ( (This)->lpVtbl -> GetZoneCustomPolicy(This,dwZone,guidKey,ppPolicy,pcbPolicy,urlZoneReg) ) 

#define IInternetZoneManager_SetZoneCustomPolicy(This,dwZone,guidKey,pPolicy,cbPolicy,urlZoneReg)	\
    ( (This)->lpVtbl -> SetZoneCustomPolicy(This,dwZone,guidKey,pPolicy,cbPolicy,urlZoneReg) ) 

#define IInternetZoneManager_GetZoneActionPolicy(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg)	\
    ( (This)->lpVtbl -> GetZoneActionPolicy(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg) ) 

#define IInternetZoneManager_SetZoneActionPolicy(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg)	\
    ( (This)->lpVtbl -> SetZoneActionPolicy(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg) ) 

#define IInternetZoneManager_PromptAction(This,dwAction,hwndParent,pwszUrl,pwszText,dwPromptFlags)	\
    ( (This)->lpVtbl -> PromptAction(This,dwAction,hwndParent,pwszUrl,pwszText,dwPromptFlags) ) 

#define IInternetZoneManager_LogAction(This,dwAction,pwszUrl,pwszText,dwLogFlags)	\
    ( (This)->lpVtbl -> LogAction(This,dwAction,pwszUrl,pwszText,dwLogFlags) ) 

#define IInternetZoneManager_CreateZoneEnumerator(This,pdwEnum,pdwCount,dwFlags)	\
    ( (This)->lpVtbl -> CreateZoneEnumerator(This,pdwEnum,pdwCount,dwFlags) ) 

#define IInternetZoneManager_GetZoneAt(This,dwEnum,dwIndex,pdwZone)	\
    ( (This)->lpVtbl -> GetZoneAt(This,dwEnum,dwIndex,pdwZone) ) 

#define IInternetZoneManager_DestroyZoneEnumerator(This,dwEnum)	\
    ( (This)->lpVtbl -> DestroyZoneEnumerator(This,dwEnum) ) 

#define IInternetZoneManager_CopyTemplatePoliciesToZone(This,dwTemplate,dwZone,dwReserved)	\
    ( (This)->lpVtbl -> CopyTemplatePoliciesToZone(This,dwTemplate,dwZone,dwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetZoneManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0045 */
/* [local] */ 

#endif
#if (_WIN32_IE >= _WIN32_IE_IE60SP2)
#ifndef _LPINTERNETZONEMANAGEREX_DEFINED
#define _LPINTERNETZONEMANAGEREX_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0045_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0045_v0_0_s_ifspec;

#ifndef __IInternetZoneManagerEx_INTERFACE_DEFINED__
#define __IInternetZoneManagerEx_INTERFACE_DEFINED__

/* interface IInternetZoneManagerEx */
/* [unique][helpstring][uuid][object][local] */ 

// Gets action policy associated with a zone, the builtin, fixed-length policies info.

// dwAction is the action code for the action as defined above.
// pPolicy is the caller allocated buffer for the policy data.
// cbPolicy is the size of the caller allocated buffer.
// dwRegFlags determines how registry is accessed (see above).
// dwFlags determine which registry policies are accessed (see above).
// Returns S_OK if action is valid; ??? if action is not valid.

EXTERN_C const IID IID_IInternetZoneManagerEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A4C23339-8E06-431e-9BF4-7E711C085648")
    IInternetZoneManagerEx : public IInternetZoneManager
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetZoneActionPolicyEx( 
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwAction,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetZoneActionPolicyEx( 
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwAction,
            /* [annotation][size_is][in] */ 
            _In_reads_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg,
            /* [in] */ DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetZoneManagerExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternetZoneManagerEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternetZoneManagerEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternetZoneManagerEx * This);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, GetZoneAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetZoneAttributes )( 
            IInternetZoneManagerEx * This,
            /* [in] */ DWORD dwZone,
            /* [annotation][unique][out][in] */ 
            _Inout_  ZONEATTRIBUTES *pZoneAttributes);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, SetZoneAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetZoneAttributes )( 
            IInternetZoneManagerEx * This,
            /* [in] */ DWORD dwZone,
            /* [annotation][in] */ 
            _In_  ZONEATTRIBUTES *pZoneAttributes);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, GetZoneCustomPolicy)
        HRESULT ( STDMETHODCALLTYPE *GetZoneCustomPolicy )( 
            IInternetZoneManagerEx * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ REFGUID guidKey,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_(*pcbPolicy)  BYTE **ppPolicy,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbPolicy,
            /* [in] */ URLZONEREG urlZoneReg);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, SetZoneCustomPolicy)
        HRESULT ( STDMETHODCALLTYPE *SetZoneCustomPolicy )( 
            IInternetZoneManagerEx * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ REFGUID guidKey,
            /* [annotation][size_is][in] */ 
            _In_reads_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, GetZoneActionPolicy)
        HRESULT ( STDMETHODCALLTYPE *GetZoneActionPolicy )( 
            IInternetZoneManagerEx * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwAction,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, SetZoneActionPolicy)
        HRESULT ( STDMETHODCALLTYPE *SetZoneActionPolicy )( 
            IInternetZoneManagerEx * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwAction,
            /* [annotation][size_is][in] */ 
            _In_reads_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, PromptAction)
        HRESULT ( STDMETHODCALLTYPE *PromptAction )( 
            IInternetZoneManagerEx * This,
            /* [in] */ DWORD dwAction,
            /* [in] */ HWND hwndParent,
            /* [in] */ LPCWSTR pwszUrl,
            /* [in] */ LPCWSTR pwszText,
            /* [in] */ DWORD dwPromptFlags);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, LogAction)
        HRESULT ( STDMETHODCALLTYPE *LogAction )( 
            IInternetZoneManagerEx * This,
            /* [in] */ DWORD dwAction,
            /* [in] */ LPCWSTR pwszUrl,
            /* [in] */ LPCWSTR pwszText,
            /* [in] */ DWORD dwLogFlags);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, CreateZoneEnumerator)
        HRESULT ( STDMETHODCALLTYPE *CreateZoneEnumerator )( 
            IInternetZoneManagerEx * This,
            /* [out] */ DWORD *pdwEnum,
            /* [out] */ DWORD *pdwCount,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, GetZoneAt)
        HRESULT ( STDMETHODCALLTYPE *GetZoneAt )( 
            IInternetZoneManagerEx * This,
            /* [in] */ DWORD dwEnum,
            /* [in] */ DWORD dwIndex,
            /* [out] */ DWORD *pdwZone);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, DestroyZoneEnumerator)
        HRESULT ( STDMETHODCALLTYPE *DestroyZoneEnumerator )( 
            IInternetZoneManagerEx * This,
            /* [in] */ DWORD dwEnum);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, CopyTemplatePoliciesToZone)
        HRESULT ( STDMETHODCALLTYPE *CopyTemplatePoliciesToZone )( 
            IInternetZoneManagerEx * This,
            /* [in] */ DWORD dwTemplate,
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetZoneManagerEx, GetZoneActionPolicyEx)
        HRESULT ( STDMETHODCALLTYPE *GetZoneActionPolicyEx )( 
            IInternetZoneManagerEx * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwAction,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IInternetZoneManagerEx, SetZoneActionPolicyEx)
        HRESULT ( STDMETHODCALLTYPE *SetZoneActionPolicyEx )( 
            IInternetZoneManagerEx * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwAction,
            /* [annotation][size_is][in] */ 
            _In_reads_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg,
            /* [in] */ DWORD dwFlags);
        
        END_INTERFACE
    } IInternetZoneManagerExVtbl;

    interface IInternetZoneManagerEx
    {
        CONST_VTBL struct IInternetZoneManagerExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetZoneManagerEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetZoneManagerEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetZoneManagerEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetZoneManagerEx_GetZoneAttributes(This,dwZone,pZoneAttributes)	\
    ( (This)->lpVtbl -> GetZoneAttributes(This,dwZone,pZoneAttributes) ) 

#define IInternetZoneManagerEx_SetZoneAttributes(This,dwZone,pZoneAttributes)	\
    ( (This)->lpVtbl -> SetZoneAttributes(This,dwZone,pZoneAttributes) ) 

#define IInternetZoneManagerEx_GetZoneCustomPolicy(This,dwZone,guidKey,ppPolicy,pcbPolicy,urlZoneReg)	\
    ( (This)->lpVtbl -> GetZoneCustomPolicy(This,dwZone,guidKey,ppPolicy,pcbPolicy,urlZoneReg) ) 

#define IInternetZoneManagerEx_SetZoneCustomPolicy(This,dwZone,guidKey,pPolicy,cbPolicy,urlZoneReg)	\
    ( (This)->lpVtbl -> SetZoneCustomPolicy(This,dwZone,guidKey,pPolicy,cbPolicy,urlZoneReg) ) 

#define IInternetZoneManagerEx_GetZoneActionPolicy(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg)	\
    ( (This)->lpVtbl -> GetZoneActionPolicy(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg) ) 

#define IInternetZoneManagerEx_SetZoneActionPolicy(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg)	\
    ( (This)->lpVtbl -> SetZoneActionPolicy(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg) ) 

#define IInternetZoneManagerEx_PromptAction(This,dwAction,hwndParent,pwszUrl,pwszText,dwPromptFlags)	\
    ( (This)->lpVtbl -> PromptAction(This,dwAction,hwndParent,pwszUrl,pwszText,dwPromptFlags) ) 

#define IInternetZoneManagerEx_LogAction(This,dwAction,pwszUrl,pwszText,dwLogFlags)	\
    ( (This)->lpVtbl -> LogAction(This,dwAction,pwszUrl,pwszText,dwLogFlags) ) 

#define IInternetZoneManagerEx_CreateZoneEnumerator(This,pdwEnum,pdwCount,dwFlags)	\
    ( (This)->lpVtbl -> CreateZoneEnumerator(This,pdwEnum,pdwCount,dwFlags) ) 

#define IInternetZoneManagerEx_GetZoneAt(This,dwEnum,dwIndex,pdwZone)	\
    ( (This)->lpVtbl -> GetZoneAt(This,dwEnum,dwIndex,pdwZone) ) 

#define IInternetZoneManagerEx_DestroyZoneEnumerator(This,dwEnum)	\
    ( (This)->lpVtbl -> DestroyZoneEnumerator(This,dwEnum) ) 

#define IInternetZoneManagerEx_CopyTemplatePoliciesToZone(This,dwTemplate,dwZone,dwReserved)	\
    ( (This)->lpVtbl -> CopyTemplatePoliciesToZone(This,dwTemplate,dwZone,dwReserved) ) 


#define IInternetZoneManagerEx_GetZoneActionPolicyEx(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg,dwFlags)	\
    ( (This)->lpVtbl -> GetZoneActionPolicyEx(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg,dwFlags) ) 

#define IInternetZoneManagerEx_SetZoneActionPolicyEx(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg,dwFlags)	\
    ( (This)->lpVtbl -> SetZoneActionPolicyEx(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetZoneManagerEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0046 */
/* [local] */ 

#endif
#endif //(_WIN32_IE >= _WIN32_IE_IE60SP2)
#if (_WIN32_IE >= _WIN32_IE_IE70)
#ifndef _LPINTERNETZONEMANAGEREX2_DEFINED
#define _LPINTERNETZONEMANAGEREX2_DEFINED
#define SECURITY_IE_STATE_GREEN 0x00000000
#define SECURITY_IE_STATE_RED   0x00000001


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0046_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0046_v0_0_s_ifspec;

#ifndef __IInternetZoneManagerEx2_INTERFACE_DEFINED__
#define __IInternetZoneManagerEx2_INTERFACE_DEFINED__

/* interface IInternetZoneManagerEx2 */
/* [unique][helpstring][uuid][object][local] */ 

// Gets the zone attributes (information in registry other than actual security
// policies associated with the zone).  Zone attributes are fixed as:
// Can also verify template settings by matching current settings with security template

EXTERN_C const IID IID_IInternetZoneManagerEx2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EDC17559-DD5D-4846-8EEF-8BECBA5A4ABF")
    IInternetZoneManagerEx2 : public IInternetZoneManagerEx
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetZoneAttributesEx( 
            /* [in] */ DWORD dwZone,
            /* [unique][out][in] */ ZONEATTRIBUTES *pZoneAttributes,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetZoneSecurityState( 
            /* [in] */ DWORD dwZoneIndex,
            /* [in] */ BOOL fRespectPolicy,
            /* [out][in] */ LPDWORD pdwState,
            /* [out][in] */ BOOL *pfPolicyEncountered) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIESecurityState( 
            /* [in] */ BOOL fRespectPolicy,
            /* [out][in] */ LPDWORD pdwState,
            /* [out][in] */ BOOL *pfPolicyEncountered,
            /* [in] */ BOOL fNoCache) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FixUnsecureSettings( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternetZoneManagerEx2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternetZoneManagerEx2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternetZoneManagerEx2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternetZoneManagerEx2 * This);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, GetZoneAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetZoneAttributes )( 
            IInternetZoneManagerEx2 * This,
            /* [in] */ DWORD dwZone,
            /* [annotation][unique][out][in] */ 
            _Inout_  ZONEATTRIBUTES *pZoneAttributes);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, SetZoneAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetZoneAttributes )( 
            IInternetZoneManagerEx2 * This,
            /* [in] */ DWORD dwZone,
            /* [annotation][in] */ 
            _In_  ZONEATTRIBUTES *pZoneAttributes);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, GetZoneCustomPolicy)
        HRESULT ( STDMETHODCALLTYPE *GetZoneCustomPolicy )( 
            IInternetZoneManagerEx2 * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ REFGUID guidKey,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_(*pcbPolicy)  BYTE **ppPolicy,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbPolicy,
            /* [in] */ URLZONEREG urlZoneReg);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, SetZoneCustomPolicy)
        HRESULT ( STDMETHODCALLTYPE *SetZoneCustomPolicy )( 
            IInternetZoneManagerEx2 * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ REFGUID guidKey,
            /* [annotation][size_is][in] */ 
            _In_reads_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, GetZoneActionPolicy)
        HRESULT ( STDMETHODCALLTYPE *GetZoneActionPolicy )( 
            IInternetZoneManagerEx2 * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwAction,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, SetZoneActionPolicy)
        HRESULT ( STDMETHODCALLTYPE *SetZoneActionPolicy )( 
            IInternetZoneManagerEx2 * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwAction,
            /* [annotation][size_is][in] */ 
            _In_reads_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, PromptAction)
        HRESULT ( STDMETHODCALLTYPE *PromptAction )( 
            IInternetZoneManagerEx2 * This,
            /* [in] */ DWORD dwAction,
            /* [in] */ HWND hwndParent,
            /* [in] */ LPCWSTR pwszUrl,
            /* [in] */ LPCWSTR pwszText,
            /* [in] */ DWORD dwPromptFlags);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, LogAction)
        HRESULT ( STDMETHODCALLTYPE *LogAction )( 
            IInternetZoneManagerEx2 * This,
            /* [in] */ DWORD dwAction,
            /* [in] */ LPCWSTR pwszUrl,
            /* [in] */ LPCWSTR pwszText,
            /* [in] */ DWORD dwLogFlags);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, CreateZoneEnumerator)
        HRESULT ( STDMETHODCALLTYPE *CreateZoneEnumerator )( 
            IInternetZoneManagerEx2 * This,
            /* [out] */ DWORD *pdwEnum,
            /* [out] */ DWORD *pdwCount,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, GetZoneAt)
        HRESULT ( STDMETHODCALLTYPE *GetZoneAt )( 
            IInternetZoneManagerEx2 * This,
            /* [in] */ DWORD dwEnum,
            /* [in] */ DWORD dwIndex,
            /* [out] */ DWORD *pdwZone);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, DestroyZoneEnumerator)
        HRESULT ( STDMETHODCALLTYPE *DestroyZoneEnumerator )( 
            IInternetZoneManagerEx2 * This,
            /* [in] */ DWORD dwEnum);
        
        DECLSPEC_XFGVIRT(IInternetZoneManager, CopyTemplatePoliciesToZone)
        HRESULT ( STDMETHODCALLTYPE *CopyTemplatePoliciesToZone )( 
            IInternetZoneManagerEx2 * This,
            /* [in] */ DWORD dwTemplate,
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IInternetZoneManagerEx, GetZoneActionPolicyEx)
        HRESULT ( STDMETHODCALLTYPE *GetZoneActionPolicyEx )( 
            IInternetZoneManagerEx2 * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwAction,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IInternetZoneManagerEx, SetZoneActionPolicyEx)
        HRESULT ( STDMETHODCALLTYPE *SetZoneActionPolicyEx )( 
            IInternetZoneManagerEx2 * This,
            /* [in] */ DWORD dwZone,
            /* [in] */ DWORD dwAction,
            /* [annotation][size_is][in] */ 
            _In_reads_(cbPolicy)  BYTE *pPolicy,
            /* [in] */ DWORD cbPolicy,
            /* [in] */ URLZONEREG urlZoneReg,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IInternetZoneManagerEx2, GetZoneAttributesEx)
        HRESULT ( STDMETHODCALLTYPE *GetZoneAttributesEx )( 
            IInternetZoneManagerEx2 * This,
            /* [in] */ DWORD dwZone,
            /* [unique][out][in] */ ZONEATTRIBUTES *pZoneAttributes,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IInternetZoneManagerEx2, GetZoneSecurityState)
        HRESULT ( STDMETHODCALLTYPE *GetZoneSecurityState )( 
            IInternetZoneManagerEx2 * This,
            /* [in] */ DWORD dwZoneIndex,
            /* [in] */ BOOL fRespectPolicy,
            /* [out][in] */ LPDWORD pdwState,
            /* [out][in] */ BOOL *pfPolicyEncountered);
        
        DECLSPEC_XFGVIRT(IInternetZoneManagerEx2, GetIESecurityState)
        HRESULT ( STDMETHODCALLTYPE *GetIESecurityState )( 
            IInternetZoneManagerEx2 * This,
            /* [in] */ BOOL fRespectPolicy,
            /* [out][in] */ LPDWORD pdwState,
            /* [out][in] */ BOOL *pfPolicyEncountered,
            /* [in] */ BOOL fNoCache);
        
        DECLSPEC_XFGVIRT(IInternetZoneManagerEx2, FixUnsecureSettings)
        HRESULT ( STDMETHODCALLTYPE *FixUnsecureSettings )( 
            IInternetZoneManagerEx2 * This);
        
        END_INTERFACE
    } IInternetZoneManagerEx2Vtbl;

    interface IInternetZoneManagerEx2
    {
        CONST_VTBL struct IInternetZoneManagerEx2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternetZoneManagerEx2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternetZoneManagerEx2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternetZoneManagerEx2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternetZoneManagerEx2_GetZoneAttributes(This,dwZone,pZoneAttributes)	\
    ( (This)->lpVtbl -> GetZoneAttributes(This,dwZone,pZoneAttributes) ) 

#define IInternetZoneManagerEx2_SetZoneAttributes(This,dwZone,pZoneAttributes)	\
    ( (This)->lpVtbl -> SetZoneAttributes(This,dwZone,pZoneAttributes) ) 

#define IInternetZoneManagerEx2_GetZoneCustomPolicy(This,dwZone,guidKey,ppPolicy,pcbPolicy,urlZoneReg)	\
    ( (This)->lpVtbl -> GetZoneCustomPolicy(This,dwZone,guidKey,ppPolicy,pcbPolicy,urlZoneReg) ) 

#define IInternetZoneManagerEx2_SetZoneCustomPolicy(This,dwZone,guidKey,pPolicy,cbPolicy,urlZoneReg)	\
    ( (This)->lpVtbl -> SetZoneCustomPolicy(This,dwZone,guidKey,pPolicy,cbPolicy,urlZoneReg) ) 

#define IInternetZoneManagerEx2_GetZoneActionPolicy(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg)	\
    ( (This)->lpVtbl -> GetZoneActionPolicy(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg) ) 

#define IInternetZoneManagerEx2_SetZoneActionPolicy(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg)	\
    ( (This)->lpVtbl -> SetZoneActionPolicy(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg) ) 

#define IInternetZoneManagerEx2_PromptAction(This,dwAction,hwndParent,pwszUrl,pwszText,dwPromptFlags)	\
    ( (This)->lpVtbl -> PromptAction(This,dwAction,hwndParent,pwszUrl,pwszText,dwPromptFlags) ) 

#define IInternetZoneManagerEx2_LogAction(This,dwAction,pwszUrl,pwszText,dwLogFlags)	\
    ( (This)->lpVtbl -> LogAction(This,dwAction,pwszUrl,pwszText,dwLogFlags) ) 

#define IInternetZoneManagerEx2_CreateZoneEnumerator(This,pdwEnum,pdwCount,dwFlags)	\
    ( (This)->lpVtbl -> CreateZoneEnumerator(This,pdwEnum,pdwCount,dwFlags) ) 

#define IInternetZoneManagerEx2_GetZoneAt(This,dwEnum,dwIndex,pdwZone)	\
    ( (This)->lpVtbl -> GetZoneAt(This,dwEnum,dwIndex,pdwZone) ) 

#define IInternetZoneManagerEx2_DestroyZoneEnumerator(This,dwEnum)	\
    ( (This)->lpVtbl -> DestroyZoneEnumerator(This,dwEnum) ) 

#define IInternetZoneManagerEx2_CopyTemplatePoliciesToZone(This,dwTemplate,dwZone,dwReserved)	\
    ( (This)->lpVtbl -> CopyTemplatePoliciesToZone(This,dwTemplate,dwZone,dwReserved) ) 


#define IInternetZoneManagerEx2_GetZoneActionPolicyEx(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg,dwFlags)	\
    ( (This)->lpVtbl -> GetZoneActionPolicyEx(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg,dwFlags) ) 

#define IInternetZoneManagerEx2_SetZoneActionPolicyEx(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg,dwFlags)	\
    ( (This)->lpVtbl -> SetZoneActionPolicyEx(This,dwZone,dwAction,pPolicy,cbPolicy,urlZoneReg,dwFlags) ) 


#define IInternetZoneManagerEx2_GetZoneAttributesEx(This,dwZone,pZoneAttributes,dwFlags)	\
    ( (This)->lpVtbl -> GetZoneAttributesEx(This,dwZone,pZoneAttributes,dwFlags) ) 

#define IInternetZoneManagerEx2_GetZoneSecurityState(This,dwZoneIndex,fRespectPolicy,pdwState,pfPolicyEncountered)	\
    ( (This)->lpVtbl -> GetZoneSecurityState(This,dwZoneIndex,fRespectPolicy,pdwState,pfPolicyEncountered) ) 

#define IInternetZoneManagerEx2_GetIESecurityState(This,fRespectPolicy,pdwState,pfPolicyEncountered,fNoCache)	\
    ( (This)->lpVtbl -> GetIESecurityState(This,fRespectPolicy,pdwState,pfPolicyEncountered,fNoCache) ) 

#define IInternetZoneManagerEx2_FixUnsecureSettings(This)	\
    ( (This)->lpVtbl -> FixUnsecureSettings(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternetZoneManagerEx2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0047 */
/* [local] */ 

#endif
#endif //(_WIN32_IE >= _WIN32_IE_IE70)
EXTERN_C const IID CLSID_SoftDistExt;  
#ifndef _LPSOFTDISTEXT_DEFINED
#define _LPSOFTDISTEXT_DEFINED

#define SOFTDIST_FLAG_USAGE_EMAIL        0x00000001
#define SOFTDIST_FLAG_USAGE_PRECACHE     0x00000002
#define SOFTDIST_FLAG_USAGE_AUTOINSTALL  0x00000004
#define SOFTDIST_FLAG_DELETE_SUBSCRIPTION 0x00000008


#define SOFTDIST_ADSTATE_NONE                0x00000000
#define SOFTDIST_ADSTATE_AVAILABLE       0x00000001
#define SOFTDIST_ADSTATE_DOWNLOADED      0x00000002
#define SOFTDIST_ADSTATE_INSTALLED           0x00000003

typedef struct _tagCODEBASEHOLD
    {
    ULONG cbSize;
    LPWSTR szDistUnit;
    LPWSTR szCodeBase;
    DWORD dwVersionMS;
    DWORD dwVersionLS;
    DWORD dwStyle;
    } 	CODEBASEHOLD;

typedef struct _tagCODEBASEHOLD *LPCODEBASEHOLD;

typedef struct _tagSOFTDISTINFO
    {
    ULONG cbSize;
    DWORD dwFlags;
    DWORD dwAdState;
    LPWSTR szTitle;
    LPWSTR szAbstract;
    LPWSTR szHREF;
    DWORD dwInstalledVersionMS;
    DWORD dwInstalledVersionLS;
    DWORD dwUpdateVersionMS;
    DWORD dwUpdateVersionLS;
    DWORD dwAdvertisedVersionMS;
    DWORD dwAdvertisedVersionLS;
    DWORD dwReserved;
    } 	SOFTDISTINFO;

typedef struct _tagSOFTDISTINFO *LPSOFTDISTINFO;



extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0047_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0047_v0_0_s_ifspec;

#ifndef __ISoftDistExt_INTERFACE_DEFINED__
#define __ISoftDistExt_INTERFACE_DEFINED__

/* interface ISoftDistExt */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_ISoftDistExt;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B15B8DC1-C7E1-11d0-8680-00AA00BDCB71")
    ISoftDistExt : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ProcessSoftDist( 
            /* [in] */ LPCWSTR szCDFURL,
            /* [in] */ IXMLElement *pSoftDistElement,
            /* [out][in] */ LPSOFTDISTINFO lpsdi) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFirstCodeBase( 
            /* [annotation][in] */ 
            __RPC__in  LPWSTR *szCodeBase,
            /* [in] */ LPDWORD dwMaxSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextCodeBase( 
            /* [annotation][in] */ 
            __RPC__in  LPWSTR *szCodeBase,
            /* [in] */ LPDWORD dwMaxSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AsyncInstallDistributionUnit( 
            /* [in] */ IBindCtx *pbc,
            /* [in] */ LPVOID pvReserved,
            /* [in] */ DWORD flags,
            /* [in] */ LPCODEBASEHOLD lpcbh) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISoftDistExtVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISoftDistExt * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISoftDistExt * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISoftDistExt * This);
        
        DECLSPEC_XFGVIRT(ISoftDistExt, ProcessSoftDist)
        HRESULT ( STDMETHODCALLTYPE *ProcessSoftDist )( 
            ISoftDistExt * This,
            /* [in] */ LPCWSTR szCDFURL,
            /* [in] */ IXMLElement *pSoftDistElement,
            /* [out][in] */ LPSOFTDISTINFO lpsdi);
        
        DECLSPEC_XFGVIRT(ISoftDistExt, GetFirstCodeBase)
        HRESULT ( STDMETHODCALLTYPE *GetFirstCodeBase )( 
            ISoftDistExt * This,
            /* [annotation][in] */ 
            __RPC__in  LPWSTR *szCodeBase,
            /* [in] */ LPDWORD dwMaxSize);
        
        DECLSPEC_XFGVIRT(ISoftDistExt, GetNextCodeBase)
        HRESULT ( STDMETHODCALLTYPE *GetNextCodeBase )( 
            ISoftDistExt * This,
            /* [annotation][in] */ 
            __RPC__in  LPWSTR *szCodeBase,
            /* [in] */ LPDWORD dwMaxSize);
        
        DECLSPEC_XFGVIRT(ISoftDistExt, AsyncInstallDistributionUnit)
        HRESULT ( STDMETHODCALLTYPE *AsyncInstallDistributionUnit )( 
            ISoftDistExt * This,
            /* [in] */ IBindCtx *pbc,
            /* [in] */ LPVOID pvReserved,
            /* [in] */ DWORD flags,
            /* [in] */ LPCODEBASEHOLD lpcbh);
        
        END_INTERFACE
    } ISoftDistExtVtbl;

    interface ISoftDistExt
    {
        CONST_VTBL struct ISoftDistExtVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISoftDistExt_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISoftDistExt_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISoftDistExt_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISoftDistExt_ProcessSoftDist(This,szCDFURL,pSoftDistElement,lpsdi)	\
    ( (This)->lpVtbl -> ProcessSoftDist(This,szCDFURL,pSoftDistElement,lpsdi) ) 

#define ISoftDistExt_GetFirstCodeBase(This,szCodeBase,dwMaxSize)	\
    ( (This)->lpVtbl -> GetFirstCodeBase(This,szCodeBase,dwMaxSize) ) 

#define ISoftDistExt_GetNextCodeBase(This,szCodeBase,dwMaxSize)	\
    ( (This)->lpVtbl -> GetNextCodeBase(This,szCodeBase,dwMaxSize) ) 

#define ISoftDistExt_AsyncInstallDistributionUnit(This,pbc,pvReserved,flags,lpcbh)	\
    ( (This)->lpVtbl -> AsyncInstallDistributionUnit(This,pbc,pvReserved,flags,lpcbh) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISoftDistExt_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0048 */
/* [local] */ 

STDAPI GetSoftwareUpdateInfo( LPCWSTR szDistUnit, _Out_ LPSOFTDISTINFO psdi );
STDAPI SetSoftwareUpdateAdvertisementState( LPCWSTR szDistUnit, DWORD dwAdState, DWORD dwAdvertisedVersionMS, DWORD dwAdvertisedVersionLS );
#endif
#ifndef _LPCATALOGFILEINFO_DEFINED
#define _LPCATALOGFILEINFO_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0048_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0048_v0_0_s_ifspec;

#ifndef __ICatalogFileInfo_INTERFACE_DEFINED__
#define __ICatalogFileInfo_INTERFACE_DEFINED__

/* interface ICatalogFileInfo */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ ICatalogFileInfo *LPCATALOGFILEINFO;


EXTERN_C const IID IID_ICatalogFileInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("711C7600-6B48-11d1-B403-00AA00B92AF1")
    ICatalogFileInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCatalogFile( 
            /* [annotation][out] */ 
            __RPC__out  LPSTR *ppszCatalogFile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetJavaTrust( 
            /* [out] */ void **ppJavaTrust) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICatalogFileInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICatalogFileInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICatalogFileInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICatalogFileInfo * This);
        
        DECLSPEC_XFGVIRT(ICatalogFileInfo, GetCatalogFile)
        HRESULT ( STDMETHODCALLTYPE *GetCatalogFile )( 
            ICatalogFileInfo * This,
            /* [annotation][out] */ 
            __RPC__out  LPSTR *ppszCatalogFile);
        
        DECLSPEC_XFGVIRT(ICatalogFileInfo, GetJavaTrust)
        HRESULT ( STDMETHODCALLTYPE *GetJavaTrust )( 
            ICatalogFileInfo * This,
            /* [out] */ void **ppJavaTrust);
        
        END_INTERFACE
    } ICatalogFileInfoVtbl;

    interface ICatalogFileInfo
    {
        CONST_VTBL struct ICatalogFileInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICatalogFileInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICatalogFileInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICatalogFileInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICatalogFileInfo_GetCatalogFile(This,ppszCatalogFile)	\
    ( (This)->lpVtbl -> GetCatalogFile(This,ppszCatalogFile) ) 

#define ICatalogFileInfo_GetJavaTrust(This,ppJavaTrust)	\
    ( (This)->lpVtbl -> GetJavaTrust(This,ppJavaTrust) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICatalogFileInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0049 */
/* [local] */ 

#endif
#ifndef _LPDATAFILTER_DEFINED
#define _LPDATAFILTER_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0049_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0049_v0_0_s_ifspec;

#ifndef __IDataFilter_INTERFACE_DEFINED__
#define __IDataFilter_INTERFACE_DEFINED__

/* interface IDataFilter */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IDataFilter *LPDATAFILTER;


EXTERN_C const IID IID_IDataFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("69d14c80-c18e-11d0-a9ce-006097942311")
    IDataFilter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DoEncode( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ LONG lInBufferSize,
            /* [size_is][in] */ __RPC__in_ecount_full(lInBufferSize) BYTE *pbInBuffer,
            /* [in] */ LONG lOutBufferSize,
            /* [size_is][out] */ __RPC__out_ecount_full(lOutBufferSize) BYTE *pbOutBuffer,
            /* [in] */ LONG lInBytesAvailable,
            /* [out] */ __RPC__out LONG *plInBytesRead,
            /* [out] */ __RPC__out LONG *plOutBytesWritten,
            /* [in] */ DWORD dwReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DoDecode( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ LONG lInBufferSize,
            /* [size_is][in] */ __RPC__in_ecount_full(lInBufferSize) BYTE *pbInBuffer,
            /* [in] */ LONG lOutBufferSize,
            /* [size_is][out] */ __RPC__out_ecount_full(lOutBufferSize) BYTE *pbOutBuffer,
            /* [in] */ LONG lInBytesAvailable,
            /* [out] */ __RPC__out LONG *plInBytesRead,
            /* [out] */ __RPC__out LONG *plOutBytesWritten,
            /* [in] */ DWORD dwReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEncodingLevel( 
            /* [in] */ DWORD dwEncLevel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDataFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDataFilter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDataFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDataFilter * This);
        
        DECLSPEC_XFGVIRT(IDataFilter, DoEncode)
        HRESULT ( STDMETHODCALLTYPE *DoEncode )( 
            __RPC__in IDataFilter * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ LONG lInBufferSize,
            /* [size_is][in] */ __RPC__in_ecount_full(lInBufferSize) BYTE *pbInBuffer,
            /* [in] */ LONG lOutBufferSize,
            /* [size_is][out] */ __RPC__out_ecount_full(lOutBufferSize) BYTE *pbOutBuffer,
            /* [in] */ LONG lInBytesAvailable,
            /* [out] */ __RPC__out LONG *plInBytesRead,
            /* [out] */ __RPC__out LONG *plOutBytesWritten,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IDataFilter, DoDecode)
        HRESULT ( STDMETHODCALLTYPE *DoDecode )( 
            __RPC__in IDataFilter * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ LONG lInBufferSize,
            /* [size_is][in] */ __RPC__in_ecount_full(lInBufferSize) BYTE *pbInBuffer,
            /* [in] */ LONG lOutBufferSize,
            /* [size_is][out] */ __RPC__out_ecount_full(lOutBufferSize) BYTE *pbOutBuffer,
            /* [in] */ LONG lInBytesAvailable,
            /* [out] */ __RPC__out LONG *plInBytesRead,
            /* [out] */ __RPC__out LONG *plOutBytesWritten,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IDataFilter, SetEncodingLevel)
        HRESULT ( STDMETHODCALLTYPE *SetEncodingLevel )( 
            __RPC__in IDataFilter * This,
            /* [in] */ DWORD dwEncLevel);
        
        END_INTERFACE
    } IDataFilterVtbl;

    interface IDataFilter
    {
        CONST_VTBL struct IDataFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDataFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDataFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDataFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDataFilter_DoEncode(This,dwFlags,lInBufferSize,pbInBuffer,lOutBufferSize,pbOutBuffer,lInBytesAvailable,plInBytesRead,plOutBytesWritten,dwReserved)	\
    ( (This)->lpVtbl -> DoEncode(This,dwFlags,lInBufferSize,pbInBuffer,lOutBufferSize,pbOutBuffer,lInBytesAvailable,plInBytesRead,plOutBytesWritten,dwReserved) ) 

#define IDataFilter_DoDecode(This,dwFlags,lInBufferSize,pbInBuffer,lOutBufferSize,pbOutBuffer,lInBytesAvailable,plInBytesRead,plOutBytesWritten,dwReserved)	\
    ( (This)->lpVtbl -> DoDecode(This,dwFlags,lInBufferSize,pbInBuffer,lOutBufferSize,pbOutBuffer,lInBytesAvailable,plInBytesRead,plOutBytesWritten,dwReserved) ) 

#define IDataFilter_SetEncodingLevel(This,dwEncLevel)	\
    ( (This)->lpVtbl -> SetEncodingLevel(This,dwEncLevel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDataFilter_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0050 */
/* [local] */ 

#endif
#ifndef _LPENCODINGFILTERFACTORY_DEFINED
#define _LPENCODINGFILTERFACTORY_DEFINED
typedef struct _tagPROTOCOLFILTERDATA
    {
    DWORD cbSize;
    IInternetProtocolSink *pProtocolSink;
    IInternetProtocol *pProtocol;
    IUnknown *pUnk;
    DWORD dwFilterFlags;
    } 	PROTOCOLFILTERDATA;



extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0050_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0050_v0_0_s_ifspec;

#ifndef __IEncodingFilterFactory_INTERFACE_DEFINED__
#define __IEncodingFilterFactory_INTERFACE_DEFINED__

/* interface IEncodingFilterFactory */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IEncodingFilterFactory *LPENCODINGFILTERFACTORY;

typedef struct _tagDATAINFO
    {
    ULONG ulTotalSize;
    ULONG ulavrPacketSize;
    ULONG ulConnectSpeed;
    ULONG ulProcessorSpeed;
    } 	DATAINFO;


EXTERN_C const IID IID_IEncodingFilterFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70bdde00-c18e-11d0-a9ce-006097942311")
    IEncodingFilterFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FindBestFilter( 
            /* [in] */ LPCWSTR pwzCodeIn,
            /* [in] */ LPCWSTR pwzCodeOut,
            /* [in] */ DATAINFO info,
            /* [out] */ IDataFilter **ppDF) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultFilter( 
            /* [in] */ LPCWSTR pwzCodeIn,
            /* [in] */ LPCWSTR pwzCodeOut,
            /* [out] */ IDataFilter **ppDF) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEncodingFilterFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEncodingFilterFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEncodingFilterFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEncodingFilterFactory * This);
        
        DECLSPEC_XFGVIRT(IEncodingFilterFactory, FindBestFilter)
        HRESULT ( STDMETHODCALLTYPE *FindBestFilter )( 
            IEncodingFilterFactory * This,
            /* [in] */ LPCWSTR pwzCodeIn,
            /* [in] */ LPCWSTR pwzCodeOut,
            /* [in] */ DATAINFO info,
            /* [out] */ IDataFilter **ppDF);
        
        DECLSPEC_XFGVIRT(IEncodingFilterFactory, GetDefaultFilter)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultFilter )( 
            IEncodingFilterFactory * This,
            /* [in] */ LPCWSTR pwzCodeIn,
            /* [in] */ LPCWSTR pwzCodeOut,
            /* [out] */ IDataFilter **ppDF);
        
        END_INTERFACE
    } IEncodingFilterFactoryVtbl;

    interface IEncodingFilterFactory
    {
        CONST_VTBL struct IEncodingFilterFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEncodingFilterFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEncodingFilterFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEncodingFilterFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEncodingFilterFactory_FindBestFilter(This,pwzCodeIn,pwzCodeOut,info,ppDF)	\
    ( (This)->lpVtbl -> FindBestFilter(This,pwzCodeIn,pwzCodeOut,info,ppDF) ) 

#define IEncodingFilterFactory_GetDefaultFilter(This,pwzCodeIn,pwzCodeOut,ppDF)	\
    ( (This)->lpVtbl -> GetDefaultFilter(This,pwzCodeIn,pwzCodeOut,ppDF) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEncodingFilterFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0051 */
/* [local] */ 

#endif
#ifndef _HITLOGGING_DEFINED
#define _HITLOGGING_DEFINED
// Logging-specific apis
BOOL WINAPI IsLoggingEnabledA(_In_ LPCSTR  pszUrl);                    
BOOL WINAPI IsLoggingEnabledW(_In_ LPCWSTR  pwszUrl);                  
#ifdef UNICODE                                                       
#define IsLoggingEnabled         IsLoggingEnabledW                   
#else                                                                
#define IsLoggingEnabled         IsLoggingEnabledA                   
#endif // !UNICODE                                                   
typedef struct _tagHIT_LOGGING_INFO
    {
    DWORD dwStructSize;
    LPSTR lpszLoggedUrlName;
    SYSTEMTIME StartTime;
    SYSTEMTIME EndTime;
    LPSTR lpszExtendedInfo;
    } 	HIT_LOGGING_INFO;

typedef struct _tagHIT_LOGGING_INFO *LPHIT_LOGGING_INFO;

BOOL WINAPI WriteHitLogging(_In_ LPHIT_LOGGING_INFO lpLogginginfo);    
#define CONFIRMSAFETYACTION_LOADOBJECT  0x00000001
struct CONFIRMSAFETY
    {
    CLSID clsid;
    IUnknown *pUnk;
    DWORD dwFlags;
    } ;
EXTERN_C const GUID GUID_CUSTOM_CONFIRMOBJECTSAFETY; 
#endif
#ifndef _LPIWRAPPEDPROTOCOL_DEFINED
#define _LPIWRAPPEDPROTOCOL_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0051_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0051_v0_0_s_ifspec;

#ifndef __IWrappedProtocol_INTERFACE_DEFINED__
#define __IWrappedProtocol_INTERFACE_DEFINED__

/* interface IWrappedProtocol */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IWrappedProtocol *LPIWRAPPEDPROTOCOL;


EXTERN_C const IID IID_IWrappedProtocol;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("53c84785-8425-4dc5-971b-e58d9c19f9b6")
    IWrappedProtocol : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetWrapperCode( 
            /* [out] */ LONG *pnCode,
            /* [in] */ DWORD_PTR dwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWrappedProtocolVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWrappedProtocol * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWrappedProtocol * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWrappedProtocol * This);
        
        DECLSPEC_XFGVIRT(IWrappedProtocol, GetWrapperCode)
        HRESULT ( STDMETHODCALLTYPE *GetWrapperCode )( 
            IWrappedProtocol * This,
            /* [out] */ LONG *pnCode,
            /* [in] */ DWORD_PTR dwReserved);
        
        END_INTERFACE
    } IWrappedProtocolVtbl;

    interface IWrappedProtocol
    {
        CONST_VTBL struct IWrappedProtocolVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWrappedProtocol_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWrappedProtocol_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWrappedProtocol_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWrappedProtocol_GetWrapperCode(This,pnCode,dwReserved)	\
    ( (This)->lpVtbl -> GetWrapperCode(This,pnCode,dwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWrappedProtocol_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0052 */
/* [local] */ 

#endif
#ifndef _LPGETBINDHANDLE_DEFINED
#define _LPGETBINDHANDLE_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0052_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0052_v0_0_s_ifspec;

#ifndef __IGetBindHandle_INTERFACE_DEFINED__
#define __IGetBindHandle_INTERFACE_DEFINED__

/* interface IGetBindHandle */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IGetBindHandle *LPGETBINDHANDLE;

typedef /* [public][public] */ 
enum __MIDL_IGetBindHandle_0001
    {
        BINDHANDLETYPES_APPCACHE	= 0,
        BINDHANDLETYPES_DEPENDENCY	= 0x1,
        BINDHANDLETYPES_COUNT	= ( BINDHANDLETYPES_DEPENDENCY + 1 ) 
    } 	BINDHANDLETYPES;


EXTERN_C const IID IID_IGetBindHandle;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AF0FF408-129D-4b20-91F0-02BD23D88352")
    IGetBindHandle : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetBindHandle( 
            /* [in] */ BINDHANDLETYPES enumRequestedHandle,
            /* [out] */ HANDLE *pRetHandle) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetBindHandleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IGetBindHandle * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IGetBindHandle * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IGetBindHandle * This);
        
        DECLSPEC_XFGVIRT(IGetBindHandle, GetBindHandle)
        HRESULT ( STDMETHODCALLTYPE *GetBindHandle )( 
            IGetBindHandle * This,
            /* [in] */ BINDHANDLETYPES enumRequestedHandle,
            /* [out] */ HANDLE *pRetHandle);
        
        END_INTERFACE
    } IGetBindHandleVtbl;

    interface IGetBindHandle
    {
        CONST_VTBL struct IGetBindHandleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetBindHandle_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetBindHandle_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetBindHandle_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetBindHandle_GetBindHandle(This,enumRequestedHandle,pRetHandle)	\
    ( (This)->lpVtbl -> GetBindHandle(This,enumRequestedHandle,pRetHandle) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetBindHandle_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0053 */
/* [local] */ 

#endif
#ifndef _XHRPLUGGABLEPROTOCOL_DEFINED
#define _XHRPLUGGABLEPROTOCOL_DEFINED
typedef struct _tagPROTOCOL_ARGUMENT
    {
    LPCWSTR szMethod;
    LPCWSTR szTargetUrl;
    } 	PROTOCOL_ARGUMENT;

typedef struct _tagPROTOCOL_ARGUMENT *LPPROTOCOL_ARGUMENT;

#endif
#ifndef _LPBINDCALLBACKREDIRECT_DEFINED
#define _LPBINDCALLBACKREDIRECT_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0053_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0053_v0_0_s_ifspec;

#ifndef __IBindCallbackRedirect_INTERFACE_DEFINED__
#define __IBindCallbackRedirect_INTERFACE_DEFINED__

/* interface IBindCallbackRedirect */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IBindCallbackRedirect *LPBINDCALLBACKREDIRECT;


EXTERN_C const IID IID_IBindCallbackRedirect;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("11C81BC2-121E-4ed5-B9C4-B430BD54F2C0")
    IBindCallbackRedirect : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Redirect( 
            /* [in] */ LPCWSTR lpcUrl,
            /* [out] */ VARIANT_BOOL *vbCancel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBindCallbackRedirectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IBindCallbackRedirect * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IBindCallbackRedirect * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IBindCallbackRedirect * This);
        
        DECLSPEC_XFGVIRT(IBindCallbackRedirect, Redirect)
        HRESULT ( STDMETHODCALLTYPE *Redirect )( 
            IBindCallbackRedirect * This,
            /* [in] */ LPCWSTR lpcUrl,
            /* [out] */ VARIANT_BOOL *vbCancel);
        
        END_INTERFACE
    } IBindCallbackRedirectVtbl;

    interface IBindCallbackRedirect
    {
        CONST_VTBL struct IBindCallbackRedirectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBindCallbackRedirect_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBindCallbackRedirect_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBindCallbackRedirect_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBindCallbackRedirect_Redirect(This,lpcUrl,vbCancel)	\
    ( (This)->lpVtbl -> Redirect(This,lpcUrl,vbCancel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBindCallbackRedirect_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0054 */
/* [local] */ 

#endif
#ifndef _LPIBINDHTTPSECURITY_DEFINED
#define _LPIBINDHTTPSECURITY_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0054_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0054_v0_0_s_ifspec;

#ifndef __IBindHttpSecurity_INTERFACE_DEFINED__
#define __IBindHttpSecurity_INTERFACE_DEFINED__

/* interface IBindHttpSecurity */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBindHttpSecurity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a9eda967-f50e-4a33-b358-206f6ef3086d")
    IBindHttpSecurity : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetIgnoreCertMask( 
            /* [ref][out] */ __RPC__out DWORD *pdwIgnoreCertMask) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBindHttpSecurityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBindHttpSecurity * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBindHttpSecurity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBindHttpSecurity * This);
        
        DECLSPEC_XFGVIRT(IBindHttpSecurity, GetIgnoreCertMask)
        HRESULT ( STDMETHODCALLTYPE *GetIgnoreCertMask )( 
            __RPC__in IBindHttpSecurity * This,
            /* [ref][out] */ __RPC__out DWORD *pdwIgnoreCertMask);
        
        END_INTERFACE
    } IBindHttpSecurityVtbl;

    interface IBindHttpSecurity
    {
        CONST_VTBL struct IBindHttpSecurityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBindHttpSecurity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBindHttpSecurity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBindHttpSecurity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBindHttpSecurity_GetIgnoreCertMask(This,pdwIgnoreCertMask)	\
    ( (This)->lpVtbl -> GetIgnoreCertMask(This,pdwIgnoreCertMask) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBindHttpSecurity_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_urlmon_0000_0055 */
/* [local] */ 

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#if _MSC_VER >= 1200
#pragma warning(pop)
#endif


extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0055_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_urlmon_0000_0055_v0_0_s_ifspec;

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

/* [local] */ HRESULT STDMETHODCALLTYPE IBinding_GetBindResult_Proxy( 
    IBinding * This,
    /* [out] */ CLSID *pclsidProtocol,
    /* [out] */ DWORD *pdwResult,
    /* [annotation][out] */ 
    __RPC__out  LPOLESTR *pszResult,
    /* [out][in] */ DWORD *pdwReserved);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IBinding_GetBindResult_Stub( 
    __RPC__in IBinding * This,
    /* [out] */ __RPC__out CLSID *pclsidProtocol,
    /* [out] */ __RPC__out DWORD *pdwResult,
    /* [out] */ __RPC__deref_out_opt LPOLESTR *pszResult,
    /* [in] */ DWORD dwReserved);

/* [local] */ HRESULT STDMETHODCALLTYPE IBindStatusCallback_GetBindInfo_Proxy( 
    IBindStatusCallback * This,
    /* [out] */ DWORD *grfBINDF,
    /* [unique][out][in] */ BINDINFO *pbindinfo);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IBindStatusCallback_GetBindInfo_Stub( 
    __RPC__in IBindStatusCallback * This,
    /* [out] */ __RPC__out DWORD *grfBINDF,
    /* [unique][out][in] */ __RPC__inout_opt RemBINDINFO *pbindinfo,
    /* [unique][out][in] */ __RPC__inout_opt RemSTGMEDIUM *pstgmed);

/* [local] */ HRESULT STDMETHODCALLTYPE IBindStatusCallback_OnDataAvailable_Proxy( 
    IBindStatusCallback * This,
    /* [in] */ DWORD grfBSCF,
    /* [in] */ DWORD dwSize,
    /* [in] */ FORMATETC *pformatetc,
    /* [in] */ STGMEDIUM *pstgmed);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IBindStatusCallback_OnDataAvailable_Stub( 
    __RPC__in IBindStatusCallback * This,
    /* [in] */ DWORD grfBSCF,
    /* [in] */ DWORD dwSize,
    /* [in] */ __RPC__in RemFORMATETC *pformatetc,
    /* [in] */ __RPC__in RemSTGMEDIUM *pstgmed);

/* [local] */ HRESULT STDMETHODCALLTYPE IBindStatusCallbackEx_GetBindInfoEx_Proxy( 
    IBindStatusCallbackEx * This,
    /* [out] */ DWORD *grfBINDF,
    /* [unique][out][in] */ BINDINFO *pbindinfo,
    /* [out] */ DWORD *grfBINDF2,
    /* [out] */ DWORD *pdwReserved);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IBindStatusCallbackEx_GetBindInfoEx_Stub( 
    __RPC__in IBindStatusCallbackEx * This,
    /* [out] */ __RPC__out DWORD *grfBINDF,
    /* [unique][out][in] */ __RPC__inout_opt RemBINDINFO *pbindinfo,
    /* [unique][out][in] */ __RPC__inout_opt RemSTGMEDIUM *pstgmed,
    /* [out] */ __RPC__out DWORD *grfBINDF2,
    /* [out] */ __RPC__out DWORD *pdwReserved);

/* [local] */ HRESULT STDMETHODCALLTYPE IWinInetInfo_QueryOption_Proxy( 
    IWinInetInfo * This,
    /* [in] */ DWORD dwOption,
    /* [size_is][out][in] */ LPVOID pBuffer,
    /* [out][in] */ DWORD *pcbBuf);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IWinInetInfo_QueryOption_Stub( 
    __RPC__in IWinInetInfo * This,
    /* [in] */ DWORD dwOption,
    /* [size_is][out][in] */ __RPC__inout_ecount_full(*pcbBuf) BYTE *pBuffer,
    /* [out][in] */ __RPC__inout DWORD *pcbBuf);

/* [local] */ HRESULT STDMETHODCALLTYPE IWinInetHttpInfo_QueryInfo_Proxy( 
    IWinInetHttpInfo * This,
    /* [in] */ DWORD dwOption,
    /* [size_is][out][in] */ LPVOID pBuffer,
    /* [out][in] */ DWORD *pcbBuf,
    /* [out][in] */ DWORD *pdwFlags,
    /* [out][in] */ DWORD *pdwReserved);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IWinInetHttpInfo_QueryInfo_Stub( 
    __RPC__in IWinInetHttpInfo * This,
    /* [in] */ DWORD dwOption,
    /* [size_is][out][in] */ __RPC__inout_ecount_full(*pcbBuf) BYTE *pBuffer,
    /* [out][in] */ __RPC__inout DWORD *pcbBuf,
    /* [out][in] */ __RPC__inout DWORD *pdwFlags,
    /* [out][in] */ __RPC__inout DWORD *pdwReserved);

/* [local] */ HRESULT STDMETHODCALLTYPE IBindHost_MonikerBindToStorage_Proxy( 
    IBindHost * This,
    /* [in] */ IMoniker *pMk,
    /* [in] */ IBindCtx *pBC,
    /* [in] */ IBindStatusCallback *pBSC,
    /* [in] */ REFIID riid,
    /* [out] */ void **ppvObj);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IBindHost_MonikerBindToStorage_Stub( 
    __RPC__in IBindHost * This,
    /* [unique][in] */ __RPC__in_opt IMoniker *pMk,
    /* [unique][in] */ __RPC__in_opt IBindCtx *pBC,
    /* [unique][in] */ __RPC__in_opt IBindStatusCallback *pBSC,
    /* [in] */ __RPC__in REFIID riid,
    /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvObj);

/* [local] */ HRESULT STDMETHODCALLTYPE IBindHost_MonikerBindToObject_Proxy( 
    IBindHost * This,
    /* [in] */ IMoniker *pMk,
    /* [in] */ IBindCtx *pBC,
    /* [in] */ IBindStatusCallback *pBSC,
    /* [in] */ REFIID riid,
    /* [out] */ void **ppvObj);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IBindHost_MonikerBindToObject_Stub( 
    __RPC__in IBindHost * This,
    /* [unique][in] */ __RPC__in_opt IMoniker *pMk,
    /* [unique][in] */ __RPC__in_opt IBindCtx *pBC,
    /* [unique][in] */ __RPC__in_opt IBindStatusCallback *pBSC,
    /* [in] */ __RPC__in REFIID riid,
    /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvObj);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


