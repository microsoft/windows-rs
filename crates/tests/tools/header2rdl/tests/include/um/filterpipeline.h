

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

#ifndef __filterpipeline_h__
#define __filterpipeline_h__

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

#ifndef __IPrintReadStream_FWD_DEFINED__
#define __IPrintReadStream_FWD_DEFINED__
typedef interface IPrintReadStream IPrintReadStream;

#endif 	/* __IPrintReadStream_FWD_DEFINED__ */


#ifndef __IPrintWriteStream_FWD_DEFINED__
#define __IPrintWriteStream_FWD_DEFINED__
typedef interface IPrintWriteStream IPrintWriteStream;

#endif 	/* __IPrintWriteStream_FWD_DEFINED__ */


#ifndef __IPrintWriteStreamFlush_FWD_DEFINED__
#define __IPrintWriteStreamFlush_FWD_DEFINED__
typedef interface IPrintWriteStreamFlush IPrintWriteStreamFlush;

#endif 	/* __IPrintWriteStreamFlush_FWD_DEFINED__ */


#ifndef __IInterFilterCommunicator_FWD_DEFINED__
#define __IInterFilterCommunicator_FWD_DEFINED__
typedef interface IInterFilterCommunicator IInterFilterCommunicator;

#endif 	/* __IInterFilterCommunicator_FWD_DEFINED__ */


#ifndef __IPrintPipelineManagerControl_FWD_DEFINED__
#define __IPrintPipelineManagerControl_FWD_DEFINED__
typedef interface IPrintPipelineManagerControl IPrintPipelineManagerControl;

#endif 	/* __IPrintPipelineManagerControl_FWD_DEFINED__ */


#ifndef __IPrintPipelinePropertyBag_FWD_DEFINED__
#define __IPrintPipelinePropertyBag_FWD_DEFINED__
typedef interface IPrintPipelinePropertyBag IPrintPipelinePropertyBag;

#endif 	/* __IPrintPipelinePropertyBag_FWD_DEFINED__ */


#ifndef __IPrintPipelineProgressReport_FWD_DEFINED__
#define __IPrintPipelineProgressReport_FWD_DEFINED__
typedef interface IPrintPipelineProgressReport IPrintPipelineProgressReport;

#endif 	/* __IPrintPipelineProgressReport_FWD_DEFINED__ */


#ifndef __IPrintClassObjectFactory_FWD_DEFINED__
#define __IPrintClassObjectFactory_FWD_DEFINED__
typedef interface IPrintClassObjectFactory IPrintClassObjectFactory;

#endif 	/* __IPrintClassObjectFactory_FWD_DEFINED__ */


#ifndef __IPrintPipelineFilter_FWD_DEFINED__
#define __IPrintPipelineFilter_FWD_DEFINED__
typedef interface IPrintPipelineFilter IPrintPipelineFilter;

#endif 	/* __IPrintPipelineFilter_FWD_DEFINED__ */


#ifndef __IXpsDocumentProvider_FWD_DEFINED__
#define __IXpsDocumentProvider_FWD_DEFINED__
typedef interface IXpsDocumentProvider IXpsDocumentProvider;

#endif 	/* __IXpsDocumentProvider_FWD_DEFINED__ */


#ifndef __IXpsDocumentConsumer_FWD_DEFINED__
#define __IXpsDocumentConsumer_FWD_DEFINED__
typedef interface IXpsDocumentConsumer IXpsDocumentConsumer;

#endif 	/* __IXpsDocumentConsumer_FWD_DEFINED__ */


#ifndef __IXpsDocument_FWD_DEFINED__
#define __IXpsDocument_FWD_DEFINED__
typedef interface IXpsDocument IXpsDocument;

#endif 	/* __IXpsDocument_FWD_DEFINED__ */


#ifndef __IFixedDocumentSequence_FWD_DEFINED__
#define __IFixedDocumentSequence_FWD_DEFINED__
typedef interface IFixedDocumentSequence IFixedDocumentSequence;

#endif 	/* __IFixedDocumentSequence_FWD_DEFINED__ */


#ifndef __IFixedDocument_FWD_DEFINED__
#define __IFixedDocument_FWD_DEFINED__
typedef interface IFixedDocument IFixedDocument;

#endif 	/* __IFixedDocument_FWD_DEFINED__ */


#ifndef __IPartBase_FWD_DEFINED__
#define __IPartBase_FWD_DEFINED__
typedef interface IPartBase IPartBase;

#endif 	/* __IPartBase_FWD_DEFINED__ */


#ifndef __IFixedPage_FWD_DEFINED__
#define __IFixedPage_FWD_DEFINED__
typedef interface IFixedPage IFixedPage;

#endif 	/* __IFixedPage_FWD_DEFINED__ */


#ifndef __IPartImage_FWD_DEFINED__
#define __IPartImage_FWD_DEFINED__
typedef interface IPartImage IPartImage;

#endif 	/* __IPartImage_FWD_DEFINED__ */


#ifndef __IPartFont_FWD_DEFINED__
#define __IPartFont_FWD_DEFINED__
typedef interface IPartFont IPartFont;

#endif 	/* __IPartFont_FWD_DEFINED__ */


#ifndef __IPartFont2_FWD_DEFINED__
#define __IPartFont2_FWD_DEFINED__
typedef interface IPartFont2 IPartFont2;

#endif 	/* __IPartFont2_FWD_DEFINED__ */


#ifndef __IPartThumbnail_FWD_DEFINED__
#define __IPartThumbnail_FWD_DEFINED__
typedef interface IPartThumbnail IPartThumbnail;

#endif 	/* __IPartThumbnail_FWD_DEFINED__ */


#ifndef __IPartPrintTicket_FWD_DEFINED__
#define __IPartPrintTicket_FWD_DEFINED__
typedef interface IPartPrintTicket IPartPrintTicket;

#endif 	/* __IPartPrintTicket_FWD_DEFINED__ */


#ifndef __IPartColorProfile_FWD_DEFINED__
#define __IPartColorProfile_FWD_DEFINED__
typedef interface IPartColorProfile IPartColorProfile;

#endif 	/* __IPartColorProfile_FWD_DEFINED__ */


#ifndef __IPartResourceDictionary_FWD_DEFINED__
#define __IPartResourceDictionary_FWD_DEFINED__
typedef interface IPartResourceDictionary IPartResourceDictionary;

#endif 	/* __IPartResourceDictionary_FWD_DEFINED__ */


#ifndef __IXpsPartIterator_FWD_DEFINED__
#define __IXpsPartIterator_FWD_DEFINED__
typedef interface IXpsPartIterator IXpsPartIterator;

#endif 	/* __IXpsPartIterator_FWD_DEFINED__ */


#ifndef __IPrintReadStreamFactory_FWD_DEFINED__
#define __IPrintReadStreamFactory_FWD_DEFINED__
typedef interface IPrintReadStreamFactory IPrintReadStreamFactory;

#endif 	/* __IPrintReadStreamFactory_FWD_DEFINED__ */


#ifndef __IPartDiscardControl_FWD_DEFINED__
#define __IPartDiscardControl_FWD_DEFINED__
typedef interface IPartDiscardControl IPartDiscardControl;

#endif 	/* __IPartDiscardControl_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "imgerror.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_filterpipeline_0000_0000 */
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
#define E_ELEMENT_NOT_FOUND  HRESULT_FROM_WIN32(ERROR_NOT_FOUND)
#define XPS_FP_PRINTER_NAME          L"PrinterName"
#define XPS_FP_PROGRESS_REPORT       L"ProgressReport"
#define XPS_FP_PRINTER_HANDLE        L"PrinterHandle"
#define XPS_FP_USER_PRINT_TICKET     L"PerUserPrintTicket"
#define XPS_FP_USER_TOKEN            L"UserSecurityToken"
#define XPS_FP_JOB_ID                L"PrintJobId"
#define XPS_FP_PRINT_CLASS_FACTORY   L"PrintClassFactory"
#define XPS_FP_OUTPUT_FILE           L"PrintOutputFileName"
#define XPS_FP_MS_CONTENT_TYPE           L"DriverMultiContentType"
#define XPS_FP_MS_CONTENT_TYPE_XPS       L"XPS"
#define XPS_FP_MS_CONTENT_TYPE_OPENXPS   L"OpenXPS"
#define XPS_FP_DRIVER_PROPERTY_BAG   L"DriverPropertyBag"
#define XPS_FP_QUEUE_PROPERTY_BAG    L"QueuePropertyBag"
#define XPS_FP_MERGED_DATAFILE_PATH  L"MergedDataFilePath"
#define XPS_FP_RESOURCE_DLL_PATHS    L"ResourceDLLPaths"
#define XPS_FP_JOB_LEVEL_PRINTTICKET    L"JobPrintTicket"
#define XPS_FP_PRINTDEVICECAPABILITIES    L"PrintDeviceCapabilities"
#define XPS_FP_FAX_JOB_PROPERTIES    L"JobFaxProperties"






















typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_filterpipeline_0000_0000_0001
    {
        Compression_NotCompressed	= 0,
        Compression_Normal	= ( Compression_NotCompressed + 1 ) ,
        Compression_Small	= ( Compression_Normal + 1 ) ,
        Compression_Fast	= ( Compression_Small + 1 ) 
    } 	EXpsCompressionOptions;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_filterpipeline_0000_0000_0002
    {
        Font_Normal	= 0,
        Font_Obfusticate	= ( Font_Normal + 1 ) 
    } 	EXpsFontOptions;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_filterpipeline_0000_0000_0003
    {
        XpsJob_DocumentSequenceAdded	= 0,
        XpsJob_FixedDocumentAdded	= ( XpsJob_DocumentSequenceAdded + 1 ) ,
        XpsJob_FixedPageAdded	= ( XpsJob_FixedDocumentAdded + 1 ) 
    } 	EXpsJobConsumption;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_filterpipeline_0000_0000_0004
    {
        Xps_Restricted_Font_Installable	= 0,
        Xps_Restricted_Font_NoEmbedding	= 0x2,
        Xps_Restricted_Font_PreviewPrint	= 0x4,
        Xps_Restricted_Font_Editable	= 0x8
    } 	EXpsFontRestriction;

//
// XPSDrv driver can specify filter service provider module using the <FilterServiceProvider>
// or <OptionalFilterServiceProvider> element in the filter pipeline configuration XML file.
// The service provider module must export the following DrvPopulateFilterServices function, which
// is called by the XPSDrv filter pipeline manager to allow the service provider to instantiate
// filter service objects in the filter pipeline property bag specified by the pPropertyBag parameter.
//
HRESULT
WINAPI
DrvPopulateFilterServices(
    _In_ IPrintPipelinePropertyBag   *pPropertyBag
    );


extern RPC_IF_HANDLE __MIDL_itf_filterpipeline_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_filterpipeline_0000_0000_v0_0_s_ifspec;

#ifndef __IPrintReadStream_INTERFACE_DEFINED__
#define __IPrintReadStream_INTERFACE_DEFINED__

/* interface IPrintReadStream */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPrintReadStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4d47a67c-66cc-4430-850e-daf466fe5bc4")
    IPrintReadStream : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Seek( 
            /* [annotation][in] */ 
            _In_  LONGLONG dlibMove,
            /* [annotation][in] */ 
            _In_  DWORD dwOrigin,
            /* [annotation][out] */ 
            _Out_opt_  ULONGLONG *plibNewPosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReadBytes( 
            /* [annotation][length_is][size_is][out] */ 
            _Out_writes_bytes_to_(cbRequested, *pcbRead)  void *pvBuffer,
            /* [annotation][in] */ 
            _In_  ULONG cbRequested,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbRead,
            /* [annotation][out] */ 
            _Out_  BOOL *pbEndOfFile) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintReadStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrintReadStream * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrintReadStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrintReadStream * This);
        
        DECLSPEC_XFGVIRT(IPrintReadStream, Seek)
        HRESULT ( STDMETHODCALLTYPE *Seek )( 
            IPrintReadStream * This,
            /* [annotation][in] */ 
            _In_  LONGLONG dlibMove,
            /* [annotation][in] */ 
            _In_  DWORD dwOrigin,
            /* [annotation][out] */ 
            _Out_opt_  ULONGLONG *plibNewPosition);
        
        DECLSPEC_XFGVIRT(IPrintReadStream, ReadBytes)
        HRESULT ( STDMETHODCALLTYPE *ReadBytes )( 
            IPrintReadStream * This,
            /* [annotation][length_is][size_is][out] */ 
            _Out_writes_bytes_to_(cbRequested, *pcbRead)  void *pvBuffer,
            /* [annotation][in] */ 
            _In_  ULONG cbRequested,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbRead,
            /* [annotation][out] */ 
            _Out_  BOOL *pbEndOfFile);
        
        END_INTERFACE
    } IPrintReadStreamVtbl;

    interface IPrintReadStream
    {
        CONST_VTBL struct IPrintReadStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintReadStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintReadStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintReadStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintReadStream_Seek(This,dlibMove,dwOrigin,plibNewPosition)	\
    ( (This)->lpVtbl -> Seek(This,dlibMove,dwOrigin,plibNewPosition) ) 

#define IPrintReadStream_ReadBytes(This,pvBuffer,cbRequested,pcbRead,pbEndOfFile)	\
    ( (This)->lpVtbl -> ReadBytes(This,pvBuffer,cbRequested,pcbRead,pbEndOfFile) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintReadStream_INTERFACE_DEFINED__ */


#ifndef __IPrintWriteStream_INTERFACE_DEFINED__
#define __IPrintWriteStream_INTERFACE_DEFINED__

/* interface IPrintWriteStream */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPrintWriteStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("65bb7f1b-371e-4571-8ac7-912f510c1a38")
    IPrintWriteStream : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE WriteBytes( 
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbBuffer)  const void *pvBuffer,
            /* [annotation][in] */ 
            _In_  ULONG cbBuffer,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbWritten) = 0;
        
        virtual void STDMETHODCALLTYPE Close( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintWriteStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrintWriteStream * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrintWriteStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrintWriteStream * This);
        
        DECLSPEC_XFGVIRT(IPrintWriteStream, WriteBytes)
        HRESULT ( STDMETHODCALLTYPE *WriteBytes )( 
            IPrintWriteStream * This,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbBuffer)  const void *pvBuffer,
            /* [annotation][in] */ 
            _In_  ULONG cbBuffer,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbWritten);
        
        DECLSPEC_XFGVIRT(IPrintWriteStream, Close)
        void ( STDMETHODCALLTYPE *Close )( 
            IPrintWriteStream * This);
        
        END_INTERFACE
    } IPrintWriteStreamVtbl;

    interface IPrintWriteStream
    {
        CONST_VTBL struct IPrintWriteStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintWriteStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintWriteStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintWriteStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintWriteStream_WriteBytes(This,pvBuffer,cbBuffer,pcbWritten)	\
    ( (This)->lpVtbl -> WriteBytes(This,pvBuffer,cbBuffer,pcbWritten) ) 

#define IPrintWriteStream_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintWriteStream_INTERFACE_DEFINED__ */


#ifndef __IPrintWriteStreamFlush_INTERFACE_DEFINED__
#define __IPrintWriteStreamFlush_INTERFACE_DEFINED__

/* interface IPrintWriteStreamFlush */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPrintWriteStreamFlush;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("07d11ff8-1753-4873-b749-6cdaf068e4c3")
    IPrintWriteStreamFlush : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FlushData( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintWriteStreamFlushVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrintWriteStreamFlush * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrintWriteStreamFlush * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrintWriteStreamFlush * This);
        
        DECLSPEC_XFGVIRT(IPrintWriteStreamFlush, FlushData)
        HRESULT ( STDMETHODCALLTYPE *FlushData )( 
            IPrintWriteStreamFlush * This);
        
        END_INTERFACE
    } IPrintWriteStreamFlushVtbl;

    interface IPrintWriteStreamFlush
    {
        CONST_VTBL struct IPrintWriteStreamFlushVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintWriteStreamFlush_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintWriteStreamFlush_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintWriteStreamFlush_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintWriteStreamFlush_FlushData(This)	\
    ( (This)->lpVtbl -> FlushData(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintWriteStreamFlush_INTERFACE_DEFINED__ */


#ifndef __IInterFilterCommunicator_INTERFACE_DEFINED__
#define __IInterFilterCommunicator_INTERFACE_DEFINED__

/* interface IInterFilterCommunicator */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IInterFilterCommunicator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4daf1e69-81fd-462d-940f-8cd3ddf56fca")
    IInterFilterCommunicator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RequestReader( 
            /* [annotation][out] */ 
            _Out_  void **ppIReader) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestWriter( 
            /* [annotation][out] */ 
            _Out_  void **ppIWriter) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInterFilterCommunicatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInterFilterCommunicator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInterFilterCommunicator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInterFilterCommunicator * This);
        
        DECLSPEC_XFGVIRT(IInterFilterCommunicator, RequestReader)
        HRESULT ( STDMETHODCALLTYPE *RequestReader )( 
            IInterFilterCommunicator * This,
            /* [annotation][out] */ 
            _Out_  void **ppIReader);
        
        DECLSPEC_XFGVIRT(IInterFilterCommunicator, RequestWriter)
        HRESULT ( STDMETHODCALLTYPE *RequestWriter )( 
            IInterFilterCommunicator * This,
            /* [annotation][out] */ 
            _Out_  void **ppIWriter);
        
        END_INTERFACE
    } IInterFilterCommunicatorVtbl;

    interface IInterFilterCommunicator
    {
        CONST_VTBL struct IInterFilterCommunicatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInterFilterCommunicator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInterFilterCommunicator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInterFilterCommunicator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInterFilterCommunicator_RequestReader(This,ppIReader)	\
    ( (This)->lpVtbl -> RequestReader(This,ppIReader) ) 

#define IInterFilterCommunicator_RequestWriter(This,ppIWriter)	\
    ( (This)->lpVtbl -> RequestWriter(This,ppIWriter) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInterFilterCommunicator_INTERFACE_DEFINED__ */


#ifndef __IPrintPipelineManagerControl_INTERFACE_DEFINED__
#define __IPrintPipelineManagerControl_INTERFACE_DEFINED__

/* interface IPrintPipelineManagerControl */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPrintPipelineManagerControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aa3e4910-5889-4681-91ef-823ad4ed4e44")
    IPrintPipelineManagerControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RequestShutdown( 
            /* [annotation][in] */ 
            _In_  HRESULT hrReason,
            /* [annotation][in] */ 
            _In_  IImgErrorInfo *pReason) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FilterFinished( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintPipelineManagerControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrintPipelineManagerControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrintPipelineManagerControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrintPipelineManagerControl * This);
        
        DECLSPEC_XFGVIRT(IPrintPipelineManagerControl, RequestShutdown)
        HRESULT ( STDMETHODCALLTYPE *RequestShutdown )( 
            IPrintPipelineManagerControl * This,
            /* [annotation][in] */ 
            _In_  HRESULT hrReason,
            /* [annotation][in] */ 
            _In_  IImgErrorInfo *pReason);
        
        DECLSPEC_XFGVIRT(IPrintPipelineManagerControl, FilterFinished)
        HRESULT ( STDMETHODCALLTYPE *FilterFinished )( 
            IPrintPipelineManagerControl * This);
        
        END_INTERFACE
    } IPrintPipelineManagerControlVtbl;

    interface IPrintPipelineManagerControl
    {
        CONST_VTBL struct IPrintPipelineManagerControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintPipelineManagerControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintPipelineManagerControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintPipelineManagerControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintPipelineManagerControl_RequestShutdown(This,hrReason,pReason)	\
    ( (This)->lpVtbl -> RequestShutdown(This,hrReason,pReason) ) 

#define IPrintPipelineManagerControl_FilterFinished(This)	\
    ( (This)->lpVtbl -> FilterFinished(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintPipelineManagerControl_INTERFACE_DEFINED__ */


#ifndef __IPrintPipelinePropertyBag_INTERFACE_DEFINED__
#define __IPrintPipelinePropertyBag_INTERFACE_DEFINED__

/* interface IPrintPipelinePropertyBag */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPrintPipelinePropertyBag;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8b8c99dc-7892-4a95-8a04-57422e9fbb47")
    IPrintPipelinePropertyBag : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddProperty( 
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *pszName,
            /* [annotation][in] */ 
            _In_  const VARIANT *pVar) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *pszName,
            /* [annotation][out] */ 
            _Out_  VARIANT *pVar) = 0;
        
        virtual BOOL STDMETHODCALLTYPE DeleteProperty( 
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *pszName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintPipelinePropertyBagVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrintPipelinePropertyBag * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrintPipelinePropertyBag * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrintPipelinePropertyBag * This);
        
        DECLSPEC_XFGVIRT(IPrintPipelinePropertyBag, AddProperty)
        HRESULT ( STDMETHODCALLTYPE *AddProperty )( 
            IPrintPipelinePropertyBag * This,
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *pszName,
            /* [annotation][in] */ 
            _In_  const VARIANT *pVar);
        
        DECLSPEC_XFGVIRT(IPrintPipelinePropertyBag, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            IPrintPipelinePropertyBag * This,
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *pszName,
            /* [annotation][out] */ 
            _Out_  VARIANT *pVar);
        
        DECLSPEC_XFGVIRT(IPrintPipelinePropertyBag, DeleteProperty)
        BOOL ( STDMETHODCALLTYPE *DeleteProperty )( 
            IPrintPipelinePropertyBag * This,
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *pszName);
        
        END_INTERFACE
    } IPrintPipelinePropertyBagVtbl;

    interface IPrintPipelinePropertyBag
    {
        CONST_VTBL struct IPrintPipelinePropertyBagVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintPipelinePropertyBag_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintPipelinePropertyBag_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintPipelinePropertyBag_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintPipelinePropertyBag_AddProperty(This,pszName,pVar)	\
    ( (This)->lpVtbl -> AddProperty(This,pszName,pVar) ) 

#define IPrintPipelinePropertyBag_GetProperty(This,pszName,pVar)	\
    ( (This)->lpVtbl -> GetProperty(This,pszName,pVar) ) 

#define IPrintPipelinePropertyBag_DeleteProperty(This,pszName)	\
    ( (This)->lpVtbl -> DeleteProperty(This,pszName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintPipelinePropertyBag_INTERFACE_DEFINED__ */


#ifndef __IPrintPipelineProgressReport_INTERFACE_DEFINED__
#define __IPrintPipelineProgressReport_INTERFACE_DEFINED__

/* interface IPrintPipelineProgressReport */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPrintPipelineProgressReport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("edc12c7c-ed40-4ea5-96a6-5e4397497a61")
    IPrintPipelineProgressReport : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ReportProgress( 
            /* [annotation][in] */ 
            _In_  EXpsJobConsumption update) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintPipelineProgressReportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrintPipelineProgressReport * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrintPipelineProgressReport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrintPipelineProgressReport * This);
        
        DECLSPEC_XFGVIRT(IPrintPipelineProgressReport, ReportProgress)
        HRESULT ( STDMETHODCALLTYPE *ReportProgress )( 
            IPrintPipelineProgressReport * This,
            /* [annotation][in] */ 
            _In_  EXpsJobConsumption update);
        
        END_INTERFACE
    } IPrintPipelineProgressReportVtbl;

    interface IPrintPipelineProgressReport
    {
        CONST_VTBL struct IPrintPipelineProgressReportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintPipelineProgressReport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintPipelineProgressReport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintPipelineProgressReport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintPipelineProgressReport_ReportProgress(This,update)	\
    ( (This)->lpVtbl -> ReportProgress(This,update) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintPipelineProgressReport_INTERFACE_DEFINED__ */


#ifndef __IPrintClassObjectFactory_INTERFACE_DEFINED__
#define __IPrintClassObjectFactory_INTERFACE_DEFINED__

/* interface IPrintClassObjectFactory */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPrintClassObjectFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9af593dd-9b02-48a8-9bad-69ace423f88b")
    IPrintClassObjectFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPrintClassObject( 
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *pszPrinterName,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppNewObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintClassObjectFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrintClassObjectFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrintClassObjectFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrintClassObjectFactory * This);
        
        DECLSPEC_XFGVIRT(IPrintClassObjectFactory, GetPrintClassObject)
        HRESULT ( STDMETHODCALLTYPE *GetPrintClassObject )( 
            IPrintClassObjectFactory * This,
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *pszPrinterName,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppNewObject);
        
        END_INTERFACE
    } IPrintClassObjectFactoryVtbl;

    interface IPrintClassObjectFactory
    {
        CONST_VTBL struct IPrintClassObjectFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintClassObjectFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintClassObjectFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintClassObjectFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintClassObjectFactory_GetPrintClassObject(This,pszPrinterName,riid,ppNewObject)	\
    ( (This)->lpVtbl -> GetPrintClassObject(This,pszPrinterName,riid,ppNewObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintClassObjectFactory_INTERFACE_DEFINED__ */


#ifndef __IPrintPipelineFilter_INTERFACE_DEFINED__
#define __IPrintPipelineFilter_INTERFACE_DEFINED__

/* interface IPrintPipelineFilter */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPrintPipelineFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("cdb62fc0-8bed-434e-86fb-a2cae55f19ea")
    IPrintPipelineFilter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitializeFilter( 
            /* [annotation][in] */ 
            _In_  IInterFilterCommunicator *pINegotiation,
            /* [annotation][in] */ 
            _In_  IPrintPipelinePropertyBag *pIPropertyBag,
            /* [annotation][in] */ 
            _In_  IPrintPipelineManagerControl *pIPipelineControl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShutdownOperation( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StartOperation( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintPipelineFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrintPipelineFilter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrintPipelineFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrintPipelineFilter * This);
        
        DECLSPEC_XFGVIRT(IPrintPipelineFilter, InitializeFilter)
        HRESULT ( STDMETHODCALLTYPE *InitializeFilter )( 
            IPrintPipelineFilter * This,
            /* [annotation][in] */ 
            _In_  IInterFilterCommunicator *pINegotiation,
            /* [annotation][in] */ 
            _In_  IPrintPipelinePropertyBag *pIPropertyBag,
            /* [annotation][in] */ 
            _In_  IPrintPipelineManagerControl *pIPipelineControl);
        
        DECLSPEC_XFGVIRT(IPrintPipelineFilter, ShutdownOperation)
        HRESULT ( STDMETHODCALLTYPE *ShutdownOperation )( 
            IPrintPipelineFilter * This);
        
        DECLSPEC_XFGVIRT(IPrintPipelineFilter, StartOperation)
        HRESULT ( STDMETHODCALLTYPE *StartOperation )( 
            IPrintPipelineFilter * This);
        
        END_INTERFACE
    } IPrintPipelineFilterVtbl;

    interface IPrintPipelineFilter
    {
        CONST_VTBL struct IPrintPipelineFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintPipelineFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintPipelineFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintPipelineFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintPipelineFilter_InitializeFilter(This,pINegotiation,pIPropertyBag,pIPipelineControl)	\
    ( (This)->lpVtbl -> InitializeFilter(This,pINegotiation,pIPropertyBag,pIPipelineControl) ) 

#define IPrintPipelineFilter_ShutdownOperation(This)	\
    ( (This)->lpVtbl -> ShutdownOperation(This) ) 

#define IPrintPipelineFilter_StartOperation(This)	\
    ( (This)->lpVtbl -> StartOperation(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintPipelineFilter_INTERFACE_DEFINED__ */


#ifndef __IXpsDocumentProvider_INTERFACE_DEFINED__
#define __IXpsDocumentProvider_INTERFACE_DEFINED__

/* interface IXpsDocumentProvider */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IXpsDocumentProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b8cf8530-5562-47c4-ab67-b1f69ecf961e")
    IXpsDocumentProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetXpsPart( 
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppIXpsPart) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsDocumentProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXpsDocumentProvider * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXpsDocumentProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXpsDocumentProvider * This);
        
        DECLSPEC_XFGVIRT(IXpsDocumentProvider, GetXpsPart)
        HRESULT ( STDMETHODCALLTYPE *GetXpsPart )( 
            IXpsDocumentProvider * This,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppIXpsPart);
        
        END_INTERFACE
    } IXpsDocumentProviderVtbl;

    interface IXpsDocumentProvider
    {
        CONST_VTBL struct IXpsDocumentProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsDocumentProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsDocumentProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsDocumentProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsDocumentProvider_GetXpsPart(This,ppIXpsPart)	\
    ( (This)->lpVtbl -> GetXpsPart(This,ppIXpsPart) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsDocumentProvider_INTERFACE_DEFINED__ */


#ifndef __IXpsDocumentConsumer_INTERFACE_DEFINED__
#define __IXpsDocumentConsumer_INTERFACE_DEFINED__

/* interface IXpsDocumentConsumer */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IXpsDocumentConsumer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4368d8a2-4181-4a9f-b295-3d9a38bb9ba0")
    IXpsDocumentConsumer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SendXpsUnknown( 
            /* [annotation][in] */ 
            _In_  IUnknown *pUnknown) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendXpsDocument( 
            /* [annotation][in] */ 
            _In_  IXpsDocument *pIXpsDocument) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendFixedDocumentSequence( 
            /* [annotation][in] */ 
            _In_  IFixedDocumentSequence *pIFixedDocumentSequence) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendFixedDocument( 
            /* [annotation][in] */ 
            _In_  IFixedDocument *pIFixedDocument) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendFixedPage( 
            /* [annotation][in] */ 
            _In_  IFixedPage *pIFixedPage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CloseSender( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNewEmptyPart( 
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *uri,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppNewObject,
            /* [annotation][out] */ 
            _Outptr_  IPrintWriteStream **ppWriteStream) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsDocumentConsumerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXpsDocumentConsumer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXpsDocumentConsumer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXpsDocumentConsumer * This);
        
        DECLSPEC_XFGVIRT(IXpsDocumentConsumer, SendXpsUnknown)
        HRESULT ( STDMETHODCALLTYPE *SendXpsUnknown )( 
            IXpsDocumentConsumer * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IXpsDocumentConsumer, SendXpsDocument)
        HRESULT ( STDMETHODCALLTYPE *SendXpsDocument )( 
            IXpsDocumentConsumer * This,
            /* [annotation][in] */ 
            _In_  IXpsDocument *pIXpsDocument);
        
        DECLSPEC_XFGVIRT(IXpsDocumentConsumer, SendFixedDocumentSequence)
        HRESULT ( STDMETHODCALLTYPE *SendFixedDocumentSequence )( 
            IXpsDocumentConsumer * This,
            /* [annotation][in] */ 
            _In_  IFixedDocumentSequence *pIFixedDocumentSequence);
        
        DECLSPEC_XFGVIRT(IXpsDocumentConsumer, SendFixedDocument)
        HRESULT ( STDMETHODCALLTYPE *SendFixedDocument )( 
            IXpsDocumentConsumer * This,
            /* [annotation][in] */ 
            _In_  IFixedDocument *pIFixedDocument);
        
        DECLSPEC_XFGVIRT(IXpsDocumentConsumer, SendFixedPage)
        HRESULT ( STDMETHODCALLTYPE *SendFixedPage )( 
            IXpsDocumentConsumer * This,
            /* [annotation][in] */ 
            _In_  IFixedPage *pIFixedPage);
        
        DECLSPEC_XFGVIRT(IXpsDocumentConsumer, CloseSender)
        HRESULT ( STDMETHODCALLTYPE *CloseSender )( 
            IXpsDocumentConsumer * This);
        
        DECLSPEC_XFGVIRT(IXpsDocumentConsumer, GetNewEmptyPart)
        HRESULT ( STDMETHODCALLTYPE *GetNewEmptyPart )( 
            IXpsDocumentConsumer * This,
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *uri,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppNewObject,
            /* [annotation][out] */ 
            _Outptr_  IPrintWriteStream **ppWriteStream);
        
        END_INTERFACE
    } IXpsDocumentConsumerVtbl;

    interface IXpsDocumentConsumer
    {
        CONST_VTBL struct IXpsDocumentConsumerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsDocumentConsumer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsDocumentConsumer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsDocumentConsumer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsDocumentConsumer_SendXpsUnknown(This,pUnknown)	\
    ( (This)->lpVtbl -> SendXpsUnknown(This,pUnknown) ) 

#define IXpsDocumentConsumer_SendXpsDocument(This,pIXpsDocument)	\
    ( (This)->lpVtbl -> SendXpsDocument(This,pIXpsDocument) ) 

#define IXpsDocumentConsumer_SendFixedDocumentSequence(This,pIFixedDocumentSequence)	\
    ( (This)->lpVtbl -> SendFixedDocumentSequence(This,pIFixedDocumentSequence) ) 

#define IXpsDocumentConsumer_SendFixedDocument(This,pIFixedDocument)	\
    ( (This)->lpVtbl -> SendFixedDocument(This,pIFixedDocument) ) 

#define IXpsDocumentConsumer_SendFixedPage(This,pIFixedPage)	\
    ( (This)->lpVtbl -> SendFixedPage(This,pIFixedPage) ) 

#define IXpsDocumentConsumer_CloseSender(This)	\
    ( (This)->lpVtbl -> CloseSender(This) ) 

#define IXpsDocumentConsumer_GetNewEmptyPart(This,uri,riid,ppNewObject,ppWriteStream)	\
    ( (This)->lpVtbl -> GetNewEmptyPart(This,uri,riid,ppNewObject,ppWriteStream) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsDocumentConsumer_INTERFACE_DEFINED__ */


#ifndef __IXpsDocument_INTERFACE_DEFINED__
#define __IXpsDocument_INTERFACE_DEFINED__

/* interface IXpsDocument */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IXpsDocument;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e8d907db-62a9-4a95-abe7-e01763dd30f8")
    IXpsDocument : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetThumbnail( 
            /* [annotation][out] */ 
            _Outptr_  IPartThumbnail **ppThumbnail) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetThumbnail( 
            /* [annotation][in] */ 
            _In_  IPartThumbnail *pThumbnail) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsDocumentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXpsDocument * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXpsDocument * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXpsDocument * This);
        
        DECLSPEC_XFGVIRT(IXpsDocument, GetThumbnail)
        HRESULT ( STDMETHODCALLTYPE *GetThumbnail )( 
            IXpsDocument * This,
            /* [annotation][out] */ 
            _Outptr_  IPartThumbnail **ppThumbnail);
        
        DECLSPEC_XFGVIRT(IXpsDocument, SetThumbnail)
        HRESULT ( STDMETHODCALLTYPE *SetThumbnail )( 
            IXpsDocument * This,
            /* [annotation][in] */ 
            _In_  IPartThumbnail *pThumbnail);
        
        END_INTERFACE
    } IXpsDocumentVtbl;

    interface IXpsDocument
    {
        CONST_VTBL struct IXpsDocumentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsDocument_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsDocument_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsDocument_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsDocument_GetThumbnail(This,ppThumbnail)	\
    ( (This)->lpVtbl -> GetThumbnail(This,ppThumbnail) ) 

#define IXpsDocument_SetThumbnail(This,pThumbnail)	\
    ( (This)->lpVtbl -> SetThumbnail(This,pThumbnail) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsDocument_INTERFACE_DEFINED__ */


#ifndef __IFixedDocumentSequence_INTERFACE_DEFINED__
#define __IFixedDocumentSequence_INTERFACE_DEFINED__

/* interface IFixedDocumentSequence */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IFixedDocumentSequence;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8028d181-2c32-4249-8493-1bfb22045574")
    IFixedDocumentSequence : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetUri( 
            /* [annotation][out] */ 
            _Out_  BSTR *uri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrintTicket( 
            /* [annotation][out] */ 
            _Outptr_  IPartPrintTicket **ppPrintTicket) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPrintTicket( 
            /* [annotation][in] */ 
            _In_  IPartPrintTicket *pPrintTicket) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFixedDocumentSequenceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IFixedDocumentSequence * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IFixedDocumentSequence * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IFixedDocumentSequence * This);
        
        DECLSPEC_XFGVIRT(IFixedDocumentSequence, GetUri)
        HRESULT ( STDMETHODCALLTYPE *GetUri )( 
            IFixedDocumentSequence * This,
            /* [annotation][out] */ 
            _Out_  BSTR *uri);
        
        DECLSPEC_XFGVIRT(IFixedDocumentSequence, GetPrintTicket)
        HRESULT ( STDMETHODCALLTYPE *GetPrintTicket )( 
            IFixedDocumentSequence * This,
            /* [annotation][out] */ 
            _Outptr_  IPartPrintTicket **ppPrintTicket);
        
        DECLSPEC_XFGVIRT(IFixedDocumentSequence, SetPrintTicket)
        HRESULT ( STDMETHODCALLTYPE *SetPrintTicket )( 
            IFixedDocumentSequence * This,
            /* [annotation][in] */ 
            _In_  IPartPrintTicket *pPrintTicket);
        
        END_INTERFACE
    } IFixedDocumentSequenceVtbl;

    interface IFixedDocumentSequence
    {
        CONST_VTBL struct IFixedDocumentSequenceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFixedDocumentSequence_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFixedDocumentSequence_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFixedDocumentSequence_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFixedDocumentSequence_GetUri(This,uri)	\
    ( (This)->lpVtbl -> GetUri(This,uri) ) 

#define IFixedDocumentSequence_GetPrintTicket(This,ppPrintTicket)	\
    ( (This)->lpVtbl -> GetPrintTicket(This,ppPrintTicket) ) 

#define IFixedDocumentSequence_SetPrintTicket(This,pPrintTicket)	\
    ( (This)->lpVtbl -> SetPrintTicket(This,pPrintTicket) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFixedDocumentSequence_INTERFACE_DEFINED__ */


#ifndef __IFixedDocument_INTERFACE_DEFINED__
#define __IFixedDocument_INTERFACE_DEFINED__

/* interface IFixedDocument */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IFixedDocument;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f222ca9f-9968-4db9-81bd-abaebf15f93f")
    IFixedDocument : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetUri( 
            /* [annotation][out] */ 
            _Out_  BSTR *uri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrintTicket( 
            /* [annotation][out] */ 
            _Outptr_  IPartPrintTicket **ppPrintTicket) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPrintTicket( 
            /* [annotation][in] */ 
            _In_  IPartPrintTicket *pPrintTicket) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFixedDocumentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IFixedDocument * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IFixedDocument * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IFixedDocument * This);
        
        DECLSPEC_XFGVIRT(IFixedDocument, GetUri)
        HRESULT ( STDMETHODCALLTYPE *GetUri )( 
            IFixedDocument * This,
            /* [annotation][out] */ 
            _Out_  BSTR *uri);
        
        DECLSPEC_XFGVIRT(IFixedDocument, GetPrintTicket)
        HRESULT ( STDMETHODCALLTYPE *GetPrintTicket )( 
            IFixedDocument * This,
            /* [annotation][out] */ 
            _Outptr_  IPartPrintTicket **ppPrintTicket);
        
        DECLSPEC_XFGVIRT(IFixedDocument, SetPrintTicket)
        HRESULT ( STDMETHODCALLTYPE *SetPrintTicket )( 
            IFixedDocument * This,
            /* [annotation][in] */ 
            _In_  IPartPrintTicket *pPrintTicket);
        
        END_INTERFACE
    } IFixedDocumentVtbl;

    interface IFixedDocument
    {
        CONST_VTBL struct IFixedDocumentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFixedDocument_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFixedDocument_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFixedDocument_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFixedDocument_GetUri(This,uri)	\
    ( (This)->lpVtbl -> GetUri(This,uri) ) 

#define IFixedDocument_GetPrintTicket(This,ppPrintTicket)	\
    ( (This)->lpVtbl -> GetPrintTicket(This,ppPrintTicket) ) 

#define IFixedDocument_SetPrintTicket(This,pPrintTicket)	\
    ( (This)->lpVtbl -> SetPrintTicket(This,pPrintTicket) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFixedDocument_INTERFACE_DEFINED__ */


#ifndef __IPartBase_INTERFACE_DEFINED__
#define __IPartBase_INTERFACE_DEFINED__

/* interface IPartBase */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPartBase;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("36d51e28-369e-43ba-a666-9540c62c3f58")
    IPartBase : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetUri( 
            /* [annotation][out] */ 
            _Out_  BSTR *uri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStream( 
            /* [annotation][out] */ 
            _Outptr_  IPrintReadStream **ppStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPartCompression( 
            /* [annotation][out] */ 
            _Out_  EXpsCompressionOptions *pCompression) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPartCompression( 
            /* [annotation][in] */ 
            _In_  EXpsCompressionOptions compression) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPartBaseVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPartBase * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPartBase * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPartBase * This);
        
        DECLSPEC_XFGVIRT(IPartBase, GetUri)
        HRESULT ( STDMETHODCALLTYPE *GetUri )( 
            IPartBase * This,
            /* [annotation][out] */ 
            _Out_  BSTR *uri);
        
        DECLSPEC_XFGVIRT(IPartBase, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            IPartBase * This,
            /* [annotation][out] */ 
            _Outptr_  IPrintReadStream **ppStream);
        
        DECLSPEC_XFGVIRT(IPartBase, GetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *GetPartCompression )( 
            IPartBase * This,
            /* [annotation][out] */ 
            _Out_  EXpsCompressionOptions *pCompression);
        
        DECLSPEC_XFGVIRT(IPartBase, SetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *SetPartCompression )( 
            IPartBase * This,
            /* [annotation][in] */ 
            _In_  EXpsCompressionOptions compression);
        
        END_INTERFACE
    } IPartBaseVtbl;

    interface IPartBase
    {
        CONST_VTBL struct IPartBaseVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPartBase_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPartBase_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPartBase_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPartBase_GetUri(This,uri)	\
    ( (This)->lpVtbl -> GetUri(This,uri) ) 

#define IPartBase_GetStream(This,ppStream)	\
    ( (This)->lpVtbl -> GetStream(This,ppStream) ) 

#define IPartBase_GetPartCompression(This,pCompression)	\
    ( (This)->lpVtbl -> GetPartCompression(This,pCompression) ) 

#define IPartBase_SetPartCompression(This,compression)	\
    ( (This)->lpVtbl -> SetPartCompression(This,compression) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPartBase_INTERFACE_DEFINED__ */


#ifndef __IFixedPage_INTERFACE_DEFINED__
#define __IFixedPage_INTERFACE_DEFINED__

/* interface IFixedPage */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IFixedPage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3d9f6448-7e95-4cb5-94fb-0180c2883a57")
    IFixedPage : public IPartBase
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPrintTicket( 
            /* [annotation][out] */ 
            _Outptr_  IPartPrintTicket **ppPrintTicket) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPagePart( 
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *uri,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppUnk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWriteStream( 
            /* [annotation][out] */ 
            _Outptr_  IPrintWriteStream **ppWriteStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPrintTicket( 
            /* [annotation][in] */ 
            _In_  IPartPrintTicket *ppPrintTicket) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPagePart( 
            /* [annotation][in] */ 
            _In_  IUnknown *pUnk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteResource( 
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *uri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetXpsPartIterator( 
            /* [annotation][out] */ 
            _Outptr_  IXpsPartIterator **pXpsPartIt) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFixedPageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IFixedPage * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IFixedPage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IFixedPage * This);
        
        DECLSPEC_XFGVIRT(IPartBase, GetUri)
        HRESULT ( STDMETHODCALLTYPE *GetUri )( 
            IFixedPage * This,
            /* [annotation][out] */ 
            _Out_  BSTR *uri);
        
        DECLSPEC_XFGVIRT(IPartBase, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            IFixedPage * This,
            /* [annotation][out] */ 
            _Outptr_  IPrintReadStream **ppStream);
        
        DECLSPEC_XFGVIRT(IPartBase, GetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *GetPartCompression )( 
            IFixedPage * This,
            /* [annotation][out] */ 
            _Out_  EXpsCompressionOptions *pCompression);
        
        DECLSPEC_XFGVIRT(IPartBase, SetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *SetPartCompression )( 
            IFixedPage * This,
            /* [annotation][in] */ 
            _In_  EXpsCompressionOptions compression);
        
        DECLSPEC_XFGVIRT(IFixedPage, GetPrintTicket)
        HRESULT ( STDMETHODCALLTYPE *GetPrintTicket )( 
            IFixedPage * This,
            /* [annotation][out] */ 
            _Outptr_  IPartPrintTicket **ppPrintTicket);
        
        DECLSPEC_XFGVIRT(IFixedPage, GetPagePart)
        HRESULT ( STDMETHODCALLTYPE *GetPagePart )( 
            IFixedPage * This,
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *uri,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IFixedPage, GetWriteStream)
        HRESULT ( STDMETHODCALLTYPE *GetWriteStream )( 
            IFixedPage * This,
            /* [annotation][out] */ 
            _Outptr_  IPrintWriteStream **ppWriteStream);
        
        DECLSPEC_XFGVIRT(IFixedPage, SetPrintTicket)
        HRESULT ( STDMETHODCALLTYPE *SetPrintTicket )( 
            IFixedPage * This,
            /* [annotation][in] */ 
            _In_  IPartPrintTicket *ppPrintTicket);
        
        DECLSPEC_XFGVIRT(IFixedPage, SetPagePart)
        HRESULT ( STDMETHODCALLTYPE *SetPagePart )( 
            IFixedPage * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnk);
        
        DECLSPEC_XFGVIRT(IFixedPage, DeleteResource)
        HRESULT ( STDMETHODCALLTYPE *DeleteResource )( 
            IFixedPage * This,
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *uri);
        
        DECLSPEC_XFGVIRT(IFixedPage, GetXpsPartIterator)
        HRESULT ( STDMETHODCALLTYPE *GetXpsPartIterator )( 
            IFixedPage * This,
            /* [annotation][out] */ 
            _Outptr_  IXpsPartIterator **pXpsPartIt);
        
        END_INTERFACE
    } IFixedPageVtbl;

    interface IFixedPage
    {
        CONST_VTBL struct IFixedPageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFixedPage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFixedPage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFixedPage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFixedPage_GetUri(This,uri)	\
    ( (This)->lpVtbl -> GetUri(This,uri) ) 

#define IFixedPage_GetStream(This,ppStream)	\
    ( (This)->lpVtbl -> GetStream(This,ppStream) ) 

#define IFixedPage_GetPartCompression(This,pCompression)	\
    ( (This)->lpVtbl -> GetPartCompression(This,pCompression) ) 

#define IFixedPage_SetPartCompression(This,compression)	\
    ( (This)->lpVtbl -> SetPartCompression(This,compression) ) 


#define IFixedPage_GetPrintTicket(This,ppPrintTicket)	\
    ( (This)->lpVtbl -> GetPrintTicket(This,ppPrintTicket) ) 

#define IFixedPage_GetPagePart(This,uri,ppUnk)	\
    ( (This)->lpVtbl -> GetPagePart(This,uri,ppUnk) ) 

#define IFixedPage_GetWriteStream(This,ppWriteStream)	\
    ( (This)->lpVtbl -> GetWriteStream(This,ppWriteStream) ) 

#define IFixedPage_SetPrintTicket(This,ppPrintTicket)	\
    ( (This)->lpVtbl -> SetPrintTicket(This,ppPrintTicket) ) 

#define IFixedPage_SetPagePart(This,pUnk)	\
    ( (This)->lpVtbl -> SetPagePart(This,pUnk) ) 

#define IFixedPage_DeleteResource(This,uri)	\
    ( (This)->lpVtbl -> DeleteResource(This,uri) ) 

#define IFixedPage_GetXpsPartIterator(This,pXpsPartIt)	\
    ( (This)->lpVtbl -> GetXpsPartIterator(This,pXpsPartIt) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFixedPage_INTERFACE_DEFINED__ */


#ifndef __IPartImage_INTERFACE_DEFINED__
#define __IPartImage_INTERFACE_DEFINED__

/* interface IPartImage */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPartImage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("725f2e3c-401a-4705-9de0-fe6f1353b87f")
    IPartImage : public IPartBase
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetImageProperties( 
            /* [annotation][out] */ 
            _Out_  BSTR *pContentType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetImageContent( 
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *pContentType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPartImageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPartImage * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPartImage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPartImage * This);
        
        DECLSPEC_XFGVIRT(IPartBase, GetUri)
        HRESULT ( STDMETHODCALLTYPE *GetUri )( 
            IPartImage * This,
            /* [annotation][out] */ 
            _Out_  BSTR *uri);
        
        DECLSPEC_XFGVIRT(IPartBase, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            IPartImage * This,
            /* [annotation][out] */ 
            _Outptr_  IPrintReadStream **ppStream);
        
        DECLSPEC_XFGVIRT(IPartBase, GetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *GetPartCompression )( 
            IPartImage * This,
            /* [annotation][out] */ 
            _Out_  EXpsCompressionOptions *pCompression);
        
        DECLSPEC_XFGVIRT(IPartBase, SetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *SetPartCompression )( 
            IPartImage * This,
            /* [annotation][in] */ 
            _In_  EXpsCompressionOptions compression);
        
        DECLSPEC_XFGVIRT(IPartImage, GetImageProperties)
        HRESULT ( STDMETHODCALLTYPE *GetImageProperties )( 
            IPartImage * This,
            /* [annotation][out] */ 
            _Out_  BSTR *pContentType);
        
        DECLSPEC_XFGVIRT(IPartImage, SetImageContent)
        HRESULT ( STDMETHODCALLTYPE *SetImageContent )( 
            IPartImage * This,
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *pContentType);
        
        END_INTERFACE
    } IPartImageVtbl;

    interface IPartImage
    {
        CONST_VTBL struct IPartImageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPartImage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPartImage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPartImage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPartImage_GetUri(This,uri)	\
    ( (This)->lpVtbl -> GetUri(This,uri) ) 

#define IPartImage_GetStream(This,ppStream)	\
    ( (This)->lpVtbl -> GetStream(This,ppStream) ) 

#define IPartImage_GetPartCompression(This,pCompression)	\
    ( (This)->lpVtbl -> GetPartCompression(This,pCompression) ) 

#define IPartImage_SetPartCompression(This,compression)	\
    ( (This)->lpVtbl -> SetPartCompression(This,compression) ) 


#define IPartImage_GetImageProperties(This,pContentType)	\
    ( (This)->lpVtbl -> GetImageProperties(This,pContentType) ) 

#define IPartImage_SetImageContent(This,pContentType)	\
    ( (This)->lpVtbl -> SetImageContent(This,pContentType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPartImage_INTERFACE_DEFINED__ */


#ifndef __IPartFont_INTERFACE_DEFINED__
#define __IPartFont_INTERFACE_DEFINED__

/* interface IPartFont */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPartFont;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e07fe0ab-1124-43d0-a865-e8ffb6a3ea82")
    IPartFont : public IPartBase
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFontProperties( 
            /* [annotation][out] */ 
            _Out_  BSTR *pContentType,
            /* [annotation][out] */ 
            _Out_  EXpsFontOptions *pFontOptions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFontContent( 
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *pContentType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFontOptions( 
            /* [annotation][in] */ 
            _In_  EXpsFontOptions options) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPartFontVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPartFont * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPartFont * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPartFont * This);
        
        DECLSPEC_XFGVIRT(IPartBase, GetUri)
        HRESULT ( STDMETHODCALLTYPE *GetUri )( 
            IPartFont * This,
            /* [annotation][out] */ 
            _Out_  BSTR *uri);
        
        DECLSPEC_XFGVIRT(IPartBase, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            IPartFont * This,
            /* [annotation][out] */ 
            _Outptr_  IPrintReadStream **ppStream);
        
        DECLSPEC_XFGVIRT(IPartBase, GetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *GetPartCompression )( 
            IPartFont * This,
            /* [annotation][out] */ 
            _Out_  EXpsCompressionOptions *pCompression);
        
        DECLSPEC_XFGVIRT(IPartBase, SetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *SetPartCompression )( 
            IPartFont * This,
            /* [annotation][in] */ 
            _In_  EXpsCompressionOptions compression);
        
        DECLSPEC_XFGVIRT(IPartFont, GetFontProperties)
        HRESULT ( STDMETHODCALLTYPE *GetFontProperties )( 
            IPartFont * This,
            /* [annotation][out] */ 
            _Out_  BSTR *pContentType,
            /* [annotation][out] */ 
            _Out_  EXpsFontOptions *pFontOptions);
        
        DECLSPEC_XFGVIRT(IPartFont, SetFontContent)
        HRESULT ( STDMETHODCALLTYPE *SetFontContent )( 
            IPartFont * This,
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *pContentType);
        
        DECLSPEC_XFGVIRT(IPartFont, SetFontOptions)
        HRESULT ( STDMETHODCALLTYPE *SetFontOptions )( 
            IPartFont * This,
            /* [annotation][in] */ 
            _In_  EXpsFontOptions options);
        
        END_INTERFACE
    } IPartFontVtbl;

    interface IPartFont
    {
        CONST_VTBL struct IPartFontVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPartFont_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPartFont_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPartFont_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPartFont_GetUri(This,uri)	\
    ( (This)->lpVtbl -> GetUri(This,uri) ) 

#define IPartFont_GetStream(This,ppStream)	\
    ( (This)->lpVtbl -> GetStream(This,ppStream) ) 

#define IPartFont_GetPartCompression(This,pCompression)	\
    ( (This)->lpVtbl -> GetPartCompression(This,pCompression) ) 

#define IPartFont_SetPartCompression(This,compression)	\
    ( (This)->lpVtbl -> SetPartCompression(This,compression) ) 


#define IPartFont_GetFontProperties(This,pContentType,pFontOptions)	\
    ( (This)->lpVtbl -> GetFontProperties(This,pContentType,pFontOptions) ) 

#define IPartFont_SetFontContent(This,pContentType)	\
    ( (This)->lpVtbl -> SetFontContent(This,pContentType) ) 

#define IPartFont_SetFontOptions(This,options)	\
    ( (This)->lpVtbl -> SetFontOptions(This,options) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPartFont_INTERFACE_DEFINED__ */


#ifndef __IPartFont2_INTERFACE_DEFINED__
#define __IPartFont2_INTERFACE_DEFINED__

/* interface IPartFont2 */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPartFont2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("511e025f-d6cb-43be-bf65-63fe88515a39")
    IPartFont2 : public IPartFont
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFontRestriction( 
            /* [annotation][out] */ 
            _Out_  EXpsFontRestriction *pRestriction) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPartFont2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPartFont2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPartFont2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPartFont2 * This);
        
        DECLSPEC_XFGVIRT(IPartBase, GetUri)
        HRESULT ( STDMETHODCALLTYPE *GetUri )( 
            IPartFont2 * This,
            /* [annotation][out] */ 
            _Out_  BSTR *uri);
        
        DECLSPEC_XFGVIRT(IPartBase, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            IPartFont2 * This,
            /* [annotation][out] */ 
            _Outptr_  IPrintReadStream **ppStream);
        
        DECLSPEC_XFGVIRT(IPartBase, GetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *GetPartCompression )( 
            IPartFont2 * This,
            /* [annotation][out] */ 
            _Out_  EXpsCompressionOptions *pCompression);
        
        DECLSPEC_XFGVIRT(IPartBase, SetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *SetPartCompression )( 
            IPartFont2 * This,
            /* [annotation][in] */ 
            _In_  EXpsCompressionOptions compression);
        
        DECLSPEC_XFGVIRT(IPartFont, GetFontProperties)
        HRESULT ( STDMETHODCALLTYPE *GetFontProperties )( 
            IPartFont2 * This,
            /* [annotation][out] */ 
            _Out_  BSTR *pContentType,
            /* [annotation][out] */ 
            _Out_  EXpsFontOptions *pFontOptions);
        
        DECLSPEC_XFGVIRT(IPartFont, SetFontContent)
        HRESULT ( STDMETHODCALLTYPE *SetFontContent )( 
            IPartFont2 * This,
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *pContentType);
        
        DECLSPEC_XFGVIRT(IPartFont, SetFontOptions)
        HRESULT ( STDMETHODCALLTYPE *SetFontOptions )( 
            IPartFont2 * This,
            /* [annotation][in] */ 
            _In_  EXpsFontOptions options);
        
        DECLSPEC_XFGVIRT(IPartFont2, GetFontRestriction)
        HRESULT ( STDMETHODCALLTYPE *GetFontRestriction )( 
            IPartFont2 * This,
            /* [annotation][out] */ 
            _Out_  EXpsFontRestriction *pRestriction);
        
        END_INTERFACE
    } IPartFont2Vtbl;

    interface IPartFont2
    {
        CONST_VTBL struct IPartFont2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPartFont2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPartFont2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPartFont2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPartFont2_GetUri(This,uri)	\
    ( (This)->lpVtbl -> GetUri(This,uri) ) 

#define IPartFont2_GetStream(This,ppStream)	\
    ( (This)->lpVtbl -> GetStream(This,ppStream) ) 

#define IPartFont2_GetPartCompression(This,pCompression)	\
    ( (This)->lpVtbl -> GetPartCompression(This,pCompression) ) 

#define IPartFont2_SetPartCompression(This,compression)	\
    ( (This)->lpVtbl -> SetPartCompression(This,compression) ) 


#define IPartFont2_GetFontProperties(This,pContentType,pFontOptions)	\
    ( (This)->lpVtbl -> GetFontProperties(This,pContentType,pFontOptions) ) 

#define IPartFont2_SetFontContent(This,pContentType)	\
    ( (This)->lpVtbl -> SetFontContent(This,pContentType) ) 

#define IPartFont2_SetFontOptions(This,options)	\
    ( (This)->lpVtbl -> SetFontOptions(This,options) ) 


#define IPartFont2_GetFontRestriction(This,pRestriction)	\
    ( (This)->lpVtbl -> GetFontRestriction(This,pRestriction) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPartFont2_INTERFACE_DEFINED__ */


#ifndef __IPartThumbnail_INTERFACE_DEFINED__
#define __IPartThumbnail_INTERFACE_DEFINED__

/* interface IPartThumbnail */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPartThumbnail;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("027ed1c9-ba39-4cc5-aa55-7ec3a0de171a")
    IPartThumbnail : public IPartBase
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetThumbnailProperties( 
            /* [annotation][out] */ 
            _Out_  BSTR *pContentType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetThumbnailContent( 
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *pContentType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPartThumbnailVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPartThumbnail * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPartThumbnail * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPartThumbnail * This);
        
        DECLSPEC_XFGVIRT(IPartBase, GetUri)
        HRESULT ( STDMETHODCALLTYPE *GetUri )( 
            IPartThumbnail * This,
            /* [annotation][out] */ 
            _Out_  BSTR *uri);
        
        DECLSPEC_XFGVIRT(IPartBase, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            IPartThumbnail * This,
            /* [annotation][out] */ 
            _Outptr_  IPrintReadStream **ppStream);
        
        DECLSPEC_XFGVIRT(IPartBase, GetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *GetPartCompression )( 
            IPartThumbnail * This,
            /* [annotation][out] */ 
            _Out_  EXpsCompressionOptions *pCompression);
        
        DECLSPEC_XFGVIRT(IPartBase, SetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *SetPartCompression )( 
            IPartThumbnail * This,
            /* [annotation][in] */ 
            _In_  EXpsCompressionOptions compression);
        
        DECLSPEC_XFGVIRT(IPartThumbnail, GetThumbnailProperties)
        HRESULT ( STDMETHODCALLTYPE *GetThumbnailProperties )( 
            IPartThumbnail * This,
            /* [annotation][out] */ 
            _Out_  BSTR *pContentType);
        
        DECLSPEC_XFGVIRT(IPartThumbnail, SetThumbnailContent)
        HRESULT ( STDMETHODCALLTYPE *SetThumbnailContent )( 
            IPartThumbnail * This,
            /* [annotation][string][in] */ 
            _In_z_  const wchar_t *pContentType);
        
        END_INTERFACE
    } IPartThumbnailVtbl;

    interface IPartThumbnail
    {
        CONST_VTBL struct IPartThumbnailVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPartThumbnail_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPartThumbnail_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPartThumbnail_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPartThumbnail_GetUri(This,uri)	\
    ( (This)->lpVtbl -> GetUri(This,uri) ) 

#define IPartThumbnail_GetStream(This,ppStream)	\
    ( (This)->lpVtbl -> GetStream(This,ppStream) ) 

#define IPartThumbnail_GetPartCompression(This,pCompression)	\
    ( (This)->lpVtbl -> GetPartCompression(This,pCompression) ) 

#define IPartThumbnail_SetPartCompression(This,compression)	\
    ( (This)->lpVtbl -> SetPartCompression(This,compression) ) 


#define IPartThumbnail_GetThumbnailProperties(This,pContentType)	\
    ( (This)->lpVtbl -> GetThumbnailProperties(This,pContentType) ) 

#define IPartThumbnail_SetThumbnailContent(This,pContentType)	\
    ( (This)->lpVtbl -> SetThumbnailContent(This,pContentType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPartThumbnail_INTERFACE_DEFINED__ */


#ifndef __IPartPrintTicket_INTERFACE_DEFINED__
#define __IPartPrintTicket_INTERFACE_DEFINED__

/* interface IPartPrintTicket */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPartPrintTicket;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4a0f50f6-f9a2-41f0-99e7-5ae955be8e9e")
    IPartPrintTicket : public IPartBase
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IPartPrintTicketVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPartPrintTicket * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPartPrintTicket * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPartPrintTicket * This);
        
        DECLSPEC_XFGVIRT(IPartBase, GetUri)
        HRESULT ( STDMETHODCALLTYPE *GetUri )( 
            IPartPrintTicket * This,
            /* [annotation][out] */ 
            _Out_  BSTR *uri);
        
        DECLSPEC_XFGVIRT(IPartBase, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            IPartPrintTicket * This,
            /* [annotation][out] */ 
            _Outptr_  IPrintReadStream **ppStream);
        
        DECLSPEC_XFGVIRT(IPartBase, GetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *GetPartCompression )( 
            IPartPrintTicket * This,
            /* [annotation][out] */ 
            _Out_  EXpsCompressionOptions *pCompression);
        
        DECLSPEC_XFGVIRT(IPartBase, SetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *SetPartCompression )( 
            IPartPrintTicket * This,
            /* [annotation][in] */ 
            _In_  EXpsCompressionOptions compression);
        
        END_INTERFACE
    } IPartPrintTicketVtbl;

    interface IPartPrintTicket
    {
        CONST_VTBL struct IPartPrintTicketVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPartPrintTicket_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPartPrintTicket_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPartPrintTicket_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPartPrintTicket_GetUri(This,uri)	\
    ( (This)->lpVtbl -> GetUri(This,uri) ) 

#define IPartPrintTicket_GetStream(This,ppStream)	\
    ( (This)->lpVtbl -> GetStream(This,ppStream) ) 

#define IPartPrintTicket_GetPartCompression(This,pCompression)	\
    ( (This)->lpVtbl -> GetPartCompression(This,pCompression) ) 

#define IPartPrintTicket_SetPartCompression(This,compression)	\
    ( (This)->lpVtbl -> SetPartCompression(This,compression) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPartPrintTicket_INTERFACE_DEFINED__ */


#ifndef __IPartColorProfile_INTERFACE_DEFINED__
#define __IPartColorProfile_INTERFACE_DEFINED__

/* interface IPartColorProfile */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPartColorProfile;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("63cca95b-7d18-4762-b15e-98658693d24a")
    IPartColorProfile : public IPartBase
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IPartColorProfileVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPartColorProfile * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPartColorProfile * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPartColorProfile * This);
        
        DECLSPEC_XFGVIRT(IPartBase, GetUri)
        HRESULT ( STDMETHODCALLTYPE *GetUri )( 
            IPartColorProfile * This,
            /* [annotation][out] */ 
            _Out_  BSTR *uri);
        
        DECLSPEC_XFGVIRT(IPartBase, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            IPartColorProfile * This,
            /* [annotation][out] */ 
            _Outptr_  IPrintReadStream **ppStream);
        
        DECLSPEC_XFGVIRT(IPartBase, GetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *GetPartCompression )( 
            IPartColorProfile * This,
            /* [annotation][out] */ 
            _Out_  EXpsCompressionOptions *pCompression);
        
        DECLSPEC_XFGVIRT(IPartBase, SetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *SetPartCompression )( 
            IPartColorProfile * This,
            /* [annotation][in] */ 
            _In_  EXpsCompressionOptions compression);
        
        END_INTERFACE
    } IPartColorProfileVtbl;

    interface IPartColorProfile
    {
        CONST_VTBL struct IPartColorProfileVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPartColorProfile_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPartColorProfile_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPartColorProfile_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPartColorProfile_GetUri(This,uri)	\
    ( (This)->lpVtbl -> GetUri(This,uri) ) 

#define IPartColorProfile_GetStream(This,ppStream)	\
    ( (This)->lpVtbl -> GetStream(This,ppStream) ) 

#define IPartColorProfile_GetPartCompression(This,pCompression)	\
    ( (This)->lpVtbl -> GetPartCompression(This,pCompression) ) 

#define IPartColorProfile_SetPartCompression(This,compression)	\
    ( (This)->lpVtbl -> SetPartCompression(This,compression) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPartColorProfile_INTERFACE_DEFINED__ */


#ifndef __IPartResourceDictionary_INTERFACE_DEFINED__
#define __IPartResourceDictionary_INTERFACE_DEFINED__

/* interface IPartResourceDictionary */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPartResourceDictionary;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("16cfce6d-e744-4fb3-b474-f1d54f024a01")
    IPartResourceDictionary : public IPartBase
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IPartResourceDictionaryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPartResourceDictionary * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPartResourceDictionary * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPartResourceDictionary * This);
        
        DECLSPEC_XFGVIRT(IPartBase, GetUri)
        HRESULT ( STDMETHODCALLTYPE *GetUri )( 
            IPartResourceDictionary * This,
            /* [annotation][out] */ 
            _Out_  BSTR *uri);
        
        DECLSPEC_XFGVIRT(IPartBase, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            IPartResourceDictionary * This,
            /* [annotation][out] */ 
            _Outptr_  IPrintReadStream **ppStream);
        
        DECLSPEC_XFGVIRT(IPartBase, GetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *GetPartCompression )( 
            IPartResourceDictionary * This,
            /* [annotation][out] */ 
            _Out_  EXpsCompressionOptions *pCompression);
        
        DECLSPEC_XFGVIRT(IPartBase, SetPartCompression)
        HRESULT ( STDMETHODCALLTYPE *SetPartCompression )( 
            IPartResourceDictionary * This,
            /* [annotation][in] */ 
            _In_  EXpsCompressionOptions compression);
        
        END_INTERFACE
    } IPartResourceDictionaryVtbl;

    interface IPartResourceDictionary
    {
        CONST_VTBL struct IPartResourceDictionaryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPartResourceDictionary_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPartResourceDictionary_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPartResourceDictionary_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPartResourceDictionary_GetUri(This,uri)	\
    ( (This)->lpVtbl -> GetUri(This,uri) ) 

#define IPartResourceDictionary_GetStream(This,ppStream)	\
    ( (This)->lpVtbl -> GetStream(This,ppStream) ) 

#define IPartResourceDictionary_GetPartCompression(This,pCompression)	\
    ( (This)->lpVtbl -> GetPartCompression(This,pCompression) ) 

#define IPartResourceDictionary_SetPartCompression(This,compression)	\
    ( (This)->lpVtbl -> SetPartCompression(This,compression) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPartResourceDictionary_INTERFACE_DEFINED__ */


#ifndef __IXpsPartIterator_INTERFACE_DEFINED__
#define __IXpsPartIterator_INTERFACE_DEFINED__

/* interface IXpsPartIterator */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IXpsPartIterator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0021d3cd-af6f-42ab-9999-14bc82a62d2e")
    IXpsPartIterator : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Current( 
            /* [annotation][out] */ 
            _Out_  BSTR *pUri,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppXpsPart) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsDone( void) = 0;
        
        virtual void STDMETHODCALLTYPE Next( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsPartIteratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXpsPartIterator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXpsPartIterator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXpsPartIterator * This);
        
        DECLSPEC_XFGVIRT(IXpsPartIterator, Reset)
        void ( STDMETHODCALLTYPE *Reset )( 
            IXpsPartIterator * This);
        
        DECLSPEC_XFGVIRT(IXpsPartIterator, Current)
        HRESULT ( STDMETHODCALLTYPE *Current )( 
            IXpsPartIterator * This,
            /* [annotation][out] */ 
            _Out_  BSTR *pUri,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppXpsPart);
        
        DECLSPEC_XFGVIRT(IXpsPartIterator, IsDone)
        BOOL ( STDMETHODCALLTYPE *IsDone )( 
            IXpsPartIterator * This);
        
        DECLSPEC_XFGVIRT(IXpsPartIterator, Next)
        void ( STDMETHODCALLTYPE *Next )( 
            IXpsPartIterator * This);
        
        END_INTERFACE
    } IXpsPartIteratorVtbl;

    interface IXpsPartIterator
    {
        CONST_VTBL struct IXpsPartIteratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsPartIterator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsPartIterator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsPartIterator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsPartIterator_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IXpsPartIterator_Current(This,pUri,ppXpsPart)	\
    ( (This)->lpVtbl -> Current(This,pUri,ppXpsPart) ) 

#define IXpsPartIterator_IsDone(This)	\
    ( (This)->lpVtbl -> IsDone(This) ) 

#define IXpsPartIterator_Next(This)	\
    ( (This)->lpVtbl -> Next(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsPartIterator_INTERFACE_DEFINED__ */


#ifndef __IPrintReadStreamFactory_INTERFACE_DEFINED__
#define __IPrintReadStreamFactory_INTERFACE_DEFINED__

/* interface IPrintReadStreamFactory */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPrintReadStreamFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("acb971e3-df8d-4fc2-bee6-0609d15f3cf9")
    IPrintReadStreamFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStream( 
            /* [annotation][out] */ 
            _Outptr_  IPrintReadStream **ppStream) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintReadStreamFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrintReadStreamFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrintReadStreamFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrintReadStreamFactory * This);
        
        DECLSPEC_XFGVIRT(IPrintReadStreamFactory, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            IPrintReadStreamFactory * This,
            /* [annotation][out] */ 
            _Outptr_  IPrintReadStream **ppStream);
        
        END_INTERFACE
    } IPrintReadStreamFactoryVtbl;

    interface IPrintReadStreamFactory
    {
        CONST_VTBL struct IPrintReadStreamFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintReadStreamFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintReadStreamFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintReadStreamFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintReadStreamFactory_GetStream(This,ppStream)	\
    ( (This)->lpVtbl -> GetStream(This,ppStream) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintReadStreamFactory_INTERFACE_DEFINED__ */


#ifndef __IPartDiscardControl_INTERFACE_DEFINED__
#define __IPartDiscardControl_INTERFACE_DEFINED__

/* interface IPartDiscardControl */
/* [ref][helpstring][local][uuid][object] */ 


EXTERN_C const IID IID_IPartDiscardControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("cc350c00-095b-42a5-bf0f-c8780edadb3c")
    IPartDiscardControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDiscardProperties( 
            /* [annotation][out] */ 
            _Out_  BSTR *uriSentinelPage,
            /* [annotation][out] */ 
            _Out_  BSTR *uriPartToDiscard) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPartDiscardControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPartDiscardControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPartDiscardControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPartDiscardControl * This);
        
        DECLSPEC_XFGVIRT(IPartDiscardControl, GetDiscardProperties)
        HRESULT ( STDMETHODCALLTYPE *GetDiscardProperties )( 
            IPartDiscardControl * This,
            /* [annotation][out] */ 
            _Out_  BSTR *uriSentinelPage,
            /* [annotation][out] */ 
            _Out_  BSTR *uriPartToDiscard);
        
        END_INTERFACE
    } IPartDiscardControlVtbl;

    interface IPartDiscardControl
    {
        CONST_VTBL struct IPartDiscardControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPartDiscardControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPartDiscardControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPartDiscardControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPartDiscardControl_GetDiscardProperties(This,uriSentinelPage,uriPartToDiscard)	\
    ( (This)->lpVtbl -> GetDiscardProperties(This,uriSentinelPage,uriPartToDiscard) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPartDiscardControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_filterpipeline_0000_0026 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_filterpipeline_0000_0026_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_filterpipeline_0000_0026_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


