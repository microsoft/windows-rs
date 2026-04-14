

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

#ifndef __atscpsipparser_h__
#define __atscpsipparser_h__

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

#ifndef __IAtscPsipParser_FWD_DEFINED__
#define __IAtscPsipParser_FWD_DEFINED__
typedef interface IAtscPsipParser IAtscPsipParser;

#endif 	/* __IAtscPsipParser_FWD_DEFINED__ */


#ifndef __IATSC_MGT_FWD_DEFINED__
#define __IATSC_MGT_FWD_DEFINED__
typedef interface IATSC_MGT IATSC_MGT;

#endif 	/* __IATSC_MGT_FWD_DEFINED__ */


#ifndef __IATSC_VCT_FWD_DEFINED__
#define __IATSC_VCT_FWD_DEFINED__
typedef interface IATSC_VCT IATSC_VCT;

#endif 	/* __IATSC_VCT_FWD_DEFINED__ */


#ifndef __IATSC_EIT_FWD_DEFINED__
#define __IATSC_EIT_FWD_DEFINED__
typedef interface IATSC_EIT IATSC_EIT;

#endif 	/* __IATSC_EIT_FWD_DEFINED__ */


#ifndef __IATSC_ETT_FWD_DEFINED__
#define __IATSC_ETT_FWD_DEFINED__
typedef interface IATSC_ETT IATSC_ETT;

#endif 	/* __IATSC_ETT_FWD_DEFINED__ */


#ifndef __IATSC_STT_FWD_DEFINED__
#define __IATSC_STT_FWD_DEFINED__
typedef interface IATSC_STT IATSC_STT;

#endif 	/* __IATSC_STT_FWD_DEFINED__ */


#ifndef __ISCTE_EAS_FWD_DEFINED__
#define __ISCTE_EAS_FWD_DEFINED__
typedef interface ISCTE_EAS ISCTE_EAS;

#endif 	/* __ISCTE_EAS_FWD_DEFINED__ */


#ifndef __IAtscContentAdvisoryDescriptor_FWD_DEFINED__
#define __IAtscContentAdvisoryDescriptor_FWD_DEFINED__
typedef interface IAtscContentAdvisoryDescriptor IAtscContentAdvisoryDescriptor;

#endif 	/* __IAtscContentAdvisoryDescriptor_FWD_DEFINED__ */


#ifndef __ICaptionServiceDescriptor_FWD_DEFINED__
#define __ICaptionServiceDescriptor_FWD_DEFINED__
typedef interface ICaptionServiceDescriptor ICaptionServiceDescriptor;

#endif 	/* __ICaptionServiceDescriptor_FWD_DEFINED__ */


#ifndef __IServiceLocationDescriptor_FWD_DEFINED__
#define __IServiceLocationDescriptor_FWD_DEFINED__
typedef interface IServiceLocationDescriptor IServiceLocationDescriptor;

#endif 	/* __IServiceLocationDescriptor_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "mpeg2structs.h"
#include "mpeg2data.h"
#include "mpeg2psiparser.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_atscpsipparser_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)










#define ATSC_ETM_LOCATION_NOT_PRESENT        0x00
#define ATSC_ETM_LOCATION_IN_PTC_FOR_PSIP    0x01
#define ATSC_ETM_LOCATION_IN_PTC_FOR_EVENT   0x02
#define ATSC_ETM_LOCATION_RESERVED           0x03


extern RPC_IF_HANDLE __MIDL_itf_atscpsipparser_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_atscpsipparser_0000_0000_v0_0_s_ifspec;

#ifndef __IAtscPsipParser_INTERFACE_DEFINED__
#define __IAtscPsipParser_INTERFACE_DEFINED__

/* interface IAtscPsipParser */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IAtscPsipParser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B2C98995-5EB2-4fb1-B406-F3E8E2026A9A")
    IAtscPsipParser : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt IUnknown *punkMpeg2Data) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPAT( 
            /* [out] */ __RPC__deref_out_opt IPAT **ppPAT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCAT( 
            /* [in] */ DWORD dwTimeout,
            /* [out] */ __RPC__deref_out_opt ICAT **ppCAT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPMT( 
            /* [in] */ PID pid,
            /* [in] */ __RPC__in WORD *pwProgramNumber,
            /* [out] */ __RPC__deref_out_opt IPMT **ppPMT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTSDT( 
            /* [out] */ __RPC__deref_out_opt ITSDT **ppTSDT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMGT( 
            /* [out] */ __RPC__deref_out_opt IATSC_MGT **ppMGT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVCT( 
            /* [in] */ TID tableId,
            /* [in] */ BOOL fGetNextTable,
            /* [out] */ __RPC__deref_out_opt IATSC_VCT **ppVCT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEIT( 
            /* [in] */ PID pid,
            /* [in] */ __RPC__in WORD *pwSourceId,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ __RPC__deref_out_opt IATSC_EIT **ppEIT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetETT( 
            /* [in] */ PID pid,
            /* [in] */ __RPC__in WORD *wSourceId,
            /* [in] */ __RPC__in WORD *pwEventId,
            /* [out] */ __RPC__deref_out_opt IATSC_ETT **ppETT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSTT( 
            /* [out] */ __RPC__deref_out_opt IATSC_STT **ppSTT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEAS( 
            /* [in] */ PID pid,
            /* [out] */ __RPC__deref_out_opt ISCTE_EAS **ppEAS) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAtscPsipParserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAtscPsipParser * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAtscPsipParser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAtscPsipParser * This);
        
        DECLSPEC_XFGVIRT(IAtscPsipParser, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IAtscPsipParser * This,
            /* [in] */ __RPC__in_opt IUnknown *punkMpeg2Data);
        
        DECLSPEC_XFGVIRT(IAtscPsipParser, GetPAT)
        HRESULT ( STDMETHODCALLTYPE *GetPAT )( 
            __RPC__in IAtscPsipParser * This,
            /* [out] */ __RPC__deref_out_opt IPAT **ppPAT);
        
        DECLSPEC_XFGVIRT(IAtscPsipParser, GetCAT)
        HRESULT ( STDMETHODCALLTYPE *GetCAT )( 
            __RPC__in IAtscPsipParser * This,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ __RPC__deref_out_opt ICAT **ppCAT);
        
        DECLSPEC_XFGVIRT(IAtscPsipParser, GetPMT)
        HRESULT ( STDMETHODCALLTYPE *GetPMT )( 
            __RPC__in IAtscPsipParser * This,
            /* [in] */ PID pid,
            /* [in] */ __RPC__in WORD *pwProgramNumber,
            /* [out] */ __RPC__deref_out_opt IPMT **ppPMT);
        
        DECLSPEC_XFGVIRT(IAtscPsipParser, GetTSDT)
        HRESULT ( STDMETHODCALLTYPE *GetTSDT )( 
            __RPC__in IAtscPsipParser * This,
            /* [out] */ __RPC__deref_out_opt ITSDT **ppTSDT);
        
        DECLSPEC_XFGVIRT(IAtscPsipParser, GetMGT)
        HRESULT ( STDMETHODCALLTYPE *GetMGT )( 
            __RPC__in IAtscPsipParser * This,
            /* [out] */ __RPC__deref_out_opt IATSC_MGT **ppMGT);
        
        DECLSPEC_XFGVIRT(IAtscPsipParser, GetVCT)
        HRESULT ( STDMETHODCALLTYPE *GetVCT )( 
            __RPC__in IAtscPsipParser * This,
            /* [in] */ TID tableId,
            /* [in] */ BOOL fGetNextTable,
            /* [out] */ __RPC__deref_out_opt IATSC_VCT **ppVCT);
        
        DECLSPEC_XFGVIRT(IAtscPsipParser, GetEIT)
        HRESULT ( STDMETHODCALLTYPE *GetEIT )( 
            __RPC__in IAtscPsipParser * This,
            /* [in] */ PID pid,
            /* [in] */ __RPC__in WORD *pwSourceId,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ __RPC__deref_out_opt IATSC_EIT **ppEIT);
        
        DECLSPEC_XFGVIRT(IAtscPsipParser, GetETT)
        HRESULT ( STDMETHODCALLTYPE *GetETT )( 
            __RPC__in IAtscPsipParser * This,
            /* [in] */ PID pid,
            /* [in] */ __RPC__in WORD *wSourceId,
            /* [in] */ __RPC__in WORD *pwEventId,
            /* [out] */ __RPC__deref_out_opt IATSC_ETT **ppETT);
        
        DECLSPEC_XFGVIRT(IAtscPsipParser, GetSTT)
        HRESULT ( STDMETHODCALLTYPE *GetSTT )( 
            __RPC__in IAtscPsipParser * This,
            /* [out] */ __RPC__deref_out_opt IATSC_STT **ppSTT);
        
        DECLSPEC_XFGVIRT(IAtscPsipParser, GetEAS)
        HRESULT ( STDMETHODCALLTYPE *GetEAS )( 
            __RPC__in IAtscPsipParser * This,
            /* [in] */ PID pid,
            /* [out] */ __RPC__deref_out_opt ISCTE_EAS **ppEAS);
        
        END_INTERFACE
    } IAtscPsipParserVtbl;

    interface IAtscPsipParser
    {
        CONST_VTBL struct IAtscPsipParserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAtscPsipParser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAtscPsipParser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAtscPsipParser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAtscPsipParser_Initialize(This,punkMpeg2Data)	\
    ( (This)->lpVtbl -> Initialize(This,punkMpeg2Data) ) 

#define IAtscPsipParser_GetPAT(This,ppPAT)	\
    ( (This)->lpVtbl -> GetPAT(This,ppPAT) ) 

#define IAtscPsipParser_GetCAT(This,dwTimeout,ppCAT)	\
    ( (This)->lpVtbl -> GetCAT(This,dwTimeout,ppCAT) ) 

#define IAtscPsipParser_GetPMT(This,pid,pwProgramNumber,ppPMT)	\
    ( (This)->lpVtbl -> GetPMT(This,pid,pwProgramNumber,ppPMT) ) 

#define IAtscPsipParser_GetTSDT(This,ppTSDT)	\
    ( (This)->lpVtbl -> GetTSDT(This,ppTSDT) ) 

#define IAtscPsipParser_GetMGT(This,ppMGT)	\
    ( (This)->lpVtbl -> GetMGT(This,ppMGT) ) 

#define IAtscPsipParser_GetVCT(This,tableId,fGetNextTable,ppVCT)	\
    ( (This)->lpVtbl -> GetVCT(This,tableId,fGetNextTable,ppVCT) ) 

#define IAtscPsipParser_GetEIT(This,pid,pwSourceId,dwTimeout,ppEIT)	\
    ( (This)->lpVtbl -> GetEIT(This,pid,pwSourceId,dwTimeout,ppEIT) ) 

#define IAtscPsipParser_GetETT(This,pid,wSourceId,pwEventId,ppETT)	\
    ( (This)->lpVtbl -> GetETT(This,pid,wSourceId,pwEventId,ppETT) ) 

#define IAtscPsipParser_GetSTT(This,ppSTT)	\
    ( (This)->lpVtbl -> GetSTT(This,ppSTT) ) 

#define IAtscPsipParser_GetEAS(This,pid,ppEAS)	\
    ( (This)->lpVtbl -> GetEAS(This,pid,ppEAS) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAtscPsipParser_INTERFACE_DEFINED__ */


#ifndef __IATSC_MGT_INTERFACE_DEFINED__
#define __IATSC_MGT_INTERFACE_DEFINED__

/* interface IATSC_MGT */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IATSC_MGT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8877dabd-c137-4073-97e3-779407a5d87a")
    IATSC_MGT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt ISectionList *pSectionList,
            /* [in] */ __RPC__in_opt IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProtocolVersion( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ __RPC__out DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordType( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordTypePid( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out PID *ppidVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordVersionNumber( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCountOfDescriptors( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByIndex( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByTag( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [out][in] */ __RPC__inout DWORD *pdwCookie,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfTableDescriptors( 
            /* [in] */ __RPC__in DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByTag( 
            /* [in] */ BYTE bTag,
            /* [out][in] */ __RPC__inout DWORD *pdwCookie,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IATSC_MGTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IATSC_MGT * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IATSC_MGT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IATSC_MGT * This);
        
        DECLSPEC_XFGVIRT(IATSC_MGT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IATSC_MGT * This,
            /* [in] */ __RPC__in_opt ISectionList *pSectionList,
            /* [in] */ __RPC__in_opt IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IATSC_MGT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            __RPC__in IATSC_MGT * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IATSC_MGT, GetProtocolVersion)
        HRESULT ( STDMETHODCALLTYPE *GetProtocolVersion )( 
            __RPC__in IATSC_MGT * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IATSC_MGT, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            __RPC__in IATSC_MGT * This,
            /* [out] */ __RPC__out DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IATSC_MGT, GetRecordType)
        HRESULT ( STDMETHODCALLTYPE *GetRecordType )( 
            __RPC__in IATSC_MGT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IATSC_MGT, GetRecordTypePid)
        HRESULT ( STDMETHODCALLTYPE *GetRecordTypePid )( 
            __RPC__in IATSC_MGT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out PID *ppidVal);
        
        DECLSPEC_XFGVIRT(IATSC_MGT, GetRecordVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetRecordVersionNumber )( 
            __RPC__in IATSC_MGT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IATSC_MGT, GetRecordCountOfDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCountOfDescriptors )( 
            __RPC__in IATSC_MGT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IATSC_MGT, GetRecordDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByIndex )( 
            __RPC__in IATSC_MGT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IATSC_MGT, GetRecordDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByTag )( 
            __RPC__in IATSC_MGT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [out][in] */ __RPC__inout DWORD *pdwCookie,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IATSC_MGT, GetCountOfTableDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfTableDescriptors )( 
            __RPC__in IATSC_MGT * This,
            /* [in] */ __RPC__in DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IATSC_MGT, GetTableDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByIndex )( 
            __RPC__in IATSC_MGT * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IATSC_MGT, GetTableDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByTag )( 
            __RPC__in IATSC_MGT * This,
            /* [in] */ BYTE bTag,
            /* [out][in] */ __RPC__inout DWORD *pdwCookie,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor);
        
        END_INTERFACE
    } IATSC_MGTVtbl;

    interface IATSC_MGT
    {
        CONST_VTBL struct IATSC_MGTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IATSC_MGT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IATSC_MGT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IATSC_MGT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IATSC_MGT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IATSC_MGT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IATSC_MGT_GetProtocolVersion(This,pbVal)	\
    ( (This)->lpVtbl -> GetProtocolVersion(This,pbVal) ) 

#define IATSC_MGT_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IATSC_MGT_GetRecordType(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordType(This,dwRecordIndex,pwVal) ) 

#define IATSC_MGT_GetRecordTypePid(This,dwRecordIndex,ppidVal)	\
    ( (This)->lpVtbl -> GetRecordTypePid(This,dwRecordIndex,ppidVal) ) 

#define IATSC_MGT_GetRecordVersionNumber(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordVersionNumber(This,dwRecordIndex,pbVal) ) 

#define IATSC_MGT_GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal) ) 

#define IATSC_MGT_GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor) ) 

#define IATSC_MGT_GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor) ) 

#define IATSC_MGT_GetCountOfTableDescriptors(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfTableDescriptors(This,pdwVal) ) 

#define IATSC_MGT_GetTableDescriptorByIndex(This,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByIndex(This,dwIndex,ppDescriptor) ) 

#define IATSC_MGT_GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IATSC_MGT_INTERFACE_DEFINED__ */


#ifndef __IATSC_VCT_INTERFACE_DEFINED__
#define __IATSC_VCT_INTERFACE_DEFINED__

/* interface IATSC_VCT */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IATSC_VCT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("26879a18-32f9-46c6-91f0-fb6479270e8c")
    IATSC_VCT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt ISectionList *pSectionList,
            /* [in] */ __RPC__in_opt IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransportStreamId( 
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProtocolVersion( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ __RPC__out DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordName( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pwsName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordMajorChannelNumber( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordMinorChannelNumber( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordModulationMode( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCarrierFrequency( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordTransportStreamId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordProgramNumber( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordEtmLocation( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordIsAccessControlledBitSet( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BOOL *pfVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordIsHiddenBitSet( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BOOL *pfVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordIsPathSelectBitSet( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BOOL *pfVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordIsOutOfBandBitSet( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BOOL *pfVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordIsHideGuideBitSet( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BOOL *pfVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordServiceType( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordSourceId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCountOfDescriptors( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByIndex( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByTag( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [out][in] */ __RPC__inout DWORD *pdwCookie,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfTableDescriptors( 
            /* [in] */ __RPC__in DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByTag( 
            /* [in] */ BYTE bTag,
            /* [out][in] */ __RPC__inout DWORD *pdwCookie,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IATSC_VCTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IATSC_VCT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IATSC_VCT * This);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ __RPC__in_opt ISectionList *pSectionList,
            /* [in] */ __RPC__in_opt IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            __RPC__in IATSC_VCT * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetTransportStreamId)
        HRESULT ( STDMETHODCALLTYPE *GetTransportStreamId )( 
            __RPC__in IATSC_VCT * This,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetProtocolVersion)
        HRESULT ( STDMETHODCALLTYPE *GetProtocolVersion )( 
            __RPC__in IATSC_VCT * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            __RPC__in IATSC_VCT * This,
            /* [out] */ __RPC__out DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordName)
        HRESULT ( STDMETHODCALLTYPE *GetRecordName )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pwsName);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordMajorChannelNumber)
        HRESULT ( STDMETHODCALLTYPE *GetRecordMajorChannelNumber )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordMinorChannelNumber)
        HRESULT ( STDMETHODCALLTYPE *GetRecordMinorChannelNumber )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordModulationMode)
        HRESULT ( STDMETHODCALLTYPE *GetRecordModulationMode )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordCarrierFrequency)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCarrierFrequency )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordTransportStreamId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordTransportStreamId )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordProgramNumber)
        HRESULT ( STDMETHODCALLTYPE *GetRecordProgramNumber )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordEtmLocation)
        HRESULT ( STDMETHODCALLTYPE *GetRecordEtmLocation )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordIsAccessControlledBitSet)
        HRESULT ( STDMETHODCALLTYPE *GetRecordIsAccessControlledBitSet )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BOOL *pfVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordIsHiddenBitSet)
        HRESULT ( STDMETHODCALLTYPE *GetRecordIsHiddenBitSet )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BOOL *pfVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordIsPathSelectBitSet)
        HRESULT ( STDMETHODCALLTYPE *GetRecordIsPathSelectBitSet )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BOOL *pfVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordIsOutOfBandBitSet)
        HRESULT ( STDMETHODCALLTYPE *GetRecordIsOutOfBandBitSet )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BOOL *pfVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordIsHideGuideBitSet)
        HRESULT ( STDMETHODCALLTYPE *GetRecordIsHideGuideBitSet )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BOOL *pfVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordServiceType)
        HRESULT ( STDMETHODCALLTYPE *GetRecordServiceType )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordSourceId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordSourceId )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordCountOfDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCountOfDescriptors )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByIndex )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetRecordDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByTag )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [out][in] */ __RPC__inout DWORD *pdwCookie,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetCountOfTableDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfTableDescriptors )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ __RPC__in DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetTableDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByIndex )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IATSC_VCT, GetTableDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByTag )( 
            __RPC__in IATSC_VCT * This,
            /* [in] */ BYTE bTag,
            /* [out][in] */ __RPC__inout DWORD *pdwCookie,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor);
        
        END_INTERFACE
    } IATSC_VCTVtbl;

    interface IATSC_VCT
    {
        CONST_VTBL struct IATSC_VCTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IATSC_VCT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IATSC_VCT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IATSC_VCT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IATSC_VCT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IATSC_VCT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IATSC_VCT_GetTransportStreamId(This,pwVal)	\
    ( (This)->lpVtbl -> GetTransportStreamId(This,pwVal) ) 

#define IATSC_VCT_GetProtocolVersion(This,pbVal)	\
    ( (This)->lpVtbl -> GetProtocolVersion(This,pbVal) ) 

#define IATSC_VCT_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IATSC_VCT_GetRecordName(This,dwRecordIndex,pwsName)	\
    ( (This)->lpVtbl -> GetRecordName(This,dwRecordIndex,pwsName) ) 

#define IATSC_VCT_GetRecordMajorChannelNumber(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordMajorChannelNumber(This,dwRecordIndex,pwVal) ) 

#define IATSC_VCT_GetRecordMinorChannelNumber(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordMinorChannelNumber(This,dwRecordIndex,pwVal) ) 

#define IATSC_VCT_GetRecordModulationMode(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordModulationMode(This,dwRecordIndex,pbVal) ) 

#define IATSC_VCT_GetRecordCarrierFrequency(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCarrierFrequency(This,dwRecordIndex,pdwVal) ) 

#define IATSC_VCT_GetRecordTransportStreamId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordTransportStreamId(This,dwRecordIndex,pwVal) ) 

#define IATSC_VCT_GetRecordProgramNumber(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordProgramNumber(This,dwRecordIndex,pwVal) ) 

#define IATSC_VCT_GetRecordEtmLocation(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordEtmLocation(This,dwRecordIndex,pbVal) ) 

#define IATSC_VCT_GetRecordIsAccessControlledBitSet(This,dwRecordIndex,pfVal)	\
    ( (This)->lpVtbl -> GetRecordIsAccessControlledBitSet(This,dwRecordIndex,pfVal) ) 

#define IATSC_VCT_GetRecordIsHiddenBitSet(This,dwRecordIndex,pfVal)	\
    ( (This)->lpVtbl -> GetRecordIsHiddenBitSet(This,dwRecordIndex,pfVal) ) 

#define IATSC_VCT_GetRecordIsPathSelectBitSet(This,dwRecordIndex,pfVal)	\
    ( (This)->lpVtbl -> GetRecordIsPathSelectBitSet(This,dwRecordIndex,pfVal) ) 

#define IATSC_VCT_GetRecordIsOutOfBandBitSet(This,dwRecordIndex,pfVal)	\
    ( (This)->lpVtbl -> GetRecordIsOutOfBandBitSet(This,dwRecordIndex,pfVal) ) 

#define IATSC_VCT_GetRecordIsHideGuideBitSet(This,dwRecordIndex,pfVal)	\
    ( (This)->lpVtbl -> GetRecordIsHideGuideBitSet(This,dwRecordIndex,pfVal) ) 

#define IATSC_VCT_GetRecordServiceType(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordServiceType(This,dwRecordIndex,pbVal) ) 

#define IATSC_VCT_GetRecordSourceId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordSourceId(This,dwRecordIndex,pwVal) ) 

#define IATSC_VCT_GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal) ) 

#define IATSC_VCT_GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor) ) 

#define IATSC_VCT_GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor) ) 

#define IATSC_VCT_GetCountOfTableDescriptors(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfTableDescriptors(This,pdwVal) ) 

#define IATSC_VCT_GetTableDescriptorByIndex(This,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByIndex(This,dwIndex,ppDescriptor) ) 

#define IATSC_VCT_GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IATSC_VCT_INTERFACE_DEFINED__ */


#ifndef __IATSC_EIT_INTERFACE_DEFINED__
#define __IATSC_EIT_INTERFACE_DEFINED__

/* interface IATSC_EIT */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IATSC_EIT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d7c212d7-76a2-4b4b-aa56-846879a80096")
    IATSC_EIT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt ISectionList *pSectionList,
            /* [in] */ __RPC__in_opt IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourceId( 
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProtocolVersion( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ __RPC__out DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordEventId( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordStartTime( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out MPEG_DATE_AND_TIME *pmdtVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordEtmLocation( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDuration( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out MPEG_DURATION *pmdVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordTitleText( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out DWORD *pdwLength,
            /* [out] */ __RPC__deref_out_opt BYTE **ppText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCountOfDescriptors( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByIndex( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByTag( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [out][in] */ __RPC__inout DWORD *pdwCookie,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IATSC_EITVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IATSC_EIT * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IATSC_EIT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IATSC_EIT * This);
        
        DECLSPEC_XFGVIRT(IATSC_EIT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IATSC_EIT * This,
            /* [in] */ __RPC__in_opt ISectionList *pSectionList,
            /* [in] */ __RPC__in_opt IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IATSC_EIT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            __RPC__in IATSC_EIT * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IATSC_EIT, GetSourceId)
        HRESULT ( STDMETHODCALLTYPE *GetSourceId )( 
            __RPC__in IATSC_EIT * This,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IATSC_EIT, GetProtocolVersion)
        HRESULT ( STDMETHODCALLTYPE *GetProtocolVersion )( 
            __RPC__in IATSC_EIT * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IATSC_EIT, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            __RPC__in IATSC_EIT * This,
            /* [out] */ __RPC__out DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IATSC_EIT, GetRecordEventId)
        HRESULT ( STDMETHODCALLTYPE *GetRecordEventId )( 
            __RPC__in IATSC_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IATSC_EIT, GetRecordStartTime)
        HRESULT ( STDMETHODCALLTYPE *GetRecordStartTime )( 
            __RPC__in IATSC_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out MPEG_DATE_AND_TIME *pmdtVal);
        
        DECLSPEC_XFGVIRT(IATSC_EIT, GetRecordEtmLocation)
        HRESULT ( STDMETHODCALLTYPE *GetRecordEtmLocation )( 
            __RPC__in IATSC_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IATSC_EIT, GetRecordDuration)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDuration )( 
            __RPC__in IATSC_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out MPEG_DURATION *pmdVal);
        
        DECLSPEC_XFGVIRT(IATSC_EIT, GetRecordTitleText)
        HRESULT ( STDMETHODCALLTYPE *GetRecordTitleText )( 
            __RPC__in IATSC_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out DWORD *pdwLength,
            /* [out] */ __RPC__deref_out_opt BYTE **ppText);
        
        DECLSPEC_XFGVIRT(IATSC_EIT, GetRecordCountOfDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCountOfDescriptors )( 
            __RPC__in IATSC_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ __RPC__out DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IATSC_EIT, GetRecordDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByIndex )( 
            __RPC__in IATSC_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IATSC_EIT, GetRecordDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByTag )( 
            __RPC__in IATSC_EIT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [out][in] */ __RPC__inout DWORD *pdwCookie,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor);
        
        END_INTERFACE
    } IATSC_EITVtbl;

    interface IATSC_EIT
    {
        CONST_VTBL struct IATSC_EITVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IATSC_EIT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IATSC_EIT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IATSC_EIT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IATSC_EIT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IATSC_EIT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IATSC_EIT_GetSourceId(This,pwVal)	\
    ( (This)->lpVtbl -> GetSourceId(This,pwVal) ) 

#define IATSC_EIT_GetProtocolVersion(This,pbVal)	\
    ( (This)->lpVtbl -> GetProtocolVersion(This,pbVal) ) 

#define IATSC_EIT_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IATSC_EIT_GetRecordEventId(This,dwRecordIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordEventId(This,dwRecordIndex,pwVal) ) 

#define IATSC_EIT_GetRecordStartTime(This,dwRecordIndex,pmdtVal)	\
    ( (This)->lpVtbl -> GetRecordStartTime(This,dwRecordIndex,pmdtVal) ) 

#define IATSC_EIT_GetRecordEtmLocation(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordEtmLocation(This,dwRecordIndex,pbVal) ) 

#define IATSC_EIT_GetRecordDuration(This,dwRecordIndex,pmdVal)	\
    ( (This)->lpVtbl -> GetRecordDuration(This,dwRecordIndex,pmdVal) ) 

#define IATSC_EIT_GetRecordTitleText(This,dwRecordIndex,pdwLength,ppText)	\
    ( (This)->lpVtbl -> GetRecordTitleText(This,dwRecordIndex,pdwLength,ppText) ) 

#define IATSC_EIT_GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal) ) 

#define IATSC_EIT_GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByIndex(This,dwRecordIndex,dwIndex,ppDescriptor) ) 

#define IATSC_EIT_GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IATSC_EIT_INTERFACE_DEFINED__ */


#ifndef __IATSC_ETT_INTERFACE_DEFINED__
#define __IATSC_ETT_INTERFACE_DEFINED__

/* interface IATSC_ETT */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IATSC_ETT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5a142cc9-b8cf-4a86-a040-e9cadf3ef3e7")
    IATSC_ETT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt ISectionList *pSectionList,
            /* [in] */ __RPC__in_opt IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProtocolVersion( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEtmId( 
            /* [out] */ __RPC__out DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExtendedMessageText( 
            /* [out] */ __RPC__out DWORD *pdwLength,
            /* [out] */ __RPC__deref_out_opt BYTE **ppText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IATSC_ETTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IATSC_ETT * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IATSC_ETT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IATSC_ETT * This);
        
        DECLSPEC_XFGVIRT(IATSC_ETT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IATSC_ETT * This,
            /* [in] */ __RPC__in_opt ISectionList *pSectionList,
            /* [in] */ __RPC__in_opt IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IATSC_ETT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            __RPC__in IATSC_ETT * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IATSC_ETT, GetProtocolVersion)
        HRESULT ( STDMETHODCALLTYPE *GetProtocolVersion )( 
            __RPC__in IATSC_ETT * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IATSC_ETT, GetEtmId)
        HRESULT ( STDMETHODCALLTYPE *GetEtmId )( 
            __RPC__in IATSC_ETT * This,
            /* [out] */ __RPC__out DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IATSC_ETT, GetExtendedMessageText)
        HRESULT ( STDMETHODCALLTYPE *GetExtendedMessageText )( 
            __RPC__in IATSC_ETT * This,
            /* [out] */ __RPC__out DWORD *pdwLength,
            /* [out] */ __RPC__deref_out_opt BYTE **ppText);
        
        END_INTERFACE
    } IATSC_ETTVtbl;

    interface IATSC_ETT
    {
        CONST_VTBL struct IATSC_ETTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IATSC_ETT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IATSC_ETT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IATSC_ETT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IATSC_ETT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IATSC_ETT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IATSC_ETT_GetProtocolVersion(This,pbVal)	\
    ( (This)->lpVtbl -> GetProtocolVersion(This,pbVal) ) 

#define IATSC_ETT_GetEtmId(This,pdwVal)	\
    ( (This)->lpVtbl -> GetEtmId(This,pdwVal) ) 

#define IATSC_ETT_GetExtendedMessageText(This,pdwLength,ppText)	\
    ( (This)->lpVtbl -> GetExtendedMessageText(This,pdwLength,ppText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IATSC_ETT_INTERFACE_DEFINED__ */


#ifndef __IATSC_STT_INTERFACE_DEFINED__
#define __IATSC_STT_INTERFACE_DEFINED__

/* interface IATSC_STT */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IATSC_STT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6bf42423-217d-4d6f-81e1-3a7b360ec896")
    IATSC_STT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt ISectionList *pSectionList,
            /* [in] */ __RPC__in_opt IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProtocolVersion( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSystemTime( 
            /* [out] */ __RPC__out MPEG_DATE_AND_TIME *pmdtSystemTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGpsUtcOffset( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDaylightSavings( 
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfTableDescriptors( 
            /* [out] */ __RPC__out DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByTag( 
            /* [in] */ BYTE bTag,
            /* [out][in] */ __RPC__inout DWORD *pdwCookie,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IATSC_STTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IATSC_STT * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IATSC_STT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IATSC_STT * This);
        
        DECLSPEC_XFGVIRT(IATSC_STT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IATSC_STT * This,
            /* [in] */ __RPC__in_opt ISectionList *pSectionList,
            /* [in] */ __RPC__in_opt IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IATSC_STT, GetProtocolVersion)
        HRESULT ( STDMETHODCALLTYPE *GetProtocolVersion )( 
            __RPC__in IATSC_STT * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IATSC_STT, GetSystemTime)
        HRESULT ( STDMETHODCALLTYPE *GetSystemTime )( 
            __RPC__in IATSC_STT * This,
            /* [out] */ __RPC__out MPEG_DATE_AND_TIME *pmdtSystemTime);
        
        DECLSPEC_XFGVIRT(IATSC_STT, GetGpsUtcOffset)
        HRESULT ( STDMETHODCALLTYPE *GetGpsUtcOffset )( 
            __RPC__in IATSC_STT * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IATSC_STT, GetDaylightSavings)
        HRESULT ( STDMETHODCALLTYPE *GetDaylightSavings )( 
            __RPC__in IATSC_STT * This,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IATSC_STT, GetCountOfTableDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfTableDescriptors )( 
            __RPC__in IATSC_STT * This,
            /* [out] */ __RPC__out DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IATSC_STT, GetTableDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByIndex )( 
            __RPC__in IATSC_STT * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IATSC_STT, GetTableDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByTag )( 
            __RPC__in IATSC_STT * This,
            /* [in] */ BYTE bTag,
            /* [out][in] */ __RPC__inout DWORD *pdwCookie,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor);
        
        END_INTERFACE
    } IATSC_STTVtbl;

    interface IATSC_STT
    {
        CONST_VTBL struct IATSC_STTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IATSC_STT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IATSC_STT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IATSC_STT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IATSC_STT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IATSC_STT_GetProtocolVersion(This,pbVal)	\
    ( (This)->lpVtbl -> GetProtocolVersion(This,pbVal) ) 

#define IATSC_STT_GetSystemTime(This,pmdtSystemTime)	\
    ( (This)->lpVtbl -> GetSystemTime(This,pmdtSystemTime) ) 

#define IATSC_STT_GetGpsUtcOffset(This,pbVal)	\
    ( (This)->lpVtbl -> GetGpsUtcOffset(This,pbVal) ) 

#define IATSC_STT_GetDaylightSavings(This,pwVal)	\
    ( (This)->lpVtbl -> GetDaylightSavings(This,pwVal) ) 

#define IATSC_STT_GetCountOfTableDescriptors(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfTableDescriptors(This,pdwVal) ) 

#define IATSC_STT_GetTableDescriptorByIndex(This,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByIndex(This,dwIndex,ppDescriptor) ) 

#define IATSC_STT_GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IATSC_STT_INTERFACE_DEFINED__ */


#ifndef __ISCTE_EAS_INTERFACE_DEFINED__
#define __ISCTE_EAS_INTERFACE_DEFINED__

/* interface ISCTE_EAS */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISCTE_EAS;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1FF544D6-161D-4fae-9FAA-4F9F492AE999")
    ISCTE_EAS : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt ISectionList *pSectionList,
            /* [in] */ __RPC__in_opt IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSequencyNumber( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProtocolVersion( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEASEventID( 
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOriginatorCode( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEASEventCodeLen( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEASEventCode( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRawNatureOfActivationTextLen( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRawNatureOfActivationText( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNatureOfActivationText( 
            /* [in] */ __RPC__in BSTR bstrIS0639code,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTimeRemaining( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStartTime( 
            /* [out] */ __RPC__out DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDuration( 
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAlertPriority( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDetailsOOBSourceID( 
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDetailsMajor( 
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDetailsMinor( 
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDetailsAudioOOBSourceID( 
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAlertText( 
            /* [in] */ __RPC__in BSTR bstrIS0639code,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRawAlertTextLen( 
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRawAlertText( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLocationCount( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLocationCodes( 
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbState,
            /* [out] */ __RPC__out BYTE *pbCountySubdivision,
            /* [out] */ __RPC__out WORD *pwCounty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExceptionCount( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExceptionService( 
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbIBRef,
            /* [out] */ __RPC__out WORD *pwFirst,
            /* [out] */ __RPC__out WORD *pwSecond) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfTableDescriptors( 
            /* [out] */ __RPC__out DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByTag( 
            /* [in] */ BYTE bTag,
            /* [out][in] */ __RPC__inout DWORD *pdwCookie,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISCTE_EASVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISCTE_EAS * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISCTE_EAS * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISCTE_EAS * This);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in ISCTE_EAS * This,
            /* [in] */ __RPC__in_opt ISectionList *pSectionList,
            /* [in] */ __RPC__in_opt IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetSequencyNumber)
        HRESULT ( STDMETHODCALLTYPE *GetSequencyNumber )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetProtocolVersion)
        HRESULT ( STDMETHODCALLTYPE *GetProtocolVersion )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetEASEventID)
        HRESULT ( STDMETHODCALLTYPE *GetEASEventID )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetOriginatorCode)
        HRESULT ( STDMETHODCALLTYPE *GetOriginatorCode )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetEASEventCodeLen)
        HRESULT ( STDMETHODCALLTYPE *GetEASEventCodeLen )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetEASEventCode)
        HRESULT ( STDMETHODCALLTYPE *GetEASEventCode )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetRawNatureOfActivationTextLen)
        HRESULT ( STDMETHODCALLTYPE *GetRawNatureOfActivationTextLen )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetRawNatureOfActivationText)
        HRESULT ( STDMETHODCALLTYPE *GetRawNatureOfActivationText )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetNatureOfActivationText)
        HRESULT ( STDMETHODCALLTYPE *GetNatureOfActivationText )( 
            __RPC__in ISCTE_EAS * This,
            /* [in] */ __RPC__in BSTR bstrIS0639code,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrString);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetTimeRemaining)
        HRESULT ( STDMETHODCALLTYPE *GetTimeRemaining )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetStartTime)
        HRESULT ( STDMETHODCALLTYPE *GetStartTime )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetDuration)
        HRESULT ( STDMETHODCALLTYPE *GetDuration )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetAlertPriority)
        HRESULT ( STDMETHODCALLTYPE *GetAlertPriority )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetDetailsOOBSourceID)
        HRESULT ( STDMETHODCALLTYPE *GetDetailsOOBSourceID )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetDetailsMajor)
        HRESULT ( STDMETHODCALLTYPE *GetDetailsMajor )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetDetailsMinor)
        HRESULT ( STDMETHODCALLTYPE *GetDetailsMinor )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetDetailsAudioOOBSourceID)
        HRESULT ( STDMETHODCALLTYPE *GetDetailsAudioOOBSourceID )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetAlertText)
        HRESULT ( STDMETHODCALLTYPE *GetAlertText )( 
            __RPC__in ISCTE_EAS * This,
            /* [in] */ __RPC__in BSTR bstrIS0639code,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrString);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetRawAlertTextLen)
        HRESULT ( STDMETHODCALLTYPE *GetRawAlertTextLen )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetRawAlertText)
        HRESULT ( STDMETHODCALLTYPE *GetRawAlertText )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetLocationCount)
        HRESULT ( STDMETHODCALLTYPE *GetLocationCount )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetLocationCodes)
        HRESULT ( STDMETHODCALLTYPE *GetLocationCodes )( 
            __RPC__in ISCTE_EAS * This,
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbState,
            /* [out] */ __RPC__out BYTE *pbCountySubdivision,
            /* [out] */ __RPC__out WORD *pwCounty);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetExceptionCount)
        HRESULT ( STDMETHODCALLTYPE *GetExceptionCount )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetExceptionService)
        HRESULT ( STDMETHODCALLTYPE *GetExceptionService )( 
            __RPC__in ISCTE_EAS * This,
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbIBRef,
            /* [out] */ __RPC__out WORD *pwFirst,
            /* [out] */ __RPC__out WORD *pwSecond);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetCountOfTableDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfTableDescriptors )( 
            __RPC__in ISCTE_EAS * This,
            /* [out] */ __RPC__out DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetTableDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByIndex )( 
            __RPC__in ISCTE_EAS * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(ISCTE_EAS, GetTableDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByTag )( 
            __RPC__in ISCTE_EAS * This,
            /* [in] */ BYTE bTag,
            /* [out][in] */ __RPC__inout DWORD *pdwCookie,
            /* [out] */ __RPC__deref_out_opt IGenericDescriptor **ppDescriptor);
        
        END_INTERFACE
    } ISCTE_EASVtbl;

    interface ISCTE_EAS
    {
        CONST_VTBL struct ISCTE_EASVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISCTE_EAS_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISCTE_EAS_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISCTE_EAS_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISCTE_EAS_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define ISCTE_EAS_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define ISCTE_EAS_GetSequencyNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetSequencyNumber(This,pbVal) ) 

#define ISCTE_EAS_GetProtocolVersion(This,pbVal)	\
    ( (This)->lpVtbl -> GetProtocolVersion(This,pbVal) ) 

#define ISCTE_EAS_GetEASEventID(This,pwVal)	\
    ( (This)->lpVtbl -> GetEASEventID(This,pwVal) ) 

#define ISCTE_EAS_GetOriginatorCode(This,pbVal)	\
    ( (This)->lpVtbl -> GetOriginatorCode(This,pbVal) ) 

#define ISCTE_EAS_GetEASEventCodeLen(This,pbVal)	\
    ( (This)->lpVtbl -> GetEASEventCodeLen(This,pbVal) ) 

#define ISCTE_EAS_GetEASEventCode(This,pbVal)	\
    ( (This)->lpVtbl -> GetEASEventCode(This,pbVal) ) 

#define ISCTE_EAS_GetRawNatureOfActivationTextLen(This,pbVal)	\
    ( (This)->lpVtbl -> GetRawNatureOfActivationTextLen(This,pbVal) ) 

#define ISCTE_EAS_GetRawNatureOfActivationText(This,pbVal)	\
    ( (This)->lpVtbl -> GetRawNatureOfActivationText(This,pbVal) ) 

#define ISCTE_EAS_GetNatureOfActivationText(This,bstrIS0639code,pbstrString)	\
    ( (This)->lpVtbl -> GetNatureOfActivationText(This,bstrIS0639code,pbstrString) ) 

#define ISCTE_EAS_GetTimeRemaining(This,pbVal)	\
    ( (This)->lpVtbl -> GetTimeRemaining(This,pbVal) ) 

#define ISCTE_EAS_GetStartTime(This,pdwVal)	\
    ( (This)->lpVtbl -> GetStartTime(This,pdwVal) ) 

#define ISCTE_EAS_GetDuration(This,pwVal)	\
    ( (This)->lpVtbl -> GetDuration(This,pwVal) ) 

#define ISCTE_EAS_GetAlertPriority(This,pbVal)	\
    ( (This)->lpVtbl -> GetAlertPriority(This,pbVal) ) 

#define ISCTE_EAS_GetDetailsOOBSourceID(This,pwVal)	\
    ( (This)->lpVtbl -> GetDetailsOOBSourceID(This,pwVal) ) 

#define ISCTE_EAS_GetDetailsMajor(This,pwVal)	\
    ( (This)->lpVtbl -> GetDetailsMajor(This,pwVal) ) 

#define ISCTE_EAS_GetDetailsMinor(This,pwVal)	\
    ( (This)->lpVtbl -> GetDetailsMinor(This,pwVal) ) 

#define ISCTE_EAS_GetDetailsAudioOOBSourceID(This,pwVal)	\
    ( (This)->lpVtbl -> GetDetailsAudioOOBSourceID(This,pwVal) ) 

#define ISCTE_EAS_GetAlertText(This,bstrIS0639code,pbstrString)	\
    ( (This)->lpVtbl -> GetAlertText(This,bstrIS0639code,pbstrString) ) 

#define ISCTE_EAS_GetRawAlertTextLen(This,pwVal)	\
    ( (This)->lpVtbl -> GetRawAlertTextLen(This,pwVal) ) 

#define ISCTE_EAS_GetRawAlertText(This,pbVal)	\
    ( (This)->lpVtbl -> GetRawAlertText(This,pbVal) ) 

#define ISCTE_EAS_GetLocationCount(This,pbVal)	\
    ( (This)->lpVtbl -> GetLocationCount(This,pbVal) ) 

#define ISCTE_EAS_GetLocationCodes(This,bIndex,pbState,pbCountySubdivision,pwCounty)	\
    ( (This)->lpVtbl -> GetLocationCodes(This,bIndex,pbState,pbCountySubdivision,pwCounty) ) 

#define ISCTE_EAS_GetExceptionCount(This,pbVal)	\
    ( (This)->lpVtbl -> GetExceptionCount(This,pbVal) ) 

#define ISCTE_EAS_GetExceptionService(This,bIndex,pbIBRef,pwFirst,pwSecond)	\
    ( (This)->lpVtbl -> GetExceptionService(This,bIndex,pbIBRef,pwFirst,pwSecond) ) 

#define ISCTE_EAS_GetCountOfTableDescriptors(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfTableDescriptors(This,pdwVal) ) 

#define ISCTE_EAS_GetTableDescriptorByIndex(This,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByIndex(This,dwIndex,ppDescriptor) ) 

#define ISCTE_EAS_GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISCTE_EAS_INTERFACE_DEFINED__ */


#ifndef __IAtscContentAdvisoryDescriptor_INTERFACE_DEFINED__
#define __IAtscContentAdvisoryDescriptor_INTERFACE_DEFINED__

/* interface IAtscContentAdvisoryDescriptor */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IAtscContentAdvisoryDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FF76E60C-0283-43ea-BA32-B422238547EE")
    IAtscContentAdvisoryDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRatingRegionCount( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordRatingRegion( 
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordRatedDimensions( 
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordRatingDimension( 
            /* [in] */ BYTE bIndexOuter,
            /* [in] */ BYTE bIndexInner,
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordRatingValue( 
            /* [in] */ BYTE bIndexOuter,
            /* [in] */ BYTE bIndexInner,
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordRatingDescriptionText( 
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbLength,
            /* [out] */ __RPC__deref_out_opt BYTE **ppText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAtscContentAdvisoryDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAtscContentAdvisoryDescriptor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAtscContentAdvisoryDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAtscContentAdvisoryDescriptor * This);
        
        DECLSPEC_XFGVIRT(IAtscContentAdvisoryDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            __RPC__in IAtscContentAdvisoryDescriptor * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IAtscContentAdvisoryDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            __RPC__in IAtscContentAdvisoryDescriptor * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IAtscContentAdvisoryDescriptor, GetRatingRegionCount)
        HRESULT ( STDMETHODCALLTYPE *GetRatingRegionCount )( 
            __RPC__in IAtscContentAdvisoryDescriptor * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IAtscContentAdvisoryDescriptor, GetRecordRatingRegion)
        HRESULT ( STDMETHODCALLTYPE *GetRecordRatingRegion )( 
            __RPC__in IAtscContentAdvisoryDescriptor * This,
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IAtscContentAdvisoryDescriptor, GetRecordRatedDimensions)
        HRESULT ( STDMETHODCALLTYPE *GetRecordRatedDimensions )( 
            __RPC__in IAtscContentAdvisoryDescriptor * This,
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IAtscContentAdvisoryDescriptor, GetRecordRatingDimension)
        HRESULT ( STDMETHODCALLTYPE *GetRecordRatingDimension )( 
            __RPC__in IAtscContentAdvisoryDescriptor * This,
            /* [in] */ BYTE bIndexOuter,
            /* [in] */ BYTE bIndexInner,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IAtscContentAdvisoryDescriptor, GetRecordRatingValue)
        HRESULT ( STDMETHODCALLTYPE *GetRecordRatingValue )( 
            __RPC__in IAtscContentAdvisoryDescriptor * This,
            /* [in] */ BYTE bIndexOuter,
            /* [in] */ BYTE bIndexInner,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IAtscContentAdvisoryDescriptor, GetRecordRatingDescriptionText)
        HRESULT ( STDMETHODCALLTYPE *GetRecordRatingDescriptionText )( 
            __RPC__in IAtscContentAdvisoryDescriptor * This,
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbLength,
            /* [out] */ __RPC__deref_out_opt BYTE **ppText);
        
        END_INTERFACE
    } IAtscContentAdvisoryDescriptorVtbl;

    interface IAtscContentAdvisoryDescriptor
    {
        CONST_VTBL struct IAtscContentAdvisoryDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAtscContentAdvisoryDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAtscContentAdvisoryDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAtscContentAdvisoryDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAtscContentAdvisoryDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IAtscContentAdvisoryDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IAtscContentAdvisoryDescriptor_GetRatingRegionCount(This,pbVal)	\
    ( (This)->lpVtbl -> GetRatingRegionCount(This,pbVal) ) 

#define IAtscContentAdvisoryDescriptor_GetRecordRatingRegion(This,bIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordRatingRegion(This,bIndex,pbVal) ) 

#define IAtscContentAdvisoryDescriptor_GetRecordRatedDimensions(This,bIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordRatedDimensions(This,bIndex,pbVal) ) 

#define IAtscContentAdvisoryDescriptor_GetRecordRatingDimension(This,bIndexOuter,bIndexInner,pbVal)	\
    ( (This)->lpVtbl -> GetRecordRatingDimension(This,bIndexOuter,bIndexInner,pbVal) ) 

#define IAtscContentAdvisoryDescriptor_GetRecordRatingValue(This,bIndexOuter,bIndexInner,pbVal)	\
    ( (This)->lpVtbl -> GetRecordRatingValue(This,bIndexOuter,bIndexInner,pbVal) ) 

#define IAtscContentAdvisoryDescriptor_GetRecordRatingDescriptionText(This,bIndex,pbLength,ppText)	\
    ( (This)->lpVtbl -> GetRecordRatingDescriptionText(This,bIndex,pbLength,ppText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAtscContentAdvisoryDescriptor_INTERFACE_DEFINED__ */


#ifndef __ICaptionServiceDescriptor_INTERFACE_DEFINED__
#define __ICaptionServiceDescriptor_INTERFACE_DEFINED__

/* interface ICaptionServiceDescriptor */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ICaptionServiceDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("40834007-6834-46f0-BD45-D5F6A6BE258C")
    ICaptionServiceDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNumberOfServices( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguageCode( 
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out_ecount_full(3) BYTE LangCode[ 3 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCaptionServiceNumber( 
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCCType( 
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEasyReader( 
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWideAspectRatio( 
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICaptionServiceDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICaptionServiceDescriptor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICaptionServiceDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICaptionServiceDescriptor * This);
        
        DECLSPEC_XFGVIRT(ICaptionServiceDescriptor, GetNumberOfServices)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfServices )( 
            __RPC__in ICaptionServiceDescriptor * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ICaptionServiceDescriptor, GetLanguageCode)
        HRESULT ( STDMETHODCALLTYPE *GetLanguageCode )( 
            __RPC__in ICaptionServiceDescriptor * This,
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out_ecount_full(3) BYTE LangCode[ 3 ]);
        
        DECLSPEC_XFGVIRT(ICaptionServiceDescriptor, GetCaptionServiceNumber)
        HRESULT ( STDMETHODCALLTYPE *GetCaptionServiceNumber )( 
            __RPC__in ICaptionServiceDescriptor * This,
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ICaptionServiceDescriptor, GetCCType)
        HRESULT ( STDMETHODCALLTYPE *GetCCType )( 
            __RPC__in ICaptionServiceDescriptor * This,
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ICaptionServiceDescriptor, GetEasyReader)
        HRESULT ( STDMETHODCALLTYPE *GetEasyReader )( 
            __RPC__in ICaptionServiceDescriptor * This,
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ICaptionServiceDescriptor, GetWideAspectRatio)
        HRESULT ( STDMETHODCALLTYPE *GetWideAspectRatio )( 
            __RPC__in ICaptionServiceDescriptor * This,
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        END_INTERFACE
    } ICaptionServiceDescriptorVtbl;

    interface ICaptionServiceDescriptor
    {
        CONST_VTBL struct ICaptionServiceDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICaptionServiceDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICaptionServiceDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICaptionServiceDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICaptionServiceDescriptor_GetNumberOfServices(This,pbVal)	\
    ( (This)->lpVtbl -> GetNumberOfServices(This,pbVal) ) 

#define ICaptionServiceDescriptor_GetLanguageCode(This,bIndex,LangCode)	\
    ( (This)->lpVtbl -> GetLanguageCode(This,bIndex,LangCode) ) 

#define ICaptionServiceDescriptor_GetCaptionServiceNumber(This,bIndex,pbVal)	\
    ( (This)->lpVtbl -> GetCaptionServiceNumber(This,bIndex,pbVal) ) 

#define ICaptionServiceDescriptor_GetCCType(This,bIndex,pbVal)	\
    ( (This)->lpVtbl -> GetCCType(This,bIndex,pbVal) ) 

#define ICaptionServiceDescriptor_GetEasyReader(This,bIndex,pbVal)	\
    ( (This)->lpVtbl -> GetEasyReader(This,bIndex,pbVal) ) 

#define ICaptionServiceDescriptor_GetWideAspectRatio(This,bIndex,pbVal)	\
    ( (This)->lpVtbl -> GetWideAspectRatio(This,bIndex,pbVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICaptionServiceDescriptor_INTERFACE_DEFINED__ */


#ifndef __IServiceLocationDescriptor_INTERFACE_DEFINED__
#define __IServiceLocationDescriptor_INTERFACE_DEFINED__

/* interface IServiceLocationDescriptor */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IServiceLocationDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("58C3C827-9D91-4215-BFF3-820A49F0904C")
    IServiceLocationDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPCR_PID( 
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumberOfElements( 
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetElementStreamType( 
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetElementPID( 
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetElementLanguageCode( 
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out_ecount_full(3) BYTE LangCode[ 3 ]) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServiceLocationDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IServiceLocationDescriptor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IServiceLocationDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IServiceLocationDescriptor * This);
        
        DECLSPEC_XFGVIRT(IServiceLocationDescriptor, GetPCR_PID)
        HRESULT ( STDMETHODCALLTYPE *GetPCR_PID )( 
            __RPC__in IServiceLocationDescriptor * This,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IServiceLocationDescriptor, GetNumberOfElements)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfElements )( 
            __RPC__in IServiceLocationDescriptor * This,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IServiceLocationDescriptor, GetElementStreamType)
        HRESULT ( STDMETHODCALLTYPE *GetElementStreamType )( 
            __RPC__in IServiceLocationDescriptor * This,
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IServiceLocationDescriptor, GetElementPID)
        HRESULT ( STDMETHODCALLTYPE *GetElementPID )( 
            __RPC__in IServiceLocationDescriptor * This,
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IServiceLocationDescriptor, GetElementLanguageCode)
        HRESULT ( STDMETHODCALLTYPE *GetElementLanguageCode )( 
            __RPC__in IServiceLocationDescriptor * This,
            /* [in] */ BYTE bIndex,
            /* [out] */ __RPC__out_ecount_full(3) BYTE LangCode[ 3 ]);
        
        END_INTERFACE
    } IServiceLocationDescriptorVtbl;

    interface IServiceLocationDescriptor
    {
        CONST_VTBL struct IServiceLocationDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServiceLocationDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServiceLocationDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServiceLocationDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServiceLocationDescriptor_GetPCR_PID(This,pwVal)	\
    ( (This)->lpVtbl -> GetPCR_PID(This,pwVal) ) 

#define IServiceLocationDescriptor_GetNumberOfElements(This,pbVal)	\
    ( (This)->lpVtbl -> GetNumberOfElements(This,pbVal) ) 

#define IServiceLocationDescriptor_GetElementStreamType(This,bIndex,pbVal)	\
    ( (This)->lpVtbl -> GetElementStreamType(This,bIndex,pbVal) ) 

#define IServiceLocationDescriptor_GetElementPID(This,bIndex,pwVal)	\
    ( (This)->lpVtbl -> GetElementPID(This,bIndex,pwVal) ) 

#define IServiceLocationDescriptor_GetElementLanguageCode(This,bIndex,LangCode)	\
    ( (This)->lpVtbl -> GetElementLanguageCode(This,bIndex,LangCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServiceLocationDescriptor_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_atscpsipparser_0000_0010 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_atscpsipparser_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_atscpsipparser_0000_0010_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


