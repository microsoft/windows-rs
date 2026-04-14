

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


#ifndef __xpsprint_h__
#define __xpsprint_h__

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

#ifndef __IXpsPrintJobStream_FWD_DEFINED__
#define __IXpsPrintJobStream_FWD_DEFINED__
typedef interface IXpsPrintJobStream IXpsPrintJobStream;

#endif 	/* __IXpsPrintJobStream_FWD_DEFINED__ */


#ifndef __IXpsPrintJob_FWD_DEFINED__
#define __IXpsPrintJob_FWD_DEFINED__
typedef interface IXpsPrintJob IXpsPrintJob;

#endif 	/* __IXpsPrintJob_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "xpsobjectmodel.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_xpsprint_0000_0000 */
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
#if (NTDDI_VERSION >= NTDDI_WIN7)


extern RPC_IF_HANDLE __MIDL_itf_xpsprint_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xpsprint_0000_0000_v0_0_s_ifspec;


#ifndef __XpsPrint_LIBRARY_DEFINED__
#define __XpsPrint_LIBRARY_DEFINED__

/* library XpsPrint */
/* [helpstring][version][uuid] */ 



typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsprint_0000_0000_0001
    {
        XPS_JOB_IN_PROGRESS	= 0,
        XPS_JOB_COMPLETED	= ( XPS_JOB_IN_PROGRESS + 1 ) ,
        XPS_JOB_CANCELLED	= ( XPS_JOB_COMPLETED + 1 ) ,
        XPS_JOB_FAILED	= ( XPS_JOB_CANCELLED + 1 ) 
    } 	XPS_JOB_COMPLETION;

typedef /* [public][public] */ struct __MIDL___MIDL_itf_xpsprint_0000_0000_0002
    {
    UINT32 jobId;
    INT32 currentDocument;
    INT32 currentPage;
    INT32 currentPageTotal;
    XPS_JOB_COMPLETION completion;
    HRESULT jobStatus;
    } 	XPS_JOB_STATUS;


EXTERN_C const IID LIBID_XpsPrint;

#ifndef __IXpsPrintJobStream_INTERFACE_DEFINED__
#define __IXpsPrintJobStream_INTERFACE_DEFINED__

/* interface IXpsPrintJobStream */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsPrintJobStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7a77dc5f-45d6-4dff-9307-d8cb846347ca")
    IXpsPrintJobStream : public ISequentialStream
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsPrintJobStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsPrintJobStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsPrintJobStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsPrintJobStream * This);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Read)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            IXpsPrintJobStream * This,
            /* [annotation] */ 
            _Out_writes_bytes_to_(cb, *pcbRead)  void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbRead);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Write)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Write )( 
            IXpsPrintJobStream * This,
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbWritten);
        
        DECLSPEC_XFGVIRT(IXpsPrintJobStream, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IXpsPrintJobStream * This);
        
        END_INTERFACE
    } IXpsPrintJobStreamVtbl;

    interface IXpsPrintJobStream
    {
        CONST_VTBL struct IXpsPrintJobStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsPrintJobStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsPrintJobStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsPrintJobStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsPrintJobStream_Read(This,pv,cb,pcbRead)	\
    ( (This)->lpVtbl -> Read(This,pv,cb,pcbRead) ) 

#define IXpsPrintJobStream_Write(This,pv,cb,pcbWritten)	\
    ( (This)->lpVtbl -> Write(This,pv,cb,pcbWritten) ) 


#define IXpsPrintJobStream_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsPrintJobStream_INTERFACE_DEFINED__ */


#ifndef __IXpsPrintJob_INTERFACE_DEFINED__
#define __IXpsPrintJob_INTERFACE_DEFINED__

/* interface IXpsPrintJob */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsPrintJob;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5ab89b06-8194-425f-ab3b-d7a96e350161")
    IXpsPrintJob : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetJobStatus( 
            /* [retval][out] */ __RPC__out XPS_JOB_STATUS *jobStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsPrintJobVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsPrintJob * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsPrintJob * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsPrintJob * This);
        
        DECLSPEC_XFGVIRT(IXpsPrintJob, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IXpsPrintJob * This);
        
        DECLSPEC_XFGVIRT(IXpsPrintJob, GetJobStatus)
        HRESULT ( STDMETHODCALLTYPE *GetJobStatus )( 
            __RPC__in IXpsPrintJob * This,
            /* [retval][out] */ __RPC__out XPS_JOB_STATUS *jobStatus);
        
        END_INTERFACE
    } IXpsPrintJobVtbl;

    interface IXpsPrintJob
    {
        CONST_VTBL struct IXpsPrintJobVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsPrintJob_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsPrintJob_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsPrintJob_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsPrintJob_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IXpsPrintJob_GetJobStatus(This,jobStatus)	\
    ( (This)->lpVtbl -> GetJobStatus(This,jobStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsPrintJob_INTERFACE_DEFINED__ */



#ifndef __XpsPrint_MODULE_DEFINED__
#define __XpsPrint_MODULE_DEFINED__


/* module XpsPrint */
/* [dllname][uuid] */ 

/* [entry][helpstring] */ HRESULT __stdcall StartXpsPrintJob( 
    /* [string][in] */ __RPC__in_string LPCWSTR printerName,
    /* [string][in] */ __RPC__in_string LPCWSTR jobName,
    /* [string][in] */ __RPC__in_string LPCWSTR outputFileName,
    /* [in] */ __RPC__in HANDLE progressEvent,
    /* [in] */ __RPC__in HANDLE completionEvent,
    /* [size_is][in] */ __RPC__in_ecount_full(printablePagesOnCount) UINT8 *printablePagesOn,
    /* [in] */ UINT32 printablePagesOnCount,
    /* [out] */ __RPC__deref_out_opt IXpsPrintJob **xpsPrintJob,
    /* [out] */ __RPC__deref_out_opt IXpsPrintJobStream **documentStream,
    /* [out] */ __RPC__deref_out_opt IXpsPrintJobStream **printTicketStream);

/* [entry][helpstring] */ HRESULT __stdcall StartXpsPrintJob1( 
    /* [string][in] */ __RPC__in_string LPCWSTR printerName,
    /* [string][in] */ __RPC__in_string LPCWSTR jobName,
    /* [string][in] */ __RPC__in_string LPCWSTR outputFileName,
    /* [in] */ __RPC__in HANDLE progressEvent,
    /* [in] */ __RPC__in HANDLE completionEvent,
    /* [out] */ __RPC__deref_out_opt IXpsPrintJob **xpsPrintJob,
    /* [out] */ __RPC__deref_out_opt IXpsOMPackageTarget **printContentReceiver);

#endif /* __XpsPrint_MODULE_DEFINED__ */
#endif /* __XpsPrint_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_xpsprint_0000_0001 */
/* [local] */ 

#endif //(NTDDI_VERSION >= NTDDI_WIN7)
#pragma deprecated(StartXpsPrintJob,StartXpsPrintJob1,IXpsPrintJob,IXpsPrintStream,XPS_JOB_STATUS,XPS_JOB_COMPLETION)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_xpsprint_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xpsprint_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


