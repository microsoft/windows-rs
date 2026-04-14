

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

#ifndef __vsmgmt_h__
#define __vsmgmt_h__

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

#ifndef __IVssSnapshotMgmt_FWD_DEFINED__
#define __IVssSnapshotMgmt_FWD_DEFINED__
typedef interface IVssSnapshotMgmt IVssSnapshotMgmt;

#endif 	/* __IVssSnapshotMgmt_FWD_DEFINED__ */


#ifndef __IVssSnapshotMgmt2_FWD_DEFINED__
#define __IVssSnapshotMgmt2_FWD_DEFINED__
typedef interface IVssSnapshotMgmt2 IVssSnapshotMgmt2;

#endif 	/* __IVssSnapshotMgmt2_FWD_DEFINED__ */


#ifndef __IVssDifferentialSoftwareSnapshotMgmt_FWD_DEFINED__
#define __IVssDifferentialSoftwareSnapshotMgmt_FWD_DEFINED__
typedef interface IVssDifferentialSoftwareSnapshotMgmt IVssDifferentialSoftwareSnapshotMgmt;

#endif 	/* __IVssDifferentialSoftwareSnapshotMgmt_FWD_DEFINED__ */


#ifndef __IVssDifferentialSoftwareSnapshotMgmt2_FWD_DEFINED__
#define __IVssDifferentialSoftwareSnapshotMgmt2_FWD_DEFINED__
typedef interface IVssDifferentialSoftwareSnapshotMgmt2 IVssDifferentialSoftwareSnapshotMgmt2;

#endif 	/* __IVssDifferentialSoftwareSnapshotMgmt2_FWD_DEFINED__ */


#ifndef __IVssDifferentialSoftwareSnapshotMgmt3_FWD_DEFINED__
#define __IVssDifferentialSoftwareSnapshotMgmt3_FWD_DEFINED__
typedef interface IVssDifferentialSoftwareSnapshotMgmt3 IVssDifferentialSoftwareSnapshotMgmt3;

#endif 	/* __IVssDifferentialSoftwareSnapshotMgmt3_FWD_DEFINED__ */


#ifndef __IVssEnumMgmtObject_FWD_DEFINED__
#define __IVssEnumMgmtObject_FWD_DEFINED__
typedef interface IVssEnumMgmtObject IVssEnumMgmtObject;

#endif 	/* __IVssEnumMgmtObject_FWD_DEFINED__ */


#ifndef __VssSnapshotMgmt_FWD_DEFINED__
#define __VssSnapshotMgmt_FWD_DEFINED__

#ifdef __cplusplus
typedef class VssSnapshotMgmt VssSnapshotMgmt;
#else
typedef struct VssSnapshotMgmt VssSnapshotMgmt;
#endif /* __cplusplus */

#endif 	/* __VssSnapshotMgmt_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "vss.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_vsmgmt_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef 
enum _VSS_MGMT_OBJECT_TYPE
    {
        VSS_MGMT_OBJECT_UNKNOWN	= 0,
        VSS_MGMT_OBJECT_VOLUME	= ( VSS_MGMT_OBJECT_UNKNOWN + 1 ) ,
        VSS_MGMT_OBJECT_DIFF_VOLUME	= ( VSS_MGMT_OBJECT_VOLUME + 1 ) ,
        VSS_MGMT_OBJECT_DIFF_AREA	= ( VSS_MGMT_OBJECT_DIFF_VOLUME + 1 ) 
    } 	VSS_MGMT_OBJECT_TYPE;

typedef enum _VSS_MGMT_OBJECT_TYPE *PVSS_MGMT_OBJECT_TYPE;

#define	VSS_ASSOC_NO_MAX_SPACE	( -1 )

#define	VSS_ASSOC_REMOVE	( 0 )

typedef struct _VSS_VOLUME_PROP
    {
    VSS_PWSZ m_pwszVolumeName;
    VSS_PWSZ m_pwszVolumeDisplayName;
    } 	VSS_VOLUME_PROP;

typedef struct _VSS_VOLUME_PROP *PVSS_VOLUME_PROP;

typedef struct _VSS_DIFF_VOLUME_PROP
    {
    VSS_PWSZ m_pwszVolumeName;
    VSS_PWSZ m_pwszVolumeDisplayName;
    LONGLONG m_llVolumeFreeSpace;
    LONGLONG m_llVolumeTotalSpace;
    } 	VSS_DIFF_VOLUME_PROP;

typedef struct _VSS_DIFF_VOLUME_PROP *PVSS_DIFF_VOLUME_PROP;

typedef struct _VSS_DIFF_AREA_PROP
    {
    VSS_PWSZ m_pwszVolumeName;
    VSS_PWSZ m_pwszDiffAreaVolumeName;
    LONGLONG m_llMaximumDiffSpace;
    LONGLONG m_llAllocatedDiffSpace;
    LONGLONG m_llUsedDiffSpace;
    } 	VSS_DIFF_AREA_PROP;

typedef struct _VSS_DIFF_AREA_PROP *PVSS_DIFF_AREA_PROP;

typedef /* [public][public][public][public][switch_type] */ union __MIDL___MIDL_itf_vsmgmt_0000_0000_0001
    {
    /* [case()] */ VSS_VOLUME_PROP Vol;
    /* [case()] */ VSS_DIFF_VOLUME_PROP DiffVol;
    /* [case()] */ VSS_DIFF_AREA_PROP DiffArea;
    /* [default] */  /* Empty union arm */ 
    } 	VSS_MGMT_OBJECT_UNION;

typedef /* [switch_type] */ union __MIDL___MIDL_itf_vsmgmt_0000_0000_0001 *PVSS_MGMT_OBJECT_UNION;

typedef struct _VSS_MGMT_OBJECT_PROP
    {
    VSS_MGMT_OBJECT_TYPE Type;
    /* [switch_is] */ VSS_MGMT_OBJECT_UNION Obj;
    } 	VSS_MGMT_OBJECT_PROP;

typedef struct _VSS_MGMT_OBJECT_PROP *PVSS_MGMT_OBJECT_PROP;

typedef 
enum _VSS_PROTECTION_LEVEL
    {
        VSS_PROTECTION_LEVEL_ORIGINAL_VOLUME	= 0,
        VSS_PROTECTION_LEVEL_SNAPSHOT	= ( VSS_PROTECTION_LEVEL_ORIGINAL_VOLUME + 1 ) 
    } 	VSS_PROTECTION_LEVEL;

typedef enum _VSS_PROTECTION_LEVEL *PVSS_PROTECTION_LEVEL;

typedef 
enum _VSS_PROTECTION_FAULT
    {
        VSS_PROTECTION_FAULT_NONE	= 0,
        VSS_PROTECTION_FAULT_DIFF_AREA_MISSING	= ( VSS_PROTECTION_FAULT_NONE + 1 ) ,
        VSS_PROTECTION_FAULT_IO_FAILURE_DURING_ONLINE	= ( VSS_PROTECTION_FAULT_DIFF_AREA_MISSING + 1 ) ,
        VSS_PROTECTION_FAULT_META_DATA_CORRUPTION	= ( VSS_PROTECTION_FAULT_IO_FAILURE_DURING_ONLINE + 1 ) ,
        VSS_PROTECTION_FAULT_MEMORY_ALLOCATION_FAILURE	= ( VSS_PROTECTION_FAULT_META_DATA_CORRUPTION + 1 ) ,
        VSS_PROTECTION_FAULT_MAPPED_MEMORY_FAILURE	= ( VSS_PROTECTION_FAULT_MEMORY_ALLOCATION_FAILURE + 1 ) ,
        VSS_PROTECTION_FAULT_COW_READ_FAILURE	= ( VSS_PROTECTION_FAULT_MAPPED_MEMORY_FAILURE + 1 ) ,
        VSS_PROTECTION_FAULT_COW_WRITE_FAILURE	= ( VSS_PROTECTION_FAULT_COW_READ_FAILURE + 1 ) ,
        VSS_PROTECTION_FAULT_DIFF_AREA_FULL	= ( VSS_PROTECTION_FAULT_COW_WRITE_FAILURE + 1 ) ,
        VSS_PROTECTION_FAULT_GROW_TOO_SLOW	= ( VSS_PROTECTION_FAULT_DIFF_AREA_FULL + 1 ) ,
        VSS_PROTECTION_FAULT_GROW_FAILED	= ( VSS_PROTECTION_FAULT_GROW_TOO_SLOW + 1 ) ,
        VSS_PROTECTION_FAULT_DESTROY_ALL_SNAPSHOTS	= ( VSS_PROTECTION_FAULT_GROW_FAILED + 1 ) ,
        VSS_PROTECTION_FAULT_FILE_SYSTEM_FAILURE	= ( VSS_PROTECTION_FAULT_DESTROY_ALL_SNAPSHOTS + 1 ) ,
        VSS_PROTECTION_FAULT_IO_FAILURE	= ( VSS_PROTECTION_FAULT_FILE_SYSTEM_FAILURE + 1 ) ,
        VSS_PROTECTION_FAULT_DIFF_AREA_REMOVED	= ( VSS_PROTECTION_FAULT_IO_FAILURE + 1 ) ,
        VSS_PROTECTION_FAULT_EXTERNAL_WRITER_TO_DIFF_AREA	= ( VSS_PROTECTION_FAULT_DIFF_AREA_REMOVED + 1 ) ,
        VSS_PROTECTION_FAULT_MOUNT_DURING_CLUSTER_OFFLINE	= ( VSS_PROTECTION_FAULT_EXTERNAL_WRITER_TO_DIFF_AREA + 1 ) 
    } 	VSS_PROTECTION_FAULT;

typedef enum _VSS_PROTECTION_FAULT *PVSS_PROTECTION_FAULT;

typedef struct _VSS_VOLUME_PROTECTION_INFO
    {
    VSS_PROTECTION_LEVEL m_protectionLevel;
    BOOL m_volumeIsOfflineForProtection;
    VSS_PROTECTION_FAULT m_protectionFault;
    LONG m_failureStatus;
    BOOL m_volumeHasUnusedDiffArea;
    DWORD m_reserved;
    } 	VSS_VOLUME_PROTECTION_INFO;

typedef struct _VSS_VOLUME_PROTECTION_INFO *PVSS_VOLUME_PROTECTION_INFO;






extern RPC_IF_HANDLE __MIDL_itf_vsmgmt_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_vsmgmt_0000_0000_v0_0_s_ifspec;

#ifndef __IVssSnapshotMgmt_INTERFACE_DEFINED__
#define __IVssSnapshotMgmt_INTERFACE_DEFINED__

/* interface IVssSnapshotMgmt */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IVssSnapshotMgmt;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FA7DF749-66E7-4986-A27F-E2F04AE53772")
    IVssSnapshotMgmt : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetProviderMgmtInterface( 
            /* [in] */ VSS_ID ProviderId,
            /* [in] */ __RPC__in REFIID InterfaceId,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppItf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryVolumesSupportedForSnapshots( 
            /* [in] */ VSS_ID ProviderId,
            /* [in] */ LONG lContext,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QuerySnapshotsByVolume( 
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ VSS_ID ProviderId,
            /* [out] */ __RPC__deref_out_opt IVssEnumObject **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVssSnapshotMgmtVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVssSnapshotMgmt * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVssSnapshotMgmt * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVssSnapshotMgmt * This);
        
        DECLSPEC_XFGVIRT(IVssSnapshotMgmt, GetProviderMgmtInterface)
        HRESULT ( STDMETHODCALLTYPE *GetProviderMgmtInterface )( 
            __RPC__in IVssSnapshotMgmt * This,
            /* [in] */ VSS_ID ProviderId,
            /* [in] */ __RPC__in REFIID InterfaceId,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppItf);
        
        DECLSPEC_XFGVIRT(IVssSnapshotMgmt, QueryVolumesSupportedForSnapshots)
        HRESULT ( STDMETHODCALLTYPE *QueryVolumesSupportedForSnapshots )( 
            __RPC__in IVssSnapshotMgmt * This,
            /* [in] */ VSS_ID ProviderId,
            /* [in] */ LONG lContext,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVssSnapshotMgmt, QuerySnapshotsByVolume)
        HRESULT ( STDMETHODCALLTYPE *QuerySnapshotsByVolume )( 
            __RPC__in IVssSnapshotMgmt * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ VSS_ID ProviderId,
            /* [out] */ __RPC__deref_out_opt IVssEnumObject **ppEnum);
        
        END_INTERFACE
    } IVssSnapshotMgmtVtbl;

    interface IVssSnapshotMgmt
    {
        CONST_VTBL struct IVssSnapshotMgmtVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVssSnapshotMgmt_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVssSnapshotMgmt_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVssSnapshotMgmt_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVssSnapshotMgmt_GetProviderMgmtInterface(This,ProviderId,InterfaceId,ppItf)	\
    ( (This)->lpVtbl -> GetProviderMgmtInterface(This,ProviderId,InterfaceId,ppItf) ) 

#define IVssSnapshotMgmt_QueryVolumesSupportedForSnapshots(This,ProviderId,lContext,ppEnum)	\
    ( (This)->lpVtbl -> QueryVolumesSupportedForSnapshots(This,ProviderId,lContext,ppEnum) ) 

#define IVssSnapshotMgmt_QuerySnapshotsByVolume(This,pwszVolumeName,ProviderId,ppEnum)	\
    ( (This)->lpVtbl -> QuerySnapshotsByVolume(This,pwszVolumeName,ProviderId,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVssSnapshotMgmt_INTERFACE_DEFINED__ */


#ifndef __IVssSnapshotMgmt2_INTERFACE_DEFINED__
#define __IVssSnapshotMgmt2_INTERFACE_DEFINED__

/* interface IVssSnapshotMgmt2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IVssSnapshotMgmt2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0f61ec39-fe82-45f2-a3f0-768b5d427102")
    IVssSnapshotMgmt2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMinDiffAreaSize( 
            /* [out] */ __RPC__out LONGLONG *pllMinDiffAreaSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVssSnapshotMgmt2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVssSnapshotMgmt2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVssSnapshotMgmt2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVssSnapshotMgmt2 * This);
        
        DECLSPEC_XFGVIRT(IVssSnapshotMgmt2, GetMinDiffAreaSize)
        HRESULT ( STDMETHODCALLTYPE *GetMinDiffAreaSize )( 
            __RPC__in IVssSnapshotMgmt2 * This,
            /* [out] */ __RPC__out LONGLONG *pllMinDiffAreaSize);
        
        END_INTERFACE
    } IVssSnapshotMgmt2Vtbl;

    interface IVssSnapshotMgmt2
    {
        CONST_VTBL struct IVssSnapshotMgmt2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVssSnapshotMgmt2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVssSnapshotMgmt2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVssSnapshotMgmt2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVssSnapshotMgmt2_GetMinDiffAreaSize(This,pllMinDiffAreaSize)	\
    ( (This)->lpVtbl -> GetMinDiffAreaSize(This,pllMinDiffAreaSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVssSnapshotMgmt2_INTERFACE_DEFINED__ */


#ifndef __IVssDifferentialSoftwareSnapshotMgmt_INTERFACE_DEFINED__
#define __IVssDifferentialSoftwareSnapshotMgmt_INTERFACE_DEFINED__

/* interface IVssDifferentialSoftwareSnapshotMgmt */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IVssDifferentialSoftwareSnapshotMgmt;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("214A0F28-B737-4026-B847-4F9E37D79529")
    IVssDifferentialSoftwareSnapshotMgmt : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddDiffArea( 
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName,
            /* [in] */ LONGLONG llMaximumDiffSpace) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ChangeDiffAreaMaximumSize( 
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName,
            /* [in] */ LONGLONG llMaximumDiffSpace) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryVolumesSupportedForDiffAreas( 
            /* [in] */ __RPC__in VSS_PWSZ pwszOriginalVolumeName,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryDiffAreasForVolume( 
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryDiffAreasOnVolume( 
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryDiffAreasForSnapshot( 
            /* [in] */ VSS_ID SnapshotId,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVssDifferentialSoftwareSnapshotMgmtVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt * This);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, AddDiffArea)
        HRESULT ( STDMETHODCALLTYPE *AddDiffArea )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName,
            /* [in] */ LONGLONG llMaximumDiffSpace);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, ChangeDiffAreaMaximumSize)
        HRESULT ( STDMETHODCALLTYPE *ChangeDiffAreaMaximumSize )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName,
            /* [in] */ LONGLONG llMaximumDiffSpace);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, QueryVolumesSupportedForDiffAreas)
        HRESULT ( STDMETHODCALLTYPE *QueryVolumesSupportedForDiffAreas )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszOriginalVolumeName,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, QueryDiffAreasForVolume)
        HRESULT ( STDMETHODCALLTYPE *QueryDiffAreasForVolume )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, QueryDiffAreasOnVolume)
        HRESULT ( STDMETHODCALLTYPE *QueryDiffAreasOnVolume )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, QueryDiffAreasForSnapshot)
        HRESULT ( STDMETHODCALLTYPE *QueryDiffAreasForSnapshot )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt * This,
            /* [in] */ VSS_ID SnapshotId,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum);
        
        END_INTERFACE
    } IVssDifferentialSoftwareSnapshotMgmtVtbl;

    interface IVssDifferentialSoftwareSnapshotMgmt
    {
        CONST_VTBL struct IVssDifferentialSoftwareSnapshotMgmtVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVssDifferentialSoftwareSnapshotMgmt_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVssDifferentialSoftwareSnapshotMgmt_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVssDifferentialSoftwareSnapshotMgmt_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVssDifferentialSoftwareSnapshotMgmt_AddDiffArea(This,pwszVolumeName,pwszDiffAreaVolumeName,llMaximumDiffSpace)	\
    ( (This)->lpVtbl -> AddDiffArea(This,pwszVolumeName,pwszDiffAreaVolumeName,llMaximumDiffSpace) ) 

#define IVssDifferentialSoftwareSnapshotMgmt_ChangeDiffAreaMaximumSize(This,pwszVolumeName,pwszDiffAreaVolumeName,llMaximumDiffSpace)	\
    ( (This)->lpVtbl -> ChangeDiffAreaMaximumSize(This,pwszVolumeName,pwszDiffAreaVolumeName,llMaximumDiffSpace) ) 

#define IVssDifferentialSoftwareSnapshotMgmt_QueryVolumesSupportedForDiffAreas(This,pwszOriginalVolumeName,ppEnum)	\
    ( (This)->lpVtbl -> QueryVolumesSupportedForDiffAreas(This,pwszOriginalVolumeName,ppEnum) ) 

#define IVssDifferentialSoftwareSnapshotMgmt_QueryDiffAreasForVolume(This,pwszVolumeName,ppEnum)	\
    ( (This)->lpVtbl -> QueryDiffAreasForVolume(This,pwszVolumeName,ppEnum) ) 

#define IVssDifferentialSoftwareSnapshotMgmt_QueryDiffAreasOnVolume(This,pwszVolumeName,ppEnum)	\
    ( (This)->lpVtbl -> QueryDiffAreasOnVolume(This,pwszVolumeName,ppEnum) ) 

#define IVssDifferentialSoftwareSnapshotMgmt_QueryDiffAreasForSnapshot(This,SnapshotId,ppEnum)	\
    ( (This)->lpVtbl -> QueryDiffAreasForSnapshot(This,SnapshotId,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVssDifferentialSoftwareSnapshotMgmt_INTERFACE_DEFINED__ */


#ifndef __IVssDifferentialSoftwareSnapshotMgmt2_INTERFACE_DEFINED__
#define __IVssDifferentialSoftwareSnapshotMgmt2_INTERFACE_DEFINED__

/* interface IVssDifferentialSoftwareSnapshotMgmt2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IVssDifferentialSoftwareSnapshotMgmt2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("949d7353-675f-4275-8969-f044c6277815")
    IVssDifferentialSoftwareSnapshotMgmt2 : public IVssDifferentialSoftwareSnapshotMgmt
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ChangeDiffAreaMaximumSizeEx( 
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName,
            /* [in] */ LONGLONG llMaximumDiffSpace,
            /* [in] */ BOOL bVolatile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MigrateDiffAreas( 
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszNewDiffAreaVolumeName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryMigrationStatus( 
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName,
            /* [out] */ __RPC__deref_out_opt IVssAsync **ppAsync) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSnapshotPriority( 
            /* [in] */ VSS_ID idSnapshot,
            /* [in] */ BYTE priority) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVssDifferentialSoftwareSnapshotMgmt2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt2 * This);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, AddDiffArea)
        HRESULT ( STDMETHODCALLTYPE *AddDiffArea )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt2 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName,
            /* [in] */ LONGLONG llMaximumDiffSpace);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, ChangeDiffAreaMaximumSize)
        HRESULT ( STDMETHODCALLTYPE *ChangeDiffAreaMaximumSize )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt2 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName,
            /* [in] */ LONGLONG llMaximumDiffSpace);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, QueryVolumesSupportedForDiffAreas)
        HRESULT ( STDMETHODCALLTYPE *QueryVolumesSupportedForDiffAreas )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt2 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszOriginalVolumeName,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, QueryDiffAreasForVolume)
        HRESULT ( STDMETHODCALLTYPE *QueryDiffAreasForVolume )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt2 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, QueryDiffAreasOnVolume)
        HRESULT ( STDMETHODCALLTYPE *QueryDiffAreasOnVolume )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt2 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, QueryDiffAreasForSnapshot)
        HRESULT ( STDMETHODCALLTYPE *QueryDiffAreasForSnapshot )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt2 * This,
            /* [in] */ VSS_ID SnapshotId,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt2, ChangeDiffAreaMaximumSizeEx)
        HRESULT ( STDMETHODCALLTYPE *ChangeDiffAreaMaximumSizeEx )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt2 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName,
            /* [in] */ LONGLONG llMaximumDiffSpace,
            /* [in] */ BOOL bVolatile);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt2, MigrateDiffAreas)
        HRESULT ( STDMETHODCALLTYPE *MigrateDiffAreas )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt2 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszNewDiffAreaVolumeName);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt2, QueryMigrationStatus)
        HRESULT ( STDMETHODCALLTYPE *QueryMigrationStatus )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt2 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName,
            /* [out] */ __RPC__deref_out_opt IVssAsync **ppAsync);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt2, SetSnapshotPriority)
        HRESULT ( STDMETHODCALLTYPE *SetSnapshotPriority )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt2 * This,
            /* [in] */ VSS_ID idSnapshot,
            /* [in] */ BYTE priority);
        
        END_INTERFACE
    } IVssDifferentialSoftwareSnapshotMgmt2Vtbl;

    interface IVssDifferentialSoftwareSnapshotMgmt2
    {
        CONST_VTBL struct IVssDifferentialSoftwareSnapshotMgmt2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVssDifferentialSoftwareSnapshotMgmt2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVssDifferentialSoftwareSnapshotMgmt2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVssDifferentialSoftwareSnapshotMgmt2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVssDifferentialSoftwareSnapshotMgmt2_AddDiffArea(This,pwszVolumeName,pwszDiffAreaVolumeName,llMaximumDiffSpace)	\
    ( (This)->lpVtbl -> AddDiffArea(This,pwszVolumeName,pwszDiffAreaVolumeName,llMaximumDiffSpace) ) 

#define IVssDifferentialSoftwareSnapshotMgmt2_ChangeDiffAreaMaximumSize(This,pwszVolumeName,pwszDiffAreaVolumeName,llMaximumDiffSpace)	\
    ( (This)->lpVtbl -> ChangeDiffAreaMaximumSize(This,pwszVolumeName,pwszDiffAreaVolumeName,llMaximumDiffSpace) ) 

#define IVssDifferentialSoftwareSnapshotMgmt2_QueryVolumesSupportedForDiffAreas(This,pwszOriginalVolumeName,ppEnum)	\
    ( (This)->lpVtbl -> QueryVolumesSupportedForDiffAreas(This,pwszOriginalVolumeName,ppEnum) ) 

#define IVssDifferentialSoftwareSnapshotMgmt2_QueryDiffAreasForVolume(This,pwszVolumeName,ppEnum)	\
    ( (This)->lpVtbl -> QueryDiffAreasForVolume(This,pwszVolumeName,ppEnum) ) 

#define IVssDifferentialSoftwareSnapshotMgmt2_QueryDiffAreasOnVolume(This,pwszVolumeName,ppEnum)	\
    ( (This)->lpVtbl -> QueryDiffAreasOnVolume(This,pwszVolumeName,ppEnum) ) 

#define IVssDifferentialSoftwareSnapshotMgmt2_QueryDiffAreasForSnapshot(This,SnapshotId,ppEnum)	\
    ( (This)->lpVtbl -> QueryDiffAreasForSnapshot(This,SnapshotId,ppEnum) ) 


#define IVssDifferentialSoftwareSnapshotMgmt2_ChangeDiffAreaMaximumSizeEx(This,pwszVolumeName,pwszDiffAreaVolumeName,llMaximumDiffSpace,bVolatile)	\
    ( (This)->lpVtbl -> ChangeDiffAreaMaximumSizeEx(This,pwszVolumeName,pwszDiffAreaVolumeName,llMaximumDiffSpace,bVolatile) ) 

#define IVssDifferentialSoftwareSnapshotMgmt2_MigrateDiffAreas(This,pwszVolumeName,pwszDiffAreaVolumeName,pwszNewDiffAreaVolumeName)	\
    ( (This)->lpVtbl -> MigrateDiffAreas(This,pwszVolumeName,pwszDiffAreaVolumeName,pwszNewDiffAreaVolumeName) ) 

#define IVssDifferentialSoftwareSnapshotMgmt2_QueryMigrationStatus(This,pwszVolumeName,pwszDiffAreaVolumeName,ppAsync)	\
    ( (This)->lpVtbl -> QueryMigrationStatus(This,pwszVolumeName,pwszDiffAreaVolumeName,ppAsync) ) 

#define IVssDifferentialSoftwareSnapshotMgmt2_SetSnapshotPriority(This,idSnapshot,priority)	\
    ( (This)->lpVtbl -> SetSnapshotPriority(This,idSnapshot,priority) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVssDifferentialSoftwareSnapshotMgmt2_INTERFACE_DEFINED__ */


#ifndef __IVssDifferentialSoftwareSnapshotMgmt3_INTERFACE_DEFINED__
#define __IVssDifferentialSoftwareSnapshotMgmt3_INTERFACE_DEFINED__

/* interface IVssDifferentialSoftwareSnapshotMgmt3 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IVssDifferentialSoftwareSnapshotMgmt3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("383f7e71-a4c5-401f-b27f-f826289f8458")
    IVssDifferentialSoftwareSnapshotMgmt3 : public IVssDifferentialSoftwareSnapshotMgmt2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetVolumeProtectLevel( 
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ VSS_PROTECTION_LEVEL protectionLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVolumeProtectLevel( 
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [out] */ __RPC__out VSS_VOLUME_PROTECTION_INFO *protectionLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearVolumeProtectFault( 
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteUnusedDiffAreas( 
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QuerySnapshotDeltaBitmap( 
            /* [in] */ VSS_ID idSnapshotOlder,
            /* [in] */ VSS_ID idSnapshotYounger,
            /* [out] */ __RPC__out ULONG *pcBlockSizePerBit,
            /* [out] */ __RPC__out ULONG *pcBitmapLength,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcBitmapLength) BYTE **ppbBitmap) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVssDifferentialSoftwareSnapshotMgmt3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, AddDiffArea)
        HRESULT ( STDMETHODCALLTYPE *AddDiffArea )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName,
            /* [in] */ LONGLONG llMaximumDiffSpace);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, ChangeDiffAreaMaximumSize)
        HRESULT ( STDMETHODCALLTYPE *ChangeDiffAreaMaximumSize )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName,
            /* [in] */ LONGLONG llMaximumDiffSpace);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, QueryVolumesSupportedForDiffAreas)
        HRESULT ( STDMETHODCALLTYPE *QueryVolumesSupportedForDiffAreas )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszOriginalVolumeName,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, QueryDiffAreasForVolume)
        HRESULT ( STDMETHODCALLTYPE *QueryDiffAreasForVolume )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, QueryDiffAreasOnVolume)
        HRESULT ( STDMETHODCALLTYPE *QueryDiffAreasOnVolume )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt, QueryDiffAreasForSnapshot)
        HRESULT ( STDMETHODCALLTYPE *QueryDiffAreasForSnapshot )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This,
            /* [in] */ VSS_ID SnapshotId,
            /* [out] */ __RPC__deref_out_opt IVssEnumMgmtObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt2, ChangeDiffAreaMaximumSizeEx)
        HRESULT ( STDMETHODCALLTYPE *ChangeDiffAreaMaximumSizeEx )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName,
            /* [in] */ LONGLONG llMaximumDiffSpace,
            /* [in] */ BOOL bVolatile);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt2, MigrateDiffAreas)
        HRESULT ( STDMETHODCALLTYPE *MigrateDiffAreas )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszNewDiffAreaVolumeName);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt2, QueryMigrationStatus)
        HRESULT ( STDMETHODCALLTYPE *QueryMigrationStatus )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName,
            /* [out] */ __RPC__deref_out_opt IVssAsync **ppAsync);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt2, SetSnapshotPriority)
        HRESULT ( STDMETHODCALLTYPE *SetSnapshotPriority )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This,
            /* [in] */ VSS_ID idSnapshot,
            /* [in] */ BYTE priority);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt3, SetVolumeProtectLevel)
        HRESULT ( STDMETHODCALLTYPE *SetVolumeProtectLevel )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [in] */ VSS_PROTECTION_LEVEL protectionLevel);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt3, GetVolumeProtectLevel)
        HRESULT ( STDMETHODCALLTYPE *GetVolumeProtectLevel )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName,
            /* [out] */ __RPC__out VSS_VOLUME_PROTECTION_INFO *protectionLevel);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt3, ClearVolumeProtectFault)
        HRESULT ( STDMETHODCALLTYPE *ClearVolumeProtectFault )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszVolumeName);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt3, DeleteUnusedDiffAreas)
        HRESULT ( STDMETHODCALLTYPE *DeleteUnusedDiffAreas )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This,
            /* [in] */ __RPC__in VSS_PWSZ pwszDiffAreaVolumeName);
        
        DECLSPEC_XFGVIRT(IVssDifferentialSoftwareSnapshotMgmt3, QuerySnapshotDeltaBitmap)
        HRESULT ( STDMETHODCALLTYPE *QuerySnapshotDeltaBitmap )( 
            __RPC__in IVssDifferentialSoftwareSnapshotMgmt3 * This,
            /* [in] */ VSS_ID idSnapshotOlder,
            /* [in] */ VSS_ID idSnapshotYounger,
            /* [out] */ __RPC__out ULONG *pcBlockSizePerBit,
            /* [out] */ __RPC__out ULONG *pcBitmapLength,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcBitmapLength) BYTE **ppbBitmap);
        
        END_INTERFACE
    } IVssDifferentialSoftwareSnapshotMgmt3Vtbl;

    interface IVssDifferentialSoftwareSnapshotMgmt3
    {
        CONST_VTBL struct IVssDifferentialSoftwareSnapshotMgmt3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVssDifferentialSoftwareSnapshotMgmt3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVssDifferentialSoftwareSnapshotMgmt3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVssDifferentialSoftwareSnapshotMgmt3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVssDifferentialSoftwareSnapshotMgmt3_AddDiffArea(This,pwszVolumeName,pwszDiffAreaVolumeName,llMaximumDiffSpace)	\
    ( (This)->lpVtbl -> AddDiffArea(This,pwszVolumeName,pwszDiffAreaVolumeName,llMaximumDiffSpace) ) 

#define IVssDifferentialSoftwareSnapshotMgmt3_ChangeDiffAreaMaximumSize(This,pwszVolumeName,pwszDiffAreaVolumeName,llMaximumDiffSpace)	\
    ( (This)->lpVtbl -> ChangeDiffAreaMaximumSize(This,pwszVolumeName,pwszDiffAreaVolumeName,llMaximumDiffSpace) ) 

#define IVssDifferentialSoftwareSnapshotMgmt3_QueryVolumesSupportedForDiffAreas(This,pwszOriginalVolumeName,ppEnum)	\
    ( (This)->lpVtbl -> QueryVolumesSupportedForDiffAreas(This,pwszOriginalVolumeName,ppEnum) ) 

#define IVssDifferentialSoftwareSnapshotMgmt3_QueryDiffAreasForVolume(This,pwszVolumeName,ppEnum)	\
    ( (This)->lpVtbl -> QueryDiffAreasForVolume(This,pwszVolumeName,ppEnum) ) 

#define IVssDifferentialSoftwareSnapshotMgmt3_QueryDiffAreasOnVolume(This,pwszVolumeName,ppEnum)	\
    ( (This)->lpVtbl -> QueryDiffAreasOnVolume(This,pwszVolumeName,ppEnum) ) 

#define IVssDifferentialSoftwareSnapshotMgmt3_QueryDiffAreasForSnapshot(This,SnapshotId,ppEnum)	\
    ( (This)->lpVtbl -> QueryDiffAreasForSnapshot(This,SnapshotId,ppEnum) ) 


#define IVssDifferentialSoftwareSnapshotMgmt3_ChangeDiffAreaMaximumSizeEx(This,pwszVolumeName,pwszDiffAreaVolumeName,llMaximumDiffSpace,bVolatile)	\
    ( (This)->lpVtbl -> ChangeDiffAreaMaximumSizeEx(This,pwszVolumeName,pwszDiffAreaVolumeName,llMaximumDiffSpace,bVolatile) ) 

#define IVssDifferentialSoftwareSnapshotMgmt3_MigrateDiffAreas(This,pwszVolumeName,pwszDiffAreaVolumeName,pwszNewDiffAreaVolumeName)	\
    ( (This)->lpVtbl -> MigrateDiffAreas(This,pwszVolumeName,pwszDiffAreaVolumeName,pwszNewDiffAreaVolumeName) ) 

#define IVssDifferentialSoftwareSnapshotMgmt3_QueryMigrationStatus(This,pwszVolumeName,pwszDiffAreaVolumeName,ppAsync)	\
    ( (This)->lpVtbl -> QueryMigrationStatus(This,pwszVolumeName,pwszDiffAreaVolumeName,ppAsync) ) 

#define IVssDifferentialSoftwareSnapshotMgmt3_SetSnapshotPriority(This,idSnapshot,priority)	\
    ( (This)->lpVtbl -> SetSnapshotPriority(This,idSnapshot,priority) ) 


#define IVssDifferentialSoftwareSnapshotMgmt3_SetVolumeProtectLevel(This,pwszVolumeName,protectionLevel)	\
    ( (This)->lpVtbl -> SetVolumeProtectLevel(This,pwszVolumeName,protectionLevel) ) 

#define IVssDifferentialSoftwareSnapshotMgmt3_GetVolumeProtectLevel(This,pwszVolumeName,protectionLevel)	\
    ( (This)->lpVtbl -> GetVolumeProtectLevel(This,pwszVolumeName,protectionLevel) ) 

#define IVssDifferentialSoftwareSnapshotMgmt3_ClearVolumeProtectFault(This,pwszVolumeName)	\
    ( (This)->lpVtbl -> ClearVolumeProtectFault(This,pwszVolumeName) ) 

#define IVssDifferentialSoftwareSnapshotMgmt3_DeleteUnusedDiffAreas(This,pwszDiffAreaVolumeName)	\
    ( (This)->lpVtbl -> DeleteUnusedDiffAreas(This,pwszDiffAreaVolumeName) ) 

#define IVssDifferentialSoftwareSnapshotMgmt3_QuerySnapshotDeltaBitmap(This,idSnapshotOlder,idSnapshotYounger,pcBlockSizePerBit,pcBitmapLength,ppbBitmap)	\
    ( (This)->lpVtbl -> QuerySnapshotDeltaBitmap(This,idSnapshotOlder,idSnapshotYounger,pcBlockSizePerBit,pcBitmapLength,ppbBitmap) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVssDifferentialSoftwareSnapshotMgmt3_INTERFACE_DEFINED__ */


#ifndef __IVssEnumMgmtObject_INTERFACE_DEFINED__
#define __IVssEnumMgmtObject_INTERFACE_DEFINED__

/* interface IVssEnumMgmtObject */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IVssEnumMgmtObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("01954E6B-9254-4e6e-808C-C9E05D007696")
    IVssEnumMgmtObject : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) VSS_MGMT_OBJECT_PROP *rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out][in] */ __RPC__deref_inout_opt IVssEnumMgmtObject **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVssEnumMgmtObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVssEnumMgmtObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVssEnumMgmtObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVssEnumMgmtObject * This);
        
        DECLSPEC_XFGVIRT(IVssEnumMgmtObject, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IVssEnumMgmtObject * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) VSS_MGMT_OBJECT_PROP *rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IVssEnumMgmtObject, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IVssEnumMgmtObject * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IVssEnumMgmtObject, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IVssEnumMgmtObject * This);
        
        DECLSPEC_XFGVIRT(IVssEnumMgmtObject, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IVssEnumMgmtObject * This,
            /* [out][in] */ __RPC__deref_inout_opt IVssEnumMgmtObject **ppenum);
        
        END_INTERFACE
    } IVssEnumMgmtObjectVtbl;

    interface IVssEnumMgmtObject
    {
        CONST_VTBL struct IVssEnumMgmtObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVssEnumMgmtObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVssEnumMgmtObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVssEnumMgmtObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVssEnumMgmtObject_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IVssEnumMgmtObject_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IVssEnumMgmtObject_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IVssEnumMgmtObject_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVssEnumMgmtObject_INTERFACE_DEFINED__ */



#ifndef __VSMGMT_LIBRARY_DEFINED__
#define __VSMGMT_LIBRARY_DEFINED__

/* library VSMGMT */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_VSMGMT;

EXTERN_C const CLSID CLSID_VssSnapshotMgmt;

#ifdef __cplusplus

class DECLSPEC_UUID("0B5A2C52-3EB9-470a-96E2-6C6D4570E40F")
VssSnapshotMgmt;
#endif
#endif /* __VSMGMT_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_vsmgmt_0000_0007 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_vsmgmt_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_vsmgmt_0000_0007_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


