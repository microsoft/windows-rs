

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

#ifndef __bits2_0_h__
#define __bits2_0_h__

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

#ifndef __IBackgroundCopyJob3_FWD_DEFINED__
#define __IBackgroundCopyJob3_FWD_DEFINED__
typedef interface IBackgroundCopyJob3 IBackgroundCopyJob3;

#endif 	/* __IBackgroundCopyJob3_FWD_DEFINED__ */


#ifndef __IBackgroundCopyFile2_FWD_DEFINED__
#define __IBackgroundCopyFile2_FWD_DEFINED__
typedef interface IBackgroundCopyFile2 IBackgroundCopyFile2;

#endif 	/* __IBackgroundCopyFile2_FWD_DEFINED__ */


#ifndef __BackgroundCopyManager2_0_FWD_DEFINED__
#define __BackgroundCopyManager2_0_FWD_DEFINED__

#ifdef __cplusplus
typedef class BackgroundCopyManager2_0 BackgroundCopyManager2_0;
#else
typedef struct BackgroundCopyManager2_0 BackgroundCopyManager2_0;
#endif /* __cplusplus */

#endif 	/* __BackgroundCopyManager2_0_FWD_DEFINED__ */


#ifndef __IBackgroundCopyJob3_FWD_DEFINED__
#define __IBackgroundCopyJob3_FWD_DEFINED__
typedef interface IBackgroundCopyJob3 IBackgroundCopyJob3;

#endif 	/* __IBackgroundCopyJob3_FWD_DEFINED__ */


#ifndef __IBackgroundCopyFile2_FWD_DEFINED__
#define __IBackgroundCopyFile2_FWD_DEFINED__
typedef interface IBackgroundCopyFile2 IBackgroundCopyFile2;

#endif 	/* __IBackgroundCopyFile2_FWD_DEFINED__ */


/* header files for imported files */
#include "bits.h"
#include "bits1_5.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_bits2_0_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define BG_LENGTH_TO_EOF     (UINT64)(-1)
typedef struct _BG_FILE_RANGE
    {
    UINT64 InitialOffset;
    UINT64 Length;
    } 	BG_FILE_RANGE;

#define BG_COPY_FILE_OWNER   1
#define BG_COPY_FILE_GROUP   2
#define BG_COPY_FILE_DACL    4
#define BG_COPY_FILE_SACL    8
#define BG_COPY_FILE_ALL    15


extern RPC_IF_HANDLE __MIDL_itf_bits2_0_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bits2_0_0000_0000_v0_0_s_ifspec;

#ifndef __IBackgroundCopyJob3_INTERFACE_DEFINED__
#define __IBackgroundCopyJob3_INTERFACE_DEFINED__

/* interface IBackgroundCopyJob3 */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IBackgroundCopyJob3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("443c8934-90ff-48ed-bcde-26f5c7450042")
    IBackgroundCopyJob3 : public IBackgroundCopyJob2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ReplaceRemotePrefix( 
            /* [in] */ __RPC__in LPCWSTR OldPrefix,
            /* [in] */ __RPC__in LPCWSTR NewPrefix) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddFileWithRanges( 
            /* [in] */ __RPC__in LPCWSTR RemoteUrl,
            /* [in] */ __RPC__in LPCWSTR LocalName,
            /* [in] */ DWORD RangeCount,
            /* [size_is][in] */ __RPC__in_ecount_full(RangeCount) BG_FILE_RANGE Ranges[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFileACLFlags( 
            /* [in] */ DWORD Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFileACLFlags( 
            /* [ref][out] */ __RPC__out DWORD *Flags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBackgroundCopyJob3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBackgroundCopyJob3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBackgroundCopyJob3 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, AddFileSet)
        HRESULT ( STDMETHODCALLTYPE *AddFileSet )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [in] */ ULONG cFileCount,
            /* [size_is][in] */ __RPC__in_ecount_full(cFileCount) BG_FILE_INFO *pFileSet);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, AddFile)
        HRESULT ( STDMETHODCALLTYPE *AddFile )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [in] */ __RPC__in LPCWSTR RemoteUrl,
            /* [in] */ __RPC__in LPCWSTR LocalName);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, EnumFiles)
        HRESULT ( STDMETHODCALLTYPE *EnumFiles )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__deref_out_opt IEnumBackgroundCopyFiles **pEnum);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, Suspend)
        HRESULT ( STDMETHODCALLTYPE *Suspend )( 
            __RPC__in IBackgroundCopyJob3 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, Resume)
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            __RPC__in IBackgroundCopyJob3 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IBackgroundCopyJob3 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, Complete)
        HRESULT ( STDMETHODCALLTYPE *Complete )( 
            __RPC__in IBackgroundCopyJob3 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetId)
        HRESULT ( STDMETHODCALLTYPE *GetId )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__out GUID *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__out BG_JOB_TYPE *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetProgress)
        HRESULT ( STDMETHODCALLTYPE *GetProgress )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__out BG_JOB_PROGRESS *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetTimes)
        HRESULT ( STDMETHODCALLTYPE *GetTimes )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__out BG_JOB_TIMES *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__out BG_JOB_STATE *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetError)
        HRESULT ( STDMETHODCALLTYPE *GetError )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__deref_out_opt IBackgroundCopyError **ppError);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *SetDisplayName )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [in] */ __RPC__in LPCWSTR Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetDescription)
        HRESULT ( STDMETHODCALLTYPE *SetDescription )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [in] */ __RPC__in LPCWSTR Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetPriority)
        HRESULT ( STDMETHODCALLTYPE *SetPriority )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [in] */ BG_JOB_PRIORITY Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetPriority)
        HRESULT ( STDMETHODCALLTYPE *GetPriority )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__out BG_JOB_PRIORITY *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetNotifyFlags)
        HRESULT ( STDMETHODCALLTYPE *SetNotifyFlags )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [in] */ ULONG Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetNotifyFlags)
        HRESULT ( STDMETHODCALLTYPE *GetNotifyFlags )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__out ULONG *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetNotifyInterface)
        HRESULT ( STDMETHODCALLTYPE *SetNotifyInterface )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [in] */ __RPC__in_opt IUnknown *Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetNotifyInterface)
        HRESULT ( STDMETHODCALLTYPE *GetNotifyInterface )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetMinimumRetryDelay)
        HRESULT ( STDMETHODCALLTYPE *SetMinimumRetryDelay )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [in] */ ULONG Seconds);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetMinimumRetryDelay)
        HRESULT ( STDMETHODCALLTYPE *GetMinimumRetryDelay )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__out ULONG *Seconds);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetNoProgressTimeout)
        HRESULT ( STDMETHODCALLTYPE *SetNoProgressTimeout )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [in] */ ULONG Seconds);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetNoProgressTimeout)
        HRESULT ( STDMETHODCALLTYPE *GetNoProgressTimeout )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__out ULONG *Seconds);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetErrorCount)
        HRESULT ( STDMETHODCALLTYPE *GetErrorCount )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__out ULONG *Errors);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetProxySettings)
        HRESULT ( STDMETHODCALLTYPE *SetProxySettings )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [in] */ BG_JOB_PROXY_USAGE ProxyUsage,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *ProxyList,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *ProxyBypassList);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetProxySettings)
        HRESULT ( STDMETHODCALLTYPE *GetProxySettings )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__out BG_JOB_PROXY_USAGE *pProxyUsage,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pProxyList,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pProxyBypassList);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, TakeOwnership)
        HRESULT ( STDMETHODCALLTYPE *TakeOwnership )( 
            __RPC__in IBackgroundCopyJob3 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, SetNotifyCmdLine)
        HRESULT ( STDMETHODCALLTYPE *SetNotifyCmdLine )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [unique][in] */ __RPC__in_opt LPCWSTR Program,
            /* [unique][in] */ __RPC__in_opt LPCWSTR Parameters);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, GetNotifyCmdLine)
        HRESULT ( STDMETHODCALLTYPE *GetNotifyCmdLine )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pProgram,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pParameters);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, GetReplyProgress)
        HRESULT ( STDMETHODCALLTYPE *GetReplyProgress )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out][in] */ __RPC__inout BG_JOB_REPLY_PROGRESS *pProgress);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, GetReplyData)
        HRESULT ( STDMETHODCALLTYPE *GetReplyData )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(( unsigned long  )*pLength) byte **ppBuffer,
            /* [unique][out][in] */ __RPC__inout_opt UINT64 *pLength);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, SetReplyFileName)
        HRESULT ( STDMETHODCALLTYPE *SetReplyFileName )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [unique][in] */ __RPC__in_opt LPCWSTR ReplyFileName);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, GetReplyFileName)
        HRESULT ( STDMETHODCALLTYPE *GetReplyFileName )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pReplyFileName);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, SetCredentials)
        HRESULT ( STDMETHODCALLTYPE *SetCredentials )( 
            __RPC__in IBackgroundCopyJob3 * This,
            __RPC__in BG_AUTH_CREDENTIALS *credentials);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, RemoveCredentials)
        HRESULT ( STDMETHODCALLTYPE *RemoveCredentials )( 
            __RPC__in IBackgroundCopyJob3 * This,
            BG_AUTH_TARGET Target,
            BG_AUTH_SCHEME Scheme);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob3, ReplaceRemotePrefix)
        HRESULT ( STDMETHODCALLTYPE *ReplaceRemotePrefix )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [in] */ __RPC__in LPCWSTR OldPrefix,
            /* [in] */ __RPC__in LPCWSTR NewPrefix);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob3, AddFileWithRanges)
        HRESULT ( STDMETHODCALLTYPE *AddFileWithRanges )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [in] */ __RPC__in LPCWSTR RemoteUrl,
            /* [in] */ __RPC__in LPCWSTR LocalName,
            /* [in] */ DWORD RangeCount,
            /* [size_is][in] */ __RPC__in_ecount_full(RangeCount) BG_FILE_RANGE Ranges[  ]);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob3, SetFileACLFlags)
        HRESULT ( STDMETHODCALLTYPE *SetFileACLFlags )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [in] */ DWORD Flags);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob3, GetFileACLFlags)
        HRESULT ( STDMETHODCALLTYPE *GetFileACLFlags )( 
            __RPC__in IBackgroundCopyJob3 * This,
            /* [ref][out] */ __RPC__out DWORD *Flags);
        
        END_INTERFACE
    } IBackgroundCopyJob3Vtbl;

    interface IBackgroundCopyJob3
    {
        CONST_VTBL struct IBackgroundCopyJob3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBackgroundCopyJob3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBackgroundCopyJob3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBackgroundCopyJob3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBackgroundCopyJob3_AddFileSet(This,cFileCount,pFileSet)	\
    ( (This)->lpVtbl -> AddFileSet(This,cFileCount,pFileSet) ) 

#define IBackgroundCopyJob3_AddFile(This,RemoteUrl,LocalName)	\
    ( (This)->lpVtbl -> AddFile(This,RemoteUrl,LocalName) ) 

#define IBackgroundCopyJob3_EnumFiles(This,pEnum)	\
    ( (This)->lpVtbl -> EnumFiles(This,pEnum) ) 

#define IBackgroundCopyJob3_Suspend(This)	\
    ( (This)->lpVtbl -> Suspend(This) ) 

#define IBackgroundCopyJob3_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#define IBackgroundCopyJob3_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IBackgroundCopyJob3_Complete(This)	\
    ( (This)->lpVtbl -> Complete(This) ) 

#define IBackgroundCopyJob3_GetId(This,pVal)	\
    ( (This)->lpVtbl -> GetId(This,pVal) ) 

#define IBackgroundCopyJob3_GetType(This,pVal)	\
    ( (This)->lpVtbl -> GetType(This,pVal) ) 

#define IBackgroundCopyJob3_GetProgress(This,pVal)	\
    ( (This)->lpVtbl -> GetProgress(This,pVal) ) 

#define IBackgroundCopyJob3_GetTimes(This,pVal)	\
    ( (This)->lpVtbl -> GetTimes(This,pVal) ) 

#define IBackgroundCopyJob3_GetState(This,pVal)	\
    ( (This)->lpVtbl -> GetState(This,pVal) ) 

#define IBackgroundCopyJob3_GetError(This,ppError)	\
    ( (This)->lpVtbl -> GetError(This,ppError) ) 

#define IBackgroundCopyJob3_GetOwner(This,pVal)	\
    ( (This)->lpVtbl -> GetOwner(This,pVal) ) 

#define IBackgroundCopyJob3_SetDisplayName(This,Val)	\
    ( (This)->lpVtbl -> SetDisplayName(This,Val) ) 

#define IBackgroundCopyJob3_GetDisplayName(This,pVal)	\
    ( (This)->lpVtbl -> GetDisplayName(This,pVal) ) 

#define IBackgroundCopyJob3_SetDescription(This,Val)	\
    ( (This)->lpVtbl -> SetDescription(This,Val) ) 

#define IBackgroundCopyJob3_GetDescription(This,pVal)	\
    ( (This)->lpVtbl -> GetDescription(This,pVal) ) 

#define IBackgroundCopyJob3_SetPriority(This,Val)	\
    ( (This)->lpVtbl -> SetPriority(This,Val) ) 

#define IBackgroundCopyJob3_GetPriority(This,pVal)	\
    ( (This)->lpVtbl -> GetPriority(This,pVal) ) 

#define IBackgroundCopyJob3_SetNotifyFlags(This,Val)	\
    ( (This)->lpVtbl -> SetNotifyFlags(This,Val) ) 

#define IBackgroundCopyJob3_GetNotifyFlags(This,pVal)	\
    ( (This)->lpVtbl -> GetNotifyFlags(This,pVal) ) 

#define IBackgroundCopyJob3_SetNotifyInterface(This,Val)	\
    ( (This)->lpVtbl -> SetNotifyInterface(This,Val) ) 

#define IBackgroundCopyJob3_GetNotifyInterface(This,pVal)	\
    ( (This)->lpVtbl -> GetNotifyInterface(This,pVal) ) 

#define IBackgroundCopyJob3_SetMinimumRetryDelay(This,Seconds)	\
    ( (This)->lpVtbl -> SetMinimumRetryDelay(This,Seconds) ) 

#define IBackgroundCopyJob3_GetMinimumRetryDelay(This,Seconds)	\
    ( (This)->lpVtbl -> GetMinimumRetryDelay(This,Seconds) ) 

#define IBackgroundCopyJob3_SetNoProgressTimeout(This,Seconds)	\
    ( (This)->lpVtbl -> SetNoProgressTimeout(This,Seconds) ) 

#define IBackgroundCopyJob3_GetNoProgressTimeout(This,Seconds)	\
    ( (This)->lpVtbl -> GetNoProgressTimeout(This,Seconds) ) 

#define IBackgroundCopyJob3_GetErrorCount(This,Errors)	\
    ( (This)->lpVtbl -> GetErrorCount(This,Errors) ) 

#define IBackgroundCopyJob3_SetProxySettings(This,ProxyUsage,ProxyList,ProxyBypassList)	\
    ( (This)->lpVtbl -> SetProxySettings(This,ProxyUsage,ProxyList,ProxyBypassList) ) 

#define IBackgroundCopyJob3_GetProxySettings(This,pProxyUsage,pProxyList,pProxyBypassList)	\
    ( (This)->lpVtbl -> GetProxySettings(This,pProxyUsage,pProxyList,pProxyBypassList) ) 

#define IBackgroundCopyJob3_TakeOwnership(This)	\
    ( (This)->lpVtbl -> TakeOwnership(This) ) 


#define IBackgroundCopyJob3_SetNotifyCmdLine(This,Program,Parameters)	\
    ( (This)->lpVtbl -> SetNotifyCmdLine(This,Program,Parameters) ) 

#define IBackgroundCopyJob3_GetNotifyCmdLine(This,pProgram,pParameters)	\
    ( (This)->lpVtbl -> GetNotifyCmdLine(This,pProgram,pParameters) ) 

#define IBackgroundCopyJob3_GetReplyProgress(This,pProgress)	\
    ( (This)->lpVtbl -> GetReplyProgress(This,pProgress) ) 

#define IBackgroundCopyJob3_GetReplyData(This,ppBuffer,pLength)	\
    ( (This)->lpVtbl -> GetReplyData(This,ppBuffer,pLength) ) 

#define IBackgroundCopyJob3_SetReplyFileName(This,ReplyFileName)	\
    ( (This)->lpVtbl -> SetReplyFileName(This,ReplyFileName) ) 

#define IBackgroundCopyJob3_GetReplyFileName(This,pReplyFileName)	\
    ( (This)->lpVtbl -> GetReplyFileName(This,pReplyFileName) ) 

#define IBackgroundCopyJob3_SetCredentials(This,credentials)	\
    ( (This)->lpVtbl -> SetCredentials(This,credentials) ) 

#define IBackgroundCopyJob3_RemoveCredentials(This,Target,Scheme)	\
    ( (This)->lpVtbl -> RemoveCredentials(This,Target,Scheme) ) 


#define IBackgroundCopyJob3_ReplaceRemotePrefix(This,OldPrefix,NewPrefix)	\
    ( (This)->lpVtbl -> ReplaceRemotePrefix(This,OldPrefix,NewPrefix) ) 

#define IBackgroundCopyJob3_AddFileWithRanges(This,RemoteUrl,LocalName,RangeCount,Ranges)	\
    ( (This)->lpVtbl -> AddFileWithRanges(This,RemoteUrl,LocalName,RangeCount,Ranges) ) 

#define IBackgroundCopyJob3_SetFileACLFlags(This,Flags)	\
    ( (This)->lpVtbl -> SetFileACLFlags(This,Flags) ) 

#define IBackgroundCopyJob3_GetFileACLFlags(This,Flags)	\
    ( (This)->lpVtbl -> GetFileACLFlags(This,Flags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBackgroundCopyJob3_INTERFACE_DEFINED__ */


#ifndef __IBackgroundCopyFile2_INTERFACE_DEFINED__
#define __IBackgroundCopyFile2_INTERFACE_DEFINED__

/* interface IBackgroundCopyFile2 */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IBackgroundCopyFile2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("83e81b93-0873-474d-8a8c-f2018b1a939c")
    IBackgroundCopyFile2 : public IBackgroundCopyFile
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFileRanges( 
            /* [unique][out][in] */ __RPC__inout_opt DWORD *RangeCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*RangeCount) BG_FILE_RANGE **Ranges) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRemoteName( 
            __RPC__in LPCWSTR Val) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBackgroundCopyFile2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBackgroundCopyFile2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBackgroundCopyFile2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBackgroundCopyFile2 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile, GetRemoteName)
        HRESULT ( STDMETHODCALLTYPE *GetRemoteName )( 
            __RPC__in IBackgroundCopyFile2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile, GetLocalName)
        HRESULT ( STDMETHODCALLTYPE *GetLocalName )( 
            __RPC__in IBackgroundCopyFile2 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile, GetProgress)
        HRESULT ( STDMETHODCALLTYPE *GetProgress )( 
            __RPC__in IBackgroundCopyFile2 * This,
            /* [out] */ __RPC__out BG_FILE_PROGRESS *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile2, GetFileRanges)
        HRESULT ( STDMETHODCALLTYPE *GetFileRanges )( 
            __RPC__in IBackgroundCopyFile2 * This,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *RangeCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*RangeCount) BG_FILE_RANGE **Ranges);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile2, SetRemoteName)
        HRESULT ( STDMETHODCALLTYPE *SetRemoteName )( 
            __RPC__in IBackgroundCopyFile2 * This,
            __RPC__in LPCWSTR Val);
        
        END_INTERFACE
    } IBackgroundCopyFile2Vtbl;

    interface IBackgroundCopyFile2
    {
        CONST_VTBL struct IBackgroundCopyFile2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBackgroundCopyFile2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBackgroundCopyFile2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBackgroundCopyFile2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBackgroundCopyFile2_GetRemoteName(This,pVal)	\
    ( (This)->lpVtbl -> GetRemoteName(This,pVal) ) 

#define IBackgroundCopyFile2_GetLocalName(This,pVal)	\
    ( (This)->lpVtbl -> GetLocalName(This,pVal) ) 

#define IBackgroundCopyFile2_GetProgress(This,pVal)	\
    ( (This)->lpVtbl -> GetProgress(This,pVal) ) 


#define IBackgroundCopyFile2_GetFileRanges(This,RangeCount,Ranges)	\
    ( (This)->lpVtbl -> GetFileRanges(This,RangeCount,Ranges) ) 

#define IBackgroundCopyFile2_SetRemoteName(This,Val)	\
    ( (This)->lpVtbl -> SetRemoteName(This,Val) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBackgroundCopyFile2_INTERFACE_DEFINED__ */



#ifndef __BackgroundCopyManager2_0_LIBRARY_DEFINED__
#define __BackgroundCopyManager2_0_LIBRARY_DEFINED__

/* library BackgroundCopyManager2_0 */
/* [version][lcid][helpstring][uuid] */ 





EXTERN_C const IID LIBID_BackgroundCopyManager2_0;

EXTERN_C const CLSID CLSID_BackgroundCopyManager2_0;

#ifdef __cplusplus

class DECLSPEC_UUID("6d18ad12-bde3-4393-b311-099c346e6df9")
BackgroundCopyManager2_0;
#endif
#endif /* __BackgroundCopyManager2_0_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_bits2_0_0000_0003 */
/* [local] */ 

#include "bits2_5.h"
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_bits2_0_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bits2_0_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


