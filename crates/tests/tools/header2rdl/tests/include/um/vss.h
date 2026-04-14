

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

#ifndef __vss_h__
#define __vss_h__

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

#ifndef __IVssEnumObject_FWD_DEFINED__
#define __IVssEnumObject_FWD_DEFINED__
typedef interface IVssEnumObject IVssEnumObject;

#endif 	/* __IVssEnumObject_FWD_DEFINED__ */


#ifndef __IVssAsync_FWD_DEFINED__
#define __IVssAsync_FWD_DEFINED__
typedef interface IVssAsync IVssAsync;

#endif 	/* __IVssAsync_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_vss_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#pragma pack(push, 8)
#include "vsserror.h" 


typedef 
enum _VSS_OBJECT_TYPE
    {
        VSS_OBJECT_UNKNOWN	= 0,
        VSS_OBJECT_NONE	= ( VSS_OBJECT_UNKNOWN + 1 ) ,
        VSS_OBJECT_SNAPSHOT_SET	= ( VSS_OBJECT_NONE + 1 ) ,
        VSS_OBJECT_SNAPSHOT	= ( VSS_OBJECT_SNAPSHOT_SET + 1 ) ,
        VSS_OBJECT_PROVIDER	= ( VSS_OBJECT_SNAPSHOT + 1 ) ,
        VSS_OBJECT_TYPE_COUNT	= ( VSS_OBJECT_PROVIDER + 1 ) 
    } 	VSS_OBJECT_TYPE;

typedef enum _VSS_OBJECT_TYPE *PVSS_OBJECT_TYPE;

typedef 
enum _VSS_SNAPSHOT_STATE
    {
        VSS_SS_UNKNOWN	= 0,
        VSS_SS_PREPARING	= ( VSS_SS_UNKNOWN + 1 ) ,
        VSS_SS_PROCESSING_PREPARE	= ( VSS_SS_PREPARING + 1 ) ,
        VSS_SS_PREPARED	= ( VSS_SS_PROCESSING_PREPARE + 1 ) ,
        VSS_SS_PROCESSING_PRECOMMIT	= ( VSS_SS_PREPARED + 1 ) ,
        VSS_SS_PRECOMMITTED	= ( VSS_SS_PROCESSING_PRECOMMIT + 1 ) ,
        VSS_SS_PROCESSING_COMMIT	= ( VSS_SS_PRECOMMITTED + 1 ) ,
        VSS_SS_COMMITTED	= ( VSS_SS_PROCESSING_COMMIT + 1 ) ,
        VSS_SS_PROCESSING_POSTCOMMIT	= ( VSS_SS_COMMITTED + 1 ) ,
        VSS_SS_PROCESSING_PREFINALCOMMIT	= ( VSS_SS_PROCESSING_POSTCOMMIT + 1 ) ,
        VSS_SS_PREFINALCOMMITTED	= ( VSS_SS_PROCESSING_PREFINALCOMMIT + 1 ) ,
        VSS_SS_PROCESSING_POSTFINALCOMMIT	= ( VSS_SS_PREFINALCOMMITTED + 1 ) ,
        VSS_SS_CREATED	= ( VSS_SS_PROCESSING_POSTFINALCOMMIT + 1 ) ,
        VSS_SS_ABORTED	= ( VSS_SS_CREATED + 1 ) ,
        VSS_SS_DELETED	= ( VSS_SS_ABORTED + 1 ) ,
        VSS_SS_POSTCOMMITTED	= ( VSS_SS_DELETED + 1 ) ,
        VSS_SS_COUNT	= ( VSS_SS_POSTCOMMITTED + 1 ) 
    } 	VSS_SNAPSHOT_STATE;

typedef enum _VSS_SNAPSHOT_STATE *PVSS_SNAPSHOT_STATE;

typedef 
enum _VSS_VOLUME_SNAPSHOT_ATTRIBUTES
    {
        VSS_VOLSNAP_ATTR_PERSISTENT	= 0x1,
        VSS_VOLSNAP_ATTR_NO_AUTORECOVERY	= 0x2,
        VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE	= 0x4,
        VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE	= 0x8,
        VSS_VOLSNAP_ATTR_NO_WRITERS	= 0x10,
        VSS_VOLSNAP_ATTR_TRANSPORTABLE	= 0x20,
        VSS_VOLSNAP_ATTR_NOT_SURFACED	= 0x40,
        VSS_VOLSNAP_ATTR_NOT_TRANSACTED	= 0x80,
        VSS_VOLSNAP_ATTR_HARDWARE_ASSISTED	= 0x10000,
        VSS_VOLSNAP_ATTR_DIFFERENTIAL	= 0x20000,
        VSS_VOLSNAP_ATTR_PLEX	= 0x40000,
        VSS_VOLSNAP_ATTR_IMPORTED	= 0x80000,
        VSS_VOLSNAP_ATTR_EXPOSED_LOCALLY	= 0x100000,
        VSS_VOLSNAP_ATTR_EXPOSED_REMOTELY	= 0x200000,
        VSS_VOLSNAP_ATTR_AUTORECOVER	= 0x400000,
        VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY	= 0x800000,
        VSS_VOLSNAP_ATTR_DELAYED_POSTSNAPSHOT	= 0x1000000,
        VSS_VOLSNAP_ATTR_TXF_RECOVERY	= 0x2000000,
        VSS_VOLSNAP_ATTR_FILE_SHARE	= 0x4000000
    } 	VSS_VOLUME_SNAPSHOT_ATTRIBUTES;

typedef enum _VSS_VOLUME_SNAPSHOT_ATTRIBUTES *PVSS_VOLUME_SNAPSHOT_ATTRIBUTES;

typedef 
enum _VSS_SNAPSHOT_CONTEXT
    {
        VSS_CTX_BACKUP	= 0,
        VSS_CTX_FILE_SHARE_BACKUP	= VSS_VOLSNAP_ATTR_NO_WRITERS,
        VSS_CTX_NAS_ROLLBACK	= ( ( VSS_VOLSNAP_ATTR_PERSISTENT | VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE )  | VSS_VOLSNAP_ATTR_NO_WRITERS ) ,
        VSS_CTX_APP_ROLLBACK	= ( VSS_VOLSNAP_ATTR_PERSISTENT | VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE ) ,
        VSS_CTX_CLIENT_ACCESSIBLE	= ( ( ( VSS_VOLSNAP_ATTR_PERSISTENT | VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE )  | VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE )  | VSS_VOLSNAP_ATTR_NO_WRITERS ) ,
        VSS_CTX_CLIENT_ACCESSIBLE_WRITERS	= ( ( VSS_VOLSNAP_ATTR_PERSISTENT | VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE )  | VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE ) ,
        VSS_CTX_ALL	= 0xffffffff
    } 	VSS_SNAPSHOT_CONTEXT;

typedef enum _VSS_SNAPSHOT_CONTEXT *PVSS_SNAPSHOT_CONTEXT;

typedef 
enum _VSS_PROVIDER_CAPABILITIES
    {
        VSS_PRV_CAPABILITY_LEGACY	= 0x1,
        VSS_PRV_CAPABILITY_COMPLIANT	= 0x2,
        VSS_PRV_CAPABILITY_LUN_REPOINT	= 0x4,
        VSS_PRV_CAPABILITY_LUN_RESYNC	= 0x8,
        VSS_PRV_CAPABILITY_OFFLINE_CREATION	= 0x10,
        VSS_PRV_CAPABILITY_MULTIPLE_IMPORT	= 0x20,
        VSS_PRV_CAPABILITY_RECYCLING	= 0x40,
        VSS_PRV_CAPABILITY_PLEX	= 0x80,
        VSS_PRV_CAPABILITY_DIFFERENTIAL	= 0x100,
        VSS_PRV_CAPABILITY_CLUSTERED	= 0x200
    } 	VSS_PROVIDER_CAPABILITIES;

typedef enum _VSS_PROVIDER_CAPABILITIES *PVSS_PROVIDER_CAPABILITIES;

typedef 
enum _VSS_HARDWARE_OPTIONS
    {
        VSS_BREAKEX_FLAG_MASK_LUNS	= 0x1,
        VSS_BREAKEX_FLAG_MAKE_READ_WRITE	= 0x2,
        VSS_BREAKEX_FLAG_REVERT_IDENTITY_ALL	= 0x4,
        VSS_BREAKEX_FLAG_REVERT_IDENTITY_NONE	= 0x8,
        VSS_ONLUNSTATECHANGE_NOTIFY_READ_WRITE	= 0x100,
        VSS_ONLUNSTATECHANGE_NOTIFY_LUN_PRE_RECOVERY	= 0x200,
        VSS_ONLUNSTATECHANGE_NOTIFY_LUN_POST_RECOVERY	= 0x400,
        VSS_ONLUNSTATECHANGE_DO_MASK_LUNS	= 0x800
    } 	VSS_HARDWARE_OPTIONS;

typedef enum _VSS_HARDWARE_OPTIONS *PVSS_HARDWARE_OPTIONS;

typedef 
enum _VSS_RECOVERY_OPTIONS
    {
        VSS_RECOVERY_REVERT_IDENTITY_ALL	= 0x100,
        VSS_RECOVERY_NO_VOLUME_CHECK	= 0x200
    } 	VSS_RECOVERY_OPTIONS;

typedef enum _VSS_RECOVERY_OPTIONS *PVSS_RECOVERY_OPTIONS;

typedef 
enum _VSS_WRITER_STATE
    {
        VSS_WS_UNKNOWN	= 0,
        VSS_WS_STABLE	= ( VSS_WS_UNKNOWN + 1 ) ,
        VSS_WS_WAITING_FOR_FREEZE	= ( VSS_WS_STABLE + 1 ) ,
        VSS_WS_WAITING_FOR_THAW	= ( VSS_WS_WAITING_FOR_FREEZE + 1 ) ,
        VSS_WS_WAITING_FOR_POST_SNAPSHOT	= ( VSS_WS_WAITING_FOR_THAW + 1 ) ,
        VSS_WS_WAITING_FOR_BACKUP_COMPLETE	= ( VSS_WS_WAITING_FOR_POST_SNAPSHOT + 1 ) ,
        VSS_WS_FAILED_AT_IDENTIFY	= ( VSS_WS_WAITING_FOR_BACKUP_COMPLETE + 1 ) ,
        VSS_WS_FAILED_AT_PREPARE_BACKUP	= ( VSS_WS_FAILED_AT_IDENTIFY + 1 ) ,
        VSS_WS_FAILED_AT_PREPARE_SNAPSHOT	= ( VSS_WS_FAILED_AT_PREPARE_BACKUP + 1 ) ,
        VSS_WS_FAILED_AT_FREEZE	= ( VSS_WS_FAILED_AT_PREPARE_SNAPSHOT + 1 ) ,
        VSS_WS_FAILED_AT_THAW	= ( VSS_WS_FAILED_AT_FREEZE + 1 ) ,
        VSS_WS_FAILED_AT_POST_SNAPSHOT	= ( VSS_WS_FAILED_AT_THAW + 1 ) ,
        VSS_WS_FAILED_AT_BACKUP_COMPLETE	= ( VSS_WS_FAILED_AT_POST_SNAPSHOT + 1 ) ,
        VSS_WS_FAILED_AT_PRE_RESTORE	= ( VSS_WS_FAILED_AT_BACKUP_COMPLETE + 1 ) ,
        VSS_WS_FAILED_AT_POST_RESTORE	= ( VSS_WS_FAILED_AT_PRE_RESTORE + 1 ) ,
        VSS_WS_FAILED_AT_BACKUPSHUTDOWN	= ( VSS_WS_FAILED_AT_POST_RESTORE + 1 ) ,
        VSS_WS_COUNT	= ( VSS_WS_FAILED_AT_BACKUPSHUTDOWN + 1 ) 
    } 	VSS_WRITER_STATE;

typedef enum _VSS_WRITER_STATE *PVSS_WRITER_STATE;

typedef 
enum _VSS_BACKUP_TYPE
    {
        VSS_BT_UNDEFINED	= 0,
        VSS_BT_FULL	= ( VSS_BT_UNDEFINED + 1 ) ,
        VSS_BT_INCREMENTAL	= ( VSS_BT_FULL + 1 ) ,
        VSS_BT_DIFFERENTIAL	= ( VSS_BT_INCREMENTAL + 1 ) ,
        VSS_BT_LOG	= ( VSS_BT_DIFFERENTIAL + 1 ) ,
        VSS_BT_COPY	= ( VSS_BT_LOG + 1 ) ,
        VSS_BT_OTHER	= ( VSS_BT_COPY + 1 ) 
    } 	VSS_BACKUP_TYPE;

typedef enum _VSS_BACKUP_TYPE *PVSS_BACKUP_TYPE;

typedef 
enum _VSS_RESTORE_TYPE
    {
        VSS_RTYPE_UNDEFINED	= 0,
        VSS_RTYPE_BY_COPY	= ( VSS_RTYPE_UNDEFINED + 1 ) ,
        VSS_RTYPE_IMPORT	= ( VSS_RTYPE_BY_COPY + 1 ) ,
        VSS_RTYPE_OTHER	= ( VSS_RTYPE_IMPORT + 1 ) 
    } 	VSS_RESTORE_TYPE;

typedef enum _VSS_RESTORE_TYPE *PVSS_RESTORE_TYPE;

typedef 
enum _VSS_ROLLFORWARD_TYPE
    {
        VSS_RF_UNDEFINED	= 0,
        VSS_RF_NONE	= ( VSS_RF_UNDEFINED + 1 ) ,
        VSS_RF_ALL	= ( VSS_RF_NONE + 1 ) ,
        VSS_RF_PARTIAL	= ( VSS_RF_ALL + 1 ) 
    } 	VSS_ROLLFORWARD_TYPE;

typedef enum _VSS_ROLLFORWARD_TYPE *PVSS_ROLLFORWARD_TYPE;

typedef 
enum _VSS_PROVIDER_TYPE
    {
        VSS_PROV_UNKNOWN	= 0,
        VSS_PROV_SYSTEM	= 1,
        VSS_PROV_SOFTWARE	= 2,
        VSS_PROV_HARDWARE	= 3,
        VSS_PROV_FILESHARE	= 4
    } 	VSS_PROVIDER_TYPE;

typedef enum _VSS_PROVIDER_TYPE *PVSS_PROVIDER_TYPE;

typedef 
enum _VSS_APPLICATION_LEVEL
    {
        VSS_APP_UNKNOWN	= 0,
        VSS_APP_SYSTEM	= 1,
        VSS_APP_BACK_END	= 2,
        VSS_APP_FRONT_END	= 3,
        VSS_APP_SYSTEM_RM	= 4,
        VSS_APP_AUTO	= -1
    } 	VSS_APPLICATION_LEVEL;

typedef enum _VSS_APPLICATION_LEVEL *PVSS_APPLICATION_LEVEL;

typedef 
enum _VSS_SNAPSHOT_COMPATIBILITY
    {
        VSS_SC_DISABLE_DEFRAG	= 0x1,
        VSS_SC_DISABLE_CONTENTINDEX	= 0x2
    } 	VSS_SNAPSHOT_COMPATIBILITY;

typedef 
enum _VSS_SNAPSHOT_PROPERTY_ID
    {
        VSS_SPROPID_UNKNOWN	= 0,
        VSS_SPROPID_SNAPSHOT_ID	= 0x1,
        VSS_SPROPID_SNAPSHOT_SET_ID	= 0x2,
        VSS_SPROPID_SNAPSHOTS_COUNT	= 0x3,
        VSS_SPROPID_SNAPSHOT_DEVICE	= 0x4,
        VSS_SPROPID_ORIGINAL_VOLUME	= 0x5,
        VSS_SPROPID_ORIGINATING_MACHINE	= 0x6,
        VSS_SPROPID_SERVICE_MACHINE	= 0x7,
        VSS_SPROPID_EXPOSED_NAME	= 0x8,
        VSS_SPROPID_EXPOSED_PATH	= 0x9,
        VSS_SPROPID_PROVIDER_ID	= 0xa,
        VSS_SPROPID_SNAPSHOT_ATTRIBUTES	= 0xb,
        VSS_SPROPID_CREATION_TIMESTAMP	= 0xc,
        VSS_SPROPID_STATUS	= 0xd
    } 	VSS_SNAPSHOT_PROPERTY_ID;

typedef enum _VSS_SNAPSHOT_PROPERTY_ID *PVSS_SNAPSHOT_PROPERTY_ID;

typedef 
enum _VSS_FILE_SPEC_BACKUP_TYPE
    {
        VSS_FSBT_FULL_BACKUP_REQUIRED	= 0x1,
        VSS_FSBT_DIFFERENTIAL_BACKUP_REQUIRED	= 0x2,
        VSS_FSBT_INCREMENTAL_BACKUP_REQUIRED	= 0x4,
        VSS_FSBT_LOG_BACKUP_REQUIRED	= 0x8,
        VSS_FSBT_FULL_SNAPSHOT_REQUIRED	= 0x100,
        VSS_FSBT_DIFFERENTIAL_SNAPSHOT_REQUIRED	= 0x200,
        VSS_FSBT_INCREMENTAL_SNAPSHOT_REQUIRED	= 0x400,
        VSS_FSBT_LOG_SNAPSHOT_REQUIRED	= 0x800,
        VSS_FSBT_CREATED_DURING_BACKUP	= 0x10000,
        VSS_FSBT_ALL_BACKUP_REQUIRED	= 0xf,
        VSS_FSBT_ALL_SNAPSHOT_REQUIRED	= 0xf00
    } 	VSS_FILE_SPEC_BACKUP_TYPE;

typedef enum _VSS_FILE_SPEC_BACKUP_TYPE *PVSS_FILE_SPEC_BACKUP_TYPE;

typedef 
enum _VSS_BACKUP_SCHEMA
    {
        VSS_BS_UNDEFINED	= 0,
        VSS_BS_DIFFERENTIAL	= 0x1,
        VSS_BS_INCREMENTAL	= 0x2,
        VSS_BS_EXCLUSIVE_INCREMENTAL_DIFFERENTIAL	= 0x4,
        VSS_BS_LOG	= 0x8,
        VSS_BS_COPY	= 0x10,
        VSS_BS_TIMESTAMPED	= 0x20,
        VSS_BS_LAST_MODIFY	= 0x40,
        VSS_BS_LSN	= 0x80,
        VSS_BS_WRITER_SUPPORTS_NEW_TARGET	= 0x100,
        VSS_BS_WRITER_SUPPORTS_RESTORE_WITH_MOVE	= 0x200,
        VSS_BS_INDEPENDENT_SYSTEM_STATE	= 0x400,
        VSS_BS_ROLLFORWARD_RESTORE	= 0x1000,
        VSS_BS_RESTORE_RENAME	= 0x2000,
        VSS_BS_AUTHORITATIVE_RESTORE	= 0x4000,
        VSS_BS_WRITER_SUPPORTS_PARALLEL_RESTORES	= 0x8000
    } 	VSS_BACKUP_SCHEMA;

typedef enum _VSS_BACKUP_SCHEMA *PVSS_BACKUP_SCHEMA;

typedef GUID VSS_ID;

typedef /* [string][unique] */  __RPC_unique_pointer  __RPC_string WCHAR *VSS_PWSZ;

typedef LONGLONG VSS_TIMESTAMP;

typedef struct _VSS_SNAPSHOT_PROP
    {
    VSS_ID m_SnapshotId;
    VSS_ID m_SnapshotSetId;
    LONG m_lSnapshotsCount;
    VSS_PWSZ m_pwszSnapshotDeviceObject;
    VSS_PWSZ m_pwszOriginalVolumeName;
    VSS_PWSZ m_pwszOriginatingMachine;
    VSS_PWSZ m_pwszServiceMachine;
    VSS_PWSZ m_pwszExposedName;
    VSS_PWSZ m_pwszExposedPath;
    VSS_ID m_ProviderId;
    LONG m_lSnapshotAttributes;
    VSS_TIMESTAMP m_tsCreationTimestamp;
    VSS_SNAPSHOT_STATE m_eStatus;
    } 	VSS_SNAPSHOT_PROP;

typedef struct _VSS_SNAPSHOT_PROP *PVSS_SNAPSHOT_PROP;

typedef struct _VSS_PROVIDER_PROP
    {
    VSS_ID m_ProviderId;
    VSS_PWSZ m_pwszProviderName;
    VSS_PROVIDER_TYPE m_eProviderType;
    VSS_PWSZ m_pwszProviderVersion;
    VSS_ID m_ProviderVersionId;
    CLSID m_ClassId;
    } 	VSS_PROVIDER_PROP;

typedef struct _VSS_PROVIDER_PROP *PVSS_PROVIDER_PROP;

typedef /* [public][public][public][public][switch_type] */ union __MIDL___MIDL_itf_vss_0000_0000_0001
    {
    /* [case()] */ VSS_SNAPSHOT_PROP Snap;
    /* [case()] */ VSS_PROVIDER_PROP Prov;
    /* [default] */  /* Empty union arm */ 
    } 	VSS_OBJECT_UNION;

typedef struct _VSS_OBJECT_PROP
    {
    VSS_OBJECT_TYPE Type;
    /* [switch_is] */ VSS_OBJECT_UNION Obj;
    } 	VSS_OBJECT_PROP;

typedef struct _VSS_OBJECT_PROP *PVSS_OBJECT_PROP;



extern RPC_IF_HANDLE __MIDL_itf_vss_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_vss_0000_0000_v0_0_s_ifspec;

#ifndef __IVssEnumObject_INTERFACE_DEFINED__
#define __IVssEnumObject_INTERFACE_DEFINED__

/* interface IVssEnumObject */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IVssEnumObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AE1C7110-2F60-11d3-8A39-00C04F72D8E3")
    IVssEnumObject : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) VSS_OBJECT_PROP *rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out][in] */ __RPC__deref_inout_opt IVssEnumObject **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVssEnumObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVssEnumObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVssEnumObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVssEnumObject * This);
        
        DECLSPEC_XFGVIRT(IVssEnumObject, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IVssEnumObject * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) VSS_OBJECT_PROP *rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IVssEnumObject, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IVssEnumObject * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IVssEnumObject, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IVssEnumObject * This);
        
        DECLSPEC_XFGVIRT(IVssEnumObject, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IVssEnumObject * This,
            /* [out][in] */ __RPC__deref_inout_opt IVssEnumObject **ppenum);
        
        END_INTERFACE
    } IVssEnumObjectVtbl;

    interface IVssEnumObject
    {
        CONST_VTBL struct IVssEnumObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVssEnumObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVssEnumObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVssEnumObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVssEnumObject_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IVssEnumObject_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IVssEnumObject_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IVssEnumObject_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVssEnumObject_INTERFACE_DEFINED__ */


#ifndef __IVssAsync_INTERFACE_DEFINED__
#define __IVssAsync_INTERFACE_DEFINED__

/* interface IVssAsync */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IVssAsync;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("507C37B4-CF5B-4e95-B0AF-14EB9767467E")
    IVssAsync : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Wait( 
            /* [defaultvalue][in] */ DWORD dwMilliseconds = 0xffffffff) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryStatus( 
            /* [out] */ __RPC__out HRESULT *pHrResult,
            /* [unique][out][in] */ __RPC__inout_opt INT *pReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVssAsyncVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVssAsync * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVssAsync * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVssAsync * This);
        
        DECLSPEC_XFGVIRT(IVssAsync, Cancel)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IVssAsync * This);
        
        DECLSPEC_XFGVIRT(IVssAsync, Wait)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Wait )( 
            __RPC__in IVssAsync * This,
            /* [defaultvalue][in] */ DWORD dwMilliseconds);
        
        DECLSPEC_XFGVIRT(IVssAsync, QueryStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryStatus )( 
            __RPC__in IVssAsync * This,
            /* [out] */ __RPC__out HRESULT *pHrResult,
            /* [unique][out][in] */ __RPC__inout_opt INT *pReserved);
        
        END_INTERFACE
    } IVssAsyncVtbl;

    interface IVssAsync
    {
        CONST_VTBL struct IVssAsyncVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVssAsync_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVssAsync_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVssAsync_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVssAsync_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IVssAsync_Wait(This,dwMilliseconds)	\
    ( (This)->lpVtbl -> Wait(This,dwMilliseconds) ) 

#define IVssAsync_QueryStatus(This,pHrResult,pReserved)	\
    ( (This)->lpVtbl -> QueryStatus(This,pHrResult,pReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVssAsync_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_vss_0000_0002 */
/* [local] */ 


#pragma pack(pop)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_vss_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_vss_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


