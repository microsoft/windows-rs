

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

#ifndef __qmgr_h__
#define __qmgr_h__

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

#ifndef __IBackgroundCopyJob1_FWD_DEFINED__
#define __IBackgroundCopyJob1_FWD_DEFINED__
typedef interface IBackgroundCopyJob1 IBackgroundCopyJob1;

#endif 	/* __IBackgroundCopyJob1_FWD_DEFINED__ */


#ifndef __IEnumBackgroundCopyJobs1_FWD_DEFINED__
#define __IEnumBackgroundCopyJobs1_FWD_DEFINED__
typedef interface IEnumBackgroundCopyJobs1 IEnumBackgroundCopyJobs1;

#endif 	/* __IEnumBackgroundCopyJobs1_FWD_DEFINED__ */


#ifndef __IBackgroundCopyGroup_FWD_DEFINED__
#define __IBackgroundCopyGroup_FWD_DEFINED__
typedef interface IBackgroundCopyGroup IBackgroundCopyGroup;

#endif 	/* __IBackgroundCopyGroup_FWD_DEFINED__ */


#ifndef __IEnumBackgroundCopyGroups_FWD_DEFINED__
#define __IEnumBackgroundCopyGroups_FWD_DEFINED__
typedef interface IEnumBackgroundCopyGroups IEnumBackgroundCopyGroups;

#endif 	/* __IEnumBackgroundCopyGroups_FWD_DEFINED__ */


#ifndef __IBackgroundCopyCallback1_FWD_DEFINED__
#define __IBackgroundCopyCallback1_FWD_DEFINED__
typedef interface IBackgroundCopyCallback1 IBackgroundCopyCallback1;

#endif 	/* __IBackgroundCopyCallback1_FWD_DEFINED__ */


#ifndef __IBackgroundCopyQMgr_FWD_DEFINED__
#define __IBackgroundCopyQMgr_FWD_DEFINED__
typedef interface IBackgroundCopyQMgr IBackgroundCopyQMgr;

#endif 	/* __IBackgroundCopyQMgr_FWD_DEFINED__ */


#ifndef __BackgroundCopyQMgr_FWD_DEFINED__
#define __BackgroundCopyQMgr_FWD_DEFINED__

#ifdef __cplusplus
typedef class BackgroundCopyQMgr BackgroundCopyQMgr;
#else
typedef struct BackgroundCopyQMgr BackgroundCopyQMgr;
#endif /* __cplusplus */

#endif 	/* __BackgroundCopyQMgr_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "ocidl.h"
#include "docobj.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_qmgr_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
// Background Copy QMgr Public Interface
#define  QM_NOTIFY_FILE_DONE         0x00000001
#define  QM_NOTIFY_JOB_DONE          0x00000002
#define  QM_NOTIFY_GROUP_DONE        0x00000004
#define  QM_NOTIFY_DISABLE_NOTIFY    0x00000040
#define  QM_NOTIFY_USE_PROGRESSEX    0x00000080
#define  QM_STATUS_FILE_COMPLETE     0x00000001
#define  QM_STATUS_FILE_INCOMPLETE   0x00000002
#define  QM_STATUS_JOB_COMPLETE      0x00000004
#define  QM_STATUS_JOB_INCOMPLETE    0x00000008
#define  QM_STATUS_JOB_ERROR         0x00000010
#define  QM_STATUS_JOB_FOREGROUND    0x00000020
#define  QM_STATUS_GROUP_COMPLETE    0x00000040
#define  QM_STATUS_GROUP_INCOMPLETE  0x00000080
#define  QM_STATUS_GROUP_SUSPENDED   0x00000100
#define  QM_STATUS_GROUP_ERROR       0x00000200
#define  QM_STATUS_GROUP_FOREGROUND  0x00000400
#define  QM_PROTOCOL_HTTP            1
#define  QM_PROTOCOL_FTP             2
#define  QM_PROTOCOL_SMB             3
#define  QM_PROTOCOL_CUSTOM          4
#define  QM_PROGRESS_PERCENT_DONE    1
#define  QM_PROGRESS_TIME_DONE       2
#define  QM_PROGRESS_SIZE_DONE       3
#define  QM_E_INVALID_STATE          0x81001001
#define  QM_E_SERVICE_UNAVAILABLE    0x81001002
#define  QM_E_DOWNLOADER_UNAVAILABLE 0x81001003
#define  QM_E_ITEM_NOT_FOUND         0x81001004


extern RPC_IF_HANDLE __MIDL_itf_qmgr_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_qmgr_0000_0000_v0_0_s_ifspec;

#ifndef __IBackgroundCopyJob1_INTERFACE_DEFINED__
#define __IBackgroundCopyJob1_INTERFACE_DEFINED__

/* interface IBackgroundCopyJob1 */
/* [object][helpstring][uuid] */ 

typedef struct _FILESETINFO
    {
    BSTR bstrRemoteFile;
    BSTR bstrLocalFile;
    DWORD dwSizeHint;
    } 	FILESETINFO;


EXTERN_C const IID IID_IBackgroundCopyJob1;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("59f5553c-2031-4629-bb18-2645a6970947")
    IBackgroundCopyJob1 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CancelJob( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProgress( 
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out DWORD *pdwProgress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out] */ __RPC__out DWORD *pdwStatus,
            /* [out] */ __RPC__out DWORD *pdwWin32Result,
            /* [out] */ __RPC__out DWORD *pdwTransportResult,
            /* [out] */ __RPC__out DWORD *pdwNumOfRetries) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddFiles( 
            /* [in] */ ULONG cFileCount,
            /* [size_is][in] */ __RPC__in_ecount_full(cFileCount) FILESETINFO **ppFileSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFile( 
            /* [in] */ ULONG cFileIndex,
            /* [out] */ __RPC__out FILESETINFO *pFileInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFileCount( 
            /* [out] */ __RPC__out DWORD *pdwFileCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SwitchToForeground( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_JobID( 
            /* [out] */ __RPC__out GUID *pguidJobID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBackgroundCopyJob1Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBackgroundCopyJob1 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBackgroundCopyJob1 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBackgroundCopyJob1 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob1, CancelJob)
        HRESULT ( STDMETHODCALLTYPE *CancelJob )( 
            __RPC__in IBackgroundCopyJob1 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob1, GetProgress)
        HRESULT ( STDMETHODCALLTYPE *GetProgress )( 
            __RPC__in IBackgroundCopyJob1 * This,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out DWORD *pdwProgress);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob1, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IBackgroundCopyJob1 * This,
            /* [out] */ __RPC__out DWORD *pdwStatus,
            /* [out] */ __RPC__out DWORD *pdwWin32Result,
            /* [out] */ __RPC__out DWORD *pdwTransportResult,
            /* [out] */ __RPC__out DWORD *pdwNumOfRetries);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob1, AddFiles)
        HRESULT ( STDMETHODCALLTYPE *AddFiles )( 
            __RPC__in IBackgroundCopyJob1 * This,
            /* [in] */ ULONG cFileCount,
            /* [size_is][in] */ __RPC__in_ecount_full(cFileCount) FILESETINFO **ppFileSet);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob1, GetFile)
        HRESULT ( STDMETHODCALLTYPE *GetFile )( 
            __RPC__in IBackgroundCopyJob1 * This,
            /* [in] */ ULONG cFileIndex,
            /* [out] */ __RPC__out FILESETINFO *pFileInfo);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob1, GetFileCount)
        HRESULT ( STDMETHODCALLTYPE *GetFileCount )( 
            __RPC__in IBackgroundCopyJob1 * This,
            /* [out] */ __RPC__out DWORD *pdwFileCount);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob1, SwitchToForeground)
        HRESULT ( STDMETHODCALLTYPE *SwitchToForeground )( 
            __RPC__in IBackgroundCopyJob1 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob1, get_JobID)
        HRESULT ( STDMETHODCALLTYPE *get_JobID )( 
            __RPC__in IBackgroundCopyJob1 * This,
            /* [out] */ __RPC__out GUID *pguidJobID);
        
        END_INTERFACE
    } IBackgroundCopyJob1Vtbl;

    interface IBackgroundCopyJob1
    {
        CONST_VTBL struct IBackgroundCopyJob1Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBackgroundCopyJob1_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBackgroundCopyJob1_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBackgroundCopyJob1_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBackgroundCopyJob1_CancelJob(This)	\
    ( (This)->lpVtbl -> CancelJob(This) ) 

#define IBackgroundCopyJob1_GetProgress(This,dwFlags,pdwProgress)	\
    ( (This)->lpVtbl -> GetProgress(This,dwFlags,pdwProgress) ) 

#define IBackgroundCopyJob1_GetStatus(This,pdwStatus,pdwWin32Result,pdwTransportResult,pdwNumOfRetries)	\
    ( (This)->lpVtbl -> GetStatus(This,pdwStatus,pdwWin32Result,pdwTransportResult,pdwNumOfRetries) ) 

#define IBackgroundCopyJob1_AddFiles(This,cFileCount,ppFileSet)	\
    ( (This)->lpVtbl -> AddFiles(This,cFileCount,ppFileSet) ) 

#define IBackgroundCopyJob1_GetFile(This,cFileIndex,pFileInfo)	\
    ( (This)->lpVtbl -> GetFile(This,cFileIndex,pFileInfo) ) 

#define IBackgroundCopyJob1_GetFileCount(This,pdwFileCount)	\
    ( (This)->lpVtbl -> GetFileCount(This,pdwFileCount) ) 

#define IBackgroundCopyJob1_SwitchToForeground(This)	\
    ( (This)->lpVtbl -> SwitchToForeground(This) ) 

#define IBackgroundCopyJob1_get_JobID(This,pguidJobID)	\
    ( (This)->lpVtbl -> get_JobID(This,pguidJobID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBackgroundCopyJob1_INTERFACE_DEFINED__ */


#ifndef __IEnumBackgroundCopyJobs1_INTERFACE_DEFINED__
#define __IEnumBackgroundCopyJobs1_INTERFACE_DEFINED__

/* interface IEnumBackgroundCopyJobs1 */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumBackgroundCopyJobs1;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8baeba9d-8f1c-42c4-b82c-09ae79980d25")
    IEnumBackgroundCopyJobs1 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) GUID *rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumBackgroundCopyJobs1 **ppenum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out ULONG *puCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumBackgroundCopyJobs1Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumBackgroundCopyJobs1 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumBackgroundCopyJobs1 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumBackgroundCopyJobs1 * This);
        
        DECLSPEC_XFGVIRT(IEnumBackgroundCopyJobs1, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumBackgroundCopyJobs1 * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) GUID *rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumBackgroundCopyJobs1, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumBackgroundCopyJobs1 * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumBackgroundCopyJobs1, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumBackgroundCopyJobs1 * This);
        
        DECLSPEC_XFGVIRT(IEnumBackgroundCopyJobs1, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumBackgroundCopyJobs1 * This,
            /* [out] */ __RPC__deref_out_opt IEnumBackgroundCopyJobs1 **ppenum);
        
        DECLSPEC_XFGVIRT(IEnumBackgroundCopyJobs1, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IEnumBackgroundCopyJobs1 * This,
            /* [out] */ __RPC__out ULONG *puCount);
        
        END_INTERFACE
    } IEnumBackgroundCopyJobs1Vtbl;

    interface IEnumBackgroundCopyJobs1
    {
        CONST_VTBL struct IEnumBackgroundCopyJobs1Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumBackgroundCopyJobs1_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumBackgroundCopyJobs1_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumBackgroundCopyJobs1_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumBackgroundCopyJobs1_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumBackgroundCopyJobs1_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumBackgroundCopyJobs1_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumBackgroundCopyJobs1_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#define IEnumBackgroundCopyJobs1_GetCount(This,puCount)	\
    ( (This)->lpVtbl -> GetCount(This,puCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumBackgroundCopyJobs1_INTERFACE_DEFINED__ */


#ifndef __IBackgroundCopyGroup_INTERFACE_DEFINED__
#define __IBackgroundCopyGroup_INTERFACE_DEFINED__

/* interface IBackgroundCopyGroup */
/* [object][helpstring][uuid] */ 

typedef 
enum GROUPPROP
    {
        GROUPPROP_PRIORITY	= 0,
        GROUPPROP_REMOTEUSERID	= 1,
        GROUPPROP_REMOTEUSERPWD	= 2,
        GROUPPROP_LOCALUSERID	= 3,
        GROUPPROP_LOCALUSERPWD	= 4,
        GROUPPROP_PROTOCOLFLAGS	= 5,
        GROUPPROP_NOTIFYFLAGS	= 6,
        GROUPPROP_NOTIFYCLSID	= 7,
        GROUPPROP_PROGRESSSIZE	= 8,
        GROUPPROP_PROGRESSPERCENT	= 9,
        GROUPPROP_PROGRESSTIME	= 10,
        GROUPPROP_DISPLAYNAME	= 11,
        GROUPPROP_DESCRIPTION	= 12
    } 	GROUPPROP;


EXTERN_C const IID IID_IBackgroundCopyGroup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1ded80a7-53ea-424f-8a04-17fea9adc4f5")
    IBackgroundCopyGroup : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetProp( 
            /* [in] */ GROUPPROP propID,
            /* [out] */ __RPC__out VARIANT *pvarVal) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE SetProp( 
            /* [in] */ GROUPPROP propID,
            /* [in] */ VARIANT *pvarVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProgress( 
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out DWORD *pdwProgress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out] */ __RPC__out DWORD *pdwStatus,
            /* [out] */ __RPC__out DWORD *pdwJobIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetJob( 
            /* [in] */ GUID jobID,
            /* [out] */ __RPC__deref_out_opt IBackgroundCopyJob1 **ppJob) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SuspendGroup( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResumeGroup( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelGroup( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Size( 
            /* [out] */ __RPC__out DWORD *pdwSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_GroupID( 
            /* [out] */ __RPC__out GUID *pguidGroupID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateJob( 
            /* [in] */ GUID guidJobID,
            /* [out] */ __RPC__deref_out_opt IBackgroundCopyJob1 **ppJob) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumJobs( 
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IEnumBackgroundCopyJobs1 **ppEnumJobs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SwitchToForeground( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryNewJobInterface( 
            /* [in] */ __RPC__in REFIID iid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **pUnk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetNotificationPointer( 
            /* [in] */ __RPC__in REFIID iid,
            /* [in] */ __RPC__in_opt IUnknown *pUnk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBackgroundCopyGroupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBackgroundCopyGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBackgroundCopyGroup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBackgroundCopyGroup * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyGroup, GetProp)
        HRESULT ( STDMETHODCALLTYPE *GetProp )( 
            __RPC__in IBackgroundCopyGroup * This,
            /* [in] */ GROUPPROP propID,
            /* [out] */ __RPC__out VARIANT *pvarVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyGroup, SetProp)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetProp )( 
            IBackgroundCopyGroup * This,
            /* [in] */ GROUPPROP propID,
            /* [in] */ VARIANT *pvarVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyGroup, GetProgress)
        HRESULT ( STDMETHODCALLTYPE *GetProgress )( 
            __RPC__in IBackgroundCopyGroup * This,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out DWORD *pdwProgress);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyGroup, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IBackgroundCopyGroup * This,
            /* [out] */ __RPC__out DWORD *pdwStatus,
            /* [out] */ __RPC__out DWORD *pdwJobIndex);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyGroup, GetJob)
        HRESULT ( STDMETHODCALLTYPE *GetJob )( 
            __RPC__in IBackgroundCopyGroup * This,
            /* [in] */ GUID jobID,
            /* [out] */ __RPC__deref_out_opt IBackgroundCopyJob1 **ppJob);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyGroup, SuspendGroup)
        HRESULT ( STDMETHODCALLTYPE *SuspendGroup )( 
            __RPC__in IBackgroundCopyGroup * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyGroup, ResumeGroup)
        HRESULT ( STDMETHODCALLTYPE *ResumeGroup )( 
            __RPC__in IBackgroundCopyGroup * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyGroup, CancelGroup)
        HRESULT ( STDMETHODCALLTYPE *CancelGroup )( 
            __RPC__in IBackgroundCopyGroup * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyGroup, get_Size)
        HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in IBackgroundCopyGroup * This,
            /* [out] */ __RPC__out DWORD *pdwSize);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyGroup, get_GroupID)
        HRESULT ( STDMETHODCALLTYPE *get_GroupID )( 
            __RPC__in IBackgroundCopyGroup * This,
            /* [out] */ __RPC__out GUID *pguidGroupID);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyGroup, CreateJob)
        HRESULT ( STDMETHODCALLTYPE *CreateJob )( 
            __RPC__in IBackgroundCopyGroup * This,
            /* [in] */ GUID guidJobID,
            /* [out] */ __RPC__deref_out_opt IBackgroundCopyJob1 **ppJob);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyGroup, EnumJobs)
        HRESULT ( STDMETHODCALLTYPE *EnumJobs )( 
            __RPC__in IBackgroundCopyGroup * This,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IEnumBackgroundCopyJobs1 **ppEnumJobs);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyGroup, SwitchToForeground)
        HRESULT ( STDMETHODCALLTYPE *SwitchToForeground )( 
            __RPC__in IBackgroundCopyGroup * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyGroup, QueryNewJobInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryNewJobInterface )( 
            __RPC__in IBackgroundCopyGroup * This,
            /* [in] */ __RPC__in REFIID iid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **pUnk);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyGroup, SetNotificationPointer)
        HRESULT ( STDMETHODCALLTYPE *SetNotificationPointer )( 
            __RPC__in IBackgroundCopyGroup * This,
            /* [in] */ __RPC__in REFIID iid,
            /* [in] */ __RPC__in_opt IUnknown *pUnk);
        
        END_INTERFACE
    } IBackgroundCopyGroupVtbl;

    interface IBackgroundCopyGroup
    {
        CONST_VTBL struct IBackgroundCopyGroupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBackgroundCopyGroup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBackgroundCopyGroup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBackgroundCopyGroup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBackgroundCopyGroup_GetProp(This,propID,pvarVal)	\
    ( (This)->lpVtbl -> GetProp(This,propID,pvarVal) ) 

#define IBackgroundCopyGroup_SetProp(This,propID,pvarVal)	\
    ( (This)->lpVtbl -> SetProp(This,propID,pvarVal) ) 

#define IBackgroundCopyGroup_GetProgress(This,dwFlags,pdwProgress)	\
    ( (This)->lpVtbl -> GetProgress(This,dwFlags,pdwProgress) ) 

#define IBackgroundCopyGroup_GetStatus(This,pdwStatus,pdwJobIndex)	\
    ( (This)->lpVtbl -> GetStatus(This,pdwStatus,pdwJobIndex) ) 

#define IBackgroundCopyGroup_GetJob(This,jobID,ppJob)	\
    ( (This)->lpVtbl -> GetJob(This,jobID,ppJob) ) 

#define IBackgroundCopyGroup_SuspendGroup(This)	\
    ( (This)->lpVtbl -> SuspendGroup(This) ) 

#define IBackgroundCopyGroup_ResumeGroup(This)	\
    ( (This)->lpVtbl -> ResumeGroup(This) ) 

#define IBackgroundCopyGroup_CancelGroup(This)	\
    ( (This)->lpVtbl -> CancelGroup(This) ) 

#define IBackgroundCopyGroup_get_Size(This,pdwSize)	\
    ( (This)->lpVtbl -> get_Size(This,pdwSize) ) 

#define IBackgroundCopyGroup_get_GroupID(This,pguidGroupID)	\
    ( (This)->lpVtbl -> get_GroupID(This,pguidGroupID) ) 

#define IBackgroundCopyGroup_CreateJob(This,guidJobID,ppJob)	\
    ( (This)->lpVtbl -> CreateJob(This,guidJobID,ppJob) ) 

#define IBackgroundCopyGroup_EnumJobs(This,dwFlags,ppEnumJobs)	\
    ( (This)->lpVtbl -> EnumJobs(This,dwFlags,ppEnumJobs) ) 

#define IBackgroundCopyGroup_SwitchToForeground(This)	\
    ( (This)->lpVtbl -> SwitchToForeground(This) ) 

#define IBackgroundCopyGroup_QueryNewJobInterface(This,iid,pUnk)	\
    ( (This)->lpVtbl -> QueryNewJobInterface(This,iid,pUnk) ) 

#define IBackgroundCopyGroup_SetNotificationPointer(This,iid,pUnk)	\
    ( (This)->lpVtbl -> SetNotificationPointer(This,iid,pUnk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IBackgroundCopyGroup_InternalSetProp_Proxy( 
    __RPC__in IBackgroundCopyGroup * This,
    /* [in] */ GROUPPROP propID,
    /* [in] */ __RPC__in VARIANT *pvarVal);


void __RPC_STUB IBackgroundCopyGroup_InternalSetProp_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IBackgroundCopyGroup_INTERFACE_DEFINED__ */


#ifndef __IEnumBackgroundCopyGroups_INTERFACE_DEFINED__
#define __IEnumBackgroundCopyGroups_INTERFACE_DEFINED__

/* interface IEnumBackgroundCopyGroups */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumBackgroundCopyGroups;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d993e603-4aa4-47c5-8665-c20d39c2ba4f")
    IEnumBackgroundCopyGroups : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) GUID *rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumBackgroundCopyGroups **ppenum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out ULONG *puCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumBackgroundCopyGroupsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumBackgroundCopyGroups * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumBackgroundCopyGroups * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumBackgroundCopyGroups * This);
        
        DECLSPEC_XFGVIRT(IEnumBackgroundCopyGroups, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumBackgroundCopyGroups * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) GUID *rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumBackgroundCopyGroups, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumBackgroundCopyGroups * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumBackgroundCopyGroups, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumBackgroundCopyGroups * This);
        
        DECLSPEC_XFGVIRT(IEnumBackgroundCopyGroups, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumBackgroundCopyGroups * This,
            /* [out] */ __RPC__deref_out_opt IEnumBackgroundCopyGroups **ppenum);
        
        DECLSPEC_XFGVIRT(IEnumBackgroundCopyGroups, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IEnumBackgroundCopyGroups * This,
            /* [out] */ __RPC__out ULONG *puCount);
        
        END_INTERFACE
    } IEnumBackgroundCopyGroupsVtbl;

    interface IEnumBackgroundCopyGroups
    {
        CONST_VTBL struct IEnumBackgroundCopyGroupsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumBackgroundCopyGroups_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumBackgroundCopyGroups_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumBackgroundCopyGroups_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumBackgroundCopyGroups_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumBackgroundCopyGroups_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumBackgroundCopyGroups_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumBackgroundCopyGroups_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#define IEnumBackgroundCopyGroups_GetCount(This,puCount)	\
    ( (This)->lpVtbl -> GetCount(This,puCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumBackgroundCopyGroups_INTERFACE_DEFINED__ */


#ifndef __IBackgroundCopyCallback1_INTERFACE_DEFINED__
#define __IBackgroundCopyCallback1_INTERFACE_DEFINED__

/* interface IBackgroundCopyCallback1 */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IBackgroundCopyCallback1;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("084f6593-3800-4e08-9b59-99fa59addf82")
    IBackgroundCopyCallback1 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnStatus( 
            /* [in] */ __RPC__in_opt IBackgroundCopyGroup *pGroup,
            /* [in] */ __RPC__in_opt IBackgroundCopyJob1 *pJob,
            /* [in] */ DWORD dwFileIndex,
            /* [in] */ DWORD dwStatus,
            /* [in] */ DWORD dwNumOfRetries,
            /* [in] */ DWORD dwWin32Result,
            /* [in] */ DWORD dwTransportResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnProgress( 
            /* [in] */ DWORD ProgressType,
            /* [in] */ __RPC__in_opt IBackgroundCopyGroup *pGroup,
            /* [in] */ __RPC__in_opt IBackgroundCopyJob1 *pJob,
            /* [in] */ DWORD dwFileIndex,
            /* [in] */ DWORD dwProgressValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnProgressEx( 
            /* [in] */ DWORD ProgressType,
            /* [in] */ __RPC__in_opt IBackgroundCopyGroup *pGroup,
            /* [in] */ __RPC__in_opt IBackgroundCopyJob1 *pJob,
            /* [in] */ DWORD dwFileIndex,
            /* [in] */ DWORD dwProgressValue,
            /* [in] */ DWORD dwByteArraySize,
            /* [size_is][in] */ __RPC__in_ecount_full(dwByteArraySize) BYTE *pByte) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBackgroundCopyCallback1Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBackgroundCopyCallback1 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBackgroundCopyCallback1 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBackgroundCopyCallback1 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyCallback1, OnStatus)
        HRESULT ( STDMETHODCALLTYPE *OnStatus )( 
            __RPC__in IBackgroundCopyCallback1 * This,
            /* [in] */ __RPC__in_opt IBackgroundCopyGroup *pGroup,
            /* [in] */ __RPC__in_opt IBackgroundCopyJob1 *pJob,
            /* [in] */ DWORD dwFileIndex,
            /* [in] */ DWORD dwStatus,
            /* [in] */ DWORD dwNumOfRetries,
            /* [in] */ DWORD dwWin32Result,
            /* [in] */ DWORD dwTransportResult);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyCallback1, OnProgress)
        HRESULT ( STDMETHODCALLTYPE *OnProgress )( 
            __RPC__in IBackgroundCopyCallback1 * This,
            /* [in] */ DWORD ProgressType,
            /* [in] */ __RPC__in_opt IBackgroundCopyGroup *pGroup,
            /* [in] */ __RPC__in_opt IBackgroundCopyJob1 *pJob,
            /* [in] */ DWORD dwFileIndex,
            /* [in] */ DWORD dwProgressValue);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyCallback1, OnProgressEx)
        HRESULT ( STDMETHODCALLTYPE *OnProgressEx )( 
            __RPC__in IBackgroundCopyCallback1 * This,
            /* [in] */ DWORD ProgressType,
            /* [in] */ __RPC__in_opt IBackgroundCopyGroup *pGroup,
            /* [in] */ __RPC__in_opt IBackgroundCopyJob1 *pJob,
            /* [in] */ DWORD dwFileIndex,
            /* [in] */ DWORD dwProgressValue,
            /* [in] */ DWORD dwByteArraySize,
            /* [size_is][in] */ __RPC__in_ecount_full(dwByteArraySize) BYTE *pByte);
        
        END_INTERFACE
    } IBackgroundCopyCallback1Vtbl;

    interface IBackgroundCopyCallback1
    {
        CONST_VTBL struct IBackgroundCopyCallback1Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBackgroundCopyCallback1_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBackgroundCopyCallback1_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBackgroundCopyCallback1_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBackgroundCopyCallback1_OnStatus(This,pGroup,pJob,dwFileIndex,dwStatus,dwNumOfRetries,dwWin32Result,dwTransportResult)	\
    ( (This)->lpVtbl -> OnStatus(This,pGroup,pJob,dwFileIndex,dwStatus,dwNumOfRetries,dwWin32Result,dwTransportResult) ) 

#define IBackgroundCopyCallback1_OnProgress(This,ProgressType,pGroup,pJob,dwFileIndex,dwProgressValue)	\
    ( (This)->lpVtbl -> OnProgress(This,ProgressType,pGroup,pJob,dwFileIndex,dwProgressValue) ) 

#define IBackgroundCopyCallback1_OnProgressEx(This,ProgressType,pGroup,pJob,dwFileIndex,dwProgressValue,dwByteArraySize,pByte)	\
    ( (This)->lpVtbl -> OnProgressEx(This,ProgressType,pGroup,pJob,dwFileIndex,dwProgressValue,dwByteArraySize,pByte) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBackgroundCopyCallback1_INTERFACE_DEFINED__ */


#ifndef __IBackgroundCopyQMgr_INTERFACE_DEFINED__
#define __IBackgroundCopyQMgr_INTERFACE_DEFINED__

/* interface IBackgroundCopyQMgr */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IBackgroundCopyQMgr;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("16f41c69-09f5-41d2-8cd8-3c08c47bc8a8")
    IBackgroundCopyQMgr : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateGroup( 
            /* [in] */ GUID guidGroupID,
            /* [out] */ __RPC__deref_out_opt IBackgroundCopyGroup **ppGroup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGroup( 
            /* [in] */ GUID groupID,
            /* [out] */ __RPC__deref_out_opt IBackgroundCopyGroup **ppGroup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumGroups( 
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IEnumBackgroundCopyGroups **ppEnumGroups) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBackgroundCopyQMgrVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBackgroundCopyQMgr * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBackgroundCopyQMgr * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBackgroundCopyQMgr * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyQMgr, CreateGroup)
        HRESULT ( STDMETHODCALLTYPE *CreateGroup )( 
            __RPC__in IBackgroundCopyQMgr * This,
            /* [in] */ GUID guidGroupID,
            /* [out] */ __RPC__deref_out_opt IBackgroundCopyGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyQMgr, GetGroup)
        HRESULT ( STDMETHODCALLTYPE *GetGroup )( 
            __RPC__in IBackgroundCopyQMgr * This,
            /* [in] */ GUID groupID,
            /* [out] */ __RPC__deref_out_opt IBackgroundCopyGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyQMgr, EnumGroups)
        HRESULT ( STDMETHODCALLTYPE *EnumGroups )( 
            __RPC__in IBackgroundCopyQMgr * This,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IEnumBackgroundCopyGroups **ppEnumGroups);
        
        END_INTERFACE
    } IBackgroundCopyQMgrVtbl;

    interface IBackgroundCopyQMgr
    {
        CONST_VTBL struct IBackgroundCopyQMgrVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBackgroundCopyQMgr_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBackgroundCopyQMgr_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBackgroundCopyQMgr_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBackgroundCopyQMgr_CreateGroup(This,guidGroupID,ppGroup)	\
    ( (This)->lpVtbl -> CreateGroup(This,guidGroupID,ppGroup) ) 

#define IBackgroundCopyQMgr_GetGroup(This,groupID,ppGroup)	\
    ( (This)->lpVtbl -> GetGroup(This,groupID,ppGroup) ) 

#define IBackgroundCopyQMgr_EnumGroups(This,dwFlags,ppEnumGroups)	\
    ( (This)->lpVtbl -> EnumGroups(This,dwFlags,ppEnumGroups) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBackgroundCopyQMgr_INTERFACE_DEFINED__ */



#ifndef __BackgroundCopyQMgr_LIBRARY_DEFINED__
#define __BackgroundCopyQMgr_LIBRARY_DEFINED__

/* library BackgroundCopyQMgr */
/* [version][lcid][helpstring][uuid] */ 


EXTERN_C const IID LIBID_BackgroundCopyQMgr;

EXTERN_C const CLSID CLSID_BackgroundCopyQMgr;

#ifdef __cplusplus

class DECLSPEC_UUID("69AD4AEE-51BE-439b-A92C-86AE490E8B30")
BackgroundCopyQMgr;
#endif
#endif /* __BackgroundCopyQMgr_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_qmgr_0000_0007 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_qmgr_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_qmgr_0000_0007_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* [local] */ HRESULT STDMETHODCALLTYPE IBackgroundCopyGroup_SetProp_Proxy( 
    IBackgroundCopyGroup * This,
    /* [in] */ GROUPPROP propID,
    /* [in] */ VARIANT *pvarVal);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IBackgroundCopyGroup_SetProp_Stub( 
    __RPC__in IBackgroundCopyGroup * This,
    /* [in] */ GROUPPROP propID,
    /* [in] */ __RPC__in VARIANT *pvarVal);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


