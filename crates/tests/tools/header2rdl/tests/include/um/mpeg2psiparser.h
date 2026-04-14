

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

#ifndef __mpeg2psiparser_h__
#define __mpeg2psiparser_h__

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

#ifndef __IGenericDescriptor_FWD_DEFINED__
#define __IGenericDescriptor_FWD_DEFINED__
typedef interface IGenericDescriptor IGenericDescriptor;

#endif 	/* __IGenericDescriptor_FWD_DEFINED__ */


#ifndef __IGenericDescriptor2_FWD_DEFINED__
#define __IGenericDescriptor2_FWD_DEFINED__
typedef interface IGenericDescriptor2 IGenericDescriptor2;

#endif 	/* __IGenericDescriptor2_FWD_DEFINED__ */


#ifndef __IPAT_FWD_DEFINED__
#define __IPAT_FWD_DEFINED__
typedef interface IPAT IPAT;

#endif 	/* __IPAT_FWD_DEFINED__ */


#ifndef __ICAT_FWD_DEFINED__
#define __ICAT_FWD_DEFINED__
typedef interface ICAT ICAT;

#endif 	/* __ICAT_FWD_DEFINED__ */


#ifndef __IPMT_FWD_DEFINED__
#define __IPMT_FWD_DEFINED__
typedef interface IPMT IPMT;

#endif 	/* __IPMT_FWD_DEFINED__ */


#ifndef __ITSDT_FWD_DEFINED__
#define __ITSDT_FWD_DEFINED__
typedef interface ITSDT ITSDT;

#endif 	/* __ITSDT_FWD_DEFINED__ */


#ifndef __IPSITables_FWD_DEFINED__
#define __IPSITables_FWD_DEFINED__
typedef interface IPSITables IPSITables;

#endif 	/* __IPSITables_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "mpeg2structs.h"
#include "mpeg2data.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mpeg2psiparser_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)




extern RPC_IF_HANDLE __MIDL_itf_mpeg2psiparser_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mpeg2psiparser_0000_0000_v0_0_s_ifspec;

#ifndef __IGenericDescriptor_INTERFACE_DEFINED__
#define __IGenericDescriptor_INTERFACE_DEFINED__

/* interface IGenericDescriptor */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IGenericDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6A5918F8-A77A-4f61-AED0-5702BDCDA3E6")
    IGenericDescriptor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ BYTE *pbDesc,
            /* [in] */ INT bCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBody( 
            /* [out] */ BYTE **ppbVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGenericDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IGenericDescriptor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IGenericDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IGenericDescriptor * This);
        
        DECLSPEC_XFGVIRT(IGenericDescriptor, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IGenericDescriptor * This,
            /* [in] */ BYTE *pbDesc,
            /* [in] */ INT bCount);
        
        DECLSPEC_XFGVIRT(IGenericDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IGenericDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IGenericDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IGenericDescriptor * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IGenericDescriptor, GetBody)
        HRESULT ( STDMETHODCALLTYPE *GetBody )( 
            IGenericDescriptor * This,
            /* [out] */ BYTE **ppbVal);
        
        END_INTERFACE
    } IGenericDescriptorVtbl;

    interface IGenericDescriptor
    {
        CONST_VTBL struct IGenericDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGenericDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGenericDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGenericDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGenericDescriptor_Initialize(This,pbDesc,bCount)	\
    ( (This)->lpVtbl -> Initialize(This,pbDesc,bCount) ) 

#define IGenericDescriptor_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IGenericDescriptor_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IGenericDescriptor_GetBody(This,ppbVal)	\
    ( (This)->lpVtbl -> GetBody(This,ppbVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGenericDescriptor_INTERFACE_DEFINED__ */


#ifndef __IGenericDescriptor2_INTERFACE_DEFINED__
#define __IGenericDescriptor2_INTERFACE_DEFINED__

/* interface IGenericDescriptor2 */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IGenericDescriptor2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BF02FB7E-9792-4e10-A68D-033A2CC246A5")
    IGenericDescriptor2 : public IGenericDescriptor
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ BYTE *pbDesc,
            /* [in] */ WORD wCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ WORD *pwVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGenericDescriptor2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IGenericDescriptor2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IGenericDescriptor2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IGenericDescriptor2 * This);
        
        DECLSPEC_XFGVIRT(IGenericDescriptor, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IGenericDescriptor2 * This,
            /* [in] */ BYTE *pbDesc,
            /* [in] */ INT bCount);
        
        DECLSPEC_XFGVIRT(IGenericDescriptor, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IGenericDescriptor2 * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IGenericDescriptor, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IGenericDescriptor2 * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IGenericDescriptor, GetBody)
        HRESULT ( STDMETHODCALLTYPE *GetBody )( 
            IGenericDescriptor2 * This,
            /* [out] */ BYTE **ppbVal);
        
        DECLSPEC_XFGVIRT(IGenericDescriptor2, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IGenericDescriptor2 * This,
            /* [in] */ BYTE *pbDesc,
            /* [in] */ WORD wCount);
        
        DECLSPEC_XFGVIRT(IGenericDescriptor2, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            IGenericDescriptor2 * This,
            /* [out] */ WORD *pwVal);
        
        END_INTERFACE
    } IGenericDescriptor2Vtbl;

    interface IGenericDescriptor2
    {
        CONST_VTBL struct IGenericDescriptor2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGenericDescriptor2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGenericDescriptor2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGenericDescriptor2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGenericDescriptor2_Initialize(This,pbDesc,bCount)	\
    ( (This)->lpVtbl -> Initialize(This,pbDesc,bCount) ) 

#define IGenericDescriptor2_GetTag(This,pbVal)	\
    ( (This)->lpVtbl -> GetTag(This,pbVal) ) 

#define IGenericDescriptor2_GetLength(This,pbVal)	\
    ( (This)->lpVtbl -> GetLength(This,pbVal) ) 

#define IGenericDescriptor2_GetBody(This,ppbVal)	\
    ( (This)->lpVtbl -> GetBody(This,ppbVal) ) 


#define IGenericDescriptor2_Initialize(This,pbDesc,wCount)	\
    ( (This)->lpVtbl -> Initialize(This,pbDesc,wCount) ) 

#define IGenericDescriptor2_GetLength(This,pwVal)	\
    ( (This)->lpVtbl -> GetLength(This,pwVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGenericDescriptor2_INTERFACE_DEFINED__ */


#ifndef __IPAT_INTERFACE_DEFINED__
#define __IPAT_INTERFACE_DEFINED__

/* interface IPAT */
/* [unique][uuid][local][object] */ 

typedef /* [public] */ struct __MIDL_IPAT_0001
    {
    WORD wProgramNumber;
    WORD wProgramMapPID;
    } 	ProgramElement;


EXTERN_C const IID IID_IPAT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6623B511-4B5F-43c3-9A01-E8FF84188060")
    IPAT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransportStreamId( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordProgramNumber( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordProgramMapPid( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindRecordProgramMapPid( 
            /* [in] */ WORD wProgramNumber,
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForNextTable( 
            /* [in] */ HANDLE hNextTableAvailable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextTable( 
            /* [out] */ IPAT **ppPAT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForWhenCurrent( 
            /* [in] */ HANDLE hNextTableIsCurrent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertNextToCurrent( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPATVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPAT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPAT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPAT * This);
        
        DECLSPEC_XFGVIRT(IPAT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IPAT * This,
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IPAT, GetTransportStreamId)
        HRESULT ( STDMETHODCALLTYPE *GetTransportStreamId )( 
            IPAT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IPAT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            IPAT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IPAT, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IPAT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IPAT, GetRecordProgramNumber)
        HRESULT ( STDMETHODCALLTYPE *GetRecordProgramNumber )( 
            IPAT * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IPAT, GetRecordProgramMapPid)
        HRESULT ( STDMETHODCALLTYPE *GetRecordProgramMapPid )( 
            IPAT * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IPAT, FindRecordProgramMapPid)
        HRESULT ( STDMETHODCALLTYPE *FindRecordProgramMapPid )( 
            IPAT * This,
            /* [in] */ WORD wProgramNumber,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IPAT, RegisterForNextTable)
        HRESULT ( STDMETHODCALLTYPE *RegisterForNextTable )( 
            IPAT * This,
            /* [in] */ HANDLE hNextTableAvailable);
        
        DECLSPEC_XFGVIRT(IPAT, GetNextTable)
        HRESULT ( STDMETHODCALLTYPE *GetNextTable )( 
            IPAT * This,
            /* [out] */ IPAT **ppPAT);
        
        DECLSPEC_XFGVIRT(IPAT, RegisterForWhenCurrent)
        HRESULT ( STDMETHODCALLTYPE *RegisterForWhenCurrent )( 
            IPAT * This,
            /* [in] */ HANDLE hNextTableIsCurrent);
        
        DECLSPEC_XFGVIRT(IPAT, ConvertNextToCurrent)
        HRESULT ( STDMETHODCALLTYPE *ConvertNextToCurrent )( 
            IPAT * This);
        
        END_INTERFACE
    } IPATVtbl;

    interface IPAT
    {
        CONST_VTBL struct IPATVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPAT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPAT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPAT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPAT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IPAT_GetTransportStreamId(This,pwVal)	\
    ( (This)->lpVtbl -> GetTransportStreamId(This,pwVal) ) 

#define IPAT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IPAT_GetCountOfRecords(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pdwVal) ) 

#define IPAT_GetRecordProgramNumber(This,dwIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordProgramNumber(This,dwIndex,pwVal) ) 

#define IPAT_GetRecordProgramMapPid(This,dwIndex,pwVal)	\
    ( (This)->lpVtbl -> GetRecordProgramMapPid(This,dwIndex,pwVal) ) 

#define IPAT_FindRecordProgramMapPid(This,wProgramNumber,pwVal)	\
    ( (This)->lpVtbl -> FindRecordProgramMapPid(This,wProgramNumber,pwVal) ) 

#define IPAT_RegisterForNextTable(This,hNextTableAvailable)	\
    ( (This)->lpVtbl -> RegisterForNextTable(This,hNextTableAvailable) ) 

#define IPAT_GetNextTable(This,ppPAT)	\
    ( (This)->lpVtbl -> GetNextTable(This,ppPAT) ) 

#define IPAT_RegisterForWhenCurrent(This,hNextTableIsCurrent)	\
    ( (This)->lpVtbl -> RegisterForWhenCurrent(This,hNextTableIsCurrent) ) 

#define IPAT_ConvertNextToCurrent(This)	\
    ( (This)->lpVtbl -> ConvertNextToCurrent(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPAT_INTERFACE_DEFINED__ */


#ifndef __ICAT_INTERFACE_DEFINED__
#define __ICAT_INTERFACE_DEFINED__

/* interface ICAT */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ICAT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7C6995FB-2A31-4bd7-953E-B1AD7FB7D31C")
    ICAT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfTableDescriptors( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByTag( 
            /* [in] */ BYTE bTag,
            /* [out][in] */ DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForNextTable( 
            /* [in] */ HANDLE hNextTableAvailable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextTable( 
            /* [in] */ DWORD dwTimeout,
            /* [out] */ ICAT **ppCAT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForWhenCurrent( 
            /* [in] */ HANDLE hNextTableIsCurrent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertNextToCurrent( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICATVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICAT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICAT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICAT * This);
        
        DECLSPEC_XFGVIRT(ICAT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            ICAT * This,
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(ICAT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            ICAT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ICAT, GetCountOfTableDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfTableDescriptors )( 
            ICAT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(ICAT, GetTableDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByIndex )( 
            ICAT * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(ICAT, GetTableDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByTag )( 
            ICAT * This,
            /* [in] */ BYTE bTag,
            /* [out][in] */ DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(ICAT, RegisterForNextTable)
        HRESULT ( STDMETHODCALLTYPE *RegisterForNextTable )( 
            ICAT * This,
            /* [in] */ HANDLE hNextTableAvailable);
        
        DECLSPEC_XFGVIRT(ICAT, GetNextTable)
        HRESULT ( STDMETHODCALLTYPE *GetNextTable )( 
            ICAT * This,
            /* [in] */ DWORD dwTimeout,
            /* [out] */ ICAT **ppCAT);
        
        DECLSPEC_XFGVIRT(ICAT, RegisterForWhenCurrent)
        HRESULT ( STDMETHODCALLTYPE *RegisterForWhenCurrent )( 
            ICAT * This,
            /* [in] */ HANDLE hNextTableIsCurrent);
        
        DECLSPEC_XFGVIRT(ICAT, ConvertNextToCurrent)
        HRESULT ( STDMETHODCALLTYPE *ConvertNextToCurrent )( 
            ICAT * This);
        
        END_INTERFACE
    } ICATVtbl;

    interface ICAT
    {
        CONST_VTBL struct ICATVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICAT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICAT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICAT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICAT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define ICAT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define ICAT_GetCountOfTableDescriptors(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfTableDescriptors(This,pdwVal) ) 

#define ICAT_GetTableDescriptorByIndex(This,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByIndex(This,dwIndex,ppDescriptor) ) 

#define ICAT_GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor) ) 

#define ICAT_RegisterForNextTable(This,hNextTableAvailable)	\
    ( (This)->lpVtbl -> RegisterForNextTable(This,hNextTableAvailable) ) 

#define ICAT_GetNextTable(This,dwTimeout,ppCAT)	\
    ( (This)->lpVtbl -> GetNextTable(This,dwTimeout,ppCAT) ) 

#define ICAT_RegisterForWhenCurrent(This,hNextTableIsCurrent)	\
    ( (This)->lpVtbl -> RegisterForWhenCurrent(This,hNextTableIsCurrent) ) 

#define ICAT_ConvertNextToCurrent(This)	\
    ( (This)->lpVtbl -> ConvertNextToCurrent(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICAT_INTERFACE_DEFINED__ */


#ifndef __IPMT_INTERFACE_DEFINED__
#define __IPMT_INTERFACE_DEFINED__

/* interface IPMT */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IPMT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("01F3B398-9527-4736-94DB-5195878E97A8")
    IPMT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProgramNumber( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPcrPid( 
            /* [out] */ PID *pPidVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfTableDescriptors( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByTag( 
            /* [in] */ BYTE bTag,
            /* [out][in] */ DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfRecords( 
            /* [out] */ WORD *pwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordStreamType( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordElementaryPid( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ PID *pPidVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordCountOfDescriptors( 
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByIndex( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwDescIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecordDescriptorByTag( 
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [out][in] */ DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryServiceGatewayInfo( 
            /* [out] */ DSMCC_ELEMENT **ppDSMCCList,
            /* [out] */ UINT *puiCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryMPEInfo( 
            /* [out] */ MPE_ELEMENT **ppMPEList,
            /* [out] */ UINT *puiCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForNextTable( 
            /* [in] */ HANDLE hNextTableAvailable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextTable( 
            /* [out] */ IPMT **ppPMT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForWhenCurrent( 
            /* [in] */ HANDLE hNextTableIsCurrent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertNextToCurrent( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPMTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPMT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPMT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPMT * This);
        
        DECLSPEC_XFGVIRT(IPMT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IPMT * This,
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(IPMT, GetProgramNumber)
        HRESULT ( STDMETHODCALLTYPE *GetProgramNumber )( 
            IPMT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IPMT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            IPMT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IPMT, GetPcrPid)
        HRESULT ( STDMETHODCALLTYPE *GetPcrPid )( 
            IPMT * This,
            /* [out] */ PID *pPidVal);
        
        DECLSPEC_XFGVIRT(IPMT, GetCountOfTableDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfTableDescriptors )( 
            IPMT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IPMT, GetTableDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByIndex )( 
            IPMT * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IPMT, GetTableDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByTag )( 
            IPMT * This,
            /* [in] */ BYTE bTag,
            /* [out][in] */ DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IPMT, GetCountOfRecords)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfRecords )( 
            IPMT * This,
            /* [out] */ WORD *pwVal);
        
        DECLSPEC_XFGVIRT(IPMT, GetRecordStreamType)
        HRESULT ( STDMETHODCALLTYPE *GetRecordStreamType )( 
            IPMT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(IPMT, GetRecordElementaryPid)
        HRESULT ( STDMETHODCALLTYPE *GetRecordElementaryPid )( 
            IPMT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ PID *pPidVal);
        
        DECLSPEC_XFGVIRT(IPMT, GetRecordCountOfDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetRecordCountOfDescriptors )( 
            IPMT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IPMT, GetRecordDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByIndex )( 
            IPMT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ DWORD dwDescIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IPMT, GetRecordDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetRecordDescriptorByTag )( 
            IPMT * This,
            /* [in] */ DWORD dwRecordIndex,
            /* [in] */ BYTE bTag,
            /* [out][in] */ DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IPMT, QueryServiceGatewayInfo)
        HRESULT ( STDMETHODCALLTYPE *QueryServiceGatewayInfo )( 
            IPMT * This,
            /* [out] */ DSMCC_ELEMENT **ppDSMCCList,
            /* [out] */ UINT *puiCount);
        
        DECLSPEC_XFGVIRT(IPMT, QueryMPEInfo)
        HRESULT ( STDMETHODCALLTYPE *QueryMPEInfo )( 
            IPMT * This,
            /* [out] */ MPE_ELEMENT **ppMPEList,
            /* [out] */ UINT *puiCount);
        
        DECLSPEC_XFGVIRT(IPMT, RegisterForNextTable)
        HRESULT ( STDMETHODCALLTYPE *RegisterForNextTable )( 
            IPMT * This,
            /* [in] */ HANDLE hNextTableAvailable);
        
        DECLSPEC_XFGVIRT(IPMT, GetNextTable)
        HRESULT ( STDMETHODCALLTYPE *GetNextTable )( 
            IPMT * This,
            /* [out] */ IPMT **ppPMT);
        
        DECLSPEC_XFGVIRT(IPMT, RegisterForWhenCurrent)
        HRESULT ( STDMETHODCALLTYPE *RegisterForWhenCurrent )( 
            IPMT * This,
            /* [in] */ HANDLE hNextTableIsCurrent);
        
        DECLSPEC_XFGVIRT(IPMT, ConvertNextToCurrent)
        HRESULT ( STDMETHODCALLTYPE *ConvertNextToCurrent )( 
            IPMT * This);
        
        END_INTERFACE
    } IPMTVtbl;

    interface IPMT
    {
        CONST_VTBL struct IPMTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPMT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPMT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPMT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPMT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define IPMT_GetProgramNumber(This,pwVal)	\
    ( (This)->lpVtbl -> GetProgramNumber(This,pwVal) ) 

#define IPMT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define IPMT_GetPcrPid(This,pPidVal)	\
    ( (This)->lpVtbl -> GetPcrPid(This,pPidVal) ) 

#define IPMT_GetCountOfTableDescriptors(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfTableDescriptors(This,pdwVal) ) 

#define IPMT_GetTableDescriptorByIndex(This,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByIndex(This,dwIndex,ppDescriptor) ) 

#define IPMT_GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor) ) 

#define IPMT_GetCountOfRecords(This,pwVal)	\
    ( (This)->lpVtbl -> GetCountOfRecords(This,pwVal) ) 

#define IPMT_GetRecordStreamType(This,dwRecordIndex,pbVal)	\
    ( (This)->lpVtbl -> GetRecordStreamType(This,dwRecordIndex,pbVal) ) 

#define IPMT_GetRecordElementaryPid(This,dwRecordIndex,pPidVal)	\
    ( (This)->lpVtbl -> GetRecordElementaryPid(This,dwRecordIndex,pPidVal) ) 

#define IPMT_GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal)	\
    ( (This)->lpVtbl -> GetRecordCountOfDescriptors(This,dwRecordIndex,pdwVal) ) 

#define IPMT_GetRecordDescriptorByIndex(This,dwRecordIndex,dwDescIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByIndex(This,dwRecordIndex,dwDescIndex,ppDescriptor) ) 

#define IPMT_GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetRecordDescriptorByTag(This,dwRecordIndex,bTag,pdwCookie,ppDescriptor) ) 

#define IPMT_QueryServiceGatewayInfo(This,ppDSMCCList,puiCount)	\
    ( (This)->lpVtbl -> QueryServiceGatewayInfo(This,ppDSMCCList,puiCount) ) 

#define IPMT_QueryMPEInfo(This,ppMPEList,puiCount)	\
    ( (This)->lpVtbl -> QueryMPEInfo(This,ppMPEList,puiCount) ) 

#define IPMT_RegisterForNextTable(This,hNextTableAvailable)	\
    ( (This)->lpVtbl -> RegisterForNextTable(This,hNextTableAvailable) ) 

#define IPMT_GetNextTable(This,ppPMT)	\
    ( (This)->lpVtbl -> GetNextTable(This,ppPMT) ) 

#define IPMT_RegisterForWhenCurrent(This,hNextTableIsCurrent)	\
    ( (This)->lpVtbl -> RegisterForWhenCurrent(This,hNextTableIsCurrent) ) 

#define IPMT_ConvertNextToCurrent(This)	\
    ( (This)->lpVtbl -> ConvertNextToCurrent(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPMT_INTERFACE_DEFINED__ */


#ifndef __ITSDT_INTERFACE_DEFINED__
#define __ITSDT_INTERFACE_DEFINED__

/* interface ITSDT */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITSDT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D19BDB43-405B-4a7c-A791-C89110C33165")
    ITSDT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionNumber( 
            /* [out] */ BYTE *pbVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCountOfTableDescriptors( 
            /* [out] */ DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableDescriptorByTag( 
            /* [in] */ BYTE bTag,
            /* [out][in] */ DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForNextTable( 
            /* [in] */ HANDLE hNextTableAvailable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextTable( 
            /* [out] */ ITSDT **ppTSDT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForWhenCurrent( 
            /* [in] */ HANDLE hNextTableIsCurrent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertNextToCurrent( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITSDTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITSDT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITSDT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITSDT * This);
        
        DECLSPEC_XFGVIRT(ITSDT, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            ITSDT * This,
            /* [in] */ ISectionList *pSectionList,
            /* [in] */ IMpeg2Data *pMPEGData);
        
        DECLSPEC_XFGVIRT(ITSDT, GetVersionNumber)
        HRESULT ( STDMETHODCALLTYPE *GetVersionNumber )( 
            ITSDT * This,
            /* [out] */ BYTE *pbVal);
        
        DECLSPEC_XFGVIRT(ITSDT, GetCountOfTableDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetCountOfTableDescriptors )( 
            ITSDT * This,
            /* [out] */ DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(ITSDT, GetTableDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByIndex )( 
            ITSDT * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(ITSDT, GetTableDescriptorByTag)
        HRESULT ( STDMETHODCALLTYPE *GetTableDescriptorByTag )( 
            ITSDT * This,
            /* [in] */ BYTE bTag,
            /* [out][in] */ DWORD *pdwCookie,
            /* [out] */ IGenericDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(ITSDT, RegisterForNextTable)
        HRESULT ( STDMETHODCALLTYPE *RegisterForNextTable )( 
            ITSDT * This,
            /* [in] */ HANDLE hNextTableAvailable);
        
        DECLSPEC_XFGVIRT(ITSDT, GetNextTable)
        HRESULT ( STDMETHODCALLTYPE *GetNextTable )( 
            ITSDT * This,
            /* [out] */ ITSDT **ppTSDT);
        
        DECLSPEC_XFGVIRT(ITSDT, RegisterForWhenCurrent)
        HRESULT ( STDMETHODCALLTYPE *RegisterForWhenCurrent )( 
            ITSDT * This,
            /* [in] */ HANDLE hNextTableIsCurrent);
        
        DECLSPEC_XFGVIRT(ITSDT, ConvertNextToCurrent)
        HRESULT ( STDMETHODCALLTYPE *ConvertNextToCurrent )( 
            ITSDT * This);
        
        END_INTERFACE
    } ITSDTVtbl;

    interface ITSDT
    {
        CONST_VTBL struct ITSDTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITSDT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITSDT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITSDT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITSDT_Initialize(This,pSectionList,pMPEGData)	\
    ( (This)->lpVtbl -> Initialize(This,pSectionList,pMPEGData) ) 

#define ITSDT_GetVersionNumber(This,pbVal)	\
    ( (This)->lpVtbl -> GetVersionNumber(This,pbVal) ) 

#define ITSDT_GetCountOfTableDescriptors(This,pdwVal)	\
    ( (This)->lpVtbl -> GetCountOfTableDescriptors(This,pdwVal) ) 

#define ITSDT_GetTableDescriptorByIndex(This,dwIndex,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByIndex(This,dwIndex,ppDescriptor) ) 

#define ITSDT_GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor)	\
    ( (This)->lpVtbl -> GetTableDescriptorByTag(This,bTag,pdwCookie,ppDescriptor) ) 

#define ITSDT_RegisterForNextTable(This,hNextTableAvailable)	\
    ( (This)->lpVtbl -> RegisterForNextTable(This,hNextTableAvailable) ) 

#define ITSDT_GetNextTable(This,ppTSDT)	\
    ( (This)->lpVtbl -> GetNextTable(This,ppTSDT) ) 

#define ITSDT_RegisterForWhenCurrent(This,hNextTableIsCurrent)	\
    ( (This)->lpVtbl -> RegisterForWhenCurrent(This,hNextTableIsCurrent) ) 

#define ITSDT_ConvertNextToCurrent(This)	\
    ( (This)->lpVtbl -> ConvertNextToCurrent(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITSDT_INTERFACE_DEFINED__ */


#ifndef __IPSITables_INTERFACE_DEFINED__
#define __IPSITables_INTERFACE_DEFINED__

/* interface IPSITables */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPSITables;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("919F24C5-7B14-42ac-A4B0-2AE08DAF00AC")
    IPSITables : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTable( 
            /* [in] */ DWORD dwTSID,
            /* [in] */ DWORD dwTID_PID,
            /* [in] */ DWORD dwHashedVer,
            /* [in] */ DWORD dwPara4,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppIUnknown) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPSITablesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPSITables * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPSITables * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPSITables * This);
        
        DECLSPEC_XFGVIRT(IPSITables, GetTable)
        HRESULT ( STDMETHODCALLTYPE *GetTable )( 
            __RPC__in IPSITables * This,
            /* [in] */ DWORD dwTSID,
            /* [in] */ DWORD dwTID_PID,
            /* [in] */ DWORD dwHashedVer,
            /* [in] */ DWORD dwPara4,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppIUnknown);
        
        END_INTERFACE
    } IPSITablesVtbl;

    interface IPSITables
    {
        CONST_VTBL struct IPSITablesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPSITables_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPSITables_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPSITables_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPSITables_GetTable(This,dwTSID,dwTID_PID,dwHashedVer,dwPara4,ppIUnknown)	\
    ( (This)->lpVtbl -> GetTable(This,dwTSID,dwTID_PID,dwHashedVer,dwPara4,ppIUnknown) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPSITables_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mpeg2psiparser_0000_0007 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mpeg2psiparser_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mpeg2psiparser_0000_0007_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


