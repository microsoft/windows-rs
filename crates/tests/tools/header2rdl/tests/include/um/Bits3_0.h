

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

#ifndef __bits3_0_h__
#define __bits3_0_h__

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

#ifndef __IBitsPeerCacheRecord_FWD_DEFINED__
#define __IBitsPeerCacheRecord_FWD_DEFINED__
typedef interface IBitsPeerCacheRecord IBitsPeerCacheRecord;

#endif 	/* __IBitsPeerCacheRecord_FWD_DEFINED__ */


#ifndef __IEnumBitsPeerCacheRecords_FWD_DEFINED__
#define __IEnumBitsPeerCacheRecords_FWD_DEFINED__
typedef interface IEnumBitsPeerCacheRecords IEnumBitsPeerCacheRecords;

#endif 	/* __IEnumBitsPeerCacheRecords_FWD_DEFINED__ */


#ifndef __IBitsPeer_FWD_DEFINED__
#define __IBitsPeer_FWD_DEFINED__
typedef interface IBitsPeer IBitsPeer;

#endif 	/* __IBitsPeer_FWD_DEFINED__ */


#ifndef __IEnumBitsPeers_FWD_DEFINED__
#define __IEnumBitsPeers_FWD_DEFINED__
typedef interface IEnumBitsPeers IEnumBitsPeers;

#endif 	/* __IEnumBitsPeers_FWD_DEFINED__ */


#ifndef __IBitsPeerCacheAdministration_FWD_DEFINED__
#define __IBitsPeerCacheAdministration_FWD_DEFINED__
typedef interface IBitsPeerCacheAdministration IBitsPeerCacheAdministration;

#endif 	/* __IBitsPeerCacheAdministration_FWD_DEFINED__ */


#ifndef __IBackgroundCopyJob4_FWD_DEFINED__
#define __IBackgroundCopyJob4_FWD_DEFINED__
typedef interface IBackgroundCopyJob4 IBackgroundCopyJob4;

#endif 	/* __IBackgroundCopyJob4_FWD_DEFINED__ */


#ifndef __IBackgroundCopyFile3_FWD_DEFINED__
#define __IBackgroundCopyFile3_FWD_DEFINED__
typedef interface IBackgroundCopyFile3 IBackgroundCopyFile3;

#endif 	/* __IBackgroundCopyFile3_FWD_DEFINED__ */


#ifndef __IBackgroundCopyCallback2_FWD_DEFINED__
#define __IBackgroundCopyCallback2_FWD_DEFINED__
typedef interface IBackgroundCopyCallback2 IBackgroundCopyCallback2;

#endif 	/* __IBackgroundCopyCallback2_FWD_DEFINED__ */


#ifndef __BackgroundCopyManager3_0_FWD_DEFINED__
#define __BackgroundCopyManager3_0_FWD_DEFINED__

#ifdef __cplusplus
typedef class BackgroundCopyManager3_0 BackgroundCopyManager3_0;
#else
typedef struct BackgroundCopyManager3_0 BackgroundCopyManager3_0;
#endif /* __cplusplus */

#endif 	/* __BackgroundCopyManager3_0_FWD_DEFINED__ */


#ifndef __IBackgroundCopyCallback2_FWD_DEFINED__
#define __IBackgroundCopyCallback2_FWD_DEFINED__
typedef interface IBackgroundCopyCallback2 IBackgroundCopyCallback2;

#endif 	/* __IBackgroundCopyCallback2_FWD_DEFINED__ */


#ifndef __IBackgroundCopyFile3_FWD_DEFINED__
#define __IBackgroundCopyFile3_FWD_DEFINED__
typedef interface IBackgroundCopyFile3 IBackgroundCopyFile3;

#endif 	/* __IBackgroundCopyFile3_FWD_DEFINED__ */


#ifndef __IBackgroundCopyJob4_FWD_DEFINED__
#define __IBackgroundCopyJob4_FWD_DEFINED__
typedef interface IBackgroundCopyJob4 IBackgroundCopyJob4;

#endif 	/* __IBackgroundCopyJob4_FWD_DEFINED__ */


#ifndef __IBitsPeer_FWD_DEFINED__
#define __IBitsPeer_FWD_DEFINED__
typedef interface IBitsPeer IBitsPeer;

#endif 	/* __IBitsPeer_FWD_DEFINED__ */


#ifndef __IBitsPeerCacheAdministration_FWD_DEFINED__
#define __IBitsPeerCacheAdministration_FWD_DEFINED__
typedef interface IBitsPeerCacheAdministration IBitsPeerCacheAdministration;

#endif 	/* __IBitsPeerCacheAdministration_FWD_DEFINED__ */


#ifndef __IBitsPeerCacheRecord_FWD_DEFINED__
#define __IBitsPeerCacheRecord_FWD_DEFINED__
typedef interface IBitsPeerCacheRecord IBitsPeerCacheRecord;

#endif 	/* __IBitsPeerCacheRecord_FWD_DEFINED__ */


#ifndef __IEnumBitsPeerCacheRecords_FWD_DEFINED__
#define __IEnumBitsPeerCacheRecords_FWD_DEFINED__
typedef interface IEnumBitsPeerCacheRecords IEnumBitsPeerCacheRecords;

#endif 	/* __IEnumBitsPeerCacheRecords_FWD_DEFINED__ */


#ifndef __IEnumBitsPeers_FWD_DEFINED__
#define __IEnumBitsPeers_FWD_DEFINED__
typedef interface IEnumBitsPeers IEnumBitsPeers;

#endif 	/* __IEnumBitsPeers_FWD_DEFINED__ */


/* header files for imported files */
#include "bits.h"
#include "bits1_5.h"
#include "bits2_0.h"
#include "bits2_5.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_bits3_0_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_bits3_0_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bits3_0_0000_0000_v0_0_s_ifspec;

#ifndef __IBitsPeerCacheRecord_INTERFACE_DEFINED__
#define __IBitsPeerCacheRecord_INTERFACE_DEFINED__

/* interface IBitsPeerCacheRecord */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IBitsPeerCacheRecord;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("659cdeaf-489e-11d9-a9cd-000d56965251")
    IBitsPeerCacheRecord : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetId( 
            /* [ref][out] */ __RPC__out GUID *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOriginUrl( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFileSize( 
            /* [ref][out] */ __RPC__out UINT64 *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFileModificationTime( 
            /* [ref][out] */ __RPC__out FILETIME *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLastAccessTime( 
            /* [ref][out] */ __RPC__out FILETIME *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsFileValidated( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFileRanges( 
            /* [ref][out] */ __RPC__out DWORD *pRangeCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pRangeCount) BG_FILE_RANGE **ppRanges) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBitsPeerCacheRecordVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBitsPeerCacheRecord * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBitsPeerCacheRecord * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBitsPeerCacheRecord * This);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheRecord, GetId)
        HRESULT ( STDMETHODCALLTYPE *GetId )( 
            __RPC__in IBitsPeerCacheRecord * This,
            /* [ref][out] */ __RPC__out GUID *pVal);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheRecord, GetOriginUrl)
        HRESULT ( STDMETHODCALLTYPE *GetOriginUrl )( 
            __RPC__in IBitsPeerCacheRecord * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheRecord, GetFileSize)
        HRESULT ( STDMETHODCALLTYPE *GetFileSize )( 
            __RPC__in IBitsPeerCacheRecord * This,
            /* [ref][out] */ __RPC__out UINT64 *pVal);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheRecord, GetFileModificationTime)
        HRESULT ( STDMETHODCALLTYPE *GetFileModificationTime )( 
            __RPC__in IBitsPeerCacheRecord * This,
            /* [ref][out] */ __RPC__out FILETIME *pVal);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheRecord, GetLastAccessTime)
        HRESULT ( STDMETHODCALLTYPE *GetLastAccessTime )( 
            __RPC__in IBitsPeerCacheRecord * This,
            /* [ref][out] */ __RPC__out FILETIME *pVal);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheRecord, IsFileValidated)
        HRESULT ( STDMETHODCALLTYPE *IsFileValidated )( 
            __RPC__in IBitsPeerCacheRecord * This);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheRecord, GetFileRanges)
        HRESULT ( STDMETHODCALLTYPE *GetFileRanges )( 
            __RPC__in IBitsPeerCacheRecord * This,
            /* [ref][out] */ __RPC__out DWORD *pRangeCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pRangeCount) BG_FILE_RANGE **ppRanges);
        
        END_INTERFACE
    } IBitsPeerCacheRecordVtbl;

    interface IBitsPeerCacheRecord
    {
        CONST_VTBL struct IBitsPeerCacheRecordVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBitsPeerCacheRecord_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBitsPeerCacheRecord_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBitsPeerCacheRecord_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBitsPeerCacheRecord_GetId(This,pVal)	\
    ( (This)->lpVtbl -> GetId(This,pVal) ) 

#define IBitsPeerCacheRecord_GetOriginUrl(This,pVal)	\
    ( (This)->lpVtbl -> GetOriginUrl(This,pVal) ) 

#define IBitsPeerCacheRecord_GetFileSize(This,pVal)	\
    ( (This)->lpVtbl -> GetFileSize(This,pVal) ) 

#define IBitsPeerCacheRecord_GetFileModificationTime(This,pVal)	\
    ( (This)->lpVtbl -> GetFileModificationTime(This,pVal) ) 

#define IBitsPeerCacheRecord_GetLastAccessTime(This,pVal)	\
    ( (This)->lpVtbl -> GetLastAccessTime(This,pVal) ) 

#define IBitsPeerCacheRecord_IsFileValidated(This)	\
    ( (This)->lpVtbl -> IsFileValidated(This) ) 

#define IBitsPeerCacheRecord_GetFileRanges(This,pRangeCount,ppRanges)	\
    ( (This)->lpVtbl -> GetFileRanges(This,pRangeCount,ppRanges) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBitsPeerCacheRecord_INTERFACE_DEFINED__ */


#ifndef __IEnumBitsPeerCacheRecords_INTERFACE_DEFINED__
#define __IEnumBitsPeerCacheRecords_INTERFACE_DEFINED__

/* interface IEnumBitsPeerCacheRecords */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumBitsPeerCacheRecords;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("659cdea4-489e-11d9-a9cd-000d56965251")
    IEnumBitsPeerCacheRecords : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, pceltFetched ? *pceltFetched : celt) IBitsPeerCacheRecord **rgelt,
            /* [unique][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumBitsPeerCacheRecords **ppenum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out ULONG *puCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumBitsPeerCacheRecordsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumBitsPeerCacheRecords * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumBitsPeerCacheRecords * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumBitsPeerCacheRecords * This);
        
        DECLSPEC_XFGVIRT(IEnumBitsPeerCacheRecords, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumBitsPeerCacheRecords * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, pceltFetched ? *pceltFetched : celt) IBitsPeerCacheRecord **rgelt,
            /* [unique][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumBitsPeerCacheRecords, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumBitsPeerCacheRecords * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumBitsPeerCacheRecords, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumBitsPeerCacheRecords * This);
        
        DECLSPEC_XFGVIRT(IEnumBitsPeerCacheRecords, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumBitsPeerCacheRecords * This,
            /* [out] */ __RPC__deref_out_opt IEnumBitsPeerCacheRecords **ppenum);
        
        DECLSPEC_XFGVIRT(IEnumBitsPeerCacheRecords, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IEnumBitsPeerCacheRecords * This,
            /* [out] */ __RPC__out ULONG *puCount);
        
        END_INTERFACE
    } IEnumBitsPeerCacheRecordsVtbl;

    interface IEnumBitsPeerCacheRecords
    {
        CONST_VTBL struct IEnumBitsPeerCacheRecordsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumBitsPeerCacheRecords_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumBitsPeerCacheRecords_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumBitsPeerCacheRecords_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumBitsPeerCacheRecords_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumBitsPeerCacheRecords_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumBitsPeerCacheRecords_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumBitsPeerCacheRecords_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#define IEnumBitsPeerCacheRecords_GetCount(This,puCount)	\
    ( (This)->lpVtbl -> GetCount(This,puCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumBitsPeerCacheRecords_INTERFACE_DEFINED__ */


#ifndef __IBitsPeer_INTERFACE_DEFINED__
#define __IBitsPeer_INTERFACE_DEFINED__

/* interface IBitsPeer */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IBitsPeer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("659cdea2-489e-11d9-a9cd-000d56965251")
    IBitsPeer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPeerName( 
            /* [ref][out] */ __RPC__deref_out_opt LPWSTR *pName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsAuthenticated( 
            /* [ref][out] */ __RPC__out BOOL *pAuth) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsAvailable( 
            /* [ref][out] */ __RPC__out BOOL *pOnline) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBitsPeerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBitsPeer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBitsPeer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBitsPeer * This);
        
        DECLSPEC_XFGVIRT(IBitsPeer, GetPeerName)
        HRESULT ( STDMETHODCALLTYPE *GetPeerName )( 
            __RPC__in IBitsPeer * This,
            /* [ref][out] */ __RPC__deref_out_opt LPWSTR *pName);
        
        DECLSPEC_XFGVIRT(IBitsPeer, IsAuthenticated)
        HRESULT ( STDMETHODCALLTYPE *IsAuthenticated )( 
            __RPC__in IBitsPeer * This,
            /* [ref][out] */ __RPC__out BOOL *pAuth);
        
        DECLSPEC_XFGVIRT(IBitsPeer, IsAvailable)
        HRESULT ( STDMETHODCALLTYPE *IsAvailable )( 
            __RPC__in IBitsPeer * This,
            /* [ref][out] */ __RPC__out BOOL *pOnline);
        
        END_INTERFACE
    } IBitsPeerVtbl;

    interface IBitsPeer
    {
        CONST_VTBL struct IBitsPeerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBitsPeer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBitsPeer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBitsPeer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBitsPeer_GetPeerName(This,pName)	\
    ( (This)->lpVtbl -> GetPeerName(This,pName) ) 

#define IBitsPeer_IsAuthenticated(This,pAuth)	\
    ( (This)->lpVtbl -> IsAuthenticated(This,pAuth) ) 

#define IBitsPeer_IsAvailable(This,pOnline)	\
    ( (This)->lpVtbl -> IsAvailable(This,pOnline) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBitsPeer_INTERFACE_DEFINED__ */


#ifndef __IEnumBitsPeers_INTERFACE_DEFINED__
#define __IEnumBitsPeers_INTERFACE_DEFINED__

/* interface IEnumBitsPeers */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumBitsPeers;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("659cdea5-489e-11d9-a9cd-000d56965251")
    IEnumBitsPeers : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, pceltFetched ? *pceltFetched : celt) IBitsPeer **rgelt,
            /* [unique][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumBitsPeers **ppenum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out ULONG *puCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumBitsPeersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumBitsPeers * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumBitsPeers * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumBitsPeers * This);
        
        DECLSPEC_XFGVIRT(IEnumBitsPeers, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumBitsPeers * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, pceltFetched ? *pceltFetched : celt) IBitsPeer **rgelt,
            /* [unique][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumBitsPeers, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumBitsPeers * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumBitsPeers, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumBitsPeers * This);
        
        DECLSPEC_XFGVIRT(IEnumBitsPeers, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumBitsPeers * This,
            /* [out] */ __RPC__deref_out_opt IEnumBitsPeers **ppenum);
        
        DECLSPEC_XFGVIRT(IEnumBitsPeers, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IEnumBitsPeers * This,
            /* [out] */ __RPC__out ULONG *puCount);
        
        END_INTERFACE
    } IEnumBitsPeersVtbl;

    interface IEnumBitsPeers
    {
        CONST_VTBL struct IEnumBitsPeersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumBitsPeers_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumBitsPeers_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumBitsPeers_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumBitsPeers_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumBitsPeers_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumBitsPeers_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumBitsPeers_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#define IEnumBitsPeers_GetCount(This,puCount)	\
    ( (This)->lpVtbl -> GetCount(This,puCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumBitsPeers_INTERFACE_DEFINED__ */


#ifndef __IBitsPeerCacheAdministration_INTERFACE_DEFINED__
#define __IBitsPeerCacheAdministration_INTERFACE_DEFINED__

/* interface IBitsPeerCacheAdministration */
/* [object][helpstring][uuid] */ 

#define   BG_ENABLE_PEERCACHING_CLIENT   0x0001
#define   BG_ENABLE_PEERCACHING_SERVER   0x0002
#define   BG_DISABLE_BRANCH_CACHE        0x0004

EXTERN_C const IID IID_IBitsPeerCacheAdministration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("659cdead-489e-11d9-a9cd-000d56965251")
    IBitsPeerCacheAdministration : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMaximumCacheSize( 
            /* [out][ref] */ __RPC__out DWORD *pBytes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMaximumCacheSize( 
            DWORD Bytes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaximumContentAge( 
            /* [out][ref] */ __RPC__out ULONG *pSeconds) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMaximumContentAge( 
            ULONG Seconds) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConfigurationFlags( 
            /* [out][ref] */ __RPC__out DWORD *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetConfigurationFlags( 
            DWORD Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumRecords( 
            /* [out] */ __RPC__deref_out_opt IEnumBitsPeerCacheRecords **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRecord( 
            /* [in] */ __RPC__in REFGUID id,
            /* [out] */ __RPC__deref_out_opt IBitsPeerCacheRecord **ppRecord) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearRecords( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteRecord( 
            /* [in] */ __RPC__in REFGUID id) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteUrl( 
            /* [ref][in] */ __RPC__in LPCWSTR url) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumPeers( 
            /* [ref][out] */ __RPC__deref_out_opt IEnumBitsPeers **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearPeers( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DiscoverPeers( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBitsPeerCacheAdministrationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBitsPeerCacheAdministration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBitsPeerCacheAdministration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBitsPeerCacheAdministration * This);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheAdministration, GetMaximumCacheSize)
        HRESULT ( STDMETHODCALLTYPE *GetMaximumCacheSize )( 
            __RPC__in IBitsPeerCacheAdministration * This,
            /* [out][ref] */ __RPC__out DWORD *pBytes);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheAdministration, SetMaximumCacheSize)
        HRESULT ( STDMETHODCALLTYPE *SetMaximumCacheSize )( 
            __RPC__in IBitsPeerCacheAdministration * This,
            DWORD Bytes);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheAdministration, GetMaximumContentAge)
        HRESULT ( STDMETHODCALLTYPE *GetMaximumContentAge )( 
            __RPC__in IBitsPeerCacheAdministration * This,
            /* [out][ref] */ __RPC__out ULONG *pSeconds);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheAdministration, SetMaximumContentAge)
        HRESULT ( STDMETHODCALLTYPE *SetMaximumContentAge )( 
            __RPC__in IBitsPeerCacheAdministration * This,
            ULONG Seconds);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheAdministration, GetConfigurationFlags)
        HRESULT ( STDMETHODCALLTYPE *GetConfigurationFlags )( 
            __RPC__in IBitsPeerCacheAdministration * This,
            /* [out][ref] */ __RPC__out DWORD *pFlags);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheAdministration, SetConfigurationFlags)
        HRESULT ( STDMETHODCALLTYPE *SetConfigurationFlags )( 
            __RPC__in IBitsPeerCacheAdministration * This,
            DWORD Flags);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheAdministration, EnumRecords)
        HRESULT ( STDMETHODCALLTYPE *EnumRecords )( 
            __RPC__in IBitsPeerCacheAdministration * This,
            /* [out] */ __RPC__deref_out_opt IEnumBitsPeerCacheRecords **ppEnum);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheAdministration, GetRecord)
        HRESULT ( STDMETHODCALLTYPE *GetRecord )( 
            __RPC__in IBitsPeerCacheAdministration * This,
            /* [in] */ __RPC__in REFGUID id,
            /* [out] */ __RPC__deref_out_opt IBitsPeerCacheRecord **ppRecord);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheAdministration, ClearRecords)
        HRESULT ( STDMETHODCALLTYPE *ClearRecords )( 
            __RPC__in IBitsPeerCacheAdministration * This);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheAdministration, DeleteRecord)
        HRESULT ( STDMETHODCALLTYPE *DeleteRecord )( 
            __RPC__in IBitsPeerCacheAdministration * This,
            /* [in] */ __RPC__in REFGUID id);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheAdministration, DeleteUrl)
        HRESULT ( STDMETHODCALLTYPE *DeleteUrl )( 
            __RPC__in IBitsPeerCacheAdministration * This,
            /* [ref][in] */ __RPC__in LPCWSTR url);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheAdministration, EnumPeers)
        HRESULT ( STDMETHODCALLTYPE *EnumPeers )( 
            __RPC__in IBitsPeerCacheAdministration * This,
            /* [ref][out] */ __RPC__deref_out_opt IEnumBitsPeers **ppEnum);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheAdministration, ClearPeers)
        HRESULT ( STDMETHODCALLTYPE *ClearPeers )( 
            __RPC__in IBitsPeerCacheAdministration * This);
        
        DECLSPEC_XFGVIRT(IBitsPeerCacheAdministration, DiscoverPeers)
        HRESULT ( STDMETHODCALLTYPE *DiscoverPeers )( 
            __RPC__in IBitsPeerCacheAdministration * This);
        
        END_INTERFACE
    } IBitsPeerCacheAdministrationVtbl;

    interface IBitsPeerCacheAdministration
    {
        CONST_VTBL struct IBitsPeerCacheAdministrationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBitsPeerCacheAdministration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBitsPeerCacheAdministration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBitsPeerCacheAdministration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBitsPeerCacheAdministration_GetMaximumCacheSize(This,pBytes)	\
    ( (This)->lpVtbl -> GetMaximumCacheSize(This,pBytes) ) 

#define IBitsPeerCacheAdministration_SetMaximumCacheSize(This,Bytes)	\
    ( (This)->lpVtbl -> SetMaximumCacheSize(This,Bytes) ) 

#define IBitsPeerCacheAdministration_GetMaximumContentAge(This,pSeconds)	\
    ( (This)->lpVtbl -> GetMaximumContentAge(This,pSeconds) ) 

#define IBitsPeerCacheAdministration_SetMaximumContentAge(This,Seconds)	\
    ( (This)->lpVtbl -> SetMaximumContentAge(This,Seconds) ) 

#define IBitsPeerCacheAdministration_GetConfigurationFlags(This,pFlags)	\
    ( (This)->lpVtbl -> GetConfigurationFlags(This,pFlags) ) 

#define IBitsPeerCacheAdministration_SetConfigurationFlags(This,Flags)	\
    ( (This)->lpVtbl -> SetConfigurationFlags(This,Flags) ) 

#define IBitsPeerCacheAdministration_EnumRecords(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumRecords(This,ppEnum) ) 

#define IBitsPeerCacheAdministration_GetRecord(This,id,ppRecord)	\
    ( (This)->lpVtbl -> GetRecord(This,id,ppRecord) ) 

#define IBitsPeerCacheAdministration_ClearRecords(This)	\
    ( (This)->lpVtbl -> ClearRecords(This) ) 

#define IBitsPeerCacheAdministration_DeleteRecord(This,id)	\
    ( (This)->lpVtbl -> DeleteRecord(This,id) ) 

#define IBitsPeerCacheAdministration_DeleteUrl(This,url)	\
    ( (This)->lpVtbl -> DeleteUrl(This,url) ) 

#define IBitsPeerCacheAdministration_EnumPeers(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumPeers(This,ppEnum) ) 

#define IBitsPeerCacheAdministration_ClearPeers(This)	\
    ( (This)->lpVtbl -> ClearPeers(This) ) 

#define IBitsPeerCacheAdministration_DiscoverPeers(This)	\
    ( (This)->lpVtbl -> DiscoverPeers(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBitsPeerCacheAdministration_INTERFACE_DEFINED__ */


#ifndef __IBackgroundCopyJob4_INTERFACE_DEFINED__
#define __IBackgroundCopyJob4_INTERFACE_DEFINED__

/* interface IBackgroundCopyJob4 */
/* [object][helpstring][uuid] */ 

#define   BG_JOB_ENABLE_PEERCACHING_CLIENT   0x0001
#define   BG_JOB_ENABLE_PEERCACHING_SERVER   0x0002
#define   BG_JOB_DISABLE_BRANCH_CACHE        0x0004

EXTERN_C const IID IID_IBackgroundCopyJob4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("659cdeae-489e-11d9-a9cd-000d56965251")
    IBackgroundCopyJob4 : public IBackgroundCopyJob3
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetPeerCachingFlags( 
            DWORD Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPeerCachingFlags( 
            /* [ref][out] */ __RPC__out DWORD *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOwnerIntegrityLevel( 
            /* [ref][out] */ __RPC__out ULONG *pLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOwnerElevationState( 
            /* [ref][out] */ __RPC__out BOOL *pElevated) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMaximumDownloadTime( 
            ULONG Timeout) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaximumDownloadTime( 
            /* [ref][out] */ __RPC__out ULONG *pTimeout) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBackgroundCopyJob4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBackgroundCopyJob4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBackgroundCopyJob4 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, AddFileSet)
        HRESULT ( STDMETHODCALLTYPE *AddFileSet )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [in] */ ULONG cFileCount,
            /* [size_is][in] */ __RPC__in_ecount_full(cFileCount) BG_FILE_INFO *pFileSet);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, AddFile)
        HRESULT ( STDMETHODCALLTYPE *AddFile )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [in] */ __RPC__in LPCWSTR RemoteUrl,
            /* [in] */ __RPC__in LPCWSTR LocalName);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, EnumFiles)
        HRESULT ( STDMETHODCALLTYPE *EnumFiles )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__deref_out_opt IEnumBackgroundCopyFiles **pEnum);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, Suspend)
        HRESULT ( STDMETHODCALLTYPE *Suspend )( 
            __RPC__in IBackgroundCopyJob4 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, Resume)
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            __RPC__in IBackgroundCopyJob4 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IBackgroundCopyJob4 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, Complete)
        HRESULT ( STDMETHODCALLTYPE *Complete )( 
            __RPC__in IBackgroundCopyJob4 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetId)
        HRESULT ( STDMETHODCALLTYPE *GetId )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__out GUID *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__out BG_JOB_TYPE *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetProgress)
        HRESULT ( STDMETHODCALLTYPE *GetProgress )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__out BG_JOB_PROGRESS *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetTimes)
        HRESULT ( STDMETHODCALLTYPE *GetTimes )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__out BG_JOB_TIMES *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__out BG_JOB_STATE *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetError)
        HRESULT ( STDMETHODCALLTYPE *GetError )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__deref_out_opt IBackgroundCopyError **ppError);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *SetDisplayName )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [in] */ __RPC__in LPCWSTR Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetDescription)
        HRESULT ( STDMETHODCALLTYPE *SetDescription )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [in] */ __RPC__in LPCWSTR Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetPriority)
        HRESULT ( STDMETHODCALLTYPE *SetPriority )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [in] */ BG_JOB_PRIORITY Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetPriority)
        HRESULT ( STDMETHODCALLTYPE *GetPriority )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__out BG_JOB_PRIORITY *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetNotifyFlags)
        HRESULT ( STDMETHODCALLTYPE *SetNotifyFlags )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [in] */ ULONG Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetNotifyFlags)
        HRESULT ( STDMETHODCALLTYPE *GetNotifyFlags )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__out ULONG *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetNotifyInterface)
        HRESULT ( STDMETHODCALLTYPE *SetNotifyInterface )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [in] */ __RPC__in_opt IUnknown *Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetNotifyInterface)
        HRESULT ( STDMETHODCALLTYPE *GetNotifyInterface )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetMinimumRetryDelay)
        HRESULT ( STDMETHODCALLTYPE *SetMinimumRetryDelay )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [in] */ ULONG Seconds);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetMinimumRetryDelay)
        HRESULT ( STDMETHODCALLTYPE *GetMinimumRetryDelay )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__out ULONG *Seconds);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetNoProgressTimeout)
        HRESULT ( STDMETHODCALLTYPE *SetNoProgressTimeout )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [in] */ ULONG Seconds);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetNoProgressTimeout)
        HRESULT ( STDMETHODCALLTYPE *GetNoProgressTimeout )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__out ULONG *Seconds);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetErrorCount)
        HRESULT ( STDMETHODCALLTYPE *GetErrorCount )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__out ULONG *Errors);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, SetProxySettings)
        HRESULT ( STDMETHODCALLTYPE *SetProxySettings )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [in] */ BG_JOB_PROXY_USAGE ProxyUsage,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *ProxyList,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *ProxyBypassList);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, GetProxySettings)
        HRESULT ( STDMETHODCALLTYPE *GetProxySettings )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__out BG_JOB_PROXY_USAGE *pProxyUsage,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pProxyList,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pProxyBypassList);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob, TakeOwnership)
        HRESULT ( STDMETHODCALLTYPE *TakeOwnership )( 
            __RPC__in IBackgroundCopyJob4 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, SetNotifyCmdLine)
        HRESULT ( STDMETHODCALLTYPE *SetNotifyCmdLine )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [unique][in] */ __RPC__in_opt LPCWSTR Program,
            /* [unique][in] */ __RPC__in_opt LPCWSTR Parameters);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, GetNotifyCmdLine)
        HRESULT ( STDMETHODCALLTYPE *GetNotifyCmdLine )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pProgram,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pParameters);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, GetReplyProgress)
        HRESULT ( STDMETHODCALLTYPE *GetReplyProgress )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out][in] */ __RPC__inout BG_JOB_REPLY_PROGRESS *pProgress);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, GetReplyData)
        HRESULT ( STDMETHODCALLTYPE *GetReplyData )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(( unsigned long  )*pLength) byte **ppBuffer,
            /* [unique][out][in] */ __RPC__inout_opt UINT64 *pLength);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, SetReplyFileName)
        HRESULT ( STDMETHODCALLTYPE *SetReplyFileName )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [unique][in] */ __RPC__in_opt LPCWSTR ReplyFileName);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, GetReplyFileName)
        HRESULT ( STDMETHODCALLTYPE *GetReplyFileName )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pReplyFileName);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, SetCredentials)
        HRESULT ( STDMETHODCALLTYPE *SetCredentials )( 
            __RPC__in IBackgroundCopyJob4 * This,
            __RPC__in BG_AUTH_CREDENTIALS *credentials);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob2, RemoveCredentials)
        HRESULT ( STDMETHODCALLTYPE *RemoveCredentials )( 
            __RPC__in IBackgroundCopyJob4 * This,
            BG_AUTH_TARGET Target,
            BG_AUTH_SCHEME Scheme);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob3, ReplaceRemotePrefix)
        HRESULT ( STDMETHODCALLTYPE *ReplaceRemotePrefix )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [in] */ __RPC__in LPCWSTR OldPrefix,
            /* [in] */ __RPC__in LPCWSTR NewPrefix);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob3, AddFileWithRanges)
        HRESULT ( STDMETHODCALLTYPE *AddFileWithRanges )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [in] */ __RPC__in LPCWSTR RemoteUrl,
            /* [in] */ __RPC__in LPCWSTR LocalName,
            /* [in] */ DWORD RangeCount,
            /* [size_is][in] */ __RPC__in_ecount_full(RangeCount) BG_FILE_RANGE Ranges[  ]);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob3, SetFileACLFlags)
        HRESULT ( STDMETHODCALLTYPE *SetFileACLFlags )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [in] */ DWORD Flags);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob3, GetFileACLFlags)
        HRESULT ( STDMETHODCALLTYPE *GetFileACLFlags )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [ref][out] */ __RPC__out DWORD *Flags);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob4, SetPeerCachingFlags)
        HRESULT ( STDMETHODCALLTYPE *SetPeerCachingFlags )( 
            __RPC__in IBackgroundCopyJob4 * This,
            DWORD Flags);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob4, GetPeerCachingFlags)
        HRESULT ( STDMETHODCALLTYPE *GetPeerCachingFlags )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [ref][out] */ __RPC__out DWORD *pFlags);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob4, GetOwnerIntegrityLevel)
        HRESULT ( STDMETHODCALLTYPE *GetOwnerIntegrityLevel )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [ref][out] */ __RPC__out ULONG *pLevel);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob4, GetOwnerElevationState)
        HRESULT ( STDMETHODCALLTYPE *GetOwnerElevationState )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [ref][out] */ __RPC__out BOOL *pElevated);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob4, SetMaximumDownloadTime)
        HRESULT ( STDMETHODCALLTYPE *SetMaximumDownloadTime )( 
            __RPC__in IBackgroundCopyJob4 * This,
            ULONG Timeout);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyJob4, GetMaximumDownloadTime)
        HRESULT ( STDMETHODCALLTYPE *GetMaximumDownloadTime )( 
            __RPC__in IBackgroundCopyJob4 * This,
            /* [ref][out] */ __RPC__out ULONG *pTimeout);
        
        END_INTERFACE
    } IBackgroundCopyJob4Vtbl;

    interface IBackgroundCopyJob4
    {
        CONST_VTBL struct IBackgroundCopyJob4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBackgroundCopyJob4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBackgroundCopyJob4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBackgroundCopyJob4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBackgroundCopyJob4_AddFileSet(This,cFileCount,pFileSet)	\
    ( (This)->lpVtbl -> AddFileSet(This,cFileCount,pFileSet) ) 

#define IBackgroundCopyJob4_AddFile(This,RemoteUrl,LocalName)	\
    ( (This)->lpVtbl -> AddFile(This,RemoteUrl,LocalName) ) 

#define IBackgroundCopyJob4_EnumFiles(This,pEnum)	\
    ( (This)->lpVtbl -> EnumFiles(This,pEnum) ) 

#define IBackgroundCopyJob4_Suspend(This)	\
    ( (This)->lpVtbl -> Suspend(This) ) 

#define IBackgroundCopyJob4_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#define IBackgroundCopyJob4_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IBackgroundCopyJob4_Complete(This)	\
    ( (This)->lpVtbl -> Complete(This) ) 

#define IBackgroundCopyJob4_GetId(This,pVal)	\
    ( (This)->lpVtbl -> GetId(This,pVal) ) 

#define IBackgroundCopyJob4_GetType(This,pVal)	\
    ( (This)->lpVtbl -> GetType(This,pVal) ) 

#define IBackgroundCopyJob4_GetProgress(This,pVal)	\
    ( (This)->lpVtbl -> GetProgress(This,pVal) ) 

#define IBackgroundCopyJob4_GetTimes(This,pVal)	\
    ( (This)->lpVtbl -> GetTimes(This,pVal) ) 

#define IBackgroundCopyJob4_GetState(This,pVal)	\
    ( (This)->lpVtbl -> GetState(This,pVal) ) 

#define IBackgroundCopyJob4_GetError(This,ppError)	\
    ( (This)->lpVtbl -> GetError(This,ppError) ) 

#define IBackgroundCopyJob4_GetOwner(This,pVal)	\
    ( (This)->lpVtbl -> GetOwner(This,pVal) ) 

#define IBackgroundCopyJob4_SetDisplayName(This,Val)	\
    ( (This)->lpVtbl -> SetDisplayName(This,Val) ) 

#define IBackgroundCopyJob4_GetDisplayName(This,pVal)	\
    ( (This)->lpVtbl -> GetDisplayName(This,pVal) ) 

#define IBackgroundCopyJob4_SetDescription(This,Val)	\
    ( (This)->lpVtbl -> SetDescription(This,Val) ) 

#define IBackgroundCopyJob4_GetDescription(This,pVal)	\
    ( (This)->lpVtbl -> GetDescription(This,pVal) ) 

#define IBackgroundCopyJob4_SetPriority(This,Val)	\
    ( (This)->lpVtbl -> SetPriority(This,Val) ) 

#define IBackgroundCopyJob4_GetPriority(This,pVal)	\
    ( (This)->lpVtbl -> GetPriority(This,pVal) ) 

#define IBackgroundCopyJob4_SetNotifyFlags(This,Val)	\
    ( (This)->lpVtbl -> SetNotifyFlags(This,Val) ) 

#define IBackgroundCopyJob4_GetNotifyFlags(This,pVal)	\
    ( (This)->lpVtbl -> GetNotifyFlags(This,pVal) ) 

#define IBackgroundCopyJob4_SetNotifyInterface(This,Val)	\
    ( (This)->lpVtbl -> SetNotifyInterface(This,Val) ) 

#define IBackgroundCopyJob4_GetNotifyInterface(This,pVal)	\
    ( (This)->lpVtbl -> GetNotifyInterface(This,pVal) ) 

#define IBackgroundCopyJob4_SetMinimumRetryDelay(This,Seconds)	\
    ( (This)->lpVtbl -> SetMinimumRetryDelay(This,Seconds) ) 

#define IBackgroundCopyJob4_GetMinimumRetryDelay(This,Seconds)	\
    ( (This)->lpVtbl -> GetMinimumRetryDelay(This,Seconds) ) 

#define IBackgroundCopyJob4_SetNoProgressTimeout(This,Seconds)	\
    ( (This)->lpVtbl -> SetNoProgressTimeout(This,Seconds) ) 

#define IBackgroundCopyJob4_GetNoProgressTimeout(This,Seconds)	\
    ( (This)->lpVtbl -> GetNoProgressTimeout(This,Seconds) ) 

#define IBackgroundCopyJob4_GetErrorCount(This,Errors)	\
    ( (This)->lpVtbl -> GetErrorCount(This,Errors) ) 

#define IBackgroundCopyJob4_SetProxySettings(This,ProxyUsage,ProxyList,ProxyBypassList)	\
    ( (This)->lpVtbl -> SetProxySettings(This,ProxyUsage,ProxyList,ProxyBypassList) ) 

#define IBackgroundCopyJob4_GetProxySettings(This,pProxyUsage,pProxyList,pProxyBypassList)	\
    ( (This)->lpVtbl -> GetProxySettings(This,pProxyUsage,pProxyList,pProxyBypassList) ) 

#define IBackgroundCopyJob4_TakeOwnership(This)	\
    ( (This)->lpVtbl -> TakeOwnership(This) ) 


#define IBackgroundCopyJob4_SetNotifyCmdLine(This,Program,Parameters)	\
    ( (This)->lpVtbl -> SetNotifyCmdLine(This,Program,Parameters) ) 

#define IBackgroundCopyJob4_GetNotifyCmdLine(This,pProgram,pParameters)	\
    ( (This)->lpVtbl -> GetNotifyCmdLine(This,pProgram,pParameters) ) 

#define IBackgroundCopyJob4_GetReplyProgress(This,pProgress)	\
    ( (This)->lpVtbl -> GetReplyProgress(This,pProgress) ) 

#define IBackgroundCopyJob4_GetReplyData(This,ppBuffer,pLength)	\
    ( (This)->lpVtbl -> GetReplyData(This,ppBuffer,pLength) ) 

#define IBackgroundCopyJob4_SetReplyFileName(This,ReplyFileName)	\
    ( (This)->lpVtbl -> SetReplyFileName(This,ReplyFileName) ) 

#define IBackgroundCopyJob4_GetReplyFileName(This,pReplyFileName)	\
    ( (This)->lpVtbl -> GetReplyFileName(This,pReplyFileName) ) 

#define IBackgroundCopyJob4_SetCredentials(This,credentials)	\
    ( (This)->lpVtbl -> SetCredentials(This,credentials) ) 

#define IBackgroundCopyJob4_RemoveCredentials(This,Target,Scheme)	\
    ( (This)->lpVtbl -> RemoveCredentials(This,Target,Scheme) ) 


#define IBackgroundCopyJob4_ReplaceRemotePrefix(This,OldPrefix,NewPrefix)	\
    ( (This)->lpVtbl -> ReplaceRemotePrefix(This,OldPrefix,NewPrefix) ) 

#define IBackgroundCopyJob4_AddFileWithRanges(This,RemoteUrl,LocalName,RangeCount,Ranges)	\
    ( (This)->lpVtbl -> AddFileWithRanges(This,RemoteUrl,LocalName,RangeCount,Ranges) ) 

#define IBackgroundCopyJob4_SetFileACLFlags(This,Flags)	\
    ( (This)->lpVtbl -> SetFileACLFlags(This,Flags) ) 

#define IBackgroundCopyJob4_GetFileACLFlags(This,Flags)	\
    ( (This)->lpVtbl -> GetFileACLFlags(This,Flags) ) 


#define IBackgroundCopyJob4_SetPeerCachingFlags(This,Flags)	\
    ( (This)->lpVtbl -> SetPeerCachingFlags(This,Flags) ) 

#define IBackgroundCopyJob4_GetPeerCachingFlags(This,pFlags)	\
    ( (This)->lpVtbl -> GetPeerCachingFlags(This,pFlags) ) 

#define IBackgroundCopyJob4_GetOwnerIntegrityLevel(This,pLevel)	\
    ( (This)->lpVtbl -> GetOwnerIntegrityLevel(This,pLevel) ) 

#define IBackgroundCopyJob4_GetOwnerElevationState(This,pElevated)	\
    ( (This)->lpVtbl -> GetOwnerElevationState(This,pElevated) ) 

#define IBackgroundCopyJob4_SetMaximumDownloadTime(This,Timeout)	\
    ( (This)->lpVtbl -> SetMaximumDownloadTime(This,Timeout) ) 

#define IBackgroundCopyJob4_GetMaximumDownloadTime(This,pTimeout)	\
    ( (This)->lpVtbl -> GetMaximumDownloadTime(This,pTimeout) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBackgroundCopyJob4_INTERFACE_DEFINED__ */


#ifndef __IBackgroundCopyFile3_INTERFACE_DEFINED__
#define __IBackgroundCopyFile3_INTERFACE_DEFINED__

/* interface IBackgroundCopyFile3 */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IBackgroundCopyFile3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("659cdeaa-489e-11d9-a9cd-000d56965251")
    IBackgroundCopyFile3 : public IBackgroundCopyFile2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTemporaryName( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *pFilename) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValidationState( 
            /* [in] */ BOOL state) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValidationState( 
            /* [out] */ __RPC__out BOOL *pState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsDownloadedFromPeer( 
            /* [out] */ __RPC__out BOOL *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBackgroundCopyFile3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBackgroundCopyFile3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBackgroundCopyFile3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBackgroundCopyFile3 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile, GetRemoteName)
        HRESULT ( STDMETHODCALLTYPE *GetRemoteName )( 
            __RPC__in IBackgroundCopyFile3 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile, GetLocalName)
        HRESULT ( STDMETHODCALLTYPE *GetLocalName )( 
            __RPC__in IBackgroundCopyFile3 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile, GetProgress)
        HRESULT ( STDMETHODCALLTYPE *GetProgress )( 
            __RPC__in IBackgroundCopyFile3 * This,
            /* [out] */ __RPC__out BG_FILE_PROGRESS *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile2, GetFileRanges)
        HRESULT ( STDMETHODCALLTYPE *GetFileRanges )( 
            __RPC__in IBackgroundCopyFile3 * This,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *RangeCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*RangeCount) BG_FILE_RANGE **Ranges);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile2, SetRemoteName)
        HRESULT ( STDMETHODCALLTYPE *SetRemoteName )( 
            __RPC__in IBackgroundCopyFile3 * This,
            __RPC__in LPCWSTR Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile3, GetTemporaryName)
        HRESULT ( STDMETHODCALLTYPE *GetTemporaryName )( 
            __RPC__in IBackgroundCopyFile3 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pFilename);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile3, SetValidationState)
        HRESULT ( STDMETHODCALLTYPE *SetValidationState )( 
            __RPC__in IBackgroundCopyFile3 * This,
            /* [in] */ BOOL state);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile3, GetValidationState)
        HRESULT ( STDMETHODCALLTYPE *GetValidationState )( 
            __RPC__in IBackgroundCopyFile3 * This,
            /* [out] */ __RPC__out BOOL *pState);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile3, IsDownloadedFromPeer)
        HRESULT ( STDMETHODCALLTYPE *IsDownloadedFromPeer )( 
            __RPC__in IBackgroundCopyFile3 * This,
            /* [out] */ __RPC__out BOOL *pVal);
        
        END_INTERFACE
    } IBackgroundCopyFile3Vtbl;

    interface IBackgroundCopyFile3
    {
        CONST_VTBL struct IBackgroundCopyFile3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBackgroundCopyFile3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBackgroundCopyFile3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBackgroundCopyFile3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBackgroundCopyFile3_GetRemoteName(This,pVal)	\
    ( (This)->lpVtbl -> GetRemoteName(This,pVal) ) 

#define IBackgroundCopyFile3_GetLocalName(This,pVal)	\
    ( (This)->lpVtbl -> GetLocalName(This,pVal) ) 

#define IBackgroundCopyFile3_GetProgress(This,pVal)	\
    ( (This)->lpVtbl -> GetProgress(This,pVal) ) 


#define IBackgroundCopyFile3_GetFileRanges(This,RangeCount,Ranges)	\
    ( (This)->lpVtbl -> GetFileRanges(This,RangeCount,Ranges) ) 

#define IBackgroundCopyFile3_SetRemoteName(This,Val)	\
    ( (This)->lpVtbl -> SetRemoteName(This,Val) ) 


#define IBackgroundCopyFile3_GetTemporaryName(This,pFilename)	\
    ( (This)->lpVtbl -> GetTemporaryName(This,pFilename) ) 

#define IBackgroundCopyFile3_SetValidationState(This,state)	\
    ( (This)->lpVtbl -> SetValidationState(This,state) ) 

#define IBackgroundCopyFile3_GetValidationState(This,pState)	\
    ( (This)->lpVtbl -> GetValidationState(This,pState) ) 

#define IBackgroundCopyFile3_IsDownloadedFromPeer(This,pVal)	\
    ( (This)->lpVtbl -> IsDownloadedFromPeer(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBackgroundCopyFile3_INTERFACE_DEFINED__ */


#ifndef __IBackgroundCopyCallback2_INTERFACE_DEFINED__
#define __IBackgroundCopyCallback2_INTERFACE_DEFINED__

/* interface IBackgroundCopyCallback2 */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IBackgroundCopyCallback2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("659cdeac-489e-11d9-a9cd-000d56965251")
    IBackgroundCopyCallback2 : public IBackgroundCopyCallback
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FileTransferred( 
            /* [in] */ __RPC__in_opt IBackgroundCopyJob *pJob,
            /* [in] */ __RPC__in_opt IBackgroundCopyFile *pFile) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBackgroundCopyCallback2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBackgroundCopyCallback2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBackgroundCopyCallback2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBackgroundCopyCallback2 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyCallback, JobTransferred)
        HRESULT ( STDMETHODCALLTYPE *JobTransferred )( 
            __RPC__in IBackgroundCopyCallback2 * This,
            /* [in] */ __RPC__in_opt IBackgroundCopyJob *pJob);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyCallback, JobError)
        HRESULT ( STDMETHODCALLTYPE *JobError )( 
            __RPC__in IBackgroundCopyCallback2 * This,
            /* [in] */ __RPC__in_opt IBackgroundCopyJob *pJob,
            /* [in] */ __RPC__in_opt IBackgroundCopyError *pError);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyCallback, JobModification)
        HRESULT ( STDMETHODCALLTYPE *JobModification )( 
            __RPC__in IBackgroundCopyCallback2 * This,
            /* [in] */ __RPC__in_opt IBackgroundCopyJob *pJob,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyCallback2, FileTransferred)
        HRESULT ( STDMETHODCALLTYPE *FileTransferred )( 
            __RPC__in IBackgroundCopyCallback2 * This,
            /* [in] */ __RPC__in_opt IBackgroundCopyJob *pJob,
            /* [in] */ __RPC__in_opt IBackgroundCopyFile *pFile);
        
        END_INTERFACE
    } IBackgroundCopyCallback2Vtbl;

    interface IBackgroundCopyCallback2
    {
        CONST_VTBL struct IBackgroundCopyCallback2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBackgroundCopyCallback2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBackgroundCopyCallback2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBackgroundCopyCallback2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBackgroundCopyCallback2_JobTransferred(This,pJob)	\
    ( (This)->lpVtbl -> JobTransferred(This,pJob) ) 

#define IBackgroundCopyCallback2_JobError(This,pJob,pError)	\
    ( (This)->lpVtbl -> JobError(This,pJob,pError) ) 

#define IBackgroundCopyCallback2_JobModification(This,pJob,dwReserved)	\
    ( (This)->lpVtbl -> JobModification(This,pJob,dwReserved) ) 


#define IBackgroundCopyCallback2_FileTransferred(This,pJob,pFile)	\
    ( (This)->lpVtbl -> FileTransferred(This,pJob,pFile) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBackgroundCopyCallback2_INTERFACE_DEFINED__ */



#ifndef __BackgroundCopyManager3_0_LIBRARY_DEFINED__
#define __BackgroundCopyManager3_0_LIBRARY_DEFINED__

/* library BackgroundCopyManager3_0 */
/* [version][lcid][helpstring][uuid] */ 











EXTERN_C const IID LIBID_BackgroundCopyManager3_0;

EXTERN_C const CLSID CLSID_BackgroundCopyManager3_0;

#ifdef __cplusplus

class DECLSPEC_UUID("659cdea7-489e-11d9-a9cd-000d56965251")
BackgroundCopyManager3_0;
#endif
#endif /* __BackgroundCopyManager3_0_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_bits3_0_0000_0009 */
/* [local] */ 

#include "bits4_0.h"
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_bits3_0_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bits3_0_0000_0009_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


