/*
 *  FullEnumSyncDeviceService.h 
 *
 *  Contains definitions for the Full Enumeration Sync Device Service 
 *
 *  Copyright (c) Microsoft Corporation, All Rights Reserved.
 *
 */


#ifndef _FULLENUMSYNCSERVICE_H_
#define _FULLENUMSYNCSERVICE_H_

#include <DeviceServices.h>

#include <SyncDeviceService.h>


/*****************************************************************************
    Full Enumeration Sync Service Info
******************************************************************************/

DEFINE_DEVSVCGUID(SERVICE_FullEnumSync,
     0x28d3aac9, 0xc075, 0x44be, 0x88, 0x81, 0x65, 0xf3, 0x8d, 0x30, 0x59, 0x09 );

#define NAME_FullEnumSyncSvc L"FullEnumSync"
#define TYPE_FullEnumSyncSvc DEVSVCTYPE_ABSTRACT


/*****************************************************************************
    Full Enumeration Sync Service Properties
******************************************************************************/

DEFINE_DEVSVCGUID(NAMESPACE_FullEnumSyncSvc,
     0x63b10e6c, 0x4f3a, 0x456d, 0x95, 0xcb, 0x98, 0x94, 0xed, 0xec, 0x9f, 0xa5 );


/*  PKEY_FullEnumSyncSvc_VersionProps
 *
 *  Provides information about change units and version properties. The
 *  format for the dataset is
 *  
 *  UINT32 Number of change units
 *  UINT128 Namespace GUID for first change unit property key
 *  UINT32 Namespace ID for the first change unit property key
 *  UINT32 Number of properties associated with this change unit
 *  UINT128 Namespace GUID for first property key in change unit
 *  UINT32 Namespace ID for first property key in change unit
 *  ... Repeat for number of property keys
 *  ... Repeat for number of change units
 *  
 *  NOTE: If all change units use the same property key specify a namespace
 *  GUID of GUID_NULL (all 0's) and a namespace ID of 0.
 *  
 *  Type: AUInt8
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_FullEnumSyncSvc_VersionProps,
     0x63b10e6c, 0x4f3a, 0x456d, 0x95, 0xcb, 0x98, 0x94, 0xed, 0xec, 0x9f, 0xa5 ,
     3 );

#define NAME_FullEnumSyncSvc_VersionProps L"FullEnumVersionProps"


/*  PKEY_FullEnumSyncSvc_ReplicaID
 *
 *  Contains the GUID representing this replica in the sync community.
 *  
 *  Type: UInt128
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_FullEnumSyncSvc_ReplicaID,
     0x63b10e6c, 0x4f3a, 0x456d, 0x95, 0xcb, 0x98, 0x94, 0xed, 0xec, 0x9f, 0xa5 ,
     4 );

#define NAME_FullEnumSyncSvc_ReplicaID L"FullEnumReplicaID"


/*  PKEY_FullEnumSyncSvc_KnowledgeObjectID
 *
 *  Object ID to be used for the knowledge object
 *  
 *  Type: UInt32
 *  Form: ObjectID
 */

DEFINE_DEVSVCPROPKEY(PKEY_FullEnumSyncSvc_KnowledgeObjectID,
     0x63b10e6c, 0x4f3a, 0x456d, 0x95, 0xcb, 0x98, 0x94, 0xed, 0xec, 0x9f, 0xa5 ,
     7 );

#define NAME_FullEnumSyncSvc_KnowledgeObjectID L"FullEnumKnowledgeObjectID"


/*  PKEY_FullEnumSyncSvc_LastSyncProxyID
 *
 *  Contains a GUID indicating the last sync proxy to perform a sync operation
 *  
 *  Type: UInt128
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_FullEnumSyncSvc_LastSyncProxyID,
     0x63b10e6c, 0x4f3a, 0x456d, 0x95, 0xcb, 0x98, 0x94, 0xed, 0xec, 0x9f, 0xa5 ,
     8 );

#define NAME_FullEnumSyncSvc_LastSyncProxyID L"FullEnumLastSyncProxyID"


/*  PKEY_FullEnumSyncSvc_ProviderVersion
 *
 *  Contains a device defined value giving the version of the provider
 *  currently in use on the device. This version must be incremented whenever
 *  new properties are added to the device implementation so that they will
 *  be recognized and managed as part of synchronization. 0 is reserved.
 *  
 *  Type: UInt16
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_FullEnumSyncSvc_ProviderVersion,
     0x63b10e6c, 0x4f3a, 0x456d, 0x95, 0xcb, 0x98, 0x94, 0xed, 0xec, 0x9f, 0xa5 ,
     9 );

#define NAME_FullEnumSyncSvc_ProviderVersion L"FullEnumProviderVersion"


/*  PKEY_FullEnumSyncSvc_SyncFormat
 *
 *  Indicates the format GUID for the object format that is to be used in the
 *  sync operation.
 *  
 *  Type: UInt128
 *  Form: None
 */

#define PKEY_FullEnumSyncSvc_SyncFormat PKEY_SyncSvc_SyncFormat
#define NAME_FullEnumSyncSvc_SyncFormat NAME_SyncSvc_SyncFormat

/*  PKEY_FullEnumSyncSvc_LocalOnlyDelete
 *
 *  Boolean flag indicating whether deletes of objects on the service host
 *  should be treated as "local only" and not propogated to other sync
 *  participants. The alternative is "true sync" in which deletes on the
 *  service host are propogated to all other sync participants.
 *  
 *  Type: UInt8
 *  Form: None
 */

#define PKEY_FullEnumSyncSvc_LocalOnlyDelete PKEY_SyncSvc_LocalOnlyDelete
#define NAME_FullEnumSyncSvc_LocalOnlyDelete NAME_SyncSvc_LocalOnlyDelete

/*  PKEY_FullEnumSyncSvc_FilterType
 *
 *  Type: UInt8
 *  Form: None
 */

#define PKEY_FullEnumSyncSvc_FilterType PKEY_SyncSvc_FilterType
#define NAME_FullEnumSyncSvc_FilterType NAME_SyncSvc_FilterType

/*****************************************************************************
    Full Enumeration Sync Service Object Formats
******************************************************************************/

/*  FORMAT_FullEnumSyncKnowledge
 *  
 *  Knowledge object format
 *  
 */

DEFINE_DEVSVCGUID(FORMAT_FullEnumSyncKnowledge,
     0x221bce32, 0x221b, 0x4f45, 0xb4, 0x8b, 0x80, 0xde, 0x9a, 0x93, 0xa4, 0x4a );

#define NAME_FullEnumSyncKnowledge L"FullEnumSyncKnowledge"



/*****************************************************************************
    Full Enumeration Sync Service Methods
******************************************************************************/


/*  Inherited methods */

#define METHOD_FullEnumSyncSvc_BeginSync METHOD_SyncSvc_BeginSync
#define NAME_FullEnumSyncSvc_BeginSync NAME_SyncSvc_BeginSync

#define METHOD_FullEnumSyncSvc_EndSync METHOD_SyncSvc_EndSync
#define NAME_FullEnumSyncSvc_EndSync NAME_SyncSvc_EndSync


#endif /*_FULLENUMSYNCSERVICE_H_*/
