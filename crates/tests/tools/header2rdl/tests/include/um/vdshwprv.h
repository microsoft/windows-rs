

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

#ifndef __vdshwprv_h__
#define __vdshwprv_h__

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

#ifndef __IEnumVdsObject_FWD_DEFINED__
#define __IEnumVdsObject_FWD_DEFINED__
typedef interface IEnumVdsObject IEnumVdsObject;

#endif 	/* __IEnumVdsObject_FWD_DEFINED__ */


#ifndef __IVdsAsync_FWD_DEFINED__
#define __IVdsAsync_FWD_DEFINED__
typedef interface IVdsAsync IVdsAsync;

#endif 	/* __IVdsAsync_FWD_DEFINED__ */


#ifndef __IVdsAdviseSink_FWD_DEFINED__
#define __IVdsAdviseSink_FWD_DEFINED__
typedef interface IVdsAdviseSink IVdsAdviseSink;

#endif 	/* __IVdsAdviseSink_FWD_DEFINED__ */


#ifndef __IVdsProvider_FWD_DEFINED__
#define __IVdsProvider_FWD_DEFINED__
typedef interface IVdsProvider IVdsProvider;

#endif 	/* __IVdsProvider_FWD_DEFINED__ */


#ifndef __IVdsProviderSupport_FWD_DEFINED__
#define __IVdsProviderSupport_FWD_DEFINED__
typedef interface IVdsProviderSupport IVdsProviderSupport;

#endif 	/* __IVdsProviderSupport_FWD_DEFINED__ */


#ifndef __IVdsProviderPrivate_FWD_DEFINED__
#define __IVdsProviderPrivate_FWD_DEFINED__
typedef interface IVdsProviderPrivate IVdsProviderPrivate;

#endif 	/* __IVdsProviderPrivate_FWD_DEFINED__ */


#ifndef __IVdsHwProvider_FWD_DEFINED__
#define __IVdsHwProvider_FWD_DEFINED__
typedef interface IVdsHwProvider IVdsHwProvider;

#endif 	/* __IVdsHwProvider_FWD_DEFINED__ */


#ifndef __IVdsHwProviderType_FWD_DEFINED__
#define __IVdsHwProviderType_FWD_DEFINED__
typedef interface IVdsHwProviderType IVdsHwProviderType;

#endif 	/* __IVdsHwProviderType_FWD_DEFINED__ */


#ifndef __IVdsHwProviderType2_FWD_DEFINED__
#define __IVdsHwProviderType2_FWD_DEFINED__
typedef interface IVdsHwProviderType2 IVdsHwProviderType2;

#endif 	/* __IVdsHwProviderType2_FWD_DEFINED__ */


#ifndef __IVdsHwProviderStoragePools_FWD_DEFINED__
#define __IVdsHwProviderStoragePools_FWD_DEFINED__
typedef interface IVdsHwProviderStoragePools IVdsHwProviderStoragePools;

#endif 	/* __IVdsHwProviderStoragePools_FWD_DEFINED__ */


#ifndef __IVdsSubSystem_FWD_DEFINED__
#define __IVdsSubSystem_FWD_DEFINED__
typedef interface IVdsSubSystem IVdsSubSystem;

#endif 	/* __IVdsSubSystem_FWD_DEFINED__ */


#ifndef __IVdsSubSystem2_FWD_DEFINED__
#define __IVdsSubSystem2_FWD_DEFINED__
typedef interface IVdsSubSystem2 IVdsSubSystem2;

#endif 	/* __IVdsSubSystem2_FWD_DEFINED__ */


#ifndef __IVdsSubSystemNaming_FWD_DEFINED__
#define __IVdsSubSystemNaming_FWD_DEFINED__
typedef interface IVdsSubSystemNaming IVdsSubSystemNaming;

#endif 	/* __IVdsSubSystemNaming_FWD_DEFINED__ */


#ifndef __IVdsSubSystemIscsi_FWD_DEFINED__
#define __IVdsSubSystemIscsi_FWD_DEFINED__
typedef interface IVdsSubSystemIscsi IVdsSubSystemIscsi;

#endif 	/* __IVdsSubSystemIscsi_FWD_DEFINED__ */


#ifndef __IVdsSubSystemInterconnect_FWD_DEFINED__
#define __IVdsSubSystemInterconnect_FWD_DEFINED__
typedef interface IVdsSubSystemInterconnect IVdsSubSystemInterconnect;

#endif 	/* __IVdsSubSystemInterconnect_FWD_DEFINED__ */


#ifndef __IVdsControllerPort_FWD_DEFINED__
#define __IVdsControllerPort_FWD_DEFINED__
typedef interface IVdsControllerPort IVdsControllerPort;

#endif 	/* __IVdsControllerPort_FWD_DEFINED__ */


#ifndef __IVdsController_FWD_DEFINED__
#define __IVdsController_FWD_DEFINED__
typedef interface IVdsController IVdsController;

#endif 	/* __IVdsController_FWD_DEFINED__ */


#ifndef __IVdsControllerControllerPort_FWD_DEFINED__
#define __IVdsControllerControllerPort_FWD_DEFINED__
typedef interface IVdsControllerControllerPort IVdsControllerControllerPort;

#endif 	/* __IVdsControllerControllerPort_FWD_DEFINED__ */


#ifndef __IVdsDrive_FWD_DEFINED__
#define __IVdsDrive_FWD_DEFINED__
typedef interface IVdsDrive IVdsDrive;

#endif 	/* __IVdsDrive_FWD_DEFINED__ */


#ifndef __IVdsDrive2_FWD_DEFINED__
#define __IVdsDrive2_FWD_DEFINED__
typedef interface IVdsDrive2 IVdsDrive2;

#endif 	/* __IVdsDrive2_FWD_DEFINED__ */


#ifndef __IVdsLun_FWD_DEFINED__
#define __IVdsLun_FWD_DEFINED__
typedef interface IVdsLun IVdsLun;

#endif 	/* __IVdsLun_FWD_DEFINED__ */


#ifndef __IVdsLun2_FWD_DEFINED__
#define __IVdsLun2_FWD_DEFINED__
typedef interface IVdsLun2 IVdsLun2;

#endif 	/* __IVdsLun2_FWD_DEFINED__ */


#ifndef __IVdsLunNaming_FWD_DEFINED__
#define __IVdsLunNaming_FWD_DEFINED__
typedef interface IVdsLunNaming IVdsLunNaming;

#endif 	/* __IVdsLunNaming_FWD_DEFINED__ */


#ifndef __IVdsLunNumber_FWD_DEFINED__
#define __IVdsLunNumber_FWD_DEFINED__
typedef interface IVdsLunNumber IVdsLunNumber;

#endif 	/* __IVdsLunNumber_FWD_DEFINED__ */


#ifndef __IVdsLunControllerPorts_FWD_DEFINED__
#define __IVdsLunControllerPorts_FWD_DEFINED__
typedef interface IVdsLunControllerPorts IVdsLunControllerPorts;

#endif 	/* __IVdsLunControllerPorts_FWD_DEFINED__ */


#ifndef __IVdsLunMpio_FWD_DEFINED__
#define __IVdsLunMpio_FWD_DEFINED__
typedef interface IVdsLunMpio IVdsLunMpio;

#endif 	/* __IVdsLunMpio_FWD_DEFINED__ */


#ifndef __IVdsLunIscsi_FWD_DEFINED__
#define __IVdsLunIscsi_FWD_DEFINED__
typedef interface IVdsLunIscsi IVdsLunIscsi;

#endif 	/* __IVdsLunIscsi_FWD_DEFINED__ */


#ifndef __IVdsLunPlex_FWD_DEFINED__
#define __IVdsLunPlex_FWD_DEFINED__
typedef interface IVdsLunPlex IVdsLunPlex;

#endif 	/* __IVdsLunPlex_FWD_DEFINED__ */


#ifndef __IVdsIscsiPortal_FWD_DEFINED__
#define __IVdsIscsiPortal_FWD_DEFINED__
typedef interface IVdsIscsiPortal IVdsIscsiPortal;

#endif 	/* __IVdsIscsiPortal_FWD_DEFINED__ */


#ifndef __IVdsIscsiTarget_FWD_DEFINED__
#define __IVdsIscsiTarget_FWD_DEFINED__
typedef interface IVdsIscsiTarget IVdsIscsiTarget;

#endif 	/* __IVdsIscsiTarget_FWD_DEFINED__ */


#ifndef __IVdsIscsiPortalGroup_FWD_DEFINED__
#define __IVdsIscsiPortalGroup_FWD_DEFINED__
typedef interface IVdsIscsiPortalGroup IVdsIscsiPortalGroup;

#endif 	/* __IVdsIscsiPortalGroup_FWD_DEFINED__ */


#ifndef __IVdsStoragePool_FWD_DEFINED__
#define __IVdsStoragePool_FWD_DEFINED__
typedef interface IVdsStoragePool IVdsStoragePool;

#endif 	/* __IVdsStoragePool_FWD_DEFINED__ */


#ifndef __IVdsMaintenance_FWD_DEFINED__
#define __IVdsMaintenance_FWD_DEFINED__
typedef interface IVdsMaintenance IVdsMaintenance;

#endif 	/* __IVdsMaintenance_FWD_DEFINED__ */


#ifndef __IVdsHwProviderPrivate_FWD_DEFINED__
#define __IVdsHwProviderPrivate_FWD_DEFINED__
typedef interface IVdsHwProviderPrivate IVdsHwProviderPrivate;

#endif 	/* __IVdsHwProviderPrivate_FWD_DEFINED__ */


#ifndef __IVdsHwProviderPrivateMpio_FWD_DEFINED__
#define __IVdsHwProviderPrivateMpio_FWD_DEFINED__
typedef interface IVdsHwProviderPrivateMpio IVdsHwProviderPrivateMpio;

#endif 	/* __IVdsHwProviderPrivateMpio_FWD_DEFINED__ */


#ifndef __IVdsAdmin_FWD_DEFINED__
#define __IVdsAdmin_FWD_DEFINED__
typedef interface IVdsAdmin IVdsAdmin;

#endif 	/* __IVdsAdmin_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "vdserr.h"
#include "vdslun.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_vdshwprv_0000_0000 */
/* [local] */ 

//+--------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) 2000 Microsoft Corporation.
//
//---------------------------------------------------------------
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif





typedef GUID VDS_OBJECT_ID;

typedef 
enum _VDS_OBJECT_TYPE
    {
        VDS_OT_UNKNOWN	= 0,
        VDS_OT_PROVIDER	= 1,
        VDS_OT_PACK	= 10,
        VDS_OT_VOLUME	= 11,
        VDS_OT_VOLUME_PLEX	= 12,
        VDS_OT_DISK	= 13,
        VDS_OT_SUB_SYSTEM	= 30,
        VDS_OT_CONTROLLER	= 31,
        VDS_OT_DRIVE	= 32,
        VDS_OT_LUN	= 33,
        VDS_OT_LUN_PLEX	= 34,
        VDS_OT_PORT	= 35,
        VDS_OT_PORTAL	= 36,
        VDS_OT_TARGET	= 37,
        VDS_OT_PORTAL_GROUP	= 38,
        VDS_OT_STORAGE_POOL	= 39,
        VDS_OT_HBAPORT	= 90,
        VDS_OT_INIT_ADAPTER	= 91,
        VDS_OT_INIT_PORTAL	= 92,
        VDS_OT_ASYNC	= 100,
        VDS_OT_ENUM	= 101,
        VDS_OT_VDISK	= 200,
        VDS_OT_OPEN_VDISK	= 201
    } 	VDS_OBJECT_TYPE;

typedef 
enum _VDS_PROVIDER_TYPE
    {
        VDS_PT_UNKNOWN	= 0,
        VDS_PT_SOFTWARE	= 1,
        VDS_PT_HARDWARE	= 2,
        VDS_PT_VIRTUALDISK	= 3,
        VDS_PT_MAX	= 4
    } 	VDS_PROVIDER_TYPE;

typedef 
enum _VDS_PROVIDER_FLAG
    {
        VDS_PF_DYNAMIC	= 0x1,
        VDS_PF_INTERNAL_HARDWARE_PROVIDER	= 0x2,
        VDS_PF_ONE_DISK_ONLY_PER_PACK	= 0x4,
        VDS_PF_ONE_PACK_ONLINE_ONLY	= 0x8,
        VDS_PF_VOLUME_SPACE_MUST_BE_CONTIGUOUS	= 0x10,
        VDS_PF_SUPPORT_DYNAMIC	= 0x80000000,
        VDS_PF_SUPPORT_FAULT_TOLERANT	= 0x40000000,
        VDS_PF_SUPPORT_DYNAMIC_1394	= 0x20000000,
        VDS_PF_SUPPORT_MIRROR	= 0x20,
        VDS_PF_SUPPORT_RAID5	= 0x40
    } 	VDS_PROVIDER_FLAG;

typedef 
enum _VDS_RECOVER_ACTION
    {
        VDS_RA_UNKNOWN	= 0,
        VDS_RA_REFRESH	= 1,
        VDS_RA_RESTART	= 2
    } 	VDS_RECOVER_ACTION;

typedef 
enum _VDS_NOTIFICATION_TARGET_TYPE
    {
        VDS_NTT_UNKNOWN	= 0,
        VDS_NTT_PACK	= VDS_OT_PACK,
        VDS_NTT_VOLUME	= VDS_OT_VOLUME,
        VDS_NTT_DISK	= VDS_OT_DISK,
        VDS_NTT_PARTITION	= 60,
        VDS_NTT_DRIVE_LETTER	= 61,
        VDS_NTT_FILE_SYSTEM	= 62,
        VDS_NTT_MOUNT_POINT	= 63,
        VDS_NTT_SUB_SYSTEM	= VDS_OT_SUB_SYSTEM,
        VDS_NTT_CONTROLLER	= VDS_OT_CONTROLLER,
        VDS_NTT_DRIVE	= VDS_OT_DRIVE,
        VDS_NTT_LUN	= VDS_OT_LUN,
        VDS_NTT_PORT	= VDS_OT_PORT,
        VDS_NTT_PORTAL	= VDS_OT_PORTAL,
        VDS_NTT_TARGET	= VDS_OT_TARGET,
        VDS_NTT_PORTAL_GROUP	= VDS_OT_PORTAL_GROUP,
        VDS_NTT_SERVICE	= 200
    } 	VDS_NOTIFICATION_TARGET_TYPE;

#define	VDS_NF_PACK_ARRIVE	( 1 )

#define	VDS_NF_PACK_DEPART	( 2 )

#define	VDS_NF_PACK_MODIFY	( 3 )

#define	VDS_NF_VOLUME_ARRIVE	( 4 )

#define	VDS_NF_VOLUME_DEPART	( 5 )

#define	VDS_NF_VOLUME_MODIFY	( 6 )

#define	VDS_NF_VOLUME_REBUILDING_PROGRESS	( 7 )

#define	VDS_NF_DISK_ARRIVE	( 8 )

#define	VDS_NF_DISK_DEPART	( 9 )

#define	VDS_NF_DISK_MODIFY	( 10 )

#define	VDS_NF_PARTITION_ARRIVE	( 11 )

#define	VDS_NF_PARTITION_DEPART	( 12 )

#define	VDS_NF_PARTITION_MODIFY	( 13 )

#define	VDS_NF_SUB_SYSTEM_ARRIVE	( 101 )

#define	VDS_NF_SUB_SYSTEM_DEPART	( 102 )

#define	VDS_NF_CONTROLLER_ARRIVE	( 103 )

#define	VDS_NF_CONTROLLER_DEPART	( 104 )

#define	VDS_NF_DRIVE_ARRIVE	( 105 )

#define	VDS_NF_DRIVE_DEPART	( 106 )

#define	VDS_NF_DRIVE_MODIFY	( 107 )

#define	VDS_NF_LUN_ARRIVE	( 108 )

#define	VDS_NF_LUN_DEPART	( 109 )

#define	VDS_NF_LUN_MODIFY	( 110 )

#define	VDS_NF_PORT_ARRIVE	( 121 )

#define	VDS_NF_PORT_DEPART	( 122 )

#define	VDS_NF_PORTAL_ARRIVE	( 123 )

#define	VDS_NF_PORTAL_DEPART	( 124 )

#define	VDS_NF_PORTAL_MODIFY	( 125 )

#define	VDS_NF_TARGET_ARRIVE	( 126 )

#define	VDS_NF_TARGET_DEPART	( 127 )

#define	VDS_NF_TARGET_MODIFY	( 128 )

#define	VDS_NF_PORTAL_GROUP_ARRIVE	( 129 )

#define	VDS_NF_PORTAL_GROUP_DEPART	( 130 )

#define	VDS_NF_PORTAL_GROUP_MODIFY	( 131 )

#define	VDS_NF_SUB_SYSTEM_MODIFY	( 151 )

#define	VDS_NF_DRIVE_LETTER_FREE	( 201 )

#define	VDS_NF_DRIVE_LETTER_ASSIGN	( 202 )

#define	VDS_NF_FILE_SYSTEM_MODIFY	( 203 )

#define	VDS_NF_FILE_SYSTEM_FORMAT_PROGRESS	( 204 )

#define	VDS_NF_MOUNT_POINTS_CHANGE	( 205 )

#define	VDS_NF_FILE_SYSTEM_SHRINKING_PROGRESS	( 206 )

#define	VDS_NF_SERVICE_OUT_OF_SYNC	( 301 )

#define	VDS_NF_CONTROLLER_MODIFY	( 350 )

#define	VDS_NF_CONTROLLER_REMOVED	( 351 )

#define	VDS_NF_PORT_MODIFY	( 352 )

#define	VDS_NF_PORT_REMOVED	( 353 )

#define	VDS_NF_DRIVE_REMOVED	( 354 )

typedef struct _VDS_PACK_NOTIFICATION
    {
    ULONG ulEvent;
    VDS_OBJECT_ID packId;
    } 	VDS_PACK_NOTIFICATION;

typedef struct _VDS_DISK_NOTIFICATION
    {
    ULONG ulEvent;
    VDS_OBJECT_ID diskId;
    } 	VDS_DISK_NOTIFICATION;

typedef struct _VDS_VOLUME_NOTIFICATION
    {
    ULONG ulEvent;
    VDS_OBJECT_ID volumeId;
    VDS_OBJECT_ID plexId;
    ULONG ulPercentCompleted;
    } 	VDS_VOLUME_NOTIFICATION;

typedef struct _VDS_PARTITION_NOTIFICATION
    {
    ULONG ulEvent;
    VDS_OBJECT_ID diskId;
    ULONGLONG ullOffset;
    } 	VDS_PARTITION_NOTIFICATION;

typedef struct _VDS_SERVICE_NOTIFICATION
    {
    ULONG ulEvent;
    VDS_RECOVER_ACTION action;
    } 	VDS_SERVICE_NOTIFICATION;

typedef struct _VDS_DRIVE_LETTER_NOTIFICATION
    {
    ULONG ulEvent;
    WCHAR wcLetter;
    VDS_OBJECT_ID volumeId;
    } 	VDS_DRIVE_LETTER_NOTIFICATION;

typedef struct _VDS_FILE_SYSTEM_NOTIFICATION
    {
    ULONG ulEvent;
    VDS_OBJECT_ID volumeId;
    DWORD dwPercentCompleted;
    } 	VDS_FILE_SYSTEM_NOTIFICATION;

typedef struct _VDS_MOUNT_POINT_NOTIFICATION
    {
    ULONG ulEvent;
    VDS_OBJECT_ID volumeId;
    } 	VDS_MOUNT_POINT_NOTIFICATION;

typedef struct _VDS_SUB_SYSTEM_NOTIFICATION
    {
    ULONG ulEvent;
    VDS_OBJECT_ID subSystemId;
    } 	VDS_SUB_SYSTEM_NOTIFICATION;

typedef struct _VDS_CONTROLLER_NOTIFICATION
    {
    ULONG ulEvent;
    VDS_OBJECT_ID controllerId;
    } 	VDS_CONTROLLER_NOTIFICATION;

typedef struct _VDS_DRIVE_NOTIFICATION
    {
    ULONG ulEvent;
    VDS_OBJECT_ID driveId;
    } 	VDS_DRIVE_NOTIFICATION;

typedef struct _VDS_LUN_NOTIFICATION
    {
    ULONG ulEvent;
    VDS_OBJECT_ID LunId;
    } 	VDS_LUN_NOTIFICATION;

typedef struct _VDS_PORT_NOTIFICATION
    {
    ULONG ulEvent;
    VDS_OBJECT_ID portId;
    } 	VDS_PORT_NOTIFICATION;

typedef struct _VDS_PORTAL_NOTIFICATION
    {
    ULONG ulEvent;
    VDS_OBJECT_ID portalId;
    } 	VDS_PORTAL_NOTIFICATION;

typedef struct _VDS_TARGET_NOTIFICATION
    {
    ULONG ulEvent;
    VDS_OBJECT_ID targetId;
    } 	VDS_TARGET_NOTIFICATION;

typedef struct _VDS_PORTAL_GROUP_NOTIFICATION
    {
    ULONG ulEvent;
    VDS_OBJECT_ID portalGroupId;
    } 	VDS_PORTAL_GROUP_NOTIFICATION;

typedef struct _VDS_NOTIFICATION
    {
    VDS_NOTIFICATION_TARGET_TYPE objectType;
    /* [switch_is] */ /* [switch_type] */ union 
        {
        /* [case()] */ VDS_PACK_NOTIFICATION Pack;
        /* [case()] */ VDS_DISK_NOTIFICATION Disk;
        /* [case()] */ VDS_VOLUME_NOTIFICATION Volume;
        /* [case()] */ VDS_PARTITION_NOTIFICATION Partition;
        /* [case()] */ VDS_DRIVE_LETTER_NOTIFICATION Letter;
        /* [case()] */ VDS_FILE_SYSTEM_NOTIFICATION FileSystem;
        /* [case()] */ VDS_MOUNT_POINT_NOTIFICATION MountPoint;
        /* [case()] */ VDS_SUB_SYSTEM_NOTIFICATION SubSystem;
        /* [case()] */ VDS_CONTROLLER_NOTIFICATION Controller;
        /* [case()] */ VDS_DRIVE_NOTIFICATION Drive;
        /* [case()] */ VDS_LUN_NOTIFICATION Lun;
        /* [case()] */ VDS_PORT_NOTIFICATION Port;
        /* [case()] */ VDS_PORTAL_NOTIFICATION Portal;
        /* [case()] */ VDS_TARGET_NOTIFICATION Target;
        /* [case()] */ VDS_PORTAL_GROUP_NOTIFICATION PortalGroup;
        /* [case()] */ VDS_SERVICE_NOTIFICATION Service;
        /* [default] */  /* Empty union arm */ 
        } 	;
    } 	VDS_NOTIFICATION;

typedef 
enum _VDS_ASYNC_OUTPUT_TYPE
    {
        VDS_ASYNCOUT_UNKNOWN	= 0,
        VDS_ASYNCOUT_CREATEVOLUME	= 1,
        VDS_ASYNCOUT_EXTENDVOLUME	= 2,
        VDS_ASYNCOUT_SHRINKVOLUME	= 3,
        VDS_ASYNCOUT_ADDVOLUMEPLEX	= 4,
        VDS_ASYNCOUT_BREAKVOLUMEPLEX	= 5,
        VDS_ASYNCOUT_REMOVEVOLUMEPLEX	= 6,
        VDS_ASYNCOUT_REPAIRVOLUMEPLEX	= 7,
        VDS_ASYNCOUT_RECOVERPACK	= 8,
        VDS_ASYNCOUT_REPLACEDISK	= 9,
        VDS_ASYNCOUT_CREATEPARTITION	= 10,
        VDS_ASYNCOUT_CLEAN	= 11,
        VDS_ASYNCOUT_CREATELUN	= 50,
        VDS_ASYNCOUT_ADDLUNPLEX	= 52,
        VDS_ASYNCOUT_REMOVELUNPLEX	= 53,
        VDS_ASYNCOUT_EXTENDLUN	= 54,
        VDS_ASYNCOUT_SHRINKLUN	= 55,
        VDS_ASYNCOUT_RECOVERLUN	= 56,
        VDS_ASYNCOUT_LOGINTOTARGET	= 60,
        VDS_ASYNCOUT_LOGOUTFROMTARGET	= 61,
        VDS_ASYNCOUT_CREATETARGET	= 62,
        VDS_ASYNCOUT_CREATEPORTALGROUP	= 63,
        VDS_ASYNCOUT_DELETETARGET	= 64,
        VDS_ASYNCOUT_ADDPORTAL	= 65,
        VDS_ASYNCOUT_REMOVEPORTAL	= 66,
        VDS_ASYNCOUT_DELETEPORTALGROUP	= 67,
        VDS_ASYNCOUT_FORMAT	= 101,
        VDS_ASYNCOUT_CREATE_VDISK	= 200,
        VDS_ASYNCOUT_ATTACH_VDISK	= 201,
        VDS_ASYNCOUT_COMPACT_VDISK	= 202,
        VDS_ASYNCOUT_MERGE_VDISK	= 203,
        VDS_ASYNCOUT_EXPAND_VDISK	= 204
    } 	VDS_ASYNC_OUTPUT_TYPE;

typedef struct _VDS_ASYNC_OUTPUT
    {
    VDS_ASYNC_OUTPUT_TYPE type;
    /* [switch_is] */ /* [switch_type] */ union 
        {
        /* [case()] */ struct _cp
            {
            ULONGLONG ullOffset;
            VDS_OBJECT_ID volumeId;
            } 	cp;
        /* [case()] */ struct _cv
            {
            IUnknown *pVolumeUnk;
            } 	cv;
        /* [case()] */ struct _bvp
            {
            IUnknown *pVolumeUnk;
            } 	bvp;
        /* [case()] */ struct _sv
            {
            ULONGLONG ullReclaimedBytes;
            } 	sv;
        /* [case()] */ struct _cl
            {
            IUnknown *pLunUnk;
            } 	cl;
        /* [case()] */ struct _ct
            {
            IUnknown *pTargetUnk;
            } 	ct;
        /* [case()] */ struct _cpg
            {
            IUnknown *pPortalGroupUnk;
            } 	cpg;
        /* [case()] */ struct _cvd
            {
            IUnknown *pVDiskUnk;
            } 	cvd;
        /* [default] */  /* Empty union arm */ 
        } 	;
    } 	VDS_ASYNC_OUTPUT;

typedef USHORT VDS_ISCSI_PORTALGROUP_TAG;

typedef 
enum VDS_IPADDRESS_TYPE
    {
        VDS_IPT_TEXT	= 0,
        VDS_IPT_IPV4	= 1,
        VDS_IPT_IPV6	= 2,
        VDS_IPT_EMPTY	= 3
    } 	VDS_IPADDRESS_TYPE;

typedef 
enum _VDS_HEALTH
    {
        VDS_H_UNKNOWN	= 0,
        VDS_H_HEALTHY	= 1,
        VDS_H_REBUILDING	= 2,
        VDS_H_STALE	= 3,
        VDS_H_FAILING	= 4,
        VDS_H_FAILING_REDUNDANCY	= 5,
        VDS_H_FAILED_REDUNDANCY	= 6,
        VDS_H_FAILED_REDUNDANCY_FAILING	= 7,
        VDS_H_FAILED	= 8,
        VDS_H_REPLACED	= 9,
        VDS_H_PENDING_FAILURE	= 10,
        VDS_H_DEGRADED	= 11
    } 	VDS_HEALTH;

typedef 
enum _VDS_TRANSITION_STATE
    {
        VDS_TS_UNKNOWN	= 0,
        VDS_TS_STABLE	= 1,
        VDS_TS_EXTENDING	= 2,
        VDS_TS_SHRINKING	= 3,
        VDS_TS_RECONFIGING	= 4,
        VDS_TS_RESTRIPING	= 5
    } 	VDS_TRANSITION_STATE;

typedef 
enum _VDS_FILE_SYSTEM_TYPE
    {
        VDS_FST_UNKNOWN	= 0,
        VDS_FST_RAW	= ( VDS_FST_UNKNOWN + 1 ) ,
        VDS_FST_FAT	= ( VDS_FST_RAW + 1 ) ,
        VDS_FST_FAT32	= ( VDS_FST_FAT + 1 ) ,
        VDS_FST_NTFS	= ( VDS_FST_FAT32 + 1 ) ,
        VDS_FST_CDFS	= ( VDS_FST_NTFS + 1 ) ,
        VDS_FST_UDF	= ( VDS_FST_CDFS + 1 ) ,
        VDS_FST_EXFAT	= ( VDS_FST_UDF + 1 ) ,
        VDS_FST_CSVFS	= ( VDS_FST_EXFAT + 1 ) ,
        VDS_FST_REFS	= ( VDS_FST_CSVFS + 1 ) 
    } 	VDS_FILE_SYSTEM_TYPE;

typedef 
enum _VDS_HBAPORT_TYPE
    {
        VDS_HPT_UNKNOWN	= 1,
        VDS_HPT_OTHER	= 2,
        VDS_HPT_NOTPRESENT	= 3,
        VDS_HPT_NPORT	= 5,
        VDS_HPT_NLPORT	= 6,
        VDS_HPT_FLPORT	= 7,
        VDS_HPT_FPORT	= 8,
        VDS_HPT_EPORT	= 9,
        VDS_HPT_GPORT	= 10,
        VDS_HPT_LPORT	= 20,
        VDS_HPT_PTP	= 21
    } 	VDS_HBAPORT_TYPE;

typedef 
enum _VDS_HBAPORT_STATUS
    {
        VDS_HPS_UNKNOWN	= 1,
        VDS_HPS_ONLINE	= 2,
        VDS_HPS_OFFLINE	= 3,
        VDS_HPS_BYPASSED	= 4,
        VDS_HPS_DIAGNOSTICS	= 5,
        VDS_HPS_LINKDOWN	= 6,
        VDS_HPS_ERROR	= 7,
        VDS_HPS_LOOPBACK	= 8
    } 	VDS_HBAPORT_STATUS;

typedef 
enum _VDS_HBAPORT_SPEED_FLAG
    {
        VDS_HSF_UNKNOWN	= 0,
        VDS_HSF_1GBIT	= 0x1,
        VDS_HSF_2GBIT	= 0x2,
        VDS_HSF_10GBIT	= 0x4,
        VDS_HSF_4GBIT	= 0x8,
        VDS_HSF_NOT_NEGOTIATED	= ( 1 << 15 ) 
    } 	VDS_HBAPORT_SPEED_FLAG;

typedef 
enum _VDS_PATH_STATUS
    {
        VDS_MPS_UNKNOWN	= 0,
        VDS_MPS_ONLINE	= 1,
        VDS_MPS_FAILED	= 5,
        VDS_MPS_STANDBY	= 7
    } 	VDS_PATH_STATUS;

typedef 
enum _VDS_LOADBALANCE_POLICY_ENUM
    {
        VDS_LBP_UNKNOWN	= 0,
        VDS_LBP_FAILOVER	= 1,
        VDS_LBP_ROUND_ROBIN	= 2,
        VDS_LBP_ROUND_ROBIN_WITH_SUBSET	= 3,
        VDS_LBP_DYN_LEAST_QUEUE_DEPTH	= 4,
        VDS_LBP_WEIGHTED_PATHS	= 5,
        VDS_LBP_LEAST_BLOCKS	= 6,
        VDS_LBP_VENDOR_SPECIFIC	= 7
    } 	VDS_LOADBALANCE_POLICY_ENUM;

typedef 
enum _VDS_PROVIDER_LBSUPPORT_FLAG
    {
        VDS_LBF_FAILOVER	= 0x1,
        VDS_LBF_ROUND_ROBIN	= 0x2,
        VDS_LBF_ROUND_ROBIN_WITH_SUBSET	= 0x4,
        VDS_LBF_DYN_LEAST_QUEUE_DEPTH	= 0x8,
        VDS_LBF_WEIGHTED_PATHS	= 0x10,
        VDS_LBF_LEAST_BLOCKS	= 0x20,
        VDS_LBF_VENDOR_SPECIFIC	= 0x40
    } 	VDS_PROVIDER_LBSUPPORT_FLAG;

typedef 
enum _VDS_VERSION_SUPPORT_FLAG
    {
        VDS_VSF_1_0	= 0x1,
        VDS_VSF_1_1	= 0x2,
        VDS_VSF_2_0	= 0x4,
        VDS_VSF_2_1	= 0x8,
        VDS_VSF_3_0	= 0x10
    } 	VDS_VERSION_SUPPORT_FLAG;

typedef 
enum _VDS_HWPROVIDER_TYPE
    {
        VDS_HWT_UNKNOWN	= 0,
        VDS_HWT_PCI_RAID	= 1,
        VDS_HWT_FIBRE_CHANNEL	= 2,
        VDS_HWT_ISCSI	= 3,
        VDS_HWT_SAS	= 4,
        VDS_HWT_HYBRID	= 5
    } 	VDS_HWPROVIDER_TYPE;

typedef 
enum _VDS_ISCSI_LOGIN_TYPE
    {
        VDS_ILT_MANUAL	= 0,
        VDS_ILT_PERSISTENT	= 1,
        VDS_ILT_BOOT	= 2
    } 	VDS_ISCSI_LOGIN_TYPE;

typedef 
enum _VDS_ISCSI_AUTH_TYPE
    {
        VDS_IAT_NONE	= 0,
        VDS_IAT_CHAP	= 1,
        VDS_IAT_MUTUAL_CHAP	= 2
    } 	VDS_ISCSI_AUTH_TYPE;

typedef 
enum _VDS_ISCSI_IPSEC_FLAG
    {
        VDS_IIF_VALID	= 0x1,
        VDS_IIF_IKE	= 0x2,
        VDS_IIF_MAIN_MODE	= 0x4,
        VDS_IIF_AGGRESSIVE_MODE	= 0x8,
        VDS_IIF_PFS_ENABLE	= 0x10,
        VDS_IIF_TRANSPORT_MODE_PREFERRED	= 0x20,
        VDS_IIF_TUNNEL_MODE_PREFERRED	= 0x40
    } 	VDS_ISCSI_IPSEC_FLAG;

typedef 
enum _VDS_ISCSI_LOGIN_FLAG
    {
        VDS_ILF_REQUIRE_IPSEC	= 0x1,
        VDS_ILF_MULTIPATH_ENABLED	= 0x2
    } 	VDS_ISCSI_LOGIN_FLAG;

typedef struct _VDS_PATH_ID
    {
    ULONGLONG ullSourceId;
    ULONGLONG ullPathId;
    } 	VDS_PATH_ID;

typedef struct _VDS_WWN
    {
    UCHAR rguchWwn[ 8 ];
    } 	VDS_WWN;

typedef struct _VDS_IPADDRESS
    {
    VDS_IPADDRESS_TYPE type;
    ULONG ipv4Address;
    UCHAR ipv6Address[ 16 ];
    ULONG ulIpv6FlowInfo;
    ULONG ulIpv6ScopeId;
    WCHAR wszTextAddress[ 257 ];
    ULONG ulPort;
    } 	VDS_IPADDRESS;

typedef struct _VDS_ISCSI_IPSEC_KEY
    {
    /* [size_is] */ UCHAR *pKey;
    ULONG ulKeySize;
    } 	VDS_ISCSI_IPSEC_KEY;

typedef struct _VDS_ISCSI_SHARED_SECRET
    {
    /* [size_is] */ UCHAR *pSharedSecret;
    ULONG ulSharedSecretSize;
    } 	VDS_ISCSI_SHARED_SECRET;

typedef struct _VDS_HBAPORT_PROP
    {
    VDS_OBJECT_ID id;
    VDS_WWN wwnNode;
    VDS_WWN wwnPort;
    VDS_HBAPORT_TYPE type;
    VDS_HBAPORT_STATUS status;
    ULONG ulPortSpeed;
    ULONG ulSupportedPortSpeed;
    } 	VDS_HBAPORT_PROP;

typedef struct _VDS_ISCSI_INITIATOR_ADAPTER_PROP
    {
    VDS_OBJECT_ID id;
    /* [string] */ LPWSTR pwszName;
    } 	VDS_ISCSI_INITIATOR_ADAPTER_PROP;

typedef struct _VDS_ISCSI_INITIATOR_PORTAL_PROP
    {
    VDS_OBJECT_ID id;
    VDS_IPADDRESS address;
    ULONG ulPortIndex;
    } 	VDS_ISCSI_INITIATOR_PORTAL_PROP;

typedef struct _VDS_PROVIDER_PROP
    {
    VDS_OBJECT_ID id;
    /* [string] */ LPWSTR pwszName;
    GUID guidVersionId;
    /* [string] */ LPWSTR pwszVersion;
    VDS_PROVIDER_TYPE type;
    ULONG ulFlags;
    ULONG ulStripeSizeFlags;
    SHORT sRebuildPriority;
    } 	VDS_PROVIDER_PROP;

typedef struct _VDS_PATH_INFO
    {
    VDS_PATH_ID pathId;
    VDS_HWPROVIDER_TYPE type;
    VDS_PATH_STATUS status;
    /* [switch_is] */ /* [switch_type] */ union 
        {
        /* [case()] */ VDS_OBJECT_ID controllerPortId;
        /* [case()] */ VDS_OBJECT_ID targetPortalId;
        /* [default] */  /* Empty union arm */ 
        } 	;
    /* [switch_is] */ /* [switch_type] */ union 
        {
        /* [case()] */ VDS_OBJECT_ID hbaPortId;
        /* [case()] */ VDS_OBJECT_ID initiatorAdapterId;
        /* [default] */  /* Empty union arm */ 
        } 	;
    /* [switch_is] */ /* [switch_type] */ union 
        {
        /* [case()] */ VDS_HBAPORT_PROP *pHbaPortProp;
        /* [case()] */ VDS_IPADDRESS *pInitiatorPortalIpAddr;
        /* [default] */  /* Empty union arm */ 
        } 	;
    } 	VDS_PATH_INFO;

typedef struct _VDS_PATH_POLICY
    {
    VDS_PATH_ID pathId;
    BOOL bPrimaryPath;
    ULONG ulWeight;
    } 	VDS_PATH_POLICY;



extern RPC_IF_HANDLE __MIDL_itf_vdshwprv_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_vdshwprv_0000_0000_v0_0_s_ifspec;

#ifndef __IEnumVdsObject_INTERFACE_DEFINED__
#define __IEnumVdsObject_INTERFACE_DEFINED__

/* interface IEnumVdsObject */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumVdsObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("118610b7-8d94-4030-b5b8-500889788e4e")
    IEnumVdsObject : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pcFetched) IUnknown **ppObjectArray,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumVdsObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumVdsObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumVdsObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumVdsObject * This);
        
        DECLSPEC_XFGVIRT(IEnumVdsObject, Next)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumVdsObject * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pcFetched) IUnknown **ppObjectArray,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumVdsObject, Skip)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumVdsObject * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumVdsObject, Reset)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumVdsObject * This);
        
        DECLSPEC_XFGVIRT(IEnumVdsObject, Clone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumVdsObject * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        END_INTERFACE
    } IEnumVdsObjectVtbl;

    interface IEnumVdsObject
    {
        CONST_VTBL struct IEnumVdsObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumVdsObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumVdsObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumVdsObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumVdsObject_Next(This,celt,ppObjectArray,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppObjectArray,pcFetched) ) 

#define IEnumVdsObject_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumVdsObject_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumVdsObject_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumVdsObject_INTERFACE_DEFINED__ */


#ifndef __IVdsAsync_INTERFACE_DEFINED__
#define __IVdsAsync_INTERFACE_DEFINED__

/* interface IVdsAsync */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsAsync;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d5d23b6d-5a55-4492-9889-397a3c2d2dbc")
    IVdsAsync : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Wait( 
            /* [out] */ __RPC__out HRESULT *pHrResult,
            /* [out] */ __RPC__out VDS_ASYNC_OUTPUT *pAsyncOut) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryStatus( 
            /* [out] */ __RPC__out HRESULT *pHrResult,
            /* [out] */ __RPC__out ULONG *pulPercentCompleted) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsAsyncVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsAsync * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsAsync * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsAsync * This);
        
        DECLSPEC_XFGVIRT(IVdsAsync, Cancel)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IVdsAsync * This);
        
        DECLSPEC_XFGVIRT(IVdsAsync, Wait)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Wait )( 
            __RPC__in IVdsAsync * This,
            /* [out] */ __RPC__out HRESULT *pHrResult,
            /* [out] */ __RPC__out VDS_ASYNC_OUTPUT *pAsyncOut);
        
        DECLSPEC_XFGVIRT(IVdsAsync, QueryStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryStatus )( 
            __RPC__in IVdsAsync * This,
            /* [out] */ __RPC__out HRESULT *pHrResult,
            /* [out] */ __RPC__out ULONG *pulPercentCompleted);
        
        END_INTERFACE
    } IVdsAsyncVtbl;

    interface IVdsAsync
    {
        CONST_VTBL struct IVdsAsyncVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsAsync_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsAsync_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsAsync_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsAsync_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IVdsAsync_Wait(This,pHrResult,pAsyncOut)	\
    ( (This)->lpVtbl -> Wait(This,pHrResult,pAsyncOut) ) 

#define IVdsAsync_QueryStatus(This,pHrResult,pulPercentCompleted)	\
    ( (This)->lpVtbl -> QueryStatus(This,pHrResult,pulPercentCompleted) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsAsync_INTERFACE_DEFINED__ */


#ifndef __IVdsAdviseSink_INTERFACE_DEFINED__
#define __IVdsAdviseSink_INTERFACE_DEFINED__

/* interface IVdsAdviseSink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsAdviseSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8326cd1d-cf59-4936-b786-5efc08798e25")
    IVdsAdviseSink : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnNotify( 
            /* [range][in] */ __RPC__in_range(1,100) LONG lNumberOfNotifications,
            /* [size_is][in] */ __RPC__in_ecount_full(lNumberOfNotifications) VDS_NOTIFICATION *pNotificationArray) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsAdviseSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsAdviseSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsAdviseSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsAdviseSink * This);
        
        DECLSPEC_XFGVIRT(IVdsAdviseSink, OnNotify)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnNotify )( 
            __RPC__in IVdsAdviseSink * This,
            /* [range][in] */ __RPC__in_range(1,100) LONG lNumberOfNotifications,
            /* [size_is][in] */ __RPC__in_ecount_full(lNumberOfNotifications) VDS_NOTIFICATION *pNotificationArray);
        
        END_INTERFACE
    } IVdsAdviseSinkVtbl;

    interface IVdsAdviseSink
    {
        CONST_VTBL struct IVdsAdviseSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsAdviseSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsAdviseSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsAdviseSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsAdviseSink_OnNotify(This,lNumberOfNotifications,pNotificationArray)	\
    ( (This)->lpVtbl -> OnNotify(This,lNumberOfNotifications,pNotificationArray) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsAdviseSink_INTERFACE_DEFINED__ */


#ifndef __IVdsProvider_INTERFACE_DEFINED__
#define __IVdsProvider_INTERFACE_DEFINED__

/* interface IVdsProvider */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("10c5e575-7984-4e81-a56b-431f5f92ae42")
    IVdsProvider : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [out] */ __RPC__out VDS_PROVIDER_PROP *pProviderProp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsProvider * This);
        
        DECLSPEC_XFGVIRT(IVdsProvider, GetProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in IVdsProvider * This,
            /* [out] */ __RPC__out VDS_PROVIDER_PROP *pProviderProp);
        
        END_INTERFACE
    } IVdsProviderVtbl;

    interface IVdsProvider
    {
        CONST_VTBL struct IVdsProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsProvider_GetProperties(This,pProviderProp)	\
    ( (This)->lpVtbl -> GetProperties(This,pProviderProp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsProvider_INTERFACE_DEFINED__ */


#ifndef __IVdsProviderSupport_INTERFACE_DEFINED__
#define __IVdsProviderSupport_INTERFACE_DEFINED__

/* interface IVdsProviderSupport */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsProviderSupport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1732be13-e8f9-4a03-bfbc-5f616aa66ce1")
    IVdsProviderSupport : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetVersionSupport( 
            /* [out] */ __RPC__out ULONG *ulVersionSupport) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsProviderSupportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsProviderSupport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsProviderSupport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsProviderSupport * This);
        
        DECLSPEC_XFGVIRT(IVdsProviderSupport, GetVersionSupport)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetVersionSupport )( 
            __RPC__in IVdsProviderSupport * This,
            /* [out] */ __RPC__out ULONG *ulVersionSupport);
        
        END_INTERFACE
    } IVdsProviderSupportVtbl;

    interface IVdsProviderSupport
    {
        CONST_VTBL struct IVdsProviderSupportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsProviderSupport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsProviderSupport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsProviderSupport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsProviderSupport_GetVersionSupport(This,ulVersionSupport)	\
    ( (This)->lpVtbl -> GetVersionSupport(This,ulVersionSupport) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsProviderSupport_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_vdshwprv_0000_0005 */
/* [local] */ 

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



extern RPC_IF_HANDLE __MIDL_itf_vdshwprv_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_vdshwprv_0000_0005_v0_0_s_ifspec;

#ifndef __IVdsProviderPrivate_INTERFACE_DEFINED__
#define __IVdsProviderPrivate_INTERFACE_DEFINED__

/* interface IVdsProviderPrivate */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsProviderPrivate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("11f3cd41-b7e8-48ff-9472-9dff018aa292")
    IVdsProviderPrivate : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetObject( 
            /* [in] */ VDS_OBJECT_ID ObjectId,
            /* [in] */ VDS_OBJECT_TYPE type,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppObjectUnk) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnLoad( 
            /* [string][in] */ __RPC__in_string LPWSTR pwszMachineName,
            /* [in] */ __RPC__in_opt IUnknown *pCallbackObject) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnUnload( 
            /* [in] */ BOOL bForceUnload) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsProviderPrivateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsProviderPrivate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsProviderPrivate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsProviderPrivate * This);
        
        DECLSPEC_XFGVIRT(IVdsProviderPrivate, GetObject)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetObject )( 
            __RPC__in IVdsProviderPrivate * This,
            /* [in] */ VDS_OBJECT_ID ObjectId,
            /* [in] */ VDS_OBJECT_TYPE type,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppObjectUnk);
        
        DECLSPEC_XFGVIRT(IVdsProviderPrivate, OnLoad)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnLoad )( 
            __RPC__in IVdsProviderPrivate * This,
            /* [string][in] */ __RPC__in_string LPWSTR pwszMachineName,
            /* [in] */ __RPC__in_opt IUnknown *pCallbackObject);
        
        DECLSPEC_XFGVIRT(IVdsProviderPrivate, OnUnload)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnUnload )( 
            __RPC__in IVdsProviderPrivate * This,
            /* [in] */ BOOL bForceUnload);
        
        END_INTERFACE
    } IVdsProviderPrivateVtbl;

    interface IVdsProviderPrivate
    {
        CONST_VTBL struct IVdsProviderPrivateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsProviderPrivate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsProviderPrivate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsProviderPrivate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsProviderPrivate_GetObject(This,ObjectId,type,ppObjectUnk)	\
    ( (This)->lpVtbl -> GetObject(This,ObjectId,type,ppObjectUnk) ) 

#define IVdsProviderPrivate_OnLoad(This,pwszMachineName,pCallbackObject)	\
    ( (This)->lpVtbl -> OnLoad(This,pwszMachineName,pCallbackObject) ) 

#define IVdsProviderPrivate_OnUnload(This,bForceUnload)	\
    ( (This)->lpVtbl -> OnUnload(This,bForceUnload) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsProviderPrivate_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_vdshwprv_0000_0006 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif


























typedef 
enum _VDS_SUB_SYSTEM_STATUS
    {
        VDS_SSS_UNKNOWN	= 0,
        VDS_SSS_ONLINE	= 1,
        VDS_SSS_NOT_READY	= 2,
        VDS_SSS_OFFLINE	= 4,
        VDS_SSS_FAILED	= 5,
        VDS_SSS_PARTIALLY_MANAGED	= 9
    } 	VDS_SUB_SYSTEM_STATUS;

typedef enum _VDS_SUB_SYSTEM_STATUS *PVDS_SUB_SYSTEM_STATUS;

typedef 
enum _VDS_SUB_SYSTEM_FLAG
    {
        VDS_SF_LUN_MASKING_CAPABLE	= 0x1,
        VDS_SF_LUN_PLEXING_CAPABLE	= 0x2,
        VDS_SF_LUN_REMAPPING_CAPABLE	= 0x4,
        VDS_SF_DRIVE_EXTENT_CAPABLE	= 0x8,
        VDS_SF_HARDWARE_CHECKSUM_CAPABLE	= 0x10,
        VDS_SF_RADIUS_CAPABLE	= 0x20,
        VDS_SF_READ_BACK_VERIFY_CAPABLE	= 0x40,
        VDS_SF_WRITE_THROUGH_CACHING_CAPABLE	= 0x80,
        VDS_SF_SUPPORTS_FAULT_TOLERANT_LUNS	= 0x200,
        VDS_SF_SUPPORTS_NON_FAULT_TOLERANT_LUNS	= 0x400,
        VDS_SF_SUPPORTS_SIMPLE_LUNS	= 0x800,
        VDS_SF_SUPPORTS_SPAN_LUNS	= 0x1000,
        VDS_SF_SUPPORTS_STRIPE_LUNS	= 0x2000,
        VDS_SF_SUPPORTS_MIRROR_LUNS	= 0x4000,
        VDS_SF_SUPPORTS_PARITY_LUNS	= 0x8000,
        VDS_SF_SUPPORTS_AUTH_CHAP	= 0x10000,
        VDS_SF_SUPPORTS_AUTH_MUTUAL_CHAP	= 0x20000,
        VDS_SF_SUPPORTS_SIMPLE_TARGET_CONFIG	= 0x40000,
        VDS_SF_SUPPORTS_LUN_NUMBER	= 0x80000,
        VDS_SF_SUPPORTS_MIRRORED_CACHE	= 0x100000,
        VDS_SF_READ_CACHING_CAPABLE	= 0x200000,
        VDS_SF_WRITE_CACHING_CAPABLE	= 0x400000,
        VDS_SF_MEDIA_SCAN_CAPABLE	= 0x800000,
        VDS_SF_CONSISTENCY_CHECK_CAPABLE	= 0x1000000
    } 	VDS_SUB_SYSTEM_FLAG;

typedef enum _VDS_SUB_SYSTEM_FLAG *PVDS_SUB_SYSTEM_FLAG;

typedef 
enum _VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG
    {
        VDS_SF_SUPPORTS_RAID2_LUNS	= 0x1,
        VDS_SF_SUPPORTS_RAID3_LUNS	= 0x2,
        VDS_SF_SUPPORTS_RAID4_LUNS	= 0x4,
        VDS_SF_SUPPORTS_RAID5_LUNS	= 0x8,
        VDS_SF_SUPPORTS_RAID6_LUNS	= 0x10,
        VDS_SF_SUPPORTS_RAID01_LUNS	= 0x20,
        VDS_SF_SUPPORTS_RAID03_LUNS	= 0x40,
        VDS_SF_SUPPORTS_RAID05_LUNS	= 0x80,
        VDS_SF_SUPPORTS_RAID10_LUNS	= 0x100,
        VDS_SF_SUPPORTS_RAID15_LUNS	= 0x200,
        VDS_SF_SUPPORTS_RAID30_LUNS	= 0x400,
        VDS_SF_SUPPORTS_RAID50_LUNS	= 0x800,
        VDS_SF_SUPPORTS_RAID51_LUNS	= 0x1000,
        VDS_SF_SUPPORTS_RAID53_LUNS	= 0x2000,
        VDS_SF_SUPPORTS_RAID60_LUNS	= 0x4000,
        VDS_SF_SUPPORTS_RAID61_LUNS	= 0x8000
    } 	VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG;

typedef enum _VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG *PVDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG;

typedef 
enum _VDS_INTERCONNECT_FLAG
    {
        VDS_ITF_PCI_RAID	= 0x1,
        VDS_ITF_FIBRE_CHANNEL	= 0x2,
        VDS_ITF_ISCSI	= 0x4,
        VDS_ITF_SAS	= 0x8
    } 	VDS_INTERCONNECT_FLAG;

typedef enum _VDS_INTERCONNECT_FLAG *PVDS_INTERCONNECT_FLAG;

typedef 
enum _VDS_CONTROLLER_STATUS
    {
        VDS_CS_UNKNOWN	= 0,
        VDS_CS_ONLINE	= 1,
        VDS_CS_NOT_READY	= 2,
        VDS_CS_OFFLINE	= 4,
        VDS_CS_FAILED	= 5,
        VDS_CS_REMOVED	= 8
    } 	VDS_CONTROLLER_STATUS;

typedef enum _VDS_CONTROLLER_STATUS *PVDS_CONTROLLER_STATUS;

typedef 
enum _VDS_PORT_STATUS
    {
        VDS_PRS_UNKNOWN	= 0,
        VDS_PRS_ONLINE	= 1,
        VDS_PRS_NOT_READY	= 2,
        VDS_PRS_OFFLINE	= 4,
        VDS_PRS_FAILED	= 5,
        VDS_PRS_REMOVED	= 8
    } 	VDS_PORT_STATUS;

typedef enum _VDS_PORT_STATUS *PVDS_PORT_STATUS;

typedef 
enum _VDS_DRIVE_STATUS
    {
        VDS_DRS_UNKNOWN	= 0,
        VDS_DRS_ONLINE	= 1,
        VDS_DRS_NOT_READY	= 2,
        VDS_DRS_OFFLINE	= 4,
        VDS_DRS_FAILED	= 5,
        VDS_DRS_REMOVED	= 8
    } 	VDS_DRIVE_STATUS;

typedef enum _VDS_DRIVE_STATUS *PVDS_DRIVE_STATUS;

typedef 
enum _VDS_DRIVE_FLAG
    {
        VDS_DRF_HOTSPARE	= 0x1,
        VDS_DRF_ASSIGNED	= 0x2,
        VDS_DRF_UNASSIGNED	= 0x4,
        VDS_DRF_HOTSPARE_IN_USE	= 0x8,
        VDS_DRF_HOTSPARE_STANDBY	= 0x10
    } 	VDS_DRIVE_FLAG;

typedef enum _VDS_DRIVE_FLAG *PVDS_DRIVE_FLAG;

typedef 
enum _VDS_LUN_TYPE
    {
        VDS_LT_UNKNOWN	= 0,
        VDS_LT_DEFAULT	= 1,
        VDS_LT_FAULT_TOLERANT	= 2,
        VDS_LT_NON_FAULT_TOLERANT	= 3,
        VDS_LT_SIMPLE	= 10,
        VDS_LT_SPAN	= 11,
        VDS_LT_STRIPE	= 12,
        VDS_LT_MIRROR	= 13,
        VDS_LT_PARITY	= 14,
        VDS_LT_RAID2	= 15,
        VDS_LT_RAID3	= 16,
        VDS_LT_RAID4	= 17,
        VDS_LT_RAID5	= 18,
        VDS_LT_RAID6	= 19,
        VDS_LT_RAID01	= 20,
        VDS_LT_RAID03	= 21,
        VDS_LT_RAID05	= 22,
        VDS_LT_RAID10	= 23,
        VDS_LT_RAID15	= 24,
        VDS_LT_RAID30	= 25,
        VDS_LT_RAID50	= 26,
        VDS_LT_RAID51	= 27,
        VDS_LT_RAID53	= 28,
        VDS_LT_RAID60	= 29,
        VDS_LT_RAID61	= 30
    } 	VDS_LUN_TYPE;

typedef enum _VDS_LUN_TYPE *PVDS_LUN_TYPE;

typedef 
enum _VDS_LUN_STATUS
    {
        VDS_LS_UNKNOWN	= 0,
        VDS_LS_ONLINE	= 1,
        VDS_LS_NOT_READY	= 2,
        VDS_LS_OFFLINE	= 4,
        VDS_LS_FAILED	= 5
    } 	VDS_LUN_STATUS;

typedef enum _VDS_LUN_STATUS *PVDS_LUN_STATUS;

typedef 
enum _VDS_LUN_FLAG
    {
        VDS_LF_LBN_REMAP_ENABLED	= 0x1,
        VDS_LF_READ_BACK_VERIFY_ENABLED	= 0x2,
        VDS_LF_WRITE_THROUGH_CACHING_ENABLED	= 0x4,
        VDS_LF_HARDWARE_CHECKSUM_ENABLED	= 0x8,
        VDS_LF_READ_CACHE_ENABLED	= 0x10,
        VDS_LF_WRITE_CACHE_ENABLED	= 0x20,
        VDS_LF_MEDIA_SCAN_ENABLED	= 0x40,
        VDS_LF_CONSISTENCY_CHECK_ENABLED	= 0x80,
        VDS_LF_SNAPSHOT	= 0x100
    } 	VDS_LUN_FLAG;

typedef enum _VDS_LUN_FLAG *PVDS_LUN_FLAG;

typedef 
enum _VDS_LUN_PLEX_TYPE
    {
        VDS_LPT_UNKNOWN	= 0,
        VDS_LPT_SIMPLE	= VDS_LT_SIMPLE,
        VDS_LPT_SPAN	= VDS_LT_SPAN,
        VDS_LPT_STRIPE	= VDS_LT_STRIPE,
        VDS_LPT_PARITY	= VDS_LT_PARITY,
        VDS_LPT_RAID2	= VDS_LT_RAID2,
        VDS_LPT_RAID3	= VDS_LT_RAID3,
        VDS_LPT_RAID4	= VDS_LT_RAID4,
        VDS_LPT_RAID5	= VDS_LT_RAID5,
        VDS_LPT_RAID6	= VDS_LT_RAID6,
        VDS_LPT_RAID03	= VDS_LT_RAID03,
        VDS_LPT_RAID05	= VDS_LT_RAID05,
        VDS_LPT_RAID10	= VDS_LT_RAID10,
        VDS_LPT_RAID15	= VDS_LT_RAID15,
        VDS_LPT_RAID30	= VDS_LT_RAID30,
        VDS_LPT_RAID50	= VDS_LT_RAID50,
        VDS_LPT_RAID53	= VDS_LT_RAID53,
        VDS_LPT_RAID60	= VDS_LT_RAID60
    } 	VDS_LUN_PLEX_TYPE;

typedef 
enum _VDS_LUN_PLEX_STATUS
    {
        VDS_LPS_UNKNOWN	= 0,
        VDS_LPS_ONLINE	= 1,
        VDS_LPS_NOT_READY	= 2,
        VDS_LPS_OFFLINE	= 4,
        VDS_LPS_FAILED	= 5
    } 	VDS_LUN_PLEX_STATUS;

typedef 
enum _VDS_LUN_PLEX_FLAG
    {
        VDS_LPF_LBN_REMAP_ENABLED	= VDS_LF_LBN_REMAP_ENABLED
    } 	VDS_LUN_PLEX_FLAG;

typedef 
enum _VDS_ISCSI_PORTAL_STATUS
    {
        VDS_IPS_UNKNOWN	= 0,
        VDS_IPS_ONLINE	= 1,
        VDS_IPS_NOT_READY	= 2,
        VDS_IPS_OFFLINE	= 4,
        VDS_IPS_FAILED	= 5
    } 	VDS_ISCSI_PORTAL_STATUS;

typedef 
enum _VDS_STORAGE_POOL_STATUS
    {
        VDS_SPS_UNKNOWN	= 0,
        VDS_SPS_ONLINE	= 1,
        VDS_SPS_NOT_READY	= 2,
        VDS_SPS_OFFLINE	= 4
    } 	VDS_STORAGE_POOL_STATUS;

typedef 
enum _VDS_STORAGE_POOL_TYPE
    {
        VDS_SPT_UNKNOWN	= 0,
        VDS_SPT_PRIMORDIAL	= 0x1,
        VDS_SPT_CONCRETE	= 0x2
    } 	VDS_STORAGE_POOL_TYPE;

typedef 
enum _VDS_MAINTENANCE_OPERATION
    {
        BlinkLight	= 1,
        BeepAlarm	= 2,
        SpinDown	= 3,
        SpinUp	= 4,
        Ping	= 5
    } 	VDS_MAINTENANCE_OPERATION;

typedef struct _VDS_HINTS
    {
    ULONGLONG ullHintMask;
    ULONGLONG ullExpectedMaximumSize;
    ULONG ulOptimalReadSize;
    ULONG ulOptimalReadAlignment;
    ULONG ulOptimalWriteSize;
    ULONG ulOptimalWriteAlignment;
    ULONG ulMaximumDriveCount;
    ULONG ulStripeSize;
    BOOL bFastCrashRecoveryRequired;
    BOOL bMostlyReads;
    BOOL bOptimizeForSequentialReads;
    BOOL bOptimizeForSequentialWrites;
    BOOL bRemapEnabled;
    BOOL bReadBackVerifyEnabled;
    BOOL bWriteThroughCachingEnabled;
    BOOL bHardwareChecksumEnabled;
    BOOL bIsYankable;
    SHORT sRebuildPriority;
    } 	VDS_HINTS;

typedef struct _VDS_HINTS *PVDS_HINTS;

#define	VDS_HINT_FASTCRASHRECOVERYREQUIRED	( 0x1L )

#define	VDS_HINT_MOSTLYREADS	( 0x2L )

#define	VDS_HINT_OPTIMIZEFORSEQUENTIALREADS	( 0x4L )

#define	VDS_HINT_OPTIMIZEFORSEQUENTIALWRITES	( 0x8L )

#define	VDS_HINT_READBACKVERIFYENABLED	( 0x10L )

#define	VDS_HINT_REMAPENABLED	( 0x20L )

#define	VDS_HINT_WRITETHROUGHCACHINGENABLED	( 0x40L )

#define	VDS_HINT_HARDWARECHECKSUMENABLED	( 0x80L )

#define	VDS_HINT_ISYANKABLE	( 0x100L )

typedef struct _VDS_HINTS2
    {
    ULONGLONG ullHintMask;
    ULONGLONG ullExpectedMaximumSize;
    ULONG ulOptimalReadSize;
    ULONG ulOptimalReadAlignment;
    ULONG ulOptimalWriteSize;
    ULONG ulOptimalWriteAlignment;
    ULONG ulMaximumDriveCount;
    ULONG ulStripeSize;
    ULONG ulReserved1;
    ULONG ulReserved2;
    ULONG ulReserved3;
    BOOL bFastCrashRecoveryRequired;
    BOOL bMostlyReads;
    BOOL bOptimizeForSequentialReads;
    BOOL bOptimizeForSequentialWrites;
    BOOL bRemapEnabled;
    BOOL bReadBackVerifyEnabled;
    BOOL bWriteThroughCachingEnabled;
    BOOL bHardwareChecksumEnabled;
    BOOL bIsYankable;
    BOOL bAllocateHotSpare;
    BOOL bUseMirroredCache;
    BOOL bReadCachingEnabled;
    BOOL bWriteCachingEnabled;
    BOOL bMediaScanEnabled;
    BOOL bConsistencyCheckEnabled;
    VDS_STORAGE_BUS_TYPE BusType;
    BOOL bReserved1;
    BOOL bReserved2;
    BOOL bReserved3;
    SHORT sRebuildPriority;
    } 	VDS_HINTS2;

typedef struct _VDS_HINTS2 *PVDS_HINTS2;

#define	VDS_HINT_ALLOCATEHOTSPARE	( 0x200L )

#define	VDS_HINT_BUSTYPE	( 0x400L )

#define	VDS_HINT_USEMIRROREDCACHE	( 0x800L )

#define	VDS_HINT_READCACHINGENABLED	( 0x1000L )

#define	VDS_HINT_WRITECACHINGENABLED	( 0x2000L )

#define	VDS_HINT_MEDIASCANENABLED	( 0x4000L )

#define	VDS_HINT_CONSISTENCYCHECKENABLED	( 0x8000L )

typedef struct _VDS_SUB_SYSTEM_PROP
    {
    VDS_OBJECT_ID id;
    /* [string] */ LPWSTR pwszFriendlyName;
    /* [string] */ LPWSTR pwszIdentification;
    ULONG ulFlags;
    ULONG ulStripeSizeFlags;
    VDS_SUB_SYSTEM_STATUS status;
    VDS_HEALTH health;
    SHORT sNumberOfInternalBuses;
    SHORT sMaxNumberOfSlotsEachBus;
    SHORT sMaxNumberOfControllers;
    SHORT sRebuildPriority;
    } 	VDS_SUB_SYSTEM_PROP;

typedef struct _VDS_SUB_SYSTEM_PROP *PVDS_SUB_SYSTEM_PROP;

typedef struct _VDS_SUB_SYSTEM_PROP2
    {
    VDS_OBJECT_ID id;
    /* [string] */ LPWSTR pwszFriendlyName;
    /* [string] */ LPWSTR pwszIdentification;
    ULONG ulFlags;
    ULONG ulStripeSizeFlags;
    ULONG ulSupportedRaidTypeFlags;
    VDS_SUB_SYSTEM_STATUS status;
    VDS_HEALTH health;
    SHORT sNumberOfInternalBuses;
    SHORT sMaxNumberOfSlotsEachBus;
    SHORT sMaxNumberOfControllers;
    SHORT sRebuildPriority;
    ULONG ulNumberOfEnclosures;
    } 	VDS_SUB_SYSTEM_PROP2;

typedef struct _VDS_SUB_SYSTEM_PROP2 *PVDS_SUB_SYSTEM_PROP2;

typedef struct _VDS_CONTROLLER_PROP
    {
    VDS_OBJECT_ID id;
    /* [string] */ LPWSTR pwszFriendlyName;
    /* [string] */ LPWSTR pwszIdentification;
    VDS_CONTROLLER_STATUS status;
    VDS_HEALTH health;
    SHORT sNumberOfPorts;
    } 	VDS_CONTROLLER_PROP;

typedef struct _VDS_CONTROLLER_PROP *PVDS_CONTROLLER_PROP;

typedef struct _VDS_DRIVE_PROP
    {
    VDS_OBJECT_ID id;
    ULONGLONG ullSize;
    /* [string] */ LPWSTR pwszFriendlyName;
    /* [string] */ LPWSTR pwszIdentification;
    ULONG ulFlags;
    VDS_DRIVE_STATUS status;
    VDS_HEALTH health;
    SHORT sInternalBusNumber;
    SHORT sSlotNumber;
    } 	VDS_DRIVE_PROP;

typedef struct _VDS_DRIVE_PROP *PVDS_DRIVE_PROP;

typedef struct _VDS_DRIVE_PROP2
    {
    VDS_OBJECT_ID id;
    ULONGLONG ullSize;
    /* [string] */ LPWSTR pwszFriendlyName;
    /* [string] */ LPWSTR pwszIdentification;
    ULONG ulFlags;
    VDS_DRIVE_STATUS status;
    VDS_HEALTH health;
    SHORT sInternalBusNumber;
    SHORT sSlotNumber;
    ULONG ulEnclosureNumber;
    VDS_STORAGE_BUS_TYPE busType;
    ULONG ulSpindleSpeed;
    } 	VDS_DRIVE_PROP2;

typedef struct _VDS_DRIVE_PROP2 *PVDS_DRIVE_PROP2;

typedef struct _VDS_DRIVE_EXTENT
    {
    VDS_OBJECT_ID id;
    VDS_OBJECT_ID LunId;
    ULONGLONG ullSize;
    BOOL bUsed;
    } 	VDS_DRIVE_EXTENT;

typedef struct _VDS_DRIVE_EXTENT *PVDS_DRIVE_EXTENT;

#define VDS_REBUILD_PRIORITY_MIN        0
#define VDS_REBUILD_PRIORITY_MAX        16
typedef struct _VDS_LUN_PROP
    {
    VDS_OBJECT_ID id;
    ULONGLONG ullSize;
    /* [string] */ LPWSTR pwszFriendlyName;
    /* [string] */ LPWSTR pwszIdentification;
    /* [string] */ LPWSTR pwszUnmaskingList;
    ULONG ulFlags;
    VDS_LUN_TYPE type;
    VDS_LUN_STATUS status;
    VDS_HEALTH health;
    VDS_TRANSITION_STATE TransitionState;
    SHORT sRebuildPriority;
    } 	VDS_LUN_PROP;

typedef struct _VDS_LUN_PROP *PVDS_LUN_PROP;

typedef struct _VDS_LUN_PLEX_PROP
    {
    VDS_OBJECT_ID id;
    ULONGLONG ullSize;
    VDS_LUN_PLEX_TYPE type;
    VDS_LUN_PLEX_STATUS status;
    VDS_HEALTH health;
    VDS_TRANSITION_STATE TransitionState;
    ULONG ulFlags;
    ULONG ulStripeSize;
    SHORT sRebuildPriority;
    } 	VDS_LUN_PLEX_PROP;

typedef struct _VDS_LUN_PLEX_PROP *PVDS_LUN_PLEX_PROP;

typedef struct _VDS_PORT_PROP
    {
    VDS_OBJECT_ID id;
    /* [string] */ LPWSTR pwszFriendlyName;
    /* [string] */ LPWSTR pwszIdentification;
    VDS_PORT_STATUS status;
    } 	VDS_PORT_PROP;

typedef struct _VDS_PORT_PROP *PVDS_PORT_PROP;

typedef struct _VDS_ISCSI_PORTAL_PROP
    {
    VDS_OBJECT_ID id;
    VDS_IPADDRESS address;
    VDS_ISCSI_PORTAL_STATUS status;
    } 	VDS_ISCSI_PORTAL_PROP;

typedef struct _VDS_ISCSI_PORTAL_PROP *PVDS_ISCSI_PORTAL_PROP;

typedef struct _VDS_ISCSI_TARGET_PROP
    {
    VDS_OBJECT_ID id;
    /* [string] */ LPWSTR pwszIscsiName;
    /* [string] */ LPWSTR pwszFriendlyName;
    BOOL bChapEnabled;
    } 	VDS_ISCSI_TARGET_PROP;

typedef struct _VDS_ISCSI_TARGET_PROP *PVDS_ISCSI_TARGET_PROP;

typedef struct _VDS_ISCSI_PORTALGROUP_PROP
    {
    VDS_OBJECT_ID id;
    VDS_ISCSI_PORTALGROUP_TAG tag;
    } 	VDS_ISCSI_PORTALGROUP_PROP;

typedef struct _VDS_ISCSI_PORTALGROUP_PROP *PVDS_ISCSI_PORTALGROUP_PROP;

typedef 
enum _VDS_RAID_TYPE
    {
        VDS_RT_UNKNOWN	= 0,
        VDS_RT_RAID0	= 10,
        VDS_RT_RAID1	= 11,
        VDS_RT_RAID2	= 12,
        VDS_RT_RAID3	= 13,
        VDS_RT_RAID4	= 14,
        VDS_RT_RAID5	= 15,
        VDS_RT_RAID6	= 16,
        VDS_RT_RAID01	= 17,
        VDS_RT_RAID03	= 18,
        VDS_RT_RAID05	= 19,
        VDS_RT_RAID10	= 20,
        VDS_RT_RAID15	= 21,
        VDS_RT_RAID30	= 22,
        VDS_RT_RAID50	= 23,
        VDS_RT_RAID51	= 24,
        VDS_RT_RAID53	= 25,
        VDS_RT_RAID60	= 26,
        VDS_RT_RAID61	= 27
    } 	VDS_RAID_TYPE;

typedef enum _VDS_RAID_TYPE *PVDS_RAID_TYPE;

typedef struct _VDS_POOL_CUSTOM_ATTRIBUTES
    {
    LPWSTR pwszName;
    LPWSTR pwszValue;
    } 	VDS_POOL_CUSTOM_ATTRIBUTES;

typedef struct _VDS_POOL_CUSTOM_ATTRIBUTES *PVDS_POOL_CUSTOM_ATTRIBUTES;

typedef struct _VDS_POOL_ATTRIBUTES
    {
    ULONGLONG ullAttributeMask;
    VDS_RAID_TYPE raidType;
    VDS_STORAGE_BUS_TYPE busType;
    LPWSTR pwszIntendedUsage;
    BOOL bSpinDown;
    BOOL bIsThinProvisioned;
    ULONGLONG ullProvisionedSpace;
    BOOL bNoSinglePointOfFailure;
    ULONG ulDataRedundancyMax;
    ULONG ulDataRedundancyMin;
    ULONG ulDataRedundancyDefault;
    ULONG ulPackageRedundancyMax;
    ULONG ulPackageRedundancyMin;
    ULONG ulPackageRedundancyDefault;
    ULONG ulStripeSize;
    ULONG ulStripeSizeMax;
    ULONG ulStripeSizeMin;
    ULONG ulDefaultStripeSize;
    ULONG ulNumberOfColumns;
    ULONG ulNumberOfColumnsMax;
    ULONG ulNumberOfColumnsMin;
    ULONG ulDefaultNumberofColumns;
    ULONG ulDataAvailabilityHint;
    ULONG ulAccessRandomnessHint;
    ULONG ulAccessDirectionHint;
    ULONG ulAccessSizeHint;
    ULONG ulAccessLatencyHint;
    ULONG ulAccessBandwidthWeightHint;
    ULONG ulStorageCostHint;
    ULONG ulStorageEfficiencyHint;
    ULONG ulNumOfCustomAttributes;
    /* [size_is] */ VDS_POOL_CUSTOM_ATTRIBUTES *pPoolCustomAttributes;
    BOOL bReserved1;
    BOOL bReserved2;
    ULONG ulReserved1;
    ULONG ulReserved2;
    ULONGLONG ullReserved1;
    ULONGLONG ullReserved2;
    } 	VDS_POOL_ATTRIBUTES;

typedef struct _VDS_POOL_ATTRIBUTES *PVDS_POOL_ATTRIBUTES;

#define	VDS_POOL_ATTRIB_RAIDTYPE	( 0x1L )

#define	VDS_POOL_ATTRIB_BUSTYPE	( 0x2L )

#define	VDS_POOL_ATTRIB_ALLOW_SPINDOWN	( 0x4L )

#define	VDS_POOL_ATTRIB_THIN_PROVISION	( 0x8L )

#define	VDS_POOL_ATTRIB_NO_SINGLE_POF	( 0x10L )

#define	VDS_POOL_ATTRIB_DATA_RDNCY_MAX	( 0x20L )

#define	VDS_POOL_ATTRIB_DATA_RDNCY_MIN	( 0x40L )

#define	VDS_POOL_ATTRIB_DATA_RDNCY_DEF	( 0x80L )

#define	VDS_POOL_ATTRIB_PKG_RDNCY_MAX	( 0x100L )

#define	VDS_POOL_ATTRIB_PKG_RDNCY_MIN	( 0x200L )

#define	VDS_POOL_ATTRIB_PKG_RDNCY_DEF	( 0x400L )

#define	VDS_POOL_ATTRIB_STRIPE_SIZE	( 0x800L )

#define	VDS_POOL_ATTRIB_STRIPE_SIZE_MAX	( 0x1000L )

#define	VDS_POOL_ATTRIB_STRIPE_SIZE_MIN	( 0x2000L )

#define	VDS_POOL_ATTRIB_STRIPE_SIZE_DEF	( 0x4000L )

#define	VDS_POOL_ATTRIB_NUM_CLMNS	( 0x8000L )

#define	VDS_POOL_ATTRIB_NUM_CLMNS_MAX	( 0x10000L )

#define	VDS_POOL_ATTRIB_NUM_CLMNS_MIN	( 0x20000L )

#define	VDS_POOL_ATTRIB_NUM_CLMNS_DEF	( 0x40000L )

#define	VDS_POOL_ATTRIB_DATA_AVL_HINT	( 0x80000L )

#define	VDS_POOL_ATTRIB_ACCS_RNDM_HINT	( 0x100000L )

#define	VDS_POOL_ATTRIB_ACCS_DIR_HINT	( 0x200000L )

#define	VDS_POOL_ATTRIB_ACCS_SIZE_HINT	( 0x400000L )

#define	VDS_POOL_ATTRIB_ACCS_LTNCY_HINT	( 0x800000L )

#define	VDS_POOL_ATTRIB_ACCS_BDW_WT_HINT	( 0x1000000L )

#define	VDS_POOL_ATTRIB_STOR_COST_HINT	( 0x2000000L )

#define	VDS_POOL_ATTRIB_STOR_EFFCY_HINT	( 0x4000000L )

#define	VDS_POOL_ATTRIB_CUSTOM_ATTRIB	( 0x8000000L )

typedef struct _VDS_STORAGE_POOL_PROP
    {
    VDS_OBJECT_ID id;
    VDS_STORAGE_POOL_STATUS status;
    VDS_HEALTH health;
    VDS_STORAGE_POOL_TYPE type;
    /* [string] */ LPWSTR pwszName;
    /* [string] */ LPWSTR pwszDescription;
    ULONGLONG ullTotalConsumedSpace;
    ULONGLONG ullTotalManagedSpace;
    ULONGLONG ullRemainingFreeSpace;
    } 	VDS_STORAGE_POOL_PROP;

typedef struct _VDS_STORAGE_POOL_PROP *PVDS_STORAGE_POOL_PROP;

typedef struct _VDS_STORAGE_POOL_DRIVE_EXTENT
    {
    VDS_OBJECT_ID id;
    ULONGLONG ullSize;
    BOOL bUsed;
    } 	VDS_STORAGE_POOL_DRIVE_EXTENT;

typedef struct _VDS_STORAGE_POOL_DRIVE_EXTENT *PVDS_STORAGE_POOL_DRIVE_EXTENT;



extern RPC_IF_HANDLE __MIDL_itf_vdshwprv_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_vdshwprv_0000_0006_v0_0_s_ifspec;

#ifndef __IVdsHwProvider_INTERFACE_DEFINED__
#define __IVdsHwProvider_INTERFACE_DEFINED__

/* interface IVdsHwProvider */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsHwProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d99bdaae-b13a-4178-9fdb-e27f16b4603e")
    IVdsHwProvider : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QuerySubSystems( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Reenumerate( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsHwProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsHwProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsHwProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsHwProvider * This);
        
        DECLSPEC_XFGVIRT(IVdsHwProvider, QuerySubSystems)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QuerySubSystems )( 
            __RPC__in IVdsHwProvider * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVdsHwProvider, Reenumerate)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Reenumerate )( 
            __RPC__in IVdsHwProvider * This);
        
        DECLSPEC_XFGVIRT(IVdsHwProvider, Refresh)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IVdsHwProvider * This);
        
        END_INTERFACE
    } IVdsHwProviderVtbl;

    interface IVdsHwProvider
    {
        CONST_VTBL struct IVdsHwProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsHwProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsHwProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsHwProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsHwProvider_QuerySubSystems(This,ppEnum)	\
    ( (This)->lpVtbl -> QuerySubSystems(This,ppEnum) ) 

#define IVdsHwProvider_Reenumerate(This)	\
    ( (This)->lpVtbl -> Reenumerate(This) ) 

#define IVdsHwProvider_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsHwProvider_INTERFACE_DEFINED__ */


#ifndef __IVdsHwProviderType_INTERFACE_DEFINED__
#define __IVdsHwProviderType_INTERFACE_DEFINED__

/* interface IVdsHwProviderType */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsHwProviderType;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3e0f5166-542d-4fc6-947a-012174240b7e")
    IVdsHwProviderType : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProviderType( 
            /* [out] */ __RPC__out VDS_HWPROVIDER_TYPE *pType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsHwProviderTypeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsHwProviderType * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsHwProviderType * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsHwProviderType * This);
        
        DECLSPEC_XFGVIRT(IVdsHwProviderType, GetProviderType)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProviderType )( 
            __RPC__in IVdsHwProviderType * This,
            /* [out] */ __RPC__out VDS_HWPROVIDER_TYPE *pType);
        
        END_INTERFACE
    } IVdsHwProviderTypeVtbl;

    interface IVdsHwProviderType
    {
        CONST_VTBL struct IVdsHwProviderTypeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsHwProviderType_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsHwProviderType_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsHwProviderType_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsHwProviderType_GetProviderType(This,pType)	\
    ( (This)->lpVtbl -> GetProviderType(This,pType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsHwProviderType_INTERFACE_DEFINED__ */


#ifndef __IVdsHwProviderType2_INTERFACE_DEFINED__
#define __IVdsHwProviderType2_INTERFACE_DEFINED__

/* interface IVdsHwProviderType2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsHwProviderType2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8190236f-c4d0-4e81-8011-d69512fcc984")
    IVdsHwProviderType2 : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProviderType2( 
            /* [out] */ __RPC__out VDS_HWPROVIDER_TYPE *pType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsHwProviderType2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsHwProviderType2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsHwProviderType2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsHwProviderType2 * This);
        
        DECLSPEC_XFGVIRT(IVdsHwProviderType2, GetProviderType2)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProviderType2 )( 
            __RPC__in IVdsHwProviderType2 * This,
            /* [out] */ __RPC__out VDS_HWPROVIDER_TYPE *pType);
        
        END_INTERFACE
    } IVdsHwProviderType2Vtbl;

    interface IVdsHwProviderType2
    {
        CONST_VTBL struct IVdsHwProviderType2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsHwProviderType2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsHwProviderType2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsHwProviderType2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsHwProviderType2_GetProviderType2(This,pType)	\
    ( (This)->lpVtbl -> GetProviderType2(This,pType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsHwProviderType2_INTERFACE_DEFINED__ */


#ifndef __IVdsHwProviderStoragePools_INTERFACE_DEFINED__
#define __IVdsHwProviderStoragePools_INTERFACE_DEFINED__

/* interface IVdsHwProviderStoragePools */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsHwProviderStoragePools;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d5b5937a-f188-4c79-b86c-11c920ad11b8")
    IVdsHwProviderStoragePools : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryStoragePools( 
            /* [in] */ ULONG ulFlags,
            /* [in] */ ULONGLONG ullRemainingFreeSpace,
            /* [unique][in] */ __RPC__in_opt VDS_POOL_ATTRIBUTES *pPoolAttributes,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateLunInStoragePool( 
            /* [in] */ VDS_LUN_TYPE type,
            /* [in] */ ULONGLONG ullSizeInBytes,
            /* [in] */ VDS_OBJECT_ID StoragePoolId,
            /* [string][in] */ __RPC__in_string LPWSTR pwszUnmaskingList,
            /* [unique][in] */ __RPC__in_opt VDS_HINTS2 *pHints2,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryMaxLunCreateSizeInStoragePool( 
            /* [in] */ VDS_LUN_TYPE type,
            /* [in] */ VDS_OBJECT_ID StoragePoolId,
            /* [unique][in] */ __RPC__in_opt VDS_HINTS2 *pHints2,
            /* [out] */ __RPC__out ULONGLONG *pullMaxLunSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsHwProviderStoragePoolsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsHwProviderStoragePools * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsHwProviderStoragePools * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsHwProviderStoragePools * This);
        
        DECLSPEC_XFGVIRT(IVdsHwProviderStoragePools, QueryStoragePools)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryStoragePools )( 
            __RPC__in IVdsHwProviderStoragePools * This,
            /* [in] */ ULONG ulFlags,
            /* [in] */ ULONGLONG ullRemainingFreeSpace,
            /* [unique][in] */ __RPC__in_opt VDS_POOL_ATTRIBUTES *pPoolAttributes,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVdsHwProviderStoragePools, CreateLunInStoragePool)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateLunInStoragePool )( 
            __RPC__in IVdsHwProviderStoragePools * This,
            /* [in] */ VDS_LUN_TYPE type,
            /* [in] */ ULONGLONG ullSizeInBytes,
            /* [in] */ VDS_OBJECT_ID StoragePoolId,
            /* [string][in] */ __RPC__in_string LPWSTR pwszUnmaskingList,
            /* [unique][in] */ __RPC__in_opt VDS_HINTS2 *pHints2,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync);
        
        DECLSPEC_XFGVIRT(IVdsHwProviderStoragePools, QueryMaxLunCreateSizeInStoragePool)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryMaxLunCreateSizeInStoragePool )( 
            __RPC__in IVdsHwProviderStoragePools * This,
            /* [in] */ VDS_LUN_TYPE type,
            /* [in] */ VDS_OBJECT_ID StoragePoolId,
            /* [unique][in] */ __RPC__in_opt VDS_HINTS2 *pHints2,
            /* [out] */ __RPC__out ULONGLONG *pullMaxLunSize);
        
        END_INTERFACE
    } IVdsHwProviderStoragePoolsVtbl;

    interface IVdsHwProviderStoragePools
    {
        CONST_VTBL struct IVdsHwProviderStoragePoolsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsHwProviderStoragePools_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsHwProviderStoragePools_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsHwProviderStoragePools_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsHwProviderStoragePools_QueryStoragePools(This,ulFlags,ullRemainingFreeSpace,pPoolAttributes,ppEnum)	\
    ( (This)->lpVtbl -> QueryStoragePools(This,ulFlags,ullRemainingFreeSpace,pPoolAttributes,ppEnum) ) 

#define IVdsHwProviderStoragePools_CreateLunInStoragePool(This,type,ullSizeInBytes,StoragePoolId,pwszUnmaskingList,pHints2,ppAsync)	\
    ( (This)->lpVtbl -> CreateLunInStoragePool(This,type,ullSizeInBytes,StoragePoolId,pwszUnmaskingList,pHints2,ppAsync) ) 

#define IVdsHwProviderStoragePools_QueryMaxLunCreateSizeInStoragePool(This,type,StoragePoolId,pHints2,pullMaxLunSize)	\
    ( (This)->lpVtbl -> QueryMaxLunCreateSizeInStoragePool(This,type,StoragePoolId,pHints2,pullMaxLunSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsHwProviderStoragePools_INTERFACE_DEFINED__ */


#ifndef __IVdsSubSystem_INTERFACE_DEFINED__
#define __IVdsSubSystem_INTERFACE_DEFINED__

/* interface IVdsSubSystem */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsSubSystem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6fcee2d3-6d90-4f91-80e2-a5c7caaca9d8")
    IVdsSubSystem : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [out] */ __RPC__out VDS_SUB_SYSTEM_PROP *pSubSystemProp) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProvider( 
            /* [out] */ __RPC__deref_out_opt IVdsProvider **ppProvider) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryControllers( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryLuns( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryDrives( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetDrive( 
            /* [in] */ SHORT sBusNumber,
            /* [in] */ SHORT sSlotNumber,
            /* [out] */ __RPC__deref_out_opt IVdsDrive **ppDrive) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Reenumerate( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetControllerStatus( 
            /* [size_is][in] */ __RPC__in_ecount_full(lNumberOfOnlineControllers) VDS_OBJECT_ID *pOnlineControllerIdArray,
            /* [in] */ LONG lNumberOfOnlineControllers,
            /* [size_is][in] */ __RPC__in_ecount_full(lNumberOfOfflineControllers) VDS_OBJECT_ID *pOfflineControllerIdArray,
            /* [in] */ LONG lNumberOfOfflineControllers) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateLun( 
            /* [in] */ VDS_LUN_TYPE type,
            /* [in] */ ULONGLONG ullSizeInBytes,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(lNumberOfDrives) VDS_OBJECT_ID *pDriveIdArray,
            /* [in] */ LONG lNumberOfDrives,
            /* [string][in] */ __RPC__in_string LPWSTR pwszUnmaskingList,
            /* [unique][in] */ __RPC__in_opt VDS_HINTS *pHints,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ReplaceDrive( 
            /* [in] */ VDS_OBJECT_ID DriveToBeReplaced,
            /* [in] */ VDS_OBJECT_ID ReplacementDrive) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetStatus( 
            /* [in] */ VDS_SUB_SYSTEM_STATUS status) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryMaxLunCreateSize( 
            /* [in] */ VDS_LUN_TYPE type,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(lNumberOfDrives) VDS_OBJECT_ID *pDriveIdArray,
            /* [in] */ LONG lNumberOfDrives,
            /* [unique][in] */ __RPC__in_opt VDS_HINTS *pHints,
            /* [out] */ __RPC__out ULONGLONG *pullMaxLunSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsSubSystemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsSubSystem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsSubSystem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsSubSystem * This);
        
        DECLSPEC_XFGVIRT(IVdsSubSystem, GetProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in IVdsSubSystem * This,
            /* [out] */ __RPC__out VDS_SUB_SYSTEM_PROP *pSubSystemProp);
        
        DECLSPEC_XFGVIRT(IVdsSubSystem, GetProvider)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProvider )( 
            __RPC__in IVdsSubSystem * This,
            /* [out] */ __RPC__deref_out_opt IVdsProvider **ppProvider);
        
        DECLSPEC_XFGVIRT(IVdsSubSystem, QueryControllers)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryControllers )( 
            __RPC__in IVdsSubSystem * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVdsSubSystem, QueryLuns)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryLuns )( 
            __RPC__in IVdsSubSystem * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVdsSubSystem, QueryDrives)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryDrives )( 
            __RPC__in IVdsSubSystem * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVdsSubSystem, GetDrive)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDrive )( 
            __RPC__in IVdsSubSystem * This,
            /* [in] */ SHORT sBusNumber,
            /* [in] */ SHORT sSlotNumber,
            /* [out] */ __RPC__deref_out_opt IVdsDrive **ppDrive);
        
        DECLSPEC_XFGVIRT(IVdsSubSystem, Reenumerate)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Reenumerate )( 
            __RPC__in IVdsSubSystem * This);
        
        DECLSPEC_XFGVIRT(IVdsSubSystem, SetControllerStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetControllerStatus )( 
            __RPC__in IVdsSubSystem * This,
            /* [size_is][in] */ __RPC__in_ecount_full(lNumberOfOnlineControllers) VDS_OBJECT_ID *pOnlineControllerIdArray,
            /* [in] */ LONG lNumberOfOnlineControllers,
            /* [size_is][in] */ __RPC__in_ecount_full(lNumberOfOfflineControllers) VDS_OBJECT_ID *pOfflineControllerIdArray,
            /* [in] */ LONG lNumberOfOfflineControllers);
        
        DECLSPEC_XFGVIRT(IVdsSubSystem, CreateLun)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateLun )( 
            __RPC__in IVdsSubSystem * This,
            /* [in] */ VDS_LUN_TYPE type,
            /* [in] */ ULONGLONG ullSizeInBytes,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(lNumberOfDrives) VDS_OBJECT_ID *pDriveIdArray,
            /* [in] */ LONG lNumberOfDrives,
            /* [string][in] */ __RPC__in_string LPWSTR pwszUnmaskingList,
            /* [unique][in] */ __RPC__in_opt VDS_HINTS *pHints,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync);
        
        DECLSPEC_XFGVIRT(IVdsSubSystem, ReplaceDrive)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ReplaceDrive )( 
            __RPC__in IVdsSubSystem * This,
            /* [in] */ VDS_OBJECT_ID DriveToBeReplaced,
            /* [in] */ VDS_OBJECT_ID ReplacementDrive);
        
        DECLSPEC_XFGVIRT(IVdsSubSystem, SetStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetStatus )( 
            __RPC__in IVdsSubSystem * This,
            /* [in] */ VDS_SUB_SYSTEM_STATUS status);
        
        DECLSPEC_XFGVIRT(IVdsSubSystem, QueryMaxLunCreateSize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryMaxLunCreateSize )( 
            __RPC__in IVdsSubSystem * This,
            /* [in] */ VDS_LUN_TYPE type,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(lNumberOfDrives) VDS_OBJECT_ID *pDriveIdArray,
            /* [in] */ LONG lNumberOfDrives,
            /* [unique][in] */ __RPC__in_opt VDS_HINTS *pHints,
            /* [out] */ __RPC__out ULONGLONG *pullMaxLunSize);
        
        END_INTERFACE
    } IVdsSubSystemVtbl;

    interface IVdsSubSystem
    {
        CONST_VTBL struct IVdsSubSystemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsSubSystem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsSubSystem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsSubSystem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsSubSystem_GetProperties(This,pSubSystemProp)	\
    ( (This)->lpVtbl -> GetProperties(This,pSubSystemProp) ) 

#define IVdsSubSystem_GetProvider(This,ppProvider)	\
    ( (This)->lpVtbl -> GetProvider(This,ppProvider) ) 

#define IVdsSubSystem_QueryControllers(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryControllers(This,ppEnum) ) 

#define IVdsSubSystem_QueryLuns(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryLuns(This,ppEnum) ) 

#define IVdsSubSystem_QueryDrives(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryDrives(This,ppEnum) ) 

#define IVdsSubSystem_GetDrive(This,sBusNumber,sSlotNumber,ppDrive)	\
    ( (This)->lpVtbl -> GetDrive(This,sBusNumber,sSlotNumber,ppDrive) ) 

#define IVdsSubSystem_Reenumerate(This)	\
    ( (This)->lpVtbl -> Reenumerate(This) ) 

#define IVdsSubSystem_SetControllerStatus(This,pOnlineControllerIdArray,lNumberOfOnlineControllers,pOfflineControllerIdArray,lNumberOfOfflineControllers)	\
    ( (This)->lpVtbl -> SetControllerStatus(This,pOnlineControllerIdArray,lNumberOfOnlineControllers,pOfflineControllerIdArray,lNumberOfOfflineControllers) ) 

#define IVdsSubSystem_CreateLun(This,type,ullSizeInBytes,pDriveIdArray,lNumberOfDrives,pwszUnmaskingList,pHints,ppAsync)	\
    ( (This)->lpVtbl -> CreateLun(This,type,ullSizeInBytes,pDriveIdArray,lNumberOfDrives,pwszUnmaskingList,pHints,ppAsync) ) 

#define IVdsSubSystem_ReplaceDrive(This,DriveToBeReplaced,ReplacementDrive)	\
    ( (This)->lpVtbl -> ReplaceDrive(This,DriveToBeReplaced,ReplacementDrive) ) 

#define IVdsSubSystem_SetStatus(This,status)	\
    ( (This)->lpVtbl -> SetStatus(This,status) ) 

#define IVdsSubSystem_QueryMaxLunCreateSize(This,type,pDriveIdArray,lNumberOfDrives,pHints,pullMaxLunSize)	\
    ( (This)->lpVtbl -> QueryMaxLunCreateSize(This,type,pDriveIdArray,lNumberOfDrives,pHints,pullMaxLunSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsSubSystem_INTERFACE_DEFINED__ */


#ifndef __IVdsSubSystem2_INTERFACE_DEFINED__
#define __IVdsSubSystem2_INTERFACE_DEFINED__

/* interface IVdsSubSystem2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsSubSystem2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("be666735-7800-4a77-9d9c-40f85b87e292")
    IVdsSubSystem2 : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperties2( 
            /* [out] */ __RPC__out VDS_SUB_SYSTEM_PROP2 *pSubSystemProp2) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetDrive2( 
            /* [in] */ SHORT sBusNumber,
            /* [in] */ SHORT sSlotNumber,
            /* [in] */ ULONG ulEnclosureNumber,
            /* [out] */ __RPC__deref_out_opt IVdsDrive **ppDrive) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateLun2( 
            /* [in] */ VDS_LUN_TYPE type,
            /* [in] */ ULONGLONG ullSizeInBytes,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(lNumberOfDrives) VDS_OBJECT_ID *pDriveIdArray,
            /* [in] */ LONG lNumberOfDrives,
            /* [string][in] */ __RPC__in_string LPWSTR pwszUnmaskingList,
            /* [unique][in] */ __RPC__in_opt VDS_HINTS2 *pHints2,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryMaxLunCreateSize2( 
            /* [in] */ VDS_LUN_TYPE type,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(lNumberOfDrives) VDS_OBJECT_ID *pDriveIdArray,
            /* [in] */ LONG lNumberOfDrives,
            /* [unique][in] */ __RPC__in_opt VDS_HINTS2 *pHints2,
            /* [out] */ __RPC__out ULONGLONG *pullMaxLunSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsSubSystem2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsSubSystem2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsSubSystem2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsSubSystem2 * This);
        
        DECLSPEC_XFGVIRT(IVdsSubSystem2, GetProperties2)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperties2 )( 
            __RPC__in IVdsSubSystem2 * This,
            /* [out] */ __RPC__out VDS_SUB_SYSTEM_PROP2 *pSubSystemProp2);
        
        DECLSPEC_XFGVIRT(IVdsSubSystem2, GetDrive2)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDrive2 )( 
            __RPC__in IVdsSubSystem2 * This,
            /* [in] */ SHORT sBusNumber,
            /* [in] */ SHORT sSlotNumber,
            /* [in] */ ULONG ulEnclosureNumber,
            /* [out] */ __RPC__deref_out_opt IVdsDrive **ppDrive);
        
        DECLSPEC_XFGVIRT(IVdsSubSystem2, CreateLun2)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateLun2 )( 
            __RPC__in IVdsSubSystem2 * This,
            /* [in] */ VDS_LUN_TYPE type,
            /* [in] */ ULONGLONG ullSizeInBytes,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(lNumberOfDrives) VDS_OBJECT_ID *pDriveIdArray,
            /* [in] */ LONG lNumberOfDrives,
            /* [string][in] */ __RPC__in_string LPWSTR pwszUnmaskingList,
            /* [unique][in] */ __RPC__in_opt VDS_HINTS2 *pHints2,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync);
        
        DECLSPEC_XFGVIRT(IVdsSubSystem2, QueryMaxLunCreateSize2)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryMaxLunCreateSize2 )( 
            __RPC__in IVdsSubSystem2 * This,
            /* [in] */ VDS_LUN_TYPE type,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(lNumberOfDrives) VDS_OBJECT_ID *pDriveIdArray,
            /* [in] */ LONG lNumberOfDrives,
            /* [unique][in] */ __RPC__in_opt VDS_HINTS2 *pHints2,
            /* [out] */ __RPC__out ULONGLONG *pullMaxLunSize);
        
        END_INTERFACE
    } IVdsSubSystem2Vtbl;

    interface IVdsSubSystem2
    {
        CONST_VTBL struct IVdsSubSystem2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsSubSystem2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsSubSystem2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsSubSystem2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsSubSystem2_GetProperties2(This,pSubSystemProp2)	\
    ( (This)->lpVtbl -> GetProperties2(This,pSubSystemProp2) ) 

#define IVdsSubSystem2_GetDrive2(This,sBusNumber,sSlotNumber,ulEnclosureNumber,ppDrive)	\
    ( (This)->lpVtbl -> GetDrive2(This,sBusNumber,sSlotNumber,ulEnclosureNumber,ppDrive) ) 

#define IVdsSubSystem2_CreateLun2(This,type,ullSizeInBytes,pDriveIdArray,lNumberOfDrives,pwszUnmaskingList,pHints2,ppAsync)	\
    ( (This)->lpVtbl -> CreateLun2(This,type,ullSizeInBytes,pDriveIdArray,lNumberOfDrives,pwszUnmaskingList,pHints2,ppAsync) ) 

#define IVdsSubSystem2_QueryMaxLunCreateSize2(This,type,pDriveIdArray,lNumberOfDrives,pHints2,pullMaxLunSize)	\
    ( (This)->lpVtbl -> QueryMaxLunCreateSize2(This,type,pDriveIdArray,lNumberOfDrives,pHints2,pullMaxLunSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsSubSystem2_INTERFACE_DEFINED__ */


#ifndef __IVdsSubSystemNaming_INTERFACE_DEFINED__
#define __IVdsSubSystemNaming_INTERFACE_DEFINED__

/* interface IVdsSubSystemNaming */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsSubSystemNaming;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0d70faa3-9cd4-4900-aa20-6981b6aafc75")
    IVdsSubSystemNaming : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetFriendlyName( 
            /* [string][in] */ __RPC__in_string LPWSTR pwszFriendlyName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsSubSystemNamingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsSubSystemNaming * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsSubSystemNaming * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsSubSystemNaming * This);
        
        DECLSPEC_XFGVIRT(IVdsSubSystemNaming, SetFriendlyName)
        HRESULT ( STDMETHODCALLTYPE *SetFriendlyName )( 
            __RPC__in IVdsSubSystemNaming * This,
            /* [string][in] */ __RPC__in_string LPWSTR pwszFriendlyName);
        
        END_INTERFACE
    } IVdsSubSystemNamingVtbl;

    interface IVdsSubSystemNaming
    {
        CONST_VTBL struct IVdsSubSystemNamingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsSubSystemNaming_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsSubSystemNaming_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsSubSystemNaming_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsSubSystemNaming_SetFriendlyName(This,pwszFriendlyName)	\
    ( (This)->lpVtbl -> SetFriendlyName(This,pwszFriendlyName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsSubSystemNaming_INTERFACE_DEFINED__ */


#ifndef __IVdsSubSystemIscsi_INTERFACE_DEFINED__
#define __IVdsSubSystemIscsi_INTERFACE_DEFINED__

/* interface IVdsSubSystemIscsi */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsSubSystemIscsi;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0027346f-40d0-4b45-8cec-5906dc0380c8")
    IVdsSubSystemIscsi : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryTargets( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryPortals( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateTarget( 
            /* [string][unique][in] */ __RPC__in_opt_string LPWSTR pwszIscsiName,
            /* [string][in] */ __RPC__in_string LPWSTR pwszFriendlyName,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetIpsecGroupPresharedKey( 
            /* [unique][in] */ __RPC__in_opt VDS_ISCSI_IPSEC_KEY *pIpsecKey) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsSubSystemIscsiVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsSubSystemIscsi * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsSubSystemIscsi * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsSubSystemIscsi * This);
        
        DECLSPEC_XFGVIRT(IVdsSubSystemIscsi, QueryTargets)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryTargets )( 
            __RPC__in IVdsSubSystemIscsi * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVdsSubSystemIscsi, QueryPortals)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryPortals )( 
            __RPC__in IVdsSubSystemIscsi * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVdsSubSystemIscsi, CreateTarget)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateTarget )( 
            __RPC__in IVdsSubSystemIscsi * This,
            /* [string][unique][in] */ __RPC__in_opt_string LPWSTR pwszIscsiName,
            /* [string][in] */ __RPC__in_string LPWSTR pwszFriendlyName,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync);
        
        DECLSPEC_XFGVIRT(IVdsSubSystemIscsi, SetIpsecGroupPresharedKey)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetIpsecGroupPresharedKey )( 
            __RPC__in IVdsSubSystemIscsi * This,
            /* [unique][in] */ __RPC__in_opt VDS_ISCSI_IPSEC_KEY *pIpsecKey);
        
        END_INTERFACE
    } IVdsSubSystemIscsiVtbl;

    interface IVdsSubSystemIscsi
    {
        CONST_VTBL struct IVdsSubSystemIscsiVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsSubSystemIscsi_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsSubSystemIscsi_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsSubSystemIscsi_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsSubSystemIscsi_QueryTargets(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryTargets(This,ppEnum) ) 

#define IVdsSubSystemIscsi_QueryPortals(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryPortals(This,ppEnum) ) 

#define IVdsSubSystemIscsi_CreateTarget(This,pwszIscsiName,pwszFriendlyName,ppAsync)	\
    ( (This)->lpVtbl -> CreateTarget(This,pwszIscsiName,pwszFriendlyName,ppAsync) ) 

#define IVdsSubSystemIscsi_SetIpsecGroupPresharedKey(This,pIpsecKey)	\
    ( (This)->lpVtbl -> SetIpsecGroupPresharedKey(This,pIpsecKey) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsSubSystemIscsi_INTERFACE_DEFINED__ */


#ifndef __IVdsSubSystemInterconnect_INTERFACE_DEFINED__
#define __IVdsSubSystemInterconnect_INTERFACE_DEFINED__

/* interface IVdsSubSystemInterconnect */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsSubSystemInterconnect;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9e6fa560-c141-477b-83ba-0b6c38f7febf")
    IVdsSubSystemInterconnect : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSupportedInterconnects( 
            /* [out] */ __RPC__out ULONG *pulSupportedInterconnectsFlag) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsSubSystemInterconnectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsSubSystemInterconnect * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsSubSystemInterconnect * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsSubSystemInterconnect * This);
        
        DECLSPEC_XFGVIRT(IVdsSubSystemInterconnect, GetSupportedInterconnects)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedInterconnects )( 
            __RPC__in IVdsSubSystemInterconnect * This,
            /* [out] */ __RPC__out ULONG *pulSupportedInterconnectsFlag);
        
        END_INTERFACE
    } IVdsSubSystemInterconnectVtbl;

    interface IVdsSubSystemInterconnect
    {
        CONST_VTBL struct IVdsSubSystemInterconnectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsSubSystemInterconnect_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsSubSystemInterconnect_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsSubSystemInterconnect_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsSubSystemInterconnect_GetSupportedInterconnects(This,pulSupportedInterconnectsFlag)	\
    ( (This)->lpVtbl -> GetSupportedInterconnects(This,pulSupportedInterconnectsFlag) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsSubSystemInterconnect_INTERFACE_DEFINED__ */


#ifndef __IVdsControllerPort_INTERFACE_DEFINED__
#define __IVdsControllerPort_INTERFACE_DEFINED__

/* interface IVdsControllerPort */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsControllerPort;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("18691d0d-4e7f-43e8-92e4-cf44beeed11c")
    IVdsControllerPort : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [out] */ __RPC__out VDS_PORT_PROP *pPortProp) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetController( 
            /* [out] */ __RPC__deref_out_opt IVdsController **ppController) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryAssociatedLuns( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetStatus( 
            /* [in] */ VDS_PORT_STATUS status) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsControllerPortVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsControllerPort * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsControllerPort * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsControllerPort * This);
        
        DECLSPEC_XFGVIRT(IVdsControllerPort, GetProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in IVdsControllerPort * This,
            /* [out] */ __RPC__out VDS_PORT_PROP *pPortProp);
        
        DECLSPEC_XFGVIRT(IVdsControllerPort, GetController)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetController )( 
            __RPC__in IVdsControllerPort * This,
            /* [out] */ __RPC__deref_out_opt IVdsController **ppController);
        
        DECLSPEC_XFGVIRT(IVdsControllerPort, QueryAssociatedLuns)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryAssociatedLuns )( 
            __RPC__in IVdsControllerPort * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVdsControllerPort, Reset)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IVdsControllerPort * This);
        
        DECLSPEC_XFGVIRT(IVdsControllerPort, SetStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetStatus )( 
            __RPC__in IVdsControllerPort * This,
            /* [in] */ VDS_PORT_STATUS status);
        
        END_INTERFACE
    } IVdsControllerPortVtbl;

    interface IVdsControllerPort
    {
        CONST_VTBL struct IVdsControllerPortVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsControllerPort_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsControllerPort_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsControllerPort_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsControllerPort_GetProperties(This,pPortProp)	\
    ( (This)->lpVtbl -> GetProperties(This,pPortProp) ) 

#define IVdsControllerPort_GetController(This,ppController)	\
    ( (This)->lpVtbl -> GetController(This,ppController) ) 

#define IVdsControllerPort_QueryAssociatedLuns(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryAssociatedLuns(This,ppEnum) ) 

#define IVdsControllerPort_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IVdsControllerPort_SetStatus(This,status)	\
    ( (This)->lpVtbl -> SetStatus(This,status) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsControllerPort_INTERFACE_DEFINED__ */


#ifndef __IVdsController_INTERFACE_DEFINED__
#define __IVdsController_INTERFACE_DEFINED__

/* interface IVdsController */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsController;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("cb53d96e-dffb-474a-a078-790d1e2bc082")
    IVdsController : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [out] */ __RPC__out VDS_CONTROLLER_PROP *pControllerProp) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSubSystem( 
            /* [out] */ __RPC__deref_out_opt IVdsSubSystem **ppSubSystem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPortProperties( 
            /* [in] */ SHORT sPortNumber,
            /* [out] */ __RPC__out VDS_PORT_PROP *pPortProp) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE FlushCache( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE InvalidateCache( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryAssociatedLuns( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetStatus( 
            /* [in] */ VDS_CONTROLLER_STATUS status) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsControllerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsController * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsController * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsController * This);
        
        DECLSPEC_XFGVIRT(IVdsController, GetProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in IVdsController * This,
            /* [out] */ __RPC__out VDS_CONTROLLER_PROP *pControllerProp);
        
        DECLSPEC_XFGVIRT(IVdsController, GetSubSystem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSubSystem )( 
            __RPC__in IVdsController * This,
            /* [out] */ __RPC__deref_out_opt IVdsSubSystem **ppSubSystem);
        
        DECLSPEC_XFGVIRT(IVdsController, GetPortProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPortProperties )( 
            __RPC__in IVdsController * This,
            /* [in] */ SHORT sPortNumber,
            /* [out] */ __RPC__out VDS_PORT_PROP *pPortProp);
        
        DECLSPEC_XFGVIRT(IVdsController, FlushCache)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FlushCache )( 
            __RPC__in IVdsController * This);
        
        DECLSPEC_XFGVIRT(IVdsController, InvalidateCache)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InvalidateCache )( 
            __RPC__in IVdsController * This);
        
        DECLSPEC_XFGVIRT(IVdsController, Reset)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IVdsController * This);
        
        DECLSPEC_XFGVIRT(IVdsController, QueryAssociatedLuns)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryAssociatedLuns )( 
            __RPC__in IVdsController * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVdsController, SetStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetStatus )( 
            __RPC__in IVdsController * This,
            /* [in] */ VDS_CONTROLLER_STATUS status);
        
        END_INTERFACE
    } IVdsControllerVtbl;

    interface IVdsController
    {
        CONST_VTBL struct IVdsControllerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsController_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsController_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsController_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsController_GetProperties(This,pControllerProp)	\
    ( (This)->lpVtbl -> GetProperties(This,pControllerProp) ) 

#define IVdsController_GetSubSystem(This,ppSubSystem)	\
    ( (This)->lpVtbl -> GetSubSystem(This,ppSubSystem) ) 

#define IVdsController_GetPortProperties(This,sPortNumber,pPortProp)	\
    ( (This)->lpVtbl -> GetPortProperties(This,sPortNumber,pPortProp) ) 

#define IVdsController_FlushCache(This)	\
    ( (This)->lpVtbl -> FlushCache(This) ) 

#define IVdsController_InvalidateCache(This)	\
    ( (This)->lpVtbl -> InvalidateCache(This) ) 

#define IVdsController_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IVdsController_QueryAssociatedLuns(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryAssociatedLuns(This,ppEnum) ) 

#define IVdsController_SetStatus(This,status)	\
    ( (This)->lpVtbl -> SetStatus(This,status) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsController_INTERFACE_DEFINED__ */


#ifndef __IVdsControllerControllerPort_INTERFACE_DEFINED__
#define __IVdsControllerControllerPort_INTERFACE_DEFINED__

/* interface IVdsControllerControllerPort */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsControllerControllerPort;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ca5d735f-6bae-42c0-b30e-f2666045ce71")
    IVdsControllerControllerPort : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryControllerPorts( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsControllerControllerPortVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsControllerControllerPort * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsControllerControllerPort * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsControllerControllerPort * This);
        
        DECLSPEC_XFGVIRT(IVdsControllerControllerPort, QueryControllerPorts)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryControllerPorts )( 
            __RPC__in IVdsControllerControllerPort * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        END_INTERFACE
    } IVdsControllerControllerPortVtbl;

    interface IVdsControllerControllerPort
    {
        CONST_VTBL struct IVdsControllerControllerPortVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsControllerControllerPort_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsControllerControllerPort_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsControllerControllerPort_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsControllerControllerPort_QueryControllerPorts(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryControllerPorts(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsControllerControllerPort_INTERFACE_DEFINED__ */


#ifndef __IVdsDrive_INTERFACE_DEFINED__
#define __IVdsDrive_INTERFACE_DEFINED__

/* interface IVdsDrive */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsDrive;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ff24efa4-aade-4b6b-898b-eaa6a20887c7")
    IVdsDrive : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [out] */ __RPC__out VDS_DRIVE_PROP *pDriveProp) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSubSystem( 
            /* [out] */ __RPC__deref_out_opt IVdsSubSystem **ppSubSystem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryExtents( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*plNumberOfExtents) VDS_DRIVE_EXTENT **ppExtentArray,
            /* [out] */ __RPC__out LONG *plNumberOfExtents) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetFlags( 
            /* [in] */ ULONG ulFlags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ClearFlags( 
            /* [in] */ ULONG ulFlags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetStatus( 
            /* [in] */ VDS_DRIVE_STATUS status) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsDriveVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsDrive * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsDrive * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsDrive * This);
        
        DECLSPEC_XFGVIRT(IVdsDrive, GetProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in IVdsDrive * This,
            /* [out] */ __RPC__out VDS_DRIVE_PROP *pDriveProp);
        
        DECLSPEC_XFGVIRT(IVdsDrive, GetSubSystem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSubSystem )( 
            __RPC__in IVdsDrive * This,
            /* [out] */ __RPC__deref_out_opt IVdsSubSystem **ppSubSystem);
        
        DECLSPEC_XFGVIRT(IVdsDrive, QueryExtents)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryExtents )( 
            __RPC__in IVdsDrive * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*plNumberOfExtents) VDS_DRIVE_EXTENT **ppExtentArray,
            /* [out] */ __RPC__out LONG *plNumberOfExtents);
        
        DECLSPEC_XFGVIRT(IVdsDrive, SetFlags)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetFlags )( 
            __RPC__in IVdsDrive * This,
            /* [in] */ ULONG ulFlags);
        
        DECLSPEC_XFGVIRT(IVdsDrive, ClearFlags)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ClearFlags )( 
            __RPC__in IVdsDrive * This,
            /* [in] */ ULONG ulFlags);
        
        DECLSPEC_XFGVIRT(IVdsDrive, SetStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetStatus )( 
            __RPC__in IVdsDrive * This,
            /* [in] */ VDS_DRIVE_STATUS status);
        
        END_INTERFACE
    } IVdsDriveVtbl;

    interface IVdsDrive
    {
        CONST_VTBL struct IVdsDriveVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsDrive_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsDrive_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsDrive_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsDrive_GetProperties(This,pDriveProp)	\
    ( (This)->lpVtbl -> GetProperties(This,pDriveProp) ) 

#define IVdsDrive_GetSubSystem(This,ppSubSystem)	\
    ( (This)->lpVtbl -> GetSubSystem(This,ppSubSystem) ) 

#define IVdsDrive_QueryExtents(This,ppExtentArray,plNumberOfExtents)	\
    ( (This)->lpVtbl -> QueryExtents(This,ppExtentArray,plNumberOfExtents) ) 

#define IVdsDrive_SetFlags(This,ulFlags)	\
    ( (This)->lpVtbl -> SetFlags(This,ulFlags) ) 

#define IVdsDrive_ClearFlags(This,ulFlags)	\
    ( (This)->lpVtbl -> ClearFlags(This,ulFlags) ) 

#define IVdsDrive_SetStatus(This,status)	\
    ( (This)->lpVtbl -> SetStatus(This,status) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsDrive_INTERFACE_DEFINED__ */


#ifndef __IVdsDrive2_INTERFACE_DEFINED__
#define __IVdsDrive2_INTERFACE_DEFINED__

/* interface IVdsDrive2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsDrive2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("60b5a730-addf-4436-8ca7-5769e2d1ffa4")
    IVdsDrive2 : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperties2( 
            /* [out] */ __RPC__out VDS_DRIVE_PROP2 *pDriveProp2) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsDrive2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsDrive2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsDrive2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsDrive2 * This);
        
        DECLSPEC_XFGVIRT(IVdsDrive2, GetProperties2)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperties2 )( 
            __RPC__in IVdsDrive2 * This,
            /* [out] */ __RPC__out VDS_DRIVE_PROP2 *pDriveProp2);
        
        END_INTERFACE
    } IVdsDrive2Vtbl;

    interface IVdsDrive2
    {
        CONST_VTBL struct IVdsDrive2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsDrive2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsDrive2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsDrive2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsDrive2_GetProperties2(This,pDriveProp2)	\
    ( (This)->lpVtbl -> GetProperties2(This,pDriveProp2) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsDrive2_INTERFACE_DEFINED__ */


#ifndef __IVdsLun_INTERFACE_DEFINED__
#define __IVdsLun_INTERFACE_DEFINED__

/* interface IVdsLun */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsLun;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3540a9c7-e60f-4111-a840-8bba6c2c83d8")
    IVdsLun : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [out] */ __RPC__out VDS_LUN_PROP *pLunProp) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSubSystem( 
            /* [out] */ __RPC__deref_out_opt IVdsSubSystem **ppSubSystem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetIdentificationData( 
            /* [out] */ __RPC__out VDS_LUN_INFORMATION *pLunInfo) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryActiveControllers( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Extend( 
            /* [in] */ ULONGLONG ullNumberOfBytesToAdd,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(lNumberOfDrives) VDS_OBJECT_ID *pDriveIdArray,
            /* [in] */ LONG lNumberOfDrives,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Shrink( 
            /* [in] */ ULONGLONG ullNumberOfBytesToRemove,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryPlexes( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddPlex( 
            /* [in] */ VDS_OBJECT_ID lunId,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemovePlex( 
            /* [in] */ VDS_OBJECT_ID plexId,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Recover( 
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetMask( 
            /* [string][in] */ __RPC__in_string LPWSTR pwszUnmaskingList) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AssociateControllers( 
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(lNumberOfActiveControllers) VDS_OBJECT_ID *pActiveControllerIdArray,
            /* [in] */ LONG lNumberOfActiveControllers,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(lNumberOfInactiveControllers) VDS_OBJECT_ID *pInactiveControllerIdArray,
            /* [in] */ LONG lNumberOfInactiveControllers) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryHints( 
            /* [out] */ __RPC__out VDS_HINTS *pHints) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ApplyHints( 
            /* [in] */ __RPC__in VDS_HINTS *pHints) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetStatus( 
            /* [in] */ VDS_LUN_STATUS status) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryMaxLunExtendSize( 
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(lNumberOfDrives) VDS_OBJECT_ID *pDriveIdArray,
            /* [in] */ LONG lNumberOfDrives,
            /* [out] */ __RPC__out ULONGLONG *pullMaxBytesToBeAdded) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsLunVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsLun * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsLun * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsLun * This);
        
        DECLSPEC_XFGVIRT(IVdsLun, GetProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in IVdsLun * This,
            /* [out] */ __RPC__out VDS_LUN_PROP *pLunProp);
        
        DECLSPEC_XFGVIRT(IVdsLun, GetSubSystem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSubSystem )( 
            __RPC__in IVdsLun * This,
            /* [out] */ __RPC__deref_out_opt IVdsSubSystem **ppSubSystem);
        
        DECLSPEC_XFGVIRT(IVdsLun, GetIdentificationData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetIdentificationData )( 
            __RPC__in IVdsLun * This,
            /* [out] */ __RPC__out VDS_LUN_INFORMATION *pLunInfo);
        
        DECLSPEC_XFGVIRT(IVdsLun, QueryActiveControllers)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryActiveControllers )( 
            __RPC__in IVdsLun * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVdsLun, Extend)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Extend )( 
            __RPC__in IVdsLun * This,
            /* [in] */ ULONGLONG ullNumberOfBytesToAdd,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(lNumberOfDrives) VDS_OBJECT_ID *pDriveIdArray,
            /* [in] */ LONG lNumberOfDrives,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync);
        
        DECLSPEC_XFGVIRT(IVdsLun, Shrink)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Shrink )( 
            __RPC__in IVdsLun * This,
            /* [in] */ ULONGLONG ullNumberOfBytesToRemove,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync);
        
        DECLSPEC_XFGVIRT(IVdsLun, QueryPlexes)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryPlexes )( 
            __RPC__in IVdsLun * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVdsLun, AddPlex)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddPlex )( 
            __RPC__in IVdsLun * This,
            /* [in] */ VDS_OBJECT_ID lunId,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync);
        
        DECLSPEC_XFGVIRT(IVdsLun, RemovePlex)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemovePlex )( 
            __RPC__in IVdsLun * This,
            /* [in] */ VDS_OBJECT_ID plexId,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync);
        
        DECLSPEC_XFGVIRT(IVdsLun, Recover)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Recover )( 
            __RPC__in IVdsLun * This,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync);
        
        DECLSPEC_XFGVIRT(IVdsLun, SetMask)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetMask )( 
            __RPC__in IVdsLun * This,
            /* [string][in] */ __RPC__in_string LPWSTR pwszUnmaskingList);
        
        DECLSPEC_XFGVIRT(IVdsLun, Delete)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IVdsLun * This);
        
        DECLSPEC_XFGVIRT(IVdsLun, AssociateControllers)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AssociateControllers )( 
            __RPC__in IVdsLun * This,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(lNumberOfActiveControllers) VDS_OBJECT_ID *pActiveControllerIdArray,
            /* [in] */ LONG lNumberOfActiveControllers,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(lNumberOfInactiveControllers) VDS_OBJECT_ID *pInactiveControllerIdArray,
            /* [in] */ LONG lNumberOfInactiveControllers);
        
        DECLSPEC_XFGVIRT(IVdsLun, QueryHints)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryHints )( 
            __RPC__in IVdsLun * This,
            /* [out] */ __RPC__out VDS_HINTS *pHints);
        
        DECLSPEC_XFGVIRT(IVdsLun, ApplyHints)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ApplyHints )( 
            __RPC__in IVdsLun * This,
            /* [in] */ __RPC__in VDS_HINTS *pHints);
        
        DECLSPEC_XFGVIRT(IVdsLun, SetStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetStatus )( 
            __RPC__in IVdsLun * This,
            /* [in] */ VDS_LUN_STATUS status);
        
        DECLSPEC_XFGVIRT(IVdsLun, QueryMaxLunExtendSize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryMaxLunExtendSize )( 
            __RPC__in IVdsLun * This,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(lNumberOfDrives) VDS_OBJECT_ID *pDriveIdArray,
            /* [in] */ LONG lNumberOfDrives,
            /* [out] */ __RPC__out ULONGLONG *pullMaxBytesToBeAdded);
        
        END_INTERFACE
    } IVdsLunVtbl;

    interface IVdsLun
    {
        CONST_VTBL struct IVdsLunVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsLun_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsLun_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsLun_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsLun_GetProperties(This,pLunProp)	\
    ( (This)->lpVtbl -> GetProperties(This,pLunProp) ) 

#define IVdsLun_GetSubSystem(This,ppSubSystem)	\
    ( (This)->lpVtbl -> GetSubSystem(This,ppSubSystem) ) 

#define IVdsLun_GetIdentificationData(This,pLunInfo)	\
    ( (This)->lpVtbl -> GetIdentificationData(This,pLunInfo) ) 

#define IVdsLun_QueryActiveControllers(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryActiveControllers(This,ppEnum) ) 

#define IVdsLun_Extend(This,ullNumberOfBytesToAdd,pDriveIdArray,lNumberOfDrives,ppAsync)	\
    ( (This)->lpVtbl -> Extend(This,ullNumberOfBytesToAdd,pDriveIdArray,lNumberOfDrives,ppAsync) ) 

#define IVdsLun_Shrink(This,ullNumberOfBytesToRemove,ppAsync)	\
    ( (This)->lpVtbl -> Shrink(This,ullNumberOfBytesToRemove,ppAsync) ) 

#define IVdsLun_QueryPlexes(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryPlexes(This,ppEnum) ) 

#define IVdsLun_AddPlex(This,lunId,ppAsync)	\
    ( (This)->lpVtbl -> AddPlex(This,lunId,ppAsync) ) 

#define IVdsLun_RemovePlex(This,plexId,ppAsync)	\
    ( (This)->lpVtbl -> RemovePlex(This,plexId,ppAsync) ) 

#define IVdsLun_Recover(This,ppAsync)	\
    ( (This)->lpVtbl -> Recover(This,ppAsync) ) 

#define IVdsLun_SetMask(This,pwszUnmaskingList)	\
    ( (This)->lpVtbl -> SetMask(This,pwszUnmaskingList) ) 

#define IVdsLun_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IVdsLun_AssociateControllers(This,pActiveControllerIdArray,lNumberOfActiveControllers,pInactiveControllerIdArray,lNumberOfInactiveControllers)	\
    ( (This)->lpVtbl -> AssociateControllers(This,pActiveControllerIdArray,lNumberOfActiveControllers,pInactiveControllerIdArray,lNumberOfInactiveControllers) ) 

#define IVdsLun_QueryHints(This,pHints)	\
    ( (This)->lpVtbl -> QueryHints(This,pHints) ) 

#define IVdsLun_ApplyHints(This,pHints)	\
    ( (This)->lpVtbl -> ApplyHints(This,pHints) ) 

#define IVdsLun_SetStatus(This,status)	\
    ( (This)->lpVtbl -> SetStatus(This,status) ) 

#define IVdsLun_QueryMaxLunExtendSize(This,pDriveIdArray,lNumberOfDrives,pullMaxBytesToBeAdded)	\
    ( (This)->lpVtbl -> QueryMaxLunExtendSize(This,pDriveIdArray,lNumberOfDrives,pullMaxBytesToBeAdded) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsLun_INTERFACE_DEFINED__ */


#ifndef __IVdsLun2_INTERFACE_DEFINED__
#define __IVdsLun2_INTERFACE_DEFINED__

/* interface IVdsLun2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsLun2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e5b3a735-9efb-499a-8071-4394d9ee6fcb")
    IVdsLun2 : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryHints2( 
            /* [out] */ __RPC__out VDS_HINTS2 *pHints2) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ApplyHints2( 
            /* [in] */ __RPC__in VDS_HINTS2 *pHints2) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsLun2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsLun2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsLun2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsLun2 * This);
        
        DECLSPEC_XFGVIRT(IVdsLun2, QueryHints2)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryHints2 )( 
            __RPC__in IVdsLun2 * This,
            /* [out] */ __RPC__out VDS_HINTS2 *pHints2);
        
        DECLSPEC_XFGVIRT(IVdsLun2, ApplyHints2)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ApplyHints2 )( 
            __RPC__in IVdsLun2 * This,
            /* [in] */ __RPC__in VDS_HINTS2 *pHints2);
        
        END_INTERFACE
    } IVdsLun2Vtbl;

    interface IVdsLun2
    {
        CONST_VTBL struct IVdsLun2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsLun2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsLun2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsLun2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsLun2_QueryHints2(This,pHints2)	\
    ( (This)->lpVtbl -> QueryHints2(This,pHints2) ) 

#define IVdsLun2_ApplyHints2(This,pHints2)	\
    ( (This)->lpVtbl -> ApplyHints2(This,pHints2) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsLun2_INTERFACE_DEFINED__ */


#ifndef __IVdsLunNaming_INTERFACE_DEFINED__
#define __IVdsLunNaming_INTERFACE_DEFINED__

/* interface IVdsLunNaming */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsLunNaming;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("907504cb-6b4e-4d88-a34d-17ba661fbb06")
    IVdsLunNaming : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetFriendlyName( 
            /* [string][in] */ __RPC__in_string LPWSTR pwszFriendlyName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsLunNamingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsLunNaming * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsLunNaming * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsLunNaming * This);
        
        DECLSPEC_XFGVIRT(IVdsLunNaming, SetFriendlyName)
        HRESULT ( STDMETHODCALLTYPE *SetFriendlyName )( 
            __RPC__in IVdsLunNaming * This,
            /* [string][in] */ __RPC__in_string LPWSTR pwszFriendlyName);
        
        END_INTERFACE
    } IVdsLunNamingVtbl;

    interface IVdsLunNaming
    {
        CONST_VTBL struct IVdsLunNamingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsLunNaming_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsLunNaming_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsLunNaming_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsLunNaming_SetFriendlyName(This,pwszFriendlyName)	\
    ( (This)->lpVtbl -> SetFriendlyName(This,pwszFriendlyName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsLunNaming_INTERFACE_DEFINED__ */


#ifndef __IVdsLunNumber_INTERFACE_DEFINED__
#define __IVdsLunNumber_INTERFACE_DEFINED__

/* interface IVdsLunNumber */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsLunNumber;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d3f95e46-54b3-41f9-b678-0f1871443a08")
    IVdsLunNumber : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetLunNumber( 
            /* [out] */ __RPC__out ULONG *pulLunNumber) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsLunNumberVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsLunNumber * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsLunNumber * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsLunNumber * This);
        
        DECLSPEC_XFGVIRT(IVdsLunNumber, GetLunNumber)
        HRESULT ( STDMETHODCALLTYPE *GetLunNumber )( 
            __RPC__in IVdsLunNumber * This,
            /* [out] */ __RPC__out ULONG *pulLunNumber);
        
        END_INTERFACE
    } IVdsLunNumberVtbl;

    interface IVdsLunNumber
    {
        CONST_VTBL struct IVdsLunNumberVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsLunNumber_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsLunNumber_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsLunNumber_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsLunNumber_GetLunNumber(This,pulLunNumber)	\
    ( (This)->lpVtbl -> GetLunNumber(This,pulLunNumber) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsLunNumber_INTERFACE_DEFINED__ */


#ifndef __IVdsLunControllerPorts_INTERFACE_DEFINED__
#define __IVdsLunControllerPorts_INTERFACE_DEFINED__

/* interface IVdsLunControllerPorts */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsLunControllerPorts;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("451fe266-da6d-406a-bb60-82e534f85aeb")
    IVdsLunControllerPorts : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AssociateControllerPorts( 
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(lNumberOfActiveControllerPorts) VDS_OBJECT_ID *pActiveControllerPortIdArray,
            /* [in] */ LONG lNumberOfActiveControllerPorts,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(lNumberOfInactiveControllerPorts) VDS_OBJECT_ID *pInactiveControllerPortIdArray,
            /* [in] */ LONG lNumberOfInactiveControllerPorts) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryActiveControllerPorts( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsLunControllerPortsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsLunControllerPorts * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsLunControllerPorts * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsLunControllerPorts * This);
        
        DECLSPEC_XFGVIRT(IVdsLunControllerPorts, AssociateControllerPorts)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AssociateControllerPorts )( 
            __RPC__in IVdsLunControllerPorts * This,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(lNumberOfActiveControllerPorts) VDS_OBJECT_ID *pActiveControllerPortIdArray,
            /* [in] */ LONG lNumberOfActiveControllerPorts,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(lNumberOfInactiveControllerPorts) VDS_OBJECT_ID *pInactiveControllerPortIdArray,
            /* [in] */ LONG lNumberOfInactiveControllerPorts);
        
        DECLSPEC_XFGVIRT(IVdsLunControllerPorts, QueryActiveControllerPorts)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryActiveControllerPorts )( 
            __RPC__in IVdsLunControllerPorts * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        END_INTERFACE
    } IVdsLunControllerPortsVtbl;

    interface IVdsLunControllerPorts
    {
        CONST_VTBL struct IVdsLunControllerPortsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsLunControllerPorts_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsLunControllerPorts_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsLunControllerPorts_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsLunControllerPorts_AssociateControllerPorts(This,pActiveControllerPortIdArray,lNumberOfActiveControllerPorts,pInactiveControllerPortIdArray,lNumberOfInactiveControllerPorts)	\
    ( (This)->lpVtbl -> AssociateControllerPorts(This,pActiveControllerPortIdArray,lNumberOfActiveControllerPorts,pInactiveControllerPortIdArray,lNumberOfInactiveControllerPorts) ) 

#define IVdsLunControllerPorts_QueryActiveControllerPorts(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryActiveControllerPorts(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsLunControllerPorts_INTERFACE_DEFINED__ */


#ifndef __IVdsLunMpio_INTERFACE_DEFINED__
#define __IVdsLunMpio_INTERFACE_DEFINED__

/* interface IVdsLunMpio */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsLunMpio;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7c5fbae3-333a-48a1-a982-33c15788cde3")
    IVdsLunMpio : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPathInfo( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*plNumberOfPaths) VDS_PATH_INFO **ppPaths,
            /* [out] */ __RPC__out LONG *plNumberOfPaths) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetLoadBalancePolicy( 
            /* [out] */ __RPC__out VDS_LOADBALANCE_POLICY_ENUM *pPolicy,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*plNumberOfPaths) VDS_PATH_POLICY **ppPaths,
            /* [out] */ __RPC__out LONG *plNumberOfPaths) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetLoadBalancePolicy( 
            /* [in] */ VDS_LOADBALANCE_POLICY_ENUM policy,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(lNumberOfPaths) VDS_PATH_POLICY *pPaths,
            /* [in] */ LONG lNumberOfPaths) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSupportedLbPolicies( 
            /* [out] */ __RPC__out ULONG *pulLbFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsLunMpioVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsLunMpio * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsLunMpio * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsLunMpio * This);
        
        DECLSPEC_XFGVIRT(IVdsLunMpio, GetPathInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPathInfo )( 
            __RPC__in IVdsLunMpio * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*plNumberOfPaths) VDS_PATH_INFO **ppPaths,
            /* [out] */ __RPC__out LONG *plNumberOfPaths);
        
        DECLSPEC_XFGVIRT(IVdsLunMpio, GetLoadBalancePolicy)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetLoadBalancePolicy )( 
            __RPC__in IVdsLunMpio * This,
            /* [out] */ __RPC__out VDS_LOADBALANCE_POLICY_ENUM *pPolicy,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*plNumberOfPaths) VDS_PATH_POLICY **ppPaths,
            /* [out] */ __RPC__out LONG *plNumberOfPaths);
        
        DECLSPEC_XFGVIRT(IVdsLunMpio, SetLoadBalancePolicy)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetLoadBalancePolicy )( 
            __RPC__in IVdsLunMpio * This,
            /* [in] */ VDS_LOADBALANCE_POLICY_ENUM policy,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(lNumberOfPaths) VDS_PATH_POLICY *pPaths,
            /* [in] */ LONG lNumberOfPaths);
        
        DECLSPEC_XFGVIRT(IVdsLunMpio, GetSupportedLbPolicies)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSupportedLbPolicies )( 
            __RPC__in IVdsLunMpio * This,
            /* [out] */ __RPC__out ULONG *pulLbFlags);
        
        END_INTERFACE
    } IVdsLunMpioVtbl;

    interface IVdsLunMpio
    {
        CONST_VTBL struct IVdsLunMpioVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsLunMpio_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsLunMpio_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsLunMpio_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsLunMpio_GetPathInfo(This,ppPaths,plNumberOfPaths)	\
    ( (This)->lpVtbl -> GetPathInfo(This,ppPaths,plNumberOfPaths) ) 

#define IVdsLunMpio_GetLoadBalancePolicy(This,pPolicy,ppPaths,plNumberOfPaths)	\
    ( (This)->lpVtbl -> GetLoadBalancePolicy(This,pPolicy,ppPaths,plNumberOfPaths) ) 

#define IVdsLunMpio_SetLoadBalancePolicy(This,policy,pPaths,lNumberOfPaths)	\
    ( (This)->lpVtbl -> SetLoadBalancePolicy(This,policy,pPaths,lNumberOfPaths) ) 

#define IVdsLunMpio_GetSupportedLbPolicies(This,pulLbFlags)	\
    ( (This)->lpVtbl -> GetSupportedLbPolicies(This,pulLbFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsLunMpio_INTERFACE_DEFINED__ */


#ifndef __IVdsLunIscsi_INTERFACE_DEFINED__
#define __IVdsLunIscsi_INTERFACE_DEFINED__

/* interface IVdsLunIscsi */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsLunIscsi;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0d7c1e64-b59b-45ae-b86a-2c2cc6a42067")
    IVdsLunIscsi : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AssociateTargets( 
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(lNumberOfTargets) VDS_OBJECT_ID *pTargetIdArray,
            /* [in] */ LONG lNumberOfTargets) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryAssociatedTargets( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsLunIscsiVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsLunIscsi * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsLunIscsi * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsLunIscsi * This);
        
        DECLSPEC_XFGVIRT(IVdsLunIscsi, AssociateTargets)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AssociateTargets )( 
            __RPC__in IVdsLunIscsi * This,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(lNumberOfTargets) VDS_OBJECT_ID *pTargetIdArray,
            /* [in] */ LONG lNumberOfTargets);
        
        DECLSPEC_XFGVIRT(IVdsLunIscsi, QueryAssociatedTargets)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryAssociatedTargets )( 
            __RPC__in IVdsLunIscsi * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        END_INTERFACE
    } IVdsLunIscsiVtbl;

    interface IVdsLunIscsi
    {
        CONST_VTBL struct IVdsLunIscsiVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsLunIscsi_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsLunIscsi_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsLunIscsi_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsLunIscsi_AssociateTargets(This,pTargetIdArray,lNumberOfTargets)	\
    ( (This)->lpVtbl -> AssociateTargets(This,pTargetIdArray,lNumberOfTargets) ) 

#define IVdsLunIscsi_QueryAssociatedTargets(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryAssociatedTargets(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsLunIscsi_INTERFACE_DEFINED__ */


#ifndef __IVdsLunPlex_INTERFACE_DEFINED__
#define __IVdsLunPlex_INTERFACE_DEFINED__

/* interface IVdsLunPlex */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsLunPlex;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0ee1a790-5d2e-4abb-8c99-c481e8be2138")
    IVdsLunPlex : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [out] */ __RPC__out VDS_LUN_PLEX_PROP *pPlexProp) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetLun( 
            /* [out] */ __RPC__deref_out_opt IVdsLun **ppLun) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryExtents( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*plNumberOfExtents) VDS_DRIVE_EXTENT **ppExtentArray,
            /* [out] */ __RPC__out LONG *plNumberOfExtents) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryHints( 
            /* [out] */ __RPC__out VDS_HINTS *pHints) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ApplyHints( 
            /* [in] */ __RPC__in VDS_HINTS *pHints) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsLunPlexVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsLunPlex * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsLunPlex * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsLunPlex * This);
        
        DECLSPEC_XFGVIRT(IVdsLunPlex, GetProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in IVdsLunPlex * This,
            /* [out] */ __RPC__out VDS_LUN_PLEX_PROP *pPlexProp);
        
        DECLSPEC_XFGVIRT(IVdsLunPlex, GetLun)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetLun )( 
            __RPC__in IVdsLunPlex * This,
            /* [out] */ __RPC__deref_out_opt IVdsLun **ppLun);
        
        DECLSPEC_XFGVIRT(IVdsLunPlex, QueryExtents)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryExtents )( 
            __RPC__in IVdsLunPlex * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*plNumberOfExtents) VDS_DRIVE_EXTENT **ppExtentArray,
            /* [out] */ __RPC__out LONG *plNumberOfExtents);
        
        DECLSPEC_XFGVIRT(IVdsLunPlex, QueryHints)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryHints )( 
            __RPC__in IVdsLunPlex * This,
            /* [out] */ __RPC__out VDS_HINTS *pHints);
        
        DECLSPEC_XFGVIRT(IVdsLunPlex, ApplyHints)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ApplyHints )( 
            __RPC__in IVdsLunPlex * This,
            /* [in] */ __RPC__in VDS_HINTS *pHints);
        
        END_INTERFACE
    } IVdsLunPlexVtbl;

    interface IVdsLunPlex
    {
        CONST_VTBL struct IVdsLunPlexVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsLunPlex_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsLunPlex_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsLunPlex_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsLunPlex_GetProperties(This,pPlexProp)	\
    ( (This)->lpVtbl -> GetProperties(This,pPlexProp) ) 

#define IVdsLunPlex_GetLun(This,ppLun)	\
    ( (This)->lpVtbl -> GetLun(This,ppLun) ) 

#define IVdsLunPlex_QueryExtents(This,ppExtentArray,plNumberOfExtents)	\
    ( (This)->lpVtbl -> QueryExtents(This,ppExtentArray,plNumberOfExtents) ) 

#define IVdsLunPlex_QueryHints(This,pHints)	\
    ( (This)->lpVtbl -> QueryHints(This,pHints) ) 

#define IVdsLunPlex_ApplyHints(This,pHints)	\
    ( (This)->lpVtbl -> ApplyHints(This,pHints) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsLunPlex_INTERFACE_DEFINED__ */


#ifndef __IVdsIscsiPortal_INTERFACE_DEFINED__
#define __IVdsIscsiPortal_INTERFACE_DEFINED__

/* interface IVdsIscsiPortal */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsIscsiPortal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7fa1499d-ec85-4a8a-a47b-ff69201fcd34")
    IVdsIscsiPortal : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [out] */ __RPC__out VDS_ISCSI_PORTAL_PROP *pPortalProp) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSubSystem( 
            /* [out] */ __RPC__deref_out_opt IVdsSubSystem **ppSubSystem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryAssociatedPortalGroups( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetStatus( 
            /* [in] */ VDS_ISCSI_PORTAL_STATUS status) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetIpsecTunnelAddress( 
            /* [in] */ __RPC__in VDS_IPADDRESS *pTunnelAddress,
            /* [in] */ __RPC__in VDS_IPADDRESS *pDestinationAddress) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetIpsecSecurity( 
            /* [in] */ __RPC__in VDS_IPADDRESS *pInitiatorPortalAddress,
            /* [out] */ __RPC__out ULONGLONG *pullSecurityFlags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetIpsecSecurity( 
            /* [in] */ __RPC__in VDS_IPADDRESS *pInitiatorPortalAddress,
            /* [in] */ ULONGLONG ullSecurityFlags,
            /* [unique][in] */ __RPC__in_opt VDS_ISCSI_IPSEC_KEY *pIpsecKey) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsIscsiPortalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsIscsiPortal * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsIscsiPortal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsIscsiPortal * This);
        
        DECLSPEC_XFGVIRT(IVdsIscsiPortal, GetProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in IVdsIscsiPortal * This,
            /* [out] */ __RPC__out VDS_ISCSI_PORTAL_PROP *pPortalProp);
        
        DECLSPEC_XFGVIRT(IVdsIscsiPortal, GetSubSystem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSubSystem )( 
            __RPC__in IVdsIscsiPortal * This,
            /* [out] */ __RPC__deref_out_opt IVdsSubSystem **ppSubSystem);
        
        DECLSPEC_XFGVIRT(IVdsIscsiPortal, QueryAssociatedPortalGroups)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryAssociatedPortalGroups )( 
            __RPC__in IVdsIscsiPortal * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVdsIscsiPortal, SetStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetStatus )( 
            __RPC__in IVdsIscsiPortal * This,
            /* [in] */ VDS_ISCSI_PORTAL_STATUS status);
        
        DECLSPEC_XFGVIRT(IVdsIscsiPortal, SetIpsecTunnelAddress)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetIpsecTunnelAddress )( 
            __RPC__in IVdsIscsiPortal * This,
            /* [in] */ __RPC__in VDS_IPADDRESS *pTunnelAddress,
            /* [in] */ __RPC__in VDS_IPADDRESS *pDestinationAddress);
        
        DECLSPEC_XFGVIRT(IVdsIscsiPortal, GetIpsecSecurity)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetIpsecSecurity )( 
            __RPC__in IVdsIscsiPortal * This,
            /* [in] */ __RPC__in VDS_IPADDRESS *pInitiatorPortalAddress,
            /* [out] */ __RPC__out ULONGLONG *pullSecurityFlags);
        
        DECLSPEC_XFGVIRT(IVdsIscsiPortal, SetIpsecSecurity)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetIpsecSecurity )( 
            __RPC__in IVdsIscsiPortal * This,
            /* [in] */ __RPC__in VDS_IPADDRESS *pInitiatorPortalAddress,
            /* [in] */ ULONGLONG ullSecurityFlags,
            /* [unique][in] */ __RPC__in_opt VDS_ISCSI_IPSEC_KEY *pIpsecKey);
        
        END_INTERFACE
    } IVdsIscsiPortalVtbl;

    interface IVdsIscsiPortal
    {
        CONST_VTBL struct IVdsIscsiPortalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsIscsiPortal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsIscsiPortal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsIscsiPortal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsIscsiPortal_GetProperties(This,pPortalProp)	\
    ( (This)->lpVtbl -> GetProperties(This,pPortalProp) ) 

#define IVdsIscsiPortal_GetSubSystem(This,ppSubSystem)	\
    ( (This)->lpVtbl -> GetSubSystem(This,ppSubSystem) ) 

#define IVdsIscsiPortal_QueryAssociatedPortalGroups(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryAssociatedPortalGroups(This,ppEnum) ) 

#define IVdsIscsiPortal_SetStatus(This,status)	\
    ( (This)->lpVtbl -> SetStatus(This,status) ) 

#define IVdsIscsiPortal_SetIpsecTunnelAddress(This,pTunnelAddress,pDestinationAddress)	\
    ( (This)->lpVtbl -> SetIpsecTunnelAddress(This,pTunnelAddress,pDestinationAddress) ) 

#define IVdsIscsiPortal_GetIpsecSecurity(This,pInitiatorPortalAddress,pullSecurityFlags)	\
    ( (This)->lpVtbl -> GetIpsecSecurity(This,pInitiatorPortalAddress,pullSecurityFlags) ) 

#define IVdsIscsiPortal_SetIpsecSecurity(This,pInitiatorPortalAddress,ullSecurityFlags,pIpsecKey)	\
    ( (This)->lpVtbl -> SetIpsecSecurity(This,pInitiatorPortalAddress,ullSecurityFlags,pIpsecKey) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsIscsiPortal_INTERFACE_DEFINED__ */


#ifndef __IVdsIscsiTarget_INTERFACE_DEFINED__
#define __IVdsIscsiTarget_INTERFACE_DEFINED__

/* interface IVdsIscsiTarget */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsIscsiTarget;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aa8f5055-83e5-4bcc-aa73-19851a36a849")
    IVdsIscsiTarget : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [out] */ __RPC__out VDS_ISCSI_TARGET_PROP *pTargetProp) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSubSystem( 
            /* [out] */ __RPC__deref_out_opt IVdsSubSystem **ppSubSystem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryPortalGroups( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryAssociatedLuns( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreatePortalGroup( 
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetFriendlyName( 
            /* [string][in] */ __RPC__in_string LPWSTR pwszFriendlyName) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetSharedSecret( 
            /* [unique][in] */ __RPC__in_opt VDS_ISCSI_SHARED_SECRET *pTargetSharedSecret,
            /* [string][unique][in] */ __RPC__in_opt_string LPWSTR pwszInitiatorName) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RememberInitiatorSharedSecret( 
            /* [string][in] */ __RPC__in_string LPWSTR pwszInitiatorName,
            /* [unique][in] */ __RPC__in_opt VDS_ISCSI_SHARED_SECRET *pInitiatorSharedSecret) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetConnectedInitiators( 
            /* [size_is][size_is][string][out] */ __RPC__deref_out_ecount_full_opt_string(*plNumberOfInitiators) LPWSTR **pppwszInitiatorList,
            /* [out] */ __RPC__out LONG *plNumberOfInitiators) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsIscsiTargetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsIscsiTarget * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsIscsiTarget * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsIscsiTarget * This);
        
        DECLSPEC_XFGVIRT(IVdsIscsiTarget, GetProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in IVdsIscsiTarget * This,
            /* [out] */ __RPC__out VDS_ISCSI_TARGET_PROP *pTargetProp);
        
        DECLSPEC_XFGVIRT(IVdsIscsiTarget, GetSubSystem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSubSystem )( 
            __RPC__in IVdsIscsiTarget * This,
            /* [out] */ __RPC__deref_out_opt IVdsSubSystem **ppSubSystem);
        
        DECLSPEC_XFGVIRT(IVdsIscsiTarget, QueryPortalGroups)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryPortalGroups )( 
            __RPC__in IVdsIscsiTarget * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVdsIscsiTarget, QueryAssociatedLuns)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryAssociatedLuns )( 
            __RPC__in IVdsIscsiTarget * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVdsIscsiTarget, CreatePortalGroup)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreatePortalGroup )( 
            __RPC__in IVdsIscsiTarget * This,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync);
        
        DECLSPEC_XFGVIRT(IVdsIscsiTarget, Delete)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IVdsIscsiTarget * This,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync);
        
        DECLSPEC_XFGVIRT(IVdsIscsiTarget, SetFriendlyName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetFriendlyName )( 
            __RPC__in IVdsIscsiTarget * This,
            /* [string][in] */ __RPC__in_string LPWSTR pwszFriendlyName);
        
        DECLSPEC_XFGVIRT(IVdsIscsiTarget, SetSharedSecret)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetSharedSecret )( 
            __RPC__in IVdsIscsiTarget * This,
            /* [unique][in] */ __RPC__in_opt VDS_ISCSI_SHARED_SECRET *pTargetSharedSecret,
            /* [string][unique][in] */ __RPC__in_opt_string LPWSTR pwszInitiatorName);
        
        DECLSPEC_XFGVIRT(IVdsIscsiTarget, RememberInitiatorSharedSecret)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RememberInitiatorSharedSecret )( 
            __RPC__in IVdsIscsiTarget * This,
            /* [string][in] */ __RPC__in_string LPWSTR pwszInitiatorName,
            /* [unique][in] */ __RPC__in_opt VDS_ISCSI_SHARED_SECRET *pInitiatorSharedSecret);
        
        DECLSPEC_XFGVIRT(IVdsIscsiTarget, GetConnectedInitiators)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetConnectedInitiators )( 
            __RPC__in IVdsIscsiTarget * This,
            /* [size_is][size_is][string][out] */ __RPC__deref_out_ecount_full_opt_string(*plNumberOfInitiators) LPWSTR **pppwszInitiatorList,
            /* [out] */ __RPC__out LONG *plNumberOfInitiators);
        
        END_INTERFACE
    } IVdsIscsiTargetVtbl;

    interface IVdsIscsiTarget
    {
        CONST_VTBL struct IVdsIscsiTargetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsIscsiTarget_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsIscsiTarget_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsIscsiTarget_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsIscsiTarget_GetProperties(This,pTargetProp)	\
    ( (This)->lpVtbl -> GetProperties(This,pTargetProp) ) 

#define IVdsIscsiTarget_GetSubSystem(This,ppSubSystem)	\
    ( (This)->lpVtbl -> GetSubSystem(This,ppSubSystem) ) 

#define IVdsIscsiTarget_QueryPortalGroups(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryPortalGroups(This,ppEnum) ) 

#define IVdsIscsiTarget_QueryAssociatedLuns(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryAssociatedLuns(This,ppEnum) ) 

#define IVdsIscsiTarget_CreatePortalGroup(This,ppAsync)	\
    ( (This)->lpVtbl -> CreatePortalGroup(This,ppAsync) ) 

#define IVdsIscsiTarget_Delete(This,ppAsync)	\
    ( (This)->lpVtbl -> Delete(This,ppAsync) ) 

#define IVdsIscsiTarget_SetFriendlyName(This,pwszFriendlyName)	\
    ( (This)->lpVtbl -> SetFriendlyName(This,pwszFriendlyName) ) 

#define IVdsIscsiTarget_SetSharedSecret(This,pTargetSharedSecret,pwszInitiatorName)	\
    ( (This)->lpVtbl -> SetSharedSecret(This,pTargetSharedSecret,pwszInitiatorName) ) 

#define IVdsIscsiTarget_RememberInitiatorSharedSecret(This,pwszInitiatorName,pInitiatorSharedSecret)	\
    ( (This)->lpVtbl -> RememberInitiatorSharedSecret(This,pwszInitiatorName,pInitiatorSharedSecret) ) 

#define IVdsIscsiTarget_GetConnectedInitiators(This,pppwszInitiatorList,plNumberOfInitiators)	\
    ( (This)->lpVtbl -> GetConnectedInitiators(This,pppwszInitiatorList,plNumberOfInitiators) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsIscsiTarget_INTERFACE_DEFINED__ */


#ifndef __IVdsIscsiPortalGroup_INTERFACE_DEFINED__
#define __IVdsIscsiPortalGroup_INTERFACE_DEFINED__

/* interface IVdsIscsiPortalGroup */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsIscsiPortalGroup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fef5f89d-a3dd-4b36-bf28-e7dde045c593")
    IVdsIscsiPortalGroup : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [out] */ __RPC__out VDS_ISCSI_PORTALGROUP_PROP *pPortalGroupProp) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetTarget( 
            /* [out] */ __RPC__deref_out_opt IVdsIscsiTarget **ppTarget) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryAssociatedPortals( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddPortal( 
            /* [in] */ VDS_OBJECT_ID portalId,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemovePortal( 
            /* [in] */ VDS_OBJECT_ID portalId,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsIscsiPortalGroupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsIscsiPortalGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsIscsiPortalGroup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsIscsiPortalGroup * This);
        
        DECLSPEC_XFGVIRT(IVdsIscsiPortalGroup, GetProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in IVdsIscsiPortalGroup * This,
            /* [out] */ __RPC__out VDS_ISCSI_PORTALGROUP_PROP *pPortalGroupProp);
        
        DECLSPEC_XFGVIRT(IVdsIscsiPortalGroup, GetTarget)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTarget )( 
            __RPC__in IVdsIscsiPortalGroup * This,
            /* [out] */ __RPC__deref_out_opt IVdsIscsiTarget **ppTarget);
        
        DECLSPEC_XFGVIRT(IVdsIscsiPortalGroup, QueryAssociatedPortals)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryAssociatedPortals )( 
            __RPC__in IVdsIscsiPortalGroup * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVdsIscsiPortalGroup, AddPortal)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddPortal )( 
            __RPC__in IVdsIscsiPortalGroup * This,
            /* [in] */ VDS_OBJECT_ID portalId,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync);
        
        DECLSPEC_XFGVIRT(IVdsIscsiPortalGroup, RemovePortal)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemovePortal )( 
            __RPC__in IVdsIscsiPortalGroup * This,
            /* [in] */ VDS_OBJECT_ID portalId,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync);
        
        DECLSPEC_XFGVIRT(IVdsIscsiPortalGroup, Delete)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IVdsIscsiPortalGroup * This,
            /* [out] */ __RPC__deref_out_opt IVdsAsync **ppAsync);
        
        END_INTERFACE
    } IVdsIscsiPortalGroupVtbl;

    interface IVdsIscsiPortalGroup
    {
        CONST_VTBL struct IVdsIscsiPortalGroupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsIscsiPortalGroup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsIscsiPortalGroup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsIscsiPortalGroup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsIscsiPortalGroup_GetProperties(This,pPortalGroupProp)	\
    ( (This)->lpVtbl -> GetProperties(This,pPortalGroupProp) ) 

#define IVdsIscsiPortalGroup_GetTarget(This,ppTarget)	\
    ( (This)->lpVtbl -> GetTarget(This,ppTarget) ) 

#define IVdsIscsiPortalGroup_QueryAssociatedPortals(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryAssociatedPortals(This,ppEnum) ) 

#define IVdsIscsiPortalGroup_AddPortal(This,portalId,ppAsync)	\
    ( (This)->lpVtbl -> AddPortal(This,portalId,ppAsync) ) 

#define IVdsIscsiPortalGroup_RemovePortal(This,portalId,ppAsync)	\
    ( (This)->lpVtbl -> RemovePortal(This,portalId,ppAsync) ) 

#define IVdsIscsiPortalGroup_Delete(This,ppAsync)	\
    ( (This)->lpVtbl -> Delete(This,ppAsync) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsIscsiPortalGroup_INTERFACE_DEFINED__ */


#ifndef __IVdsStoragePool_INTERFACE_DEFINED__
#define __IVdsStoragePool_INTERFACE_DEFINED__

/* interface IVdsStoragePool */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsStoragePool;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("932ca8cf-0eb3-4ba8-9620-22665d7f8450")
    IVdsStoragePool : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProvider( 
            /* [out] */ __RPC__deref_out_opt IVdsProvider **ppProvider) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [out] */ __RPC__out VDS_STORAGE_POOL_PROP *pStoragePoolProp) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetAttributes( 
            /* [out] */ __RPC__out VDS_POOL_ATTRIBUTES *pStoragePoolAttributes) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryDriveExtents( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*plNumberOfExtents) VDS_STORAGE_POOL_DRIVE_EXTENT **ppExtentArray,
            /* [out] */ __RPC__out LONG *plNumberOfExtents) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryAllocatedLuns( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryAllocatedStoragePools( 
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsStoragePoolVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsStoragePool * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsStoragePool * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsStoragePool * This);
        
        DECLSPEC_XFGVIRT(IVdsStoragePool, GetProvider)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProvider )( 
            __RPC__in IVdsStoragePool * This,
            /* [out] */ __RPC__deref_out_opt IVdsProvider **ppProvider);
        
        DECLSPEC_XFGVIRT(IVdsStoragePool, GetProperties)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in IVdsStoragePool * This,
            /* [out] */ __RPC__out VDS_STORAGE_POOL_PROP *pStoragePoolProp);
        
        DECLSPEC_XFGVIRT(IVdsStoragePool, GetAttributes)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetAttributes )( 
            __RPC__in IVdsStoragePool * This,
            /* [out] */ __RPC__out VDS_POOL_ATTRIBUTES *pStoragePoolAttributes);
        
        DECLSPEC_XFGVIRT(IVdsStoragePool, QueryDriveExtents)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryDriveExtents )( 
            __RPC__in IVdsStoragePool * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*plNumberOfExtents) VDS_STORAGE_POOL_DRIVE_EXTENT **ppExtentArray,
            /* [out] */ __RPC__out LONG *plNumberOfExtents);
        
        DECLSPEC_XFGVIRT(IVdsStoragePool, QueryAllocatedLuns)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryAllocatedLuns )( 
            __RPC__in IVdsStoragePool * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVdsStoragePool, QueryAllocatedStoragePools)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryAllocatedStoragePools )( 
            __RPC__in IVdsStoragePool * This,
            /* [out] */ __RPC__deref_out_opt IEnumVdsObject **ppEnum);
        
        END_INTERFACE
    } IVdsStoragePoolVtbl;

    interface IVdsStoragePool
    {
        CONST_VTBL struct IVdsStoragePoolVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsStoragePool_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsStoragePool_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsStoragePool_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsStoragePool_GetProvider(This,ppProvider)	\
    ( (This)->lpVtbl -> GetProvider(This,ppProvider) ) 

#define IVdsStoragePool_GetProperties(This,pStoragePoolProp)	\
    ( (This)->lpVtbl -> GetProperties(This,pStoragePoolProp) ) 

#define IVdsStoragePool_GetAttributes(This,pStoragePoolAttributes)	\
    ( (This)->lpVtbl -> GetAttributes(This,pStoragePoolAttributes) ) 

#define IVdsStoragePool_QueryDriveExtents(This,ppExtentArray,plNumberOfExtents)	\
    ( (This)->lpVtbl -> QueryDriveExtents(This,ppExtentArray,plNumberOfExtents) ) 

#define IVdsStoragePool_QueryAllocatedLuns(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryAllocatedLuns(This,ppEnum) ) 

#define IVdsStoragePool_QueryAllocatedStoragePools(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryAllocatedStoragePools(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsStoragePool_INTERFACE_DEFINED__ */


#ifndef __IVdsMaintenance_INTERFACE_DEFINED__
#define __IVdsMaintenance_INTERFACE_DEFINED__

/* interface IVdsMaintenance */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsMaintenance;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("daebeef3-8523-47ed-a2b9-05cecce2a1ae")
    IVdsMaintenance : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE StartMaintenance( 
            /* [in] */ VDS_MAINTENANCE_OPERATION operation) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE StopMaintenance( 
            /* [in] */ VDS_MAINTENANCE_OPERATION operation) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE PulseMaintenance( 
            /* [in] */ VDS_MAINTENANCE_OPERATION operation,
            /* [in] */ ULONG ulCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsMaintenanceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsMaintenance * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsMaintenance * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsMaintenance * This);
        
        DECLSPEC_XFGVIRT(IVdsMaintenance, StartMaintenance)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *StartMaintenance )( 
            __RPC__in IVdsMaintenance * This,
            /* [in] */ VDS_MAINTENANCE_OPERATION operation);
        
        DECLSPEC_XFGVIRT(IVdsMaintenance, StopMaintenance)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *StopMaintenance )( 
            __RPC__in IVdsMaintenance * This,
            /* [in] */ VDS_MAINTENANCE_OPERATION operation);
        
        DECLSPEC_XFGVIRT(IVdsMaintenance, PulseMaintenance)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *PulseMaintenance )( 
            __RPC__in IVdsMaintenance * This,
            /* [in] */ VDS_MAINTENANCE_OPERATION operation,
            /* [in] */ ULONG ulCount);
        
        END_INTERFACE
    } IVdsMaintenanceVtbl;

    interface IVdsMaintenance
    {
        CONST_VTBL struct IVdsMaintenanceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsMaintenance_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsMaintenance_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsMaintenance_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsMaintenance_StartMaintenance(This,operation)	\
    ( (This)->lpVtbl -> StartMaintenance(This,operation) ) 

#define IVdsMaintenance_StopMaintenance(This,operation)	\
    ( (This)->lpVtbl -> StopMaintenance(This,operation) ) 

#define IVdsMaintenance_PulseMaintenance(This,operation,ulCount)	\
    ( (This)->lpVtbl -> PulseMaintenance(This,operation,ulCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsMaintenance_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_vdshwprv_0000_0033 */
/* [local] */ 

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)




extern RPC_IF_HANDLE __MIDL_itf_vdshwprv_0000_0033_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_vdshwprv_0000_0033_v0_0_s_ifspec;

#ifndef __IVdsHwProviderPrivate_INTERFACE_DEFINED__
#define __IVdsHwProviderPrivate_INTERFACE_DEFINED__

/* interface IVdsHwProviderPrivate */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsHwProviderPrivate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("98f17bf3-9f33-4f12-8714-8b4075092c2e")
    IVdsHwProviderPrivate : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryIfCreatedLun( 
            /* [string][in] */ __RPC__in_string LPWSTR pwszDevicePath,
            /* [in] */ __RPC__in VDS_LUN_INFORMATION *pVdsLunInformation,
            /* [out] */ __RPC__out VDS_OBJECT_ID *pLunId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsHwProviderPrivateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsHwProviderPrivate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsHwProviderPrivate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsHwProviderPrivate * This);
        
        DECLSPEC_XFGVIRT(IVdsHwProviderPrivate, QueryIfCreatedLun)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryIfCreatedLun )( 
            __RPC__in IVdsHwProviderPrivate * This,
            /* [string][in] */ __RPC__in_string LPWSTR pwszDevicePath,
            /* [in] */ __RPC__in VDS_LUN_INFORMATION *pVdsLunInformation,
            /* [out] */ __RPC__out VDS_OBJECT_ID *pLunId);
        
        END_INTERFACE
    } IVdsHwProviderPrivateVtbl;

    interface IVdsHwProviderPrivate
    {
        CONST_VTBL struct IVdsHwProviderPrivateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsHwProviderPrivate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsHwProviderPrivate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsHwProviderPrivate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsHwProviderPrivate_QueryIfCreatedLun(This,pwszDevicePath,pVdsLunInformation,pLunId)	\
    ( (This)->lpVtbl -> QueryIfCreatedLun(This,pwszDevicePath,pVdsLunInformation,pLunId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsHwProviderPrivate_INTERFACE_DEFINED__ */


#ifndef __IVdsHwProviderPrivateMpio_INTERFACE_DEFINED__
#define __IVdsHwProviderPrivateMpio_INTERFACE_DEFINED__

/* interface IVdsHwProviderPrivateMpio */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsHwProviderPrivateMpio;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("310a7715-ac2b-4c6f-9827-3d742f351676")
    IVdsHwProviderPrivateMpio : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetAllPathStatusesFromHbaPort( 
            /* [in] */ VDS_HBAPORT_PROP hbaPortProp,
            /* [in] */ VDS_PATH_STATUS status) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsHwProviderPrivateMpioVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsHwProviderPrivateMpio * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsHwProviderPrivateMpio * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsHwProviderPrivateMpio * This);
        
        DECLSPEC_XFGVIRT(IVdsHwProviderPrivateMpio, SetAllPathStatusesFromHbaPort)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetAllPathStatusesFromHbaPort )( 
            __RPC__in IVdsHwProviderPrivateMpio * This,
            /* [in] */ VDS_HBAPORT_PROP hbaPortProp,
            /* [in] */ VDS_PATH_STATUS status);
        
        END_INTERFACE
    } IVdsHwProviderPrivateMpioVtbl;

    interface IVdsHwProviderPrivateMpio
    {
        CONST_VTBL struct IVdsHwProviderPrivateMpioVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsHwProviderPrivateMpio_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsHwProviderPrivateMpio_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsHwProviderPrivateMpio_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsHwProviderPrivateMpio_SetAllPathStatusesFromHbaPort(This,hbaPortProp,status)	\
    ( (This)->lpVtbl -> SetAllPathStatusesFromHbaPort(This,hbaPortProp,status) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsHwProviderPrivateMpio_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_vdshwprv_0000_0035 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



extern RPC_IF_HANDLE __MIDL_itf_vdshwprv_0000_0035_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_vdshwprv_0000_0035_v0_0_s_ifspec;

#ifndef __IVdsAdmin_INTERFACE_DEFINED__
#define __IVdsAdmin_INTERFACE_DEFINED__

/* interface IVdsAdmin */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVdsAdmin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d188e97d-85aa-4d33-abc6-26299a10ffc1")
    IVdsAdmin : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RegisterProvider( 
            /* [in] */ VDS_OBJECT_ID providerId,
            /* [in] */ CLSID providerClsid,
            /* [string][max_is][in] */ __RPC__in_ecount_full_string(( 200 + 1 ) ) LPWSTR pwszName,
            /* [in] */ VDS_PROVIDER_TYPE type,
            /* [disable_consistency_check][string][max_is][unique][in] */ __RPC__in_ecount_full_opt_string(( ( 15 - 1 )  + 1 ) ) LPWSTR pwszMachineName,
            /* [string][max_is][in] */ __RPC__in_ecount_full_string(( 16 + 1 ) ) LPWSTR pwszVersion,
            /* [in] */ GUID guidVersionId) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnregisterProvider( 
            /* [in] */ VDS_OBJECT_ID providerId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVdsAdminVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVdsAdmin * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVdsAdmin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVdsAdmin * This);
        
        DECLSPEC_XFGVIRT(IVdsAdmin, RegisterProvider)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RegisterProvider )( 
            __RPC__in IVdsAdmin * This,
            /* [in] */ VDS_OBJECT_ID providerId,
            /* [in] */ CLSID providerClsid,
            /* [string][max_is][in] */ __RPC__in_ecount_full_string(( 200 + 1 ) ) LPWSTR pwszName,
            /* [in] */ VDS_PROVIDER_TYPE type,
            /* [disable_consistency_check][string][max_is][unique][in] */ __RPC__in_ecount_full_opt_string(( ( 15 - 1 )  + 1 ) ) LPWSTR pwszMachineName,
            /* [string][max_is][in] */ __RPC__in_ecount_full_string(( 16 + 1 ) ) LPWSTR pwszVersion,
            /* [in] */ GUID guidVersionId);
        
        DECLSPEC_XFGVIRT(IVdsAdmin, UnregisterProvider)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnregisterProvider )( 
            __RPC__in IVdsAdmin * This,
            /* [in] */ VDS_OBJECT_ID providerId);
        
        END_INTERFACE
    } IVdsAdminVtbl;

    interface IVdsAdmin
    {
        CONST_VTBL struct IVdsAdminVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVdsAdmin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVdsAdmin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVdsAdmin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVdsAdmin_RegisterProvider(This,providerId,providerClsid,pwszName,type,pwszMachineName,pwszVersion,guidVersionId)	\
    ( (This)->lpVtbl -> RegisterProvider(This,providerId,providerClsid,pwszName,type,pwszMachineName,pwszVersion,guidVersionId) ) 

#define IVdsAdmin_UnregisterProvider(This,providerId)	\
    ( (This)->lpVtbl -> UnregisterProvider(This,providerId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVdsAdmin_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_vdshwprv_0000_0036 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_vdshwprv_0000_0036_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_vdshwprv_0000_0036_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


