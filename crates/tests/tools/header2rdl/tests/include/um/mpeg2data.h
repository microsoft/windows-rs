

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

#ifndef __mpeg2data_h__
#define __mpeg2data_h__

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

#ifndef __IMpeg2TableFilter_FWD_DEFINED__
#define __IMpeg2TableFilter_FWD_DEFINED__
typedef interface IMpeg2TableFilter IMpeg2TableFilter;

#endif 	/* __IMpeg2TableFilter_FWD_DEFINED__ */


#ifndef __IMpeg2Data_FWD_DEFINED__
#define __IMpeg2Data_FWD_DEFINED__
typedef interface IMpeg2Data IMpeg2Data;

#endif 	/* __IMpeg2Data_FWD_DEFINED__ */


#ifndef __ISectionList_FWD_DEFINED__
#define __ISectionList_FWD_DEFINED__
typedef interface ISectionList ISectionList;

#endif 	/* __ISectionList_FWD_DEFINED__ */


#ifndef __IMpeg2Stream_FWD_DEFINED__
#define __IMpeg2Stream_FWD_DEFINED__
typedef interface IMpeg2Stream IMpeg2Stream;

#endif 	/* __IMpeg2Stream_FWD_DEFINED__ */


#ifndef __SectionList_FWD_DEFINED__
#define __SectionList_FWD_DEFINED__

#ifdef __cplusplus
typedef class SectionList SectionList;
#else
typedef struct SectionList SectionList;
#endif /* __cplusplus */

#endif 	/* __SectionList_FWD_DEFINED__ */


#ifndef __Mpeg2Stream_FWD_DEFINED__
#define __Mpeg2Stream_FWD_DEFINED__

#ifdef __cplusplus
typedef class Mpeg2Stream Mpeg2Stream;
#else
typedef struct Mpeg2Stream Mpeg2Stream;
#endif /* __cplusplus */

#endif 	/* __Mpeg2Stream_FWD_DEFINED__ */


#ifndef __Mpeg2Data_FWD_DEFINED__
#define __Mpeg2Data_FWD_DEFINED__

#ifdef __cplusplus
typedef class Mpeg2Data Mpeg2Data;
#else
typedef struct Mpeg2Data Mpeg2Data;
#endif /* __cplusplus */

#endif 	/* __Mpeg2Data_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "bdaiface.h"
#include "mpeg2structs.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mpeg2data_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#pragma pack(push)

#pragma pack(1)


#define MPEG_PAT_PID                0x0000
#define MPEG_PAT_TID                0x00
#define MPEG_CAT_PID                0x0001
#define MPEG_CAT_TID                0x01
#define MPEG_PMT_TID                0x02
#define MPEG_TSDT_PID               0x0002
#define MPEG_TSDT_TID               0x03
#define ATSC_MGT_PID                0x1FFB
#define ATSC_MGT_TID                0xC7
#define ATSC_VCT_PID                0x1FFB
#define ATSC_VCT_TERR_TID           0xC8
#define ATSC_VCT_CABL_TID           0xC9
#define ATSC_EIT_TID                0xCB
#define ATSC_ETT_TID                0xCC
#define ATSC_RRT_TID                0xCA
#define ATSC_RRT_PID                0x1FFB
#define ATSC_STT_PID                0x1FFB
#define ATSC_STT_TID                0xCD
#define ATSC_PIT_TID                0xD0
#define DVB_NIT_PID                 0x0010
#define DVB_NIT_ACTUAL_TID          0x40
#define DVB_NIT_OTHER_TID           0x41
#define DVB_SDT_PID                 0x0011
#define DVB_SDT_ACTUAL_TID          0x42
#define DVB_SDT_OTHER_TID           0x46
#define DVB_BAT_PID                 0x0011
#define DVB_BAT_TID                 0x4A
#define DVB_EIT_PID                 0x0012
#define DVB_EIT_ACTUAL_TID          0x4E
#define DVB_EIT_OTHER_TID           0x4F
#define DVB_RST_PID                 0x0013
#define DVB_RST_TID                 0x71
#define DVB_TDT_PID                 0x0014
#define DVB_TDT_TID                 0x70
#define DVB_ST_PID_16               0x0010
#define DVB_ST_PID_17               0x0011
#define DVB_ST_PID_18               0x0012
#define DVB_ST_PID_19               0x0013
#define DVB_ST_PID_20               0x0014
#define DVB_ST_TID                  0x72
#define ISDB_ST_TID                 0x72
#define DVB_TOT_PID                 0x0014
#define DVB_TOT_TID                 0x73
#define DVB_DIT_PID                 0x001E
#define DVB_DIT_TID                 0x7E
#define DVB_SIT_PID                 0x001F
#define DVB_SIT_TID                 0x7F
#define ISDB_EMM_TID                0x85
#define ISDB_BIT_PID                0x0024
#define ISDB_BIT_TID                0xC4
#define ISDB_NBIT_PID               0x0025
#define ISDB_NBIT_MSG_TID           0xC5
#define ISDB_NBIT_REF_TID           0xC6
#define ISDB_LDT_PID                0x0025
#define ISDB_LDT_TID                0xC7
#define ISDB_SDTT_PID               0x0023
#define ISDB_SDTT_ALT_PID           0x0028
#define ISDB_SDTT_TID               0xC3
#define ISDB_CDT_PID                0x0029
#define ISDB_CDT_TID                0xC8
#define SCTE_EAS_TID                0xD8
#define SCTE_EAS_IB_PID             0x1FFB
#define SCTE_EAS_OOB_PID            0x1FFC


extern RPC_IF_HANDLE __MIDL_itf_mpeg2data_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mpeg2data_0000_0000_v0_0_s_ifspec;

#ifndef __IMpeg2TableFilter_INTERFACE_DEFINED__
#define __IMpeg2TableFilter_INTERFACE_DEFINED__

/* interface IMpeg2TableFilter */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMpeg2TableFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BDCDD913-9ECD-4fb2-81AE-ADF747EA75A5")
    IMpeg2TableFilter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddPID( 
            PID p) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddTable( 
            PID p,
            TID t) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddExtension( 
            PID p,
            TID t,
            TEID e) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemovePID( 
            PID p) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveTable( 
            PID p,
            TID t) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveExtension( 
            PID p,
            TID t,
            TEID e) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMpeg2TableFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMpeg2TableFilter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMpeg2TableFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMpeg2TableFilter * This);
        
        DECLSPEC_XFGVIRT(IMpeg2TableFilter, AddPID)
        HRESULT ( STDMETHODCALLTYPE *AddPID )( 
            __RPC__in IMpeg2TableFilter * This,
            PID p);
        
        DECLSPEC_XFGVIRT(IMpeg2TableFilter, AddTable)
        HRESULT ( STDMETHODCALLTYPE *AddTable )( 
            __RPC__in IMpeg2TableFilter * This,
            PID p,
            TID t);
        
        DECLSPEC_XFGVIRT(IMpeg2TableFilter, AddExtension)
        HRESULT ( STDMETHODCALLTYPE *AddExtension )( 
            __RPC__in IMpeg2TableFilter * This,
            PID p,
            TID t,
            TEID e);
        
        DECLSPEC_XFGVIRT(IMpeg2TableFilter, RemovePID)
        HRESULT ( STDMETHODCALLTYPE *RemovePID )( 
            __RPC__in IMpeg2TableFilter * This,
            PID p);
        
        DECLSPEC_XFGVIRT(IMpeg2TableFilter, RemoveTable)
        HRESULT ( STDMETHODCALLTYPE *RemoveTable )( 
            __RPC__in IMpeg2TableFilter * This,
            PID p,
            TID t);
        
        DECLSPEC_XFGVIRT(IMpeg2TableFilter, RemoveExtension)
        HRESULT ( STDMETHODCALLTYPE *RemoveExtension )( 
            __RPC__in IMpeg2TableFilter * This,
            PID p,
            TID t,
            TEID e);
        
        END_INTERFACE
    } IMpeg2TableFilterVtbl;

    interface IMpeg2TableFilter
    {
        CONST_VTBL struct IMpeg2TableFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMpeg2TableFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMpeg2TableFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMpeg2TableFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMpeg2TableFilter_AddPID(This,p)	\
    ( (This)->lpVtbl -> AddPID(This,p) ) 

#define IMpeg2TableFilter_AddTable(This,p,t)	\
    ( (This)->lpVtbl -> AddTable(This,p,t) ) 

#define IMpeg2TableFilter_AddExtension(This,p,t,e)	\
    ( (This)->lpVtbl -> AddExtension(This,p,t,e) ) 

#define IMpeg2TableFilter_RemovePID(This,p)	\
    ( (This)->lpVtbl -> RemovePID(This,p) ) 

#define IMpeg2TableFilter_RemoveTable(This,p,t)	\
    ( (This)->lpVtbl -> RemoveTable(This,p,t) ) 

#define IMpeg2TableFilter_RemoveExtension(This,p,t,e)	\
    ( (This)->lpVtbl -> RemoveExtension(This,p,t,e) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMpeg2TableFilter_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mpeg2data_0000_0001 */
/* [local] */ 

typedef struct Mpeg2TableSampleHdr
    {
    BYTE SectionCount;
    BYTE Reserved[ 3 ];
    long SectionOffsets[ 1 ];
    } 	Mpeg2TableSampleHdr;

// {752845F1-758F-4c83-A043-4270C593308E}
DEFINE_GUID(CLSID_Mpeg2TableFilter,
0x752845f1, 0x758f, 0x4c83, 0xa0, 0x43, 0x42, 0x70, 0xc5, 0x93, 0x30, 0x8e);
#ifdef __cplusplus
class DECLSPEC_UUID("DBAF6C1B-B6A4-4898-AE65-204F0D9509A1") Mpeg2DataLib;
#endif


extern RPC_IF_HANDLE __MIDL_itf_mpeg2data_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mpeg2data_0000_0001_v0_0_s_ifspec;


#ifndef __Mpeg2DataLib_LIBRARY_DEFINED__
#define __Mpeg2DataLib_LIBRARY_DEFINED__

/* library Mpeg2DataLib */
/* [version][uuid] */ 


EXTERN_C const IID LIBID_Mpeg2DataLib;

#ifndef __IMpeg2Data_INTERFACE_DEFINED__
#define __IMpeg2Data_INTERFACE_DEFINED__

/* interface IMpeg2Data */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMpeg2Data;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B396D40-F380-4e3c-A514-1A82BF6EBFE6")
    IMpeg2Data : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSection( 
            /* [in] */ PID pid,
            /* [in] */ TID tid,
            /* [in] */ __RPC__in PMPEG2_FILTER pFilter,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ __RPC__deref_out_opt ISectionList **ppSectionList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTable( 
            /* [in] */ PID pid,
            /* [in] */ TID tid,
            /* [in] */ __RPC__in PMPEG2_FILTER pFilter,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ __RPC__deref_out_opt ISectionList **ppSectionList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamOfSections( 
            /* [in] */ PID pid,
            /* [in] */ TID tid,
            /* [in] */ __RPC__in PMPEG2_FILTER pFilter,
            /* [in] */ __RPC__in HANDLE hDataReadyEvent,
            /* [out] */ __RPC__deref_out_opt IMpeg2Stream **ppMpegStream) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMpeg2DataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMpeg2Data * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMpeg2Data * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMpeg2Data * This);
        
        DECLSPEC_XFGVIRT(IMpeg2Data, GetSection)
        HRESULT ( STDMETHODCALLTYPE *GetSection )( 
            __RPC__in IMpeg2Data * This,
            /* [in] */ PID pid,
            /* [in] */ TID tid,
            /* [in] */ __RPC__in PMPEG2_FILTER pFilter,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ __RPC__deref_out_opt ISectionList **ppSectionList);
        
        DECLSPEC_XFGVIRT(IMpeg2Data, GetTable)
        HRESULT ( STDMETHODCALLTYPE *GetTable )( 
            __RPC__in IMpeg2Data * This,
            /* [in] */ PID pid,
            /* [in] */ TID tid,
            /* [in] */ __RPC__in PMPEG2_FILTER pFilter,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ __RPC__deref_out_opt ISectionList **ppSectionList);
        
        DECLSPEC_XFGVIRT(IMpeg2Data, GetStreamOfSections)
        HRESULT ( STDMETHODCALLTYPE *GetStreamOfSections )( 
            __RPC__in IMpeg2Data * This,
            /* [in] */ PID pid,
            /* [in] */ TID tid,
            /* [in] */ __RPC__in PMPEG2_FILTER pFilter,
            /* [in] */ __RPC__in HANDLE hDataReadyEvent,
            /* [out] */ __RPC__deref_out_opt IMpeg2Stream **ppMpegStream);
        
        END_INTERFACE
    } IMpeg2DataVtbl;

    interface IMpeg2Data
    {
        CONST_VTBL struct IMpeg2DataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMpeg2Data_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMpeg2Data_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMpeg2Data_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMpeg2Data_GetSection(This,pid,tid,pFilter,dwTimeout,ppSectionList)	\
    ( (This)->lpVtbl -> GetSection(This,pid,tid,pFilter,dwTimeout,ppSectionList) ) 

#define IMpeg2Data_GetTable(This,pid,tid,pFilter,dwTimeout,ppSectionList)	\
    ( (This)->lpVtbl -> GetTable(This,pid,tid,pFilter,dwTimeout,ppSectionList) ) 

#define IMpeg2Data_GetStreamOfSections(This,pid,tid,pFilter,hDataReadyEvent,ppMpegStream)	\
    ( (This)->lpVtbl -> GetStreamOfSections(This,pid,tid,pFilter,hDataReadyEvent,ppMpegStream) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMpeg2Data_INTERFACE_DEFINED__ */


#ifndef __ISectionList_INTERFACE_DEFINED__
#define __ISectionList_INTERFACE_DEFINED__

/* interface ISectionList */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISectionList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AFEC1EB5-2A64-46c6-BF4B-AE3CCB6AFDB0")
    ISectionList : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ MPEG_REQUEST_TYPE requestType,
            /* [in] */ __RPC__in_opt IMpeg2Data *pMpeg2Data,
            /* [in] */ __RPC__in PMPEG_CONTEXT pContext,
            /* [in] */ PID pid,
            /* [in] */ TID tid,
            /* [in] */ __RPC__in PMPEG2_FILTER pFilter,
            /* [in] */ DWORD timeout,
            /* [in] */ __RPC__in HANDLE hDoneEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitializeWithRawSections( 
            /* [in] */ __RPC__in PMPEG_PACKET_LIST pmplSections) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelPendingRequest( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumberOfSections( 
            /* [out] */ __RPC__out WORD *pCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSectionData( 
            /* [in] */ WORD sectionNumber,
            /* [out] */ __RPC__out DWORD *pdwRawPacketLength,
            /* [out] */ __RPC__deref_out_opt PSECTION *ppSection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProgramIdentifier( 
            __RPC__in PID *pPid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableIdentifier( 
            __RPC__in TID *pTableId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISectionListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISectionList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISectionList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISectionList * This);
        
        DECLSPEC_XFGVIRT(ISectionList, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in ISectionList * This,
            /* [in] */ MPEG_REQUEST_TYPE requestType,
            /* [in] */ __RPC__in_opt IMpeg2Data *pMpeg2Data,
            /* [in] */ __RPC__in PMPEG_CONTEXT pContext,
            /* [in] */ PID pid,
            /* [in] */ TID tid,
            /* [in] */ __RPC__in PMPEG2_FILTER pFilter,
            /* [in] */ DWORD timeout,
            /* [in] */ __RPC__in HANDLE hDoneEvent);
        
        DECLSPEC_XFGVIRT(ISectionList, InitializeWithRawSections)
        HRESULT ( STDMETHODCALLTYPE *InitializeWithRawSections )( 
            __RPC__in ISectionList * This,
            /* [in] */ __RPC__in PMPEG_PACKET_LIST pmplSections);
        
        DECLSPEC_XFGVIRT(ISectionList, CancelPendingRequest)
        HRESULT ( STDMETHODCALLTYPE *CancelPendingRequest )( 
            __RPC__in ISectionList * This);
        
        DECLSPEC_XFGVIRT(ISectionList, GetNumberOfSections)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfSections )( 
            __RPC__in ISectionList * This,
            /* [out] */ __RPC__out WORD *pCount);
        
        DECLSPEC_XFGVIRT(ISectionList, GetSectionData)
        HRESULT ( STDMETHODCALLTYPE *GetSectionData )( 
            __RPC__in ISectionList * This,
            /* [in] */ WORD sectionNumber,
            /* [out] */ __RPC__out DWORD *pdwRawPacketLength,
            /* [out] */ __RPC__deref_out_opt PSECTION *ppSection);
        
        DECLSPEC_XFGVIRT(ISectionList, GetProgramIdentifier)
        HRESULT ( STDMETHODCALLTYPE *GetProgramIdentifier )( 
            __RPC__in ISectionList * This,
            __RPC__in PID *pPid);
        
        DECLSPEC_XFGVIRT(ISectionList, GetTableIdentifier)
        HRESULT ( STDMETHODCALLTYPE *GetTableIdentifier )( 
            __RPC__in ISectionList * This,
            __RPC__in TID *pTableId);
        
        END_INTERFACE
    } ISectionListVtbl;

    interface ISectionList
    {
        CONST_VTBL struct ISectionListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISectionList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISectionList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISectionList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISectionList_Initialize(This,requestType,pMpeg2Data,pContext,pid,tid,pFilter,timeout,hDoneEvent)	\
    ( (This)->lpVtbl -> Initialize(This,requestType,pMpeg2Data,pContext,pid,tid,pFilter,timeout,hDoneEvent) ) 

#define ISectionList_InitializeWithRawSections(This,pmplSections)	\
    ( (This)->lpVtbl -> InitializeWithRawSections(This,pmplSections) ) 

#define ISectionList_CancelPendingRequest(This)	\
    ( (This)->lpVtbl -> CancelPendingRequest(This) ) 

#define ISectionList_GetNumberOfSections(This,pCount)	\
    ( (This)->lpVtbl -> GetNumberOfSections(This,pCount) ) 

#define ISectionList_GetSectionData(This,sectionNumber,pdwRawPacketLength,ppSection)	\
    ( (This)->lpVtbl -> GetSectionData(This,sectionNumber,pdwRawPacketLength,ppSection) ) 

#define ISectionList_GetProgramIdentifier(This,pPid)	\
    ( (This)->lpVtbl -> GetProgramIdentifier(This,pPid) ) 

#define ISectionList_GetTableIdentifier(This,pTableId)	\
    ( (This)->lpVtbl -> GetTableIdentifier(This,pTableId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISectionList_INTERFACE_DEFINED__ */


#ifndef __IMpeg2Stream_INTERFACE_DEFINED__
#define __IMpeg2Stream_INTERFACE_DEFINED__

/* interface IMpeg2Stream */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMpeg2Stream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("400CC286-32A0-4ce4-9041-39571125A635")
    IMpeg2Stream : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ MPEG_REQUEST_TYPE requestType,
            /* [in] */ __RPC__in_opt IMpeg2Data *pMpeg2Data,
            /* [in] */ __RPC__in PMPEG_CONTEXT pContext,
            /* [in] */ PID pid,
            /* [in] */ TID tid,
            /* [in] */ __RPC__in PMPEG2_FILTER pFilter,
            /* [in] */ __RPC__in HANDLE hDataReadyEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SupplyDataBuffer( 
            /* [in] */ __RPC__in PMPEG_STREAM_BUFFER pStreamBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMpeg2StreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMpeg2Stream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMpeg2Stream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMpeg2Stream * This);
        
        DECLSPEC_XFGVIRT(IMpeg2Stream, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IMpeg2Stream * This,
            /* [in] */ MPEG_REQUEST_TYPE requestType,
            /* [in] */ __RPC__in_opt IMpeg2Data *pMpeg2Data,
            /* [in] */ __RPC__in PMPEG_CONTEXT pContext,
            /* [in] */ PID pid,
            /* [in] */ TID tid,
            /* [in] */ __RPC__in PMPEG2_FILTER pFilter,
            /* [in] */ __RPC__in HANDLE hDataReadyEvent);
        
        DECLSPEC_XFGVIRT(IMpeg2Stream, SupplyDataBuffer)
        HRESULT ( STDMETHODCALLTYPE *SupplyDataBuffer )( 
            __RPC__in IMpeg2Stream * This,
            /* [in] */ __RPC__in PMPEG_STREAM_BUFFER pStreamBuffer);
        
        END_INTERFACE
    } IMpeg2StreamVtbl;

    interface IMpeg2Stream
    {
        CONST_VTBL struct IMpeg2StreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMpeg2Stream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMpeg2Stream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMpeg2Stream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMpeg2Stream_Initialize(This,requestType,pMpeg2Data,pContext,pid,tid,pFilter,hDataReadyEvent)	\
    ( (This)->lpVtbl -> Initialize(This,requestType,pMpeg2Data,pContext,pid,tid,pFilter,hDataReadyEvent) ) 

#define IMpeg2Stream_SupplyDataBuffer(This,pStreamBuffer)	\
    ( (This)->lpVtbl -> SupplyDataBuffer(This,pStreamBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMpeg2Stream_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_SectionList;

#ifdef __cplusplus

class DECLSPEC_UUID("73DA5D04-4347-45d3-A9DC-FAE9DDBE558D")
SectionList;
#endif

EXTERN_C const CLSID CLSID_Mpeg2Stream;

#ifdef __cplusplus

class DECLSPEC_UUID("F91D96C7-8509-4d0b-AB26-A0DD10904BB7")
Mpeg2Stream;
#endif

EXTERN_C const CLSID CLSID_Mpeg2Data;

#ifdef __cplusplus

class DECLSPEC_UUID("C666E115-BB62-4027-A113-82D643FE2D99")
Mpeg2Data;
#endif
#endif /* __Mpeg2DataLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_mpeg2data_0000_0002 */
/* [local] */ 


#pragma pack(pop)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mpeg2data_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mpeg2data_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


