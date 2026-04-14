

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

#ifndef __bits1_5_h__
#define __bits1_5_h__

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

#ifndef __IBackgroundCopyJob2_FWD_DEFINED__
#define __IBackgroundCopyJob2_FWD_DEFINED__
typedef interface IBackgroundCopyJob2 IBackgroundCopyJob2;

#endif 	/* __IBackgroundCopyJob2_FWD_DEFINED__ */


#ifndef __BackgroundCopyManager1_5_FWD_DEFINED__
#define __BackgroundCopyManager1_5_FWD_DEFINED__

#ifdef __cplusplus
typedef class BackgroundCopyManager1_5 BackgroundCopyManager1_5;
#else
typedef struct BackgroundCopyManager1_5 BackgroundCopyManager1_5;
#endif /* __cplusplus */

#endif 	/* __BackgroundCopyManager1_5_FWD_DEFINED__ */


#ifndef __IBackgroundCopyJob2_FWD_DEFINED__
#define __IBackgroundCopyJob2_FWD_DEFINED__
typedef interface IBackgroundCopyJob2 IBackgroundCopyJob2;

#endif 	/* __IBackgroundCopyJob2_FWD_DEFINED__ */


/* header files for imported files */
#include "bits.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_bits1_5_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_bits1_5_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bits1_5_0000_0000_v0_0_s_ifspec;

#ifndef __IBackgroundCopyJob2_INTERFACE_DEFINED__
#define __IBackgroundCopyJob2_INTERFACE_DEFINED__

/* interface IBackgroundCopyJob2 */
/* [object][helpstring][uuid] */ 

typedef struct _BG_JOB_REPLY_PROGRESS
    {
    UINT64 BytesTotal;
    UINT64 BytesTransferred;
    } 	BG_JOB_REPLY_PROGRESS;

typedef 
enum BG_AUTH_TARGET
    {
        BG_AUTH_TARGET_SERVER	= 1,
        BG_AUTH_TARGET_PROXY	= ( BG_AUTH_TARGET_SERVER + 1 ) 
    } 	BG_AUTH_TARGET;

typedef 
enum BG_AUTH_SCHEME
    {
        BG_AUTH_SCHEME_BASIC	= 1,
        BG_AUTH_SCHEME_DIGEST	= ( BG_AUTH_SCHEME_BASIC + 1 ) ,
        BG_AUTH_SCHEME_NTLM	= ( BG_AUTH_SCHEME_DIGEST + 1 ) ,
        BG_AUTH_SCHEME_NEGOTIATE	= ( BG_AUTH_SCHEME_NTLM + 1 ) ,
        BG_AUTH_SCHEME_PASSPORT	= ( BG_AUTH_SCHEME_NEGOTIATE + 1 ) 
    } 	BG_AUTH_SCHEME;

typedef /* [public][public][public][public][public][public] */ struct __MIDL_IBackgroundCopyJob2_0001
    {
    LPWSTR UserName;
    LPWSTR Password;
    } 	BG_BASIC_CREDENTIALS;

typedef BG_BASIC_CREDENTIALS *PBG_BASIC_CREDENTIALS;

typedef /* [public][public][public][public][switch_type] */ union __MIDL_IBackgroundCopyJob2_0002
    {
    /* [case()] */ BG_BASIC_CREDENTIALS Basic;
    /* [default] */  /* Empty union arm */ 
    } 	BG_AUTH_CREDENTIALS_UNION;

typedef /* [public][public][public] */ struct __MIDL_IBackgroundCopyJob2_0003
    {
    BG_AUTH_TARGET Target;
    BG_AUTH_SCHEME Scheme;
    /* [switch_is] */ BG_AUTH_CREDENTIALS_UNION Credentials;
    } 	BG_AUTH_CREDENTIALS;

typedef BG_AUTH_CREDENTIALS *PBG_AUTH_CREDENTIALS;


EXTERN_C const IID IID_IBackgroundCopyJob2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("54b50739-686f-45eb-9dff-d6a9a0faa9af")
    IBackgroundCopyJob2 : public IBackgroundCopyJob
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetNotifyCmdLine( 
            /* [unique][in] */ __RPC__in_opt LPCWSTR Program,
            /* [unique][in] */ __RPC__in_opt LPCWSTR Parameters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNotifyCmdLine( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *pProgram,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pParameters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReplyProgress( 
            /* [out][in] */ __RPC__inout BG_JOB_REPLY_PROGRESS *pProgress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReplyData( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(( unsigned long  )*pLength) byte **ppBuffer,
            /* [unique][out][in] */ __RPC__inout_opt UINT64 *pLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetReplyFileName( 
            /* [unique][in] */ __RPC__in_opt LPCWSTR ReplyFileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReplyFileName( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *pReplyFileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCredentials( 
            __RPC__in BG_AUTH_CREDENTIALS *credentials) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveCredentials( 
            BG_AUTH_TARGET Target,
            BG_AUTH_SCHEME Scheme) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBackgroundCopyJob2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBackgroundCopyJob2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBackgroundCopyJob2 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, AddFileSet)
        HRESULT ( STDMETHODCALLTYPE *AddFileSet )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [in] */ ULONG cFileCount,
            /* [size_is][in] */ __RPC__in_ecount_full(cFileCount) BG_FILE_INFO *pFileSet);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, AddFile)
        HRESULT ( STDMETHODCALLTYPE *AddFile )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [in] */ __RPC__in LPCWSTR RemoteUrl,
            /* [in] */ __RPC__in LPCWSTR LocalName);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, EnumFiles)
        HRESULT ( STDMETHODCALLTYPE *EnumFiles )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__deref_out_opt IEnumBackgroundCopyFiles **pEnum);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, Suspend)
        HRESULT ( STDMETHODCALLTYPE *Suspend )( 
            __RPC__in IBackgroundCopyJob2 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, Resume)
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            __RPC__in IBackgroundCopyJob2 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IBackgroundCopyJob2 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, Complete)
        HRESULT ( STDMETHODCALLTYPE *Complete )( 
            __RPC__in IBackgroundCopyJob2 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetId)
        HRESULT ( STDMETHODCALLTYPE *GetId )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__out GUID *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__out BG_JOB_TYPE *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetProgress)
        HRESULT ( STDMETHODCALLTYPE *GetProgress )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__out BG_JOB_PROGRESS *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetTimes)
        HRESULT ( STDMETHODCALLTYPE *GetTimes )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__out BG_JOB_TIMES *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__out BG_JOB_STATE *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetError)
        HRESULT ( STDMETHODCALLTYPE *GetError )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__deref_out_opt IBackgroundCopyError **ppError);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *SetDisplayName )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [in] */ __RPC__in LPCWSTR Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetDescription)
        HRESULT ( STDMETHODCALLTYPE *SetDescription )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [in] */ __RPC__in LPCWSTR Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetPriority)
        HRESULT ( STDMETHODCALLTYPE *SetPriority )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [in] */ BG_JOB_PRIORITY Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetPriority)
        HRESULT ( STDMETHODCALLTYPE *GetPriority )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__out BG_JOB_PRIORITY *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetNotifyFlags)
        HRESULT ( STDMETHODCALLTYPE *SetNotifyFlags )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [in] */ ULONG Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetNotifyFlags)
        HRESULT ( STDMETHODCALLTYPE *GetNotifyFlags )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__out ULONG *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetNotifyInterface)
        HRESULT ( STDMETHODCALLTYPE *SetNotifyInterface )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [in] */ __RPC__in_opt IUnknown *Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetNotifyInterface)
        HRESULT ( STDMETHODCALLTYPE *GetNotifyInterface )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetMinimumRetryDelay)
        HRESULT ( STDMETHODCALLTYPE *SetMinimumRetryDelay )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [in] */ ULONG Seconds);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetMinimumRetryDelay)
        HRESULT ( STDMETHODCALLTYPE *GetMinimumRetryDelay )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__out ULONG *Seconds);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetNoProgressTimeout)
        HRESULT ( STDMETHODCALLTYPE *SetNoProgressTimeout )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [in] */ ULONG Seconds);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetNoProgressTimeout)
        HRESULT ( STDMETHODCALLTYPE *GetNoProgressTimeout )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__out ULONG *Seconds);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetErrorCount)
        HRESULT ( STDMETHODCALLTYPE *GetErrorCount )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__out ULONG *Errors);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetProxySettings)
        HRESULT ( STDMETHODCALLTYPE *SetProxySettings )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [in] */ BG_JOB_PROXY_USAGE ProxyUsage,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *ProxyList,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *ProxyBypassList);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetProxySettings)
        HRESULT ( STDMETHODCALLTYPE *GetProxySettings )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__out BG_JOB_PROXY_USAGE *pProxyUsage,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pProxyList,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pProxyBypassList);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, TakeOwnership)
        HRESULT ( STDMETHODCALLTYPE *TakeOwnership )( 
            __RPC__in IBackgroundCopyJob2 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, SetNotifyCmdLine)
        HRESULT ( STDMETHODCALLTYPE *SetNotifyCmdLine )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [unique][in] */ __RPC__in_opt LPCWSTR Program,
            /* [unique][in] */ __RPC__in_opt LPCWSTR Parameters);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, GetNotifyCmdLine)
        HRESULT ( STDMETHODCALLTYPE *GetNotifyCmdLine )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pProgram,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pParameters);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, GetReplyProgress)
        HRESULT ( STDMETHODCALLTYPE *GetReplyProgress )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out][in] */ __RPC__inout BG_JOB_REPLY_PROGRESS *pProgress);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, GetReplyData)
        HRESULT ( STDMETHODCALLTYPE *GetReplyData )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(( unsigned long  )*pLength) byte **ppBuffer,
            /* [unique][out][in] */ __RPC__inout_opt UINT64 *pLength);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, SetReplyFileName)
        HRESULT ( STDMETHODCALLTYPE *SetReplyFileName )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [unique][in] */ __RPC__in_opt LPCWSTR ReplyFileName);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, GetReplyFileName)
        HRESULT ( STDMETHODCALLTYPE *GetReplyFileName )( 
            __RPC__in IBackgroundCopyJob2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pReplyFileName);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, SetCredentials)
        HRESULT ( STDMETHODCALLTYPE *SetCredentials )( 
            __RPC__in IBackgroundCopyJob2 * This,
            __RPC__in BG_AUTH_CREDENTIALS *credentials);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, RemoveCredentials)
        HRESULT ( STDMETHODCALLTYPE *RemoveCredentials )( 
            __RPC__in IBackgroundCopyJob2 * This,
            BG_AUTH_TARGET Target,
            BG_AUTH_SCHEME Scheme);
        
        END_INTERFACE
    } IBackgroundCopyJob2Vtbl;

    interface IBackgroundCopyJob2
    {
        CONST_VTBL struct IBackgroundCopyJob2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBackgroundCopyJob2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBackgroundCopyJob2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBackgroundCopyJob2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBackgroundCopyJob2_AddFileSet(This,cFileCount,pFileSet)	\
    ( (This)->lpVtbl -> AddFileSet(This,cFileCount,pFileSet) ) 

#define IBackgroundCopyJob2_AddFile(This,RemoteUrl,LocalName)	\
    ( (This)->lpVtbl -> AddFile(This,RemoteUrl,LocalName) ) 

#define IBackgroundCopyJob2_EnumFiles(This,pEnum)	\
    ( (This)->lpVtbl -> EnumFiles(This,pEnum) ) 

#define IBackgroundCopyJob2_Suspend(This)	\
    ( (This)->lpVtbl -> Suspend(This) ) 

#define IBackgroundCopyJob2_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#define IBackgroundCopyJob2_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IBackgroundCopyJob2_Complete(This)	\
    ( (This)->lpVtbl -> Complete(This) ) 

#define IBackgroundCopyJob2_GetId(This,pVal)	\
    ( (This)->lpVtbl -> GetId(This,pVal) ) 

#define IBackgroundCopyJob2_GetType(This,pVal)	\
    ( (This)->lpVtbl -> GetType(This,pVal) ) 

#define IBackgroundCopyJob2_GetProgress(This,pVal)	\
    ( (This)->lpVtbl -> GetProgress(This,pVal) ) 

#define IBackgroundCopyJob2_GetTimes(This,pVal)	\
    ( (This)->lpVtbl -> GetTimes(This,pVal) ) 

#define IBackgroundCopyJob2_GetState(This,pVal)	\
    ( (This)->lpVtbl -> GetState(This,pVal) ) 

#define IBackgroundCopyJob2_GetError(This,ppError)	\
    ( (This)->lpVtbl -> GetError(This,ppError) ) 

#define IBackgroundCopyJob2_GetOwner(This,pVal)	\
    ( (This)->lpVtbl -> GetOwner(This,pVal) ) 

#define IBackgroundCopyJob2_SetDisplayName(This,Val)	\
    ( (This)->lpVtbl -> SetDisplayName(This,Val) ) 

#define IBackgroundCopyJob2_GetDisplayName(This,pVal)	\
    ( (This)->lpVtbl -> GetDisplayName(This,pVal) ) 

#define IBackgroundCopyJob2_SetDescription(This,Val)	\
    ( (This)->lpVtbl -> SetDescription(This,Val) ) 

#define IBackgroundCopyJob2_GetDescription(This,pVal)	\
    ( (This)->lpVtbl -> GetDescription(This,pVal) ) 

#define IBackgroundCopyJob2_SetPriority(This,Val)	\
    ( (This)->lpVtbl -> SetPriority(This,Val) ) 

#define IBackgroundCopyJob2_GetPriority(This,pVal)	\
    ( (This)->lpVtbl -> GetPriority(This,pVal) ) 

#define IBackgroundCopyJob2_SetNotifyFlags(This,Val)	\
    ( (This)->lpVtbl -> SetNotifyFlags(This,Val) ) 

#define IBackgroundCopyJob2_GetNotifyFlags(This,pVal)	\
    ( (This)->lpVtbl -> GetNotifyFlags(This,pVal) ) 

#define IBackgroundCopyJob2_SetNotifyInterface(This,Val)	\
    ( (This)->lpVtbl -> SetNotifyInterface(This,Val) ) 

#define IBackgroundCopyJob2_GetNotifyInterface(This,pVal)	\
    ( (This)->lpVtbl -> GetNotifyInterface(This,pVal) ) 

#define IBackgroundCopyJob2_SetMinimumRetryDelay(This,Seconds)	\
    ( (This)->lpVtbl -> SetMinimumRetryDelay(This,Seconds) ) 

#define IBackgroundCopyJob2_GetMinimumRetryDelay(This,Seconds)	\
    ( (This)->lpVtbl -> GetMinimumRetryDelay(This,Seconds) ) 

#define IBackgroundCopyJob2_SetNoProgressTimeout(This,Seconds)	\
    ( (This)->lpVtbl -> SetNoProgressTimeout(This,Seconds) ) 

#define IBackgroundCopyJob2_GetNoProgressTimeout(This,Seconds)	\
    ( (This)->lpVtbl -> GetNoProgressTimeout(This,Seconds) ) 

#define IBackgroundCopyJob2_GetErrorCount(This,Errors)	\
    ( (This)->lpVtbl -> GetErrorCount(This,Errors) ) 

#define IBackgroundCopyJob2_SetProxySettings(This,ProxyUsage,ProxyList,ProxyBypassList)	\
    ( (This)->lpVtbl -> SetProxySettings(This,ProxyUsage,ProxyList,ProxyBypassList) ) 

#define IBackgroundCopyJob2_GetProxySettings(This,pProxyUsage,pProxyList,pProxyBypassList)	\
    ( (This)->lpVtbl -> GetProxySettings(This,pProxyUsage,pProxyList,pProxyBypassList) ) 

#define IBackgroundCopyJob2_TakeOwnership(This)	\
    ( (This)->lpVtbl -> TakeOwnership(This) ) 


#define IBackgroundCopyJob2_SetNotifyCmdLine(This,Program,Parameters)	\
    ( (This)->lpVtbl -> SetNotifyCmdLine(This,Program,Parameters) ) 

#define IBackgroundCopyJob2_GetNotifyCmdLine(This,pProgram,pParameters)	\
    ( (This)->lpVtbl -> GetNotifyCmdLine(This,pProgram,pParameters) ) 

#define IBackgroundCopyJob2_GetReplyProgress(This,pProgress)	\
    ( (This)->lpVtbl -> GetReplyProgress(This,pProgress) ) 

#define IBackgroundCopyJob2_GetReplyData(This,ppBuffer,pLength)	\
    ( (This)->lpVtbl -> GetReplyData(This,ppBuffer,pLength) ) 

#define IBackgroundCopyJob2_SetReplyFileName(This,ReplyFileName)	\
    ( (This)->lpVtbl -> SetReplyFileName(This,ReplyFileName) ) 

#define IBackgroundCopyJob2_GetReplyFileName(This,pReplyFileName)	\
    ( (This)->lpVtbl -> GetReplyFileName(This,pReplyFileName) ) 

#define IBackgroundCopyJob2_SetCredentials(This,credentials)	\
    ( (This)->lpVtbl -> SetCredentials(This,credentials) ) 

#define IBackgroundCopyJob2_RemoveCredentials(This,Target,Scheme)	\
    ( (This)->lpVtbl -> RemoveCredentials(This,Target,Scheme) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBackgroundCopyJob2_INTERFACE_DEFINED__ */



#ifndef __BackgroundCopyManager1_5_LIBRARY_DEFINED__
#define __BackgroundCopyManager1_5_LIBRARY_DEFINED__

/* library BackgroundCopyManager1_5 */
/* [version][lcid][helpstring][uuid] */ 




EXTERN_C const IID LIBID_BackgroundCopyManager1_5;

EXTERN_C const CLSID CLSID_BackgroundCopyManager1_5;

#ifdef __cplusplus

class DECLSPEC_UUID("f087771f-d74f-4c1a-bb8a-e16aca9124ea")
BackgroundCopyManager1_5;
#endif
#endif /* __BackgroundCopyManager1_5_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_bits1_5_0000_0002 */
/* [local] */ 

#include "bits2_0.h"
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_bits1_5_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bits1_5_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


