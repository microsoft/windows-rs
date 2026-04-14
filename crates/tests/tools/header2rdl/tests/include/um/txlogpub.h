

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

#ifndef __txlogpub_h__
#define __txlogpub_h__

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

#ifndef __ILog_FWD_DEFINED__
#define __ILog_FWD_DEFINED__
typedef interface ILog ILog;

#endif 	/* __ILog_FWD_DEFINED__ */


#ifndef __IFileBasedLogInit_FWD_DEFINED__
#define __IFileBasedLogInit_FWD_DEFINED__
typedef interface IFileBasedLogInit IFileBasedLogInit;

#endif 	/* __IFileBasedLogInit_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_txlogpub_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


// LSN
// LSN is the fundamental cookie returned from the log as the name of a
// newly-written  log record. LSNs from successively written records to a
// given log are always monotonically increasing. LSNs are directly
// comparable: lsn2 is later in the log than lsn1 if and only if as integers
// lsn2 > lsn1.
//
// Neither the value zero nor the value MAXLSN are ever used as the value of
// an actual LSN.

typedef LARGE_INTEGER LSN;

#define MAXLSN (0x7FFFFFFFFFFFFFFF)


// RECORD_READING_POLICY
// The RECORD_READING_POLICY enumeration values specify a hint about the
// order in which records will be read from a log.  It is used by
// ILog::SetAccessPolicyHint.

typedef 
enum RECORD_READING_POLICY
    {
        RECORD_READING_POLICY_FORWARD	= 1,
        RECORD_READING_POLICY_BACKWARD	= 2,
        RECORD_READING_POLICY_RANDOM	= 3
    } 	RECORD_READING_POLICY;



// ILog
// An interface to the lowest level of a log implementation. This level
// takes care of writing the records to disk in a stable manner. Recovery
// protocols, transaction awareness, and the like are provided by a higher
// semantic level.


extern RPC_IF_HANDLE __MIDL_itf_txlogpub_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_txlogpub_0000_0000_v0_0_s_ifspec;

#ifndef __ILog_INTERFACE_DEFINED__
#define __ILog_INTERFACE_DEFINED__

/* interface ILog */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ILog;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FF222117-0C6C-11d2-B89A-00C04FB9618A")
    ILog : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Force( 
            /* [in] */ LSN lsnMinToForce) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AppendRecord( 
            /* [size_is][in] */ __RPC__in_ecount_full(cBlob) BLOB *rgBlob,
            /* [in] */ ULONG cBlob,
            /* [in] */ BOOL fForceNow,
            /* [unique][in][out] */ __RPC__inout_opt LSN *plsn) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReadRecord( 
            /* [in] */ LSN lsnToRead,
            /* [unique][in][out] */ __RPC__inout_opt LSN *plsnPrev,
            /* [unique][in][out] */ __RPC__inout_opt LSN *plsnNext,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbData) BYTE **ppbData,
            /* [out] */ __RPC__out ULONG *pcbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReadRecordPrefix( 
            /* [in] */ LSN lsnToRead,
            /* [unique][in][out] */ __RPC__inout_opt LSN *plsnPrev,
            /* [unique][in][out] */ __RPC__inout_opt LSN *plsnNext,
            /* [size_is][out] */ __RPC__out_ecount_full(*pcbData) BYTE *pbData,
            /* [out][in] */ __RPC__inout ULONG *pcbData,
            /* [out] */ __RPC__out ULONG *pcbRecord) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLogLimits( 
            /* [unique][in][out] */ __RPC__inout_opt LSN *plsnFirst,
            /* [unique][in][out] */ __RPC__inout_opt LSN *plsnLast) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TruncatePrefix( 
            /* [in] */ LSN lsnFirstToKeep) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAccessPolicyHint( 
            /* [in] */ RECORD_READING_POLICY policy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILogVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILog * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILog * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILog * This);
        
        DECLSPEC_XFGVIRT(ILog, Force)
        HRESULT ( STDMETHODCALLTYPE *Force )( 
            __RPC__in ILog * This,
            /* [in] */ LSN lsnMinToForce);
        
        DECLSPEC_XFGVIRT(ILog, AppendRecord)
        HRESULT ( STDMETHODCALLTYPE *AppendRecord )( 
            __RPC__in ILog * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cBlob) BLOB *rgBlob,
            /* [in] */ ULONG cBlob,
            /* [in] */ BOOL fForceNow,
            /* [unique][in][out] */ __RPC__inout_opt LSN *plsn);
        
        DECLSPEC_XFGVIRT(ILog, ReadRecord)
        HRESULT ( STDMETHODCALLTYPE *ReadRecord )( 
            __RPC__in ILog * This,
            /* [in] */ LSN lsnToRead,
            /* [unique][in][out] */ __RPC__inout_opt LSN *plsnPrev,
            /* [unique][in][out] */ __RPC__inout_opt LSN *plsnNext,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbData) BYTE **ppbData,
            /* [out] */ __RPC__out ULONG *pcbData);
        
        DECLSPEC_XFGVIRT(ILog, ReadRecordPrefix)
        HRESULT ( STDMETHODCALLTYPE *ReadRecordPrefix )( 
            __RPC__in ILog * This,
            /* [in] */ LSN lsnToRead,
            /* [unique][in][out] */ __RPC__inout_opt LSN *plsnPrev,
            /* [unique][in][out] */ __RPC__inout_opt LSN *plsnNext,
            /* [size_is][out] */ __RPC__out_ecount_full(*pcbData) BYTE *pbData,
            /* [out][in] */ __RPC__inout ULONG *pcbData,
            /* [out] */ __RPC__out ULONG *pcbRecord);
        
        DECLSPEC_XFGVIRT(ILog, GetLogLimits)
        HRESULT ( STDMETHODCALLTYPE *GetLogLimits )( 
            __RPC__in ILog * This,
            /* [unique][in][out] */ __RPC__inout_opt LSN *plsnFirst,
            /* [unique][in][out] */ __RPC__inout_opt LSN *plsnLast);
        
        DECLSPEC_XFGVIRT(ILog, TruncatePrefix)
        HRESULT ( STDMETHODCALLTYPE *TruncatePrefix )( 
            __RPC__in ILog * This,
            /* [in] */ LSN lsnFirstToKeep);
        
        DECLSPEC_XFGVIRT(ILog, SetAccessPolicyHint)
        HRESULT ( STDMETHODCALLTYPE *SetAccessPolicyHint )( 
            __RPC__in ILog * This,
            /* [in] */ RECORD_READING_POLICY policy);
        
        END_INTERFACE
    } ILogVtbl;

    interface ILog
    {
        CONST_VTBL struct ILogVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILog_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILog_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILog_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILog_Force(This,lsnMinToForce)	\
    ( (This)->lpVtbl -> Force(This,lsnMinToForce) ) 

#define ILog_AppendRecord(This,rgBlob,cBlob,fForceNow,plsn)	\
    ( (This)->lpVtbl -> AppendRecord(This,rgBlob,cBlob,fForceNow,plsn) ) 

#define ILog_ReadRecord(This,lsnToRead,plsnPrev,plsnNext,ppbData,pcbData)	\
    ( (This)->lpVtbl -> ReadRecord(This,lsnToRead,plsnPrev,plsnNext,ppbData,pcbData) ) 

#define ILog_ReadRecordPrefix(This,lsnToRead,plsnPrev,plsnNext,pbData,pcbData,pcbRecord)	\
    ( (This)->lpVtbl -> ReadRecordPrefix(This,lsnToRead,plsnPrev,plsnNext,pbData,pcbData,pcbRecord) ) 

#define ILog_GetLogLimits(This,plsnFirst,plsnLast)	\
    ( (This)->lpVtbl -> GetLogLimits(This,plsnFirst,plsnLast) ) 

#define ILog_TruncatePrefix(This,lsnFirstToKeep)	\
    ( (This)->lpVtbl -> TruncatePrefix(This,lsnFirstToKeep) ) 

#define ILog_SetAccessPolicyHint(This,policy)	\
    ( (This)->lpVtbl -> SetAccessPolicyHint(This,policy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILog_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_txlogpub_0000_0001 */
/* [local] */ 



// IFileBasedLogInit
// An interface used to initialize an instance of a file based implementation of
// ILog.  This interface defines the single method InitNew, which is used to
// create a log on a new log file.  Objects that implement IFileBasedLogInit
// should also implement IPersistFile, to allow existing log files to be opened.


extern RPC_IF_HANDLE __MIDL_itf_txlogpub_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_txlogpub_0000_0001_v0_0_s_ifspec;

#ifndef __IFileBasedLogInit_INTERFACE_DEFINED__
#define __IFileBasedLogInit_INTERFACE_DEFINED__

/* interface IFileBasedLogInit */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IFileBasedLogInit;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00951E8C-1294-11d1-97E4-00C04FB9618A")
    IFileBasedLogInit : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitNew( 
            /* [in] */ __RPC__in LPCWSTR filename,
            /* [in] */ ULONG cbCapacityHint) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFileBasedLogInitVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFileBasedLogInit * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFileBasedLogInit * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFileBasedLogInit * This);
        
        DECLSPEC_XFGVIRT(IFileBasedLogInit, InitNew)
        HRESULT ( STDMETHODCALLTYPE *InitNew )( 
            __RPC__in IFileBasedLogInit * This,
            /* [in] */ __RPC__in LPCWSTR filename,
            /* [in] */ ULONG cbCapacityHint);
        
        END_INTERFACE
    } IFileBasedLogInitVtbl;

    interface IFileBasedLogInit
    {
        CONST_VTBL struct IFileBasedLogInitVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFileBasedLogInit_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFileBasedLogInit_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFileBasedLogInit_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFileBasedLogInit_InitNew(This,filename,cbCapacityHint)	\
    ( (This)->lpVtbl -> InitNew(This,filename,cbCapacityHint) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFileBasedLogInit_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_txlogpub_0000_0002 */
/* [local] */ 



EXTERN_C const CLSID CLSID_SimpleFileBasedLog;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_txlogpub_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_txlogpub_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


